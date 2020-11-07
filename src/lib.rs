use thiserror::Error;
use winapi::_core::ptr::null_mut;
use winapi::shared::winerror;
use winapi::um::bluetoothapis::{BLUETOOTH_DEVICE_INFO, BLUETOOTH_DEVICE_SEARCH_PARAMS, BLUETOOTH_FIND_RADIO_PARAMS, BluetoothFindFirstDevice, BluetoothFindFirstRadio, BluetoothFindNextDevice, BluetoothFindNextRadio, BluetoothFindRadioClose, HBLUETOOTH_DEVICE_FIND, HBLUETOOTH_RADIO_FIND};
use winapi::um::errhandlingapi;
use winapi::um::handleapi::CloseHandle;
use winapi::um::minwinbase::SYSTEMTIME;
use winapi::um::winnt::HANDLE;

pub mod btwinapi;
