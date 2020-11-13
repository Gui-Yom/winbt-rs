use windows::devices::enumeration::*;
use windows::devices::bluetooth::*;
use windows::foundation::TypedEventHandler;

winrt::include_bindings!();

pub fn list_devices() -> winrt::Result<()> {
    let watcher = DeviceInformation::create_watcher_aqs_filter(BluetoothAdapter::get_device_selector().unwrap())?;
    let added = watcher.added(&TypedEventHandler::new(|sender: &DeviceWatcher, info: &DeviceInformation| {
        println!("added {:?} : {}, {}", info.kind().unwrap(), info.id().unwrap(), info.name().unwrap());
        Ok(())
    }))?;
    let updated = watcher.updated(&TypedEventHandler::new(|sender: &DeviceWatcher, info: &DeviceInformationUpdate| {
        println!("updated {:?} : {}", info.kind().unwrap(), info.id().unwrap());
        Ok(())
    }))?;
    let removed = watcher.removed(&TypedEventHandler::new(|sender: &DeviceWatcher, info: &DeviceInformationUpdate| {
        println!("removed {:?} : {}", info.kind().unwrap(), info.id().unwrap());
        Ok(())
    }))?;
    watcher.start()?;
    std::thread::sleep(std::time::Duration::from_millis(2000));
    watcher.remove_added(added);
    watcher.remove_updated(updated);
    watcher.remove_removed(removed);
    watcher.stop()?;
    Ok(())
}
