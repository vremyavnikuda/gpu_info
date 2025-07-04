pub use crate::gpu_info::{GpuError, GpuInfo, Result};

pub mod gpu_info;
pub mod linux;
pub mod macos;
pub mod unknown;
pub mod vendor;
pub mod windows;

#[allow(missing_debug_implementations, missing_docs, unsafe_code)]
#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
pub mod linux;

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
pub mod imp;

#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
pub mod imp;

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
#[path = "unknown/mod.rs"]
pub mod imp;
pub mod test;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
/// Gets the `GpuInfo` of the primary GPU in the system.
///
/// If the system does not have a GPU, or the GPU is not supported, this returns
/// a default `GpuInfo` instance with unknown values.
///
/// # Linux and macOS
///
/// This function is supported on Linux and macOS.
///
/// # Windows
///
/// This function is supported on Windows.
///
/// # Other platforms
///
/// This function is not supported on other platforms.
pub fn get() -> GpuInfo {
    imp::info_gpu()
}
