use std::process::exit;
use std::str::FromStr;
use std::thread::sleep;
use std::time::{Duration, Instant};

use btleplug::api::{BDAddr, Central, CentralEvent, CharPropFlags, Peripheral};
use btleplug::winrtble::manager::Manager;

static _VENDOR: &str = "Makeblock";
static _NAME: &str = "Makeblock_LE001b1067c8af";
static ADDR: &str = "00:1B:10:67:C8:AF";
static SEARCH_TIME: u128 = 1500;

fn main() {
    let target = BDAddr::from_str(ADDR).unwrap();
    let mut found = false;

    let manager = Manager::new().unwrap();
    // get the first bluetooth adapter
    // connect to the adapter
    let adapter = {
        let adapters = manager.adapters().unwrap();
        println!("Detected {} adapters.", adapters.len());
        adapters.into_iter().nth(0).expect("Can't find a ble adapter !")
    };

    adapter.start_scan().unwrap();

    let receiver = adapter.event_receiver().unwrap();
    let time = std::time::Instant::now();
    while (Instant::now() - time).as_millis() < SEARCH_TIME {
        receiver.try_recv().and_then(|ev| match ev {
            CentralEvent::DeviceConnected(addr) => {
                println!("Device connected : {}", addr);
                Ok(())
            }
            CentralEvent::DeviceDisconnected(addr) => {
                println!("Device disconnected : {}", addr);
                Ok(())
            }
            CentralEvent::DeviceDiscovered(addr) => {
                println!("Device discovered : {}", addr);
                if target == addr {
                    found = true;
                }
                Ok(())
            }
            CentralEvent::DeviceLost(addr) => {
                println!("Device lost : {}", addr);
                Ok(())
            }
            CentralEvent::DeviceUpdated(addr) => {
                println!("Device updated : {}", addr);
                Ok(())
            }
        }).ok();
        sleep(Duration::from_millis(100))
    }

    adapter.stop_scan().unwrap();
    println!();
    println!();
    if !found {
        println!("Device not found ... (sry)");
        exit(0);
    }

    let device = adapter.peripheral(target).unwrap();
    device.connect().unwrap();
    device.discover_characteristics().unwrap().iter().for_each(|it| println!("{}", it.uuid));
    device.disconnect();
}
