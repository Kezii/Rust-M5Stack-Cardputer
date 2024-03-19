use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
};

use crate::display_driver::FramebufferTarget;
use display_interface_spi::SPIInterface;
use embedded_gfx::framebuffer::DmaReadyFramebuffer;
use esp_idf_hal::{
    gpio::{Output, OutputPin, PinDriver},
    spi::{SpiDeviceDriver, SpiDriver},
    task::thread::ThreadSpawnConfiguration,
};
use log::info;

use crate::display_driver;

pub struct DoubleBuffer<const W: usize, const H: usize> {
    sender: Option<std::sync::mpsc::Sender<usize>>,
    toggle: bool,
    fbuf0: DmaReadyFramebuffer<W, H>,
    fbuf1: DmaReadyFramebuffer<W, H>,
    mutex: Arc<Mutex<bool>>,
}

impl<const W: usize, const H: usize> DoubleBuffer<W, H> {
    pub fn new(raw_framebuffer_0: *mut c_void, raw_framebuffer_1: *mut c_void) -> Self {
        let fbuf0 = DmaReadyFramebuffer::<W, H>::new(raw_framebuffer_0, true);
        let fbuf1 = DmaReadyFramebuffer::<W, H>::new(raw_framebuffer_1, true);

        Self {
            sender: None,
            toggle: false,
            fbuf0,
            fbuf1,
            mutex: Arc::new(Mutex::new(true)),
        }
    }

    pub fn start_thread(
        &mut self,
        display: display_driver::ST7789<
            SPIInterface<
                SpiDeviceDriver<'static, SpiDriver<'static>>,
                PinDriver<'static, impl OutputPin + esp_idf_hal::gpio::Pin, Output>,
            >,
            esp_idf_hal::gpio::PinDriver<
                'static,
                impl OutputPin + esp_idf_hal::gpio::Pin + esp_idf_hal::gpio::Pin,
                esp_idf_hal::gpio::Output,
            >,
            esp_idf_hal::gpio::PinDriver<
                'static,
                impl OutputPin + esp_idf_hal::gpio::Pin,
                esp_idf_hal::gpio::Output,
            >,
        >,
    ) {
        info!("Starting fb writer thread");
        let (send, receive) = std::sync::mpsc::channel();

        self.sender = Some(send);

        let mutex2 = self.mutex.clone();
        let mut display = display;

        ThreadSpawnConfiguration {
            name: Some(b"fb writer\0"),
            pin_to_core: Some(esp_idf_svc::hal::cpu::Core::Core1),
            ..Default::default()
        }
        .set()
        .unwrap();

        std::thread::spawn(move || loop {
            let ptr = receive.recv().unwrap();
            unsafe {
                let _lock = mutex2.lock().unwrap();

                let ptr = ptr as *mut [u16; 240 * 135];
                let ptr = &mut *ptr;

                // takes about 12ms
                // 83 fps limit
                display.eat_framebuffer(ptr).unwrap();
                ptr.fill(0); // 2.2ms
            };
        });

        ThreadSpawnConfiguration::default().set().unwrap();
    }

    pub fn swap_framebuffer(&mut self) -> &mut DmaReadyFramebuffer<W, H> {
        self.toggle = !self.toggle;

        if self.toggle {
            &mut self.fbuf0
        } else {
            &mut self.fbuf1
        }
    }

    pub fn send_framebuffer(&mut self) {
        {
            let _lock = self.mutex.lock().unwrap();
            std::mem::drop(_lock);
        }

        let fbuf = if self.toggle {
            &mut self.fbuf0
        } else {
            &mut self.fbuf1
        };

        if let Some(sender) = &self.sender {
            sender.send(fbuf.framebuffer as usize).unwrap();
        }
    }
}
