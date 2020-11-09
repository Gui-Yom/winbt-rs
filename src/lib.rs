use thiserror::Error;

#[cfg(feature = "winapi")]
pub mod btwinapi;
#[cfg(feature = "winapi")]
pub mod btlewinapi;
#[cfg(feature = "winrt")]
pub mod btwinrt;
#[cfg(feature = "winrt")]
pub mod btlewinrt;
