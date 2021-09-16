use crate::ffi;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Default, Clone, Copy)]
pub struct Extent2D {
    width: u32,
    height: u32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Extent3D {
    width: u32,
    height: u32,
    depth: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    OutOfHostMemory,
    OutOfDeviceMemory,
    InitializationFailed,
    DeviceLost,
    MemoryMapFailed,
    LayerNotPresent,
    ExtensionNotPresent,
    FeatureNotPresent,
    IncompatibleDriver,
    TooManyObjects,
    FormatNotSupported,
    FragmentedPool,
    Unknown,
    OutOfPoolMemory,
    InvalidExternalHandle,
    Fragmentation,
    InvalidOpaqueCaptureAddress,
    SurfaceLostKhr,
    NativeWindowInUseKhr,
    OutOfDateKhr,
    IncompatibleDisplayKhr,
    ValidationFailedExt,
    InvalidShaderNv,
    InvalidDrmFormatModifierPlaneLayoutExt,
    NotPermittedExt,
    FullScreenExclusiveModeLostExt,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vulkan run-time error")
    }
}

impl std::error::Error for Error {}

impl Extent2D {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl From<Extent2D> for ffi::Extent2D {
    fn from(extent: Extent2D) -> Self {
        Self {
            width: extent.width,
            height: extent.height,
        }
    }
}

impl From<ffi::Extent2D> for Extent2D {
    fn from(extent: ffi::Extent2D) -> Self {
        Self {
            width: extent.width,
            height: extent.height,
        }
    }
}
