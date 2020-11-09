use std::ffi::{OsStr, OsString};
use std::process::exit;
use std::str::FromStr;
use std::thread::sleep;
use std::time::{Duration, Instant};

use btleplug::api::{BDAddr, Central, CentralEvent, CharPropFlags, Peripheral};
use btleplug::winrtble::manager::Manager;

use btwin::btwinapi::select_device;

static _VENDOR: &str = "Makeblock";
static _NAME: &str = "Makeblock_LE001b1067c8af";
static ADDR: &str = "00:1B:10:67:C8:AF";
static SEARCH_TIME: u128 = 1500;

// TODO cli here

fn main() {
    {
        let device = select_device().unwrap();
        if device.is_some() {
            println!("{}", device.unwrap().name());
        }
    }
    exit(0);

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

    if adapter.start_scan().is_err() {
        println!("Please start your bluetooth adapter ! (in action center on Windows)");
        exit(-1);
    }

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
    println!("Device : {} | TX Power : {} dBm", device.address(), device.properties().tx_power_level.unwrap_or(-1));
    for it in device.discover_characteristics().unwrap() {
        print!("{}", it.uuid);
        let can_read = (it.properties & CharPropFlags::READ) == CharPropFlags::READ;
        let can_write = (it.properties & CharPropFlags::WRITE) == CharPropFlags::WRITE;
        print!("  {} ", String::with_capacity(2) + if can_read { "r" } else { "-" } + if can_write { "w" } else { "-" });
        if can_read {
            match device.read(&it) {
                Ok(bytes) => {
                    for byte in bytes {
                        print!(" {:02x}", byte);
                    }
                }
                Err(btleplug::Error::NotSupported(msg)) => { print!(" Can't read : {}", msg) }
                _ => print!("Another error")
            }
        }
        println!();
    };
    device.disconnect().expect("Error when disconnecting !");
}
