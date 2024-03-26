use cardputer::{
    hal::cardputer_peripherals,
    terminal::FbTerminal,
    typing::{KeyboardEvent, Typing},
    SCREEN_HEIGHT, SCREEN_WIDTH,
};

use esp_idf_hal::peripherals;
use log::info;

use esp_idf_svc::wifi::{AccessPointConfiguration, EspWifi};
use esp_idf_svc::{
    espnow::{EspNow, PeerInfo},
    eventloop::EspSystemEventLoop,
    wifi::{ClientConfiguration, Configuration},
};

#[allow(clippy::approx_constant)]
fn main() {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = peripherals::Peripherals::take().unwrap();

    let sysloop = EspSystemEventLoop::take().unwrap();

    let (mut display, mut keyboard, _) = cardputer_peripherals(
        peripherals.pins,
        peripherals.spi2,
        peripherals.ledc,
        peripherals.i2s0,
    );

    let mut raw_fb = Box::new([0u16; SCREEN_WIDTH * SCREEN_HEIGHT]);
    let mut terminal =
        FbTerminal::<SCREEN_WIDTH, SCREEN_HEIGHT>::new(raw_fb.as_mut_ptr(), &mut display);
    terminal.auto_draw(true);

    terminal.println("Espnow Remote");

    let mut wifi = EspWifi::new(peripherals.modem, sysloop.clone(), None).unwrap();

    let client_cfg = ClientConfiguration {
        channel: Some(0),
        ..Default::default()
    };

    let ap_cfg = AccessPointConfiguration {
        ssid: "esp32-remote".try_into().unwrap(),
        ..Default::default()
    };

    wifi.set_configuration(&Configuration::Mixed(client_cfg, ap_cfg))
        .unwrap();

    wifi.start().unwrap();

    terminal.println("Wifi started");
    terminal.println("Scanning...");

    let peer_address = loop {
        let peer_address = find_client(&mut wifi);

        if let Some(peer_address) = peer_address {
            break peer_address;
        }

        terminal.println("No peer found. Retrying...");
    };

    terminal.println(&format!("found peer: {:?}", peer_address));

    let espnow = EspNow::take().unwrap();

    espnow
        .register_send_cb(|_, _| {
            info!("send_cb");
        })
        .unwrap();

    let peer_info = PeerInfo {
        peer_addr: peer_address,
        channel: 0,
        ifidx: esp_idf_hal::sys::wifi_interface_t_WIFI_IF_AP,
        ..Default::default()
    };

    espnow.add_peer(peer_info).unwrap();

    let mut typing = Typing::new();

    terminal.println("Ready. Type to send");

    loop {
        let evt = keyboard.read_events();
        if let Some(evt) = evt {
            if let Some(KeyboardEvent::Ascii(c)) = typing.eat_keyboard_events(evt) {
                espnow.send(peer_address, &[c as u8]).unwrap();
            }
        }
    }
}

fn find_client(wifi: &mut EspWifi) -> Option<[u8; 6]> {
    let scan = wifi.scan().unwrap();

    for ap in scan {
        if ap.ssid == "esp32" {
            return Some(ap.bssid);
        }
    }

    None
}
