use core::{net::Ipv4Addr, str::FromStr};
use crate::mk_static;
use embassy_net::{
    Ipv4Cidr,
    Runner,
    Stack,
    StackResources,
    StaticConfigV4,
};

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use esp_hal::{gpio::Output, rng::Rng};
use esp_println::{println};
use esp_radio::wifi::{Interface, WifiController};


// const SSID: &str = "ReptilianNetworkX";
// const PASSWORD: &str = "EarthIsFlat";

const SSID: &str = "FUW.makerspace";
const PASSWORD: &str = "Pi=3,14159";

//#[embassy_executor::task]
pub async fn init(wifi_device: esp_hal::peripherals::WIFI<'static>, spawner: Spawner, shared: (Output<'static>,Output<'static>,Output<'static>, Output<'static>),hall_sensor: Output<'static>) -> (Stack<'static>, Stack<'static>)
{
    let access_point_config = esp_radio::wifi::ap::AccessPointConfig::default().with_ssid("radio");
    let station_config = esp_radio::wifi::sta::StationConfig::default().with_ssid(SSID).with_password(PASSWORD.into());

    let access_point_station_config = esp_radio::wifi::Config::AccessPointStation(station_config,access_point_config);

    println!("Starting wifi");
    let (wifi_controller, interfaces) =
        esp_radio::wifi::new(wifi_device, esp_radio::wifi::ControllerConfig::default().with_initial_config(access_point_station_config))
            .expect("Failed to initialize Wi-Fi controller");

    println!("Wifi started");
    
    let sta = interfaces.station;
    let ap = interfaces.access_point;


    let gw_ip_addr_str = "192.168.2.1";
    let gw_ip_addr = Ipv4Addr::from_str(gw_ip_addr_str).expect("failed to parse gateway ip");

    let ap_ip_config = embassy_net::Config::ipv4_static(StaticConfigV4 {
        address: Ipv4Cidr::new(gw_ip_addr, 24),
        gateway: Some(gw_ip_addr),
        dns_servers: Default::default(),
    });

    let sta_ip_config = embassy_net::Config::dhcpv4(Default::default());

    let rng = Rng::new();
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;


    let (ap_stack, ap_runner) = embassy_net::new(
        ap,
        ap_ip_config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
    );

    let (sta_stack, sta_runner) = embassy_net::new(
        sta,
        sta_ip_config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
    );

    spawner.spawn(connection(wifi_controller).unwrap());
    spawner.spawn(net_task(ap_runner).unwrap());
    spawner.spawn(net_task(sta_runner).unwrap());
    spawner.spawn(run_dhcp(ap_stack,gw_ip_addr_str).unwrap());

    

    ap_stack.wait_config_up().await;
    ap_stack
        .config_v4()
        .inspect(|c| println!("ipv4 config: {c:?}"));
    spawner.spawn(crate::web::setup_character_controller_server(shared,hall_sensor,sta_stack,spawner).unwrap());
    spawner.spawn(get_sta_ip(sta_stack).unwrap());
    return (ap_stack, sta_stack);
}
#[embassy_executor::task]
async fn get_sta_ip(sta_stack: Stack<'static>)
{
    let _sta_address = loop {
        if let Some(config) = sta_stack.config_v4() {
            let address = config.address.address();
            println!("Got IP: {}", address);
            break address;
        }
        println!("Waiting for IP...");
        Timer::after(Duration::from_millis(500)).await;
    };
}
#[embassy_executor::task]
async fn run_dhcp(stack: Stack<'static>, gw_ip_addr: &'static str)
{
    use core::net::{Ipv4Addr, SocketAddrV4};

    use edge_dhcp::{
        io::{self, DEFAULT_SERVER_PORT},
        server::{Server, ServerOptions},
    };
    use edge_nal::UdpBind;
    use edge_nal_embassy::{Udp, UdpBuffers};

    println!("Starting dhcp task");

    let ip = Ipv4Addr::from_str(gw_ip_addr).expect("dhcp task failed to parse gw ip");
    let mut buf = [0u8; 1500];

    let mut gw_buf = [Ipv4Addr::UNSPECIFIED];

    let buffers = UdpBuffers::<3, 1024, 1024, 10>::new();
    let unbound_socket = Udp::new(stack, &buffers);
    let mut bound_socket = unbound_socket
        .bind(core::net::SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::UNSPECIFIED,
            DEFAULT_SERVER_PORT,
        )))
        .await
        .unwrap();
    loop {
        _ = io::server::run(
            &mut Server::<_, 64>::new_with_et(ip),
            &ServerOptions::new(ip, Some(&mut gw_buf)),
            &mut bound_socket,
            &mut buf,
        )
        .await
        .inspect_err(|e| log::warn!("DHCP server error: {e:?}"));
        Timer::after(Duration::from_millis(500)).await;
    }

}
#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>)
{
    println!("Start connection task");
    loop{
        match controller.connect_async().await
        {

            Ok(_) => loop
            {

                let info = embassy_futures::select::select(
                    controller
                    .wait_for_access_point_connected_event_async(),
                    controller.wait_for_disconnect_async()
                ).await;



                match info {
                    embassy_futures::select::Either::Second(station_disconnected) => 
                    {
                        if let Ok(station_disconnected) = station_disconnected {
                            println!("Station disconnected: {:?}", station_disconnected);
                            break;
                        }
                    }
                    embassy_futures::select::Either::First(event) =>
                    {
                        if let Ok(event) = event
                        {
                            match event {
                                esp_radio::wifi::AccessPointStationEventInfo::Connected(info) => {
                                    println!("Station connected: {:?}", info);
                                }
                                esp_radio::wifi::AccessPointStationEventInfo::Disconnected(info) => {
                                    println!("Station disconnected: {:?}", info);
                                }
                            }

                        }
                    }
                }
            }
            Err(e) => 
            {
                println!("Failed to connect to wifi: {e:?}");
                Timer::after(Duration::from_millis(5000)).await
            }

        }
    }
}

#[embassy_executor::task(pool_size=2)]
async fn net_task(mut runner: Runner<'static, Interface<'static>>) {
    println!("Starting net task");
    runner.run().await
}


