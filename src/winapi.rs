use std::ffi::OsString;
use std::mem::size_of;

use thiserror::Error;
use winapi::_core::ptr::null_mut;
use winapi::shared::winerror;
use winapi::um::bluetoothapis::{
    BluetoothFindDeviceClose, BluetoothFindFirstDevice, BluetoothFindFirstRadio,
    BluetoothFindNextDevice, BluetoothFindNextRadio, BluetoothFindRadioClose,
    BluetoothSelectDevices, BluetoothSelectDevicesFree, BLUETOOTH_DEVICE_INFO,
    BLUETOOTH_DEVICE_SEARCH_PARAMS, BLUETOOTH_FIND_RADIO_PARAMS, BLUETOOTH_NULL_ADDRESS,
    BLUETOOTH_SELECT_DEVICE_PARAMS, HBLUETOOTH_DEVICE_FIND, HBLUETOOTH_RADIO_FIND,
};
use winapi::um::errhandlingapi;
use winapi::um::handleapi::CloseHandle;
use winapi::um::minwinbase::SYSTEMTIME;
use winapi::um::winnt::HANDLE;

use crate::utils::{FromWide, ToWide};

pub struct Radio {
    handle: HANDLE,
}

impl Radio {
    fn new(handle: HANDLE) -> Self {
        Radio { handle }
    }

    pub fn find() -> Result<Vec<Radio>> {
        let mut item: HANDLE = null_mut();
        let mut items: Vec<Radio> = Vec::new();
        unsafe {
            let find: HBLUETOOTH_RADIO_FIND =
                BluetoothFindFirstRadio(&BLUETOOTH_FIND_RADIO_PARAMS { dwSize: 4 }, &mut item);
            if find == null_mut() {
                let err = Error::from_code(errhandlingapi::GetLastError());
                return match err {
                    Error::NoMoreItems => Ok(vec![]),
                    err => Err(err),
                };
            }
            items.push(Radio::new(item));
            while BluetoothFindNextRadio(find, &mut item) > 0 {
                items.push(Radio::new(item))
            }
            BluetoothFindRadioClose(find);
        }
        Ok(items)
    }

    pub fn find_devices(&self) -> Result<Vec<BtDevice>> {
        let mut devices: Vec<BtDevice> = Vec::new();
        unsafe {
            let mut info: BLUETOOTH_DEVICE_INFO = new_device_info();
            let first: HBLUETOOTH_DEVICE_FIND = BluetoothFindFirstDevice(
                &BLUETOOTH_DEVICE_SEARCH_PARAMS {
                    dwSize: 6 * 4 + 1 + 4,
                    fReturnAuthenticated: 0,
                    fReturnRemembered: 0,
                    fReturnUnknown: 0,
                    fReturnConnected: 0,
                    fIssueInquiry: 0,
                    cTimeoutMultiplier: 1,
                    hRadio: self.handle,
                },
                &mut info,
            );
            if first == null_mut() {
                let err = Error::from_code(errhandlingapi::GetLastError());
                return match err {
                    Error::NoMoreItems => Ok(vec![]),
                    err => Err(err),
                };
            }
            devices.push(BtDevice::new(info));
            while BluetoothFindNextDevice(first, &mut info) > 0 {
                devices.push(BtDevice::new(info));
            }
            BluetoothFindDeviceClose(first);
        }
        Ok(devices)
    }
}

// So that the handle is automatically freed when the struct goes out of scope.
impl Drop for Radio {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}

pub struct BtDevice {
    pub info: BLUETOOTH_DEVICE_INFO,
}

impl BtDevice {
    fn new(info: BLUETOOTH_DEVICE_INFO) -> Self {
        BtDevice { info }
    }

    pub fn name(&self) -> String {
        OsString::from_wide(&self.info.szName)
            .into_string()
            .unwrap()
    }
}

#[cfg(target_os = "windows")]
pub fn select_device() -> Result<Option<BtDevice>> {
    let mut devices = vec![new_device_info()];
    /*
    let mut params = BLUETOOTH_SELECT_DEVICE_PARAMS::default();
    params.dwSize = size_of::<BLUETOOTH_SELECT_DEVICE_PARAMS>() as u32;
    params.pszInfo = OsString::from("Robot").to_wide().as_mut_ptr();
    params.fShowUnknown = 1;
    params.fSkipServicesPage = 1;
    params.pfnDeviceCallback = None;
    params.cNumDevices = 1;
    params.pDevices = devices.as_mut_ptr();
    */
    let mut params = BLUETOOTH_SELECT_DEVICE_PARAMS {
        dwSize: size_of::<BLUETOOTH_SELECT_DEVICE_PARAMS>() as u32,
        cNumOfClasses: 0,
        prgClassOfDevices: null_mut(),
        pszInfo: OsString::from("Robot").to_wide().as_mut_ptr(),
        hwndParent: null_mut(),
        fForceAuthentication: 0,
        fShowAuthenticated: 1,
        fShowRemembered: 1,
        fShowUnknown: 1,
        fAddNewDeviceWizard: 1,
        fSkipServicesPage: 1,
        pfnDeviceCallback: None,
        pvParam: null_mut(),
        cNumDevices: 1,
        pDevices: devices.as_mut_ptr(),
    };
    // TODO handle error

    unsafe {
        if BluetoothSelectDevices(&mut params) > 0 {
            let device = BtDevice::new(devices[0]);
            println!("{}", device.info.Address);
            BluetoothSelectDevicesFree(&mut params);
            return Ok(Some(device));
        }
    }
    print!("None");
    Ok(None)
}

fn new_device_info() -> BLUETOOTH_DEVICE_INFO {
    BLUETOOTH_DEVICE_INFO {
        dwSize: size_of::<BLUETOOTH_DEVICE_INFO>() as u32,
        Address: BLUETOOTH_NULL_ADDRESS,
        ulClassofDevice: 0,
        fConnected: 0,
        fRemembered: 0,
        fAuthenticated: 0,
        stLastSeen: SYSTEMTIME::default(),
        stLastUsed: SYSTEMTIME::default(),
        szName: [0u16; 248],
    }
}

// For Win32 errors
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
            _ => Error::Other(code),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
