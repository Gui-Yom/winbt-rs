use thiserror::Error;
use winapi::_core::ptr::null_mut;
use winapi::shared::winerror;
use winapi::um::bluetoothapis::{BLUETOOTH_DEVICE_INFO, BLUETOOTH_DEVICE_SEARCH_PARAMS, BLUETOOTH_FIND_RADIO_PARAMS, BluetoothFindFirstDevice, BluetoothFindFirstRadio, BluetoothFindNextDevice, BluetoothFindNextRadio, BluetoothFindRadioClose, HBLUETOOTH_DEVICE_FIND, HBLUETOOTH_RADIO_FIND};
use winapi::um::errhandlingapi;
use winapi::um::handleapi::CloseHandle;
use winapi::um::minwinbase::SYSTEMTIME;
use winapi::um::winnt::HANDLE;

pub fn find_radios() -> Result<Vec<HANDLE>> {
    let mut item: HANDLE = null_mut();
    let mut items: Vec<HANDLE> = Vec::new();
    unsafe {
        let find: HBLUETOOTH_RADIO_FIND = BluetoothFindFirstRadio(&BLUETOOTH_FIND_RADIO_PARAMS {
            dwSize: 4
        }, &mut item);
        if find == null_mut() {
            let err = Error::from_code(errhandlingapi::GetLastError());
            return match err {
                Error::NoMoreItems => Ok(vec![]),
                err => Err(err)
            };
        }
        items.push(item);
        while BluetoothFindNextRadio(find, &mut item) > 0 {
            items.push(item)
        }
        BluetoothFindRadioClose(find);
    }
    Ok(items)
}

pub fn close_radio(handle: HANDLE) {
    unsafe {
        CloseHandle(handle);
    }
}
/*
pub fn find_devices(radio: HANDLE) -> Result<Vec<(HANDLE, BLUETOOTH_DEVICE_INFO)>> {
    let mut devices: Vec<(HANDLE, BLUETOOTH_DEVICE_INFO)> = Vec::new();
    unsafe {
        let mut info: BLUETOOTH_DEVICE_INFO = BLUETOOTH_DEVICE_INFO {
            dwSize: 6 * 4 + 2 * std::mem::size_of::<SYSTEMTIME>() + 248 * 2,
            Address: 0,
            ulClassofDevice: 0,
            fConnected: 0,
            fRemembered: 0,
            fAuthenticated: 0,
            stLastSeen: std::mem::zeroed(),
            stLastUsed: std::mem::zeroed(),
            szName: [0u16; 248],
        };
        let first: HBLUETOOTH_DEVICE_FIND = BluetoothFindFirstDevice(&BLUETOOTH_DEVICE_SEARCH_PARAMS {
            dwSize: 6 * 4 + 1 + 4,
            fReturnAuthenticated: false,
            fReturnRemembered: false,
            fReturnUnknown: false,
            fReturnConnected: false,
            fIssueInquiry: false,
            cTimeoutMultiplier: 1,
            hRadio: radio,
        }, &mut info);
        if first == null_mut() {
            let err = Error::from_code(errhandlingapi::GetLastError());
            return match err {
                Error::NoMoreItems => Ok(vec![]),
                err => Err(err)
            };
        }
        devices.push((first, info));
        let device =
            while BluetoothFindNextDevice(find, &mut item) > 0 {
                devices.push(item)
            };
        BluetoothFindRadioClose(find);
    }
    Ok(devices)
}

 */

#[derive(Error, Debug)]
pub enum Error {
    #[error("ERROR_NO_MORE_ITEMS")]
    NoMoreItems,

    #[error("ERROR_INVALID_PARAMETER")]
    InvalidParameter,

    #[error("ERROR_REVISION_MISMATCH")]
    RevisionMismatch,

    #[error("ERROR_OUTOFMEMORY")]
    OutOfMemory,

    #[error("Unknown error : {0}")]
    Other(u32),
}

impl Error {
    fn from_code(code: u32) -> Self {
        match code {
            winerror::ERROR_NO_MORE_ITEMS => Error::NoMoreItems,
            winerror::ERROR_INVALID_PARAMETER => Error::InvalidParameter,
            winerror::ERROR_REVISION_MISMATCH => Error::RevisionMismatch,
            winerror::ERROR_OUTOFMEMORY => Error::OutOfMemory,
            _ => Error::Other(code)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
