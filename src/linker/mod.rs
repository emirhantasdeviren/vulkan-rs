#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::DynamicLibrary;
#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::DynamicLibrary;
