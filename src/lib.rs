use thiserror::Error;

#[cfg(feature = "winapi")]
pub mod btwinapi;
#[cfg(feature = "winapi-ble")]
pub mod btlewinapi;
#[cfg(feature = "winrt")]
pub mod btwinrt;
#[cfg(feature = "winrt-ble")]
pub mod btlewinrt;
