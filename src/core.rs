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

#[derive(Debug, Default, Clone, Copy)]
pub struct Offset2D {
    x: i32,
    y: i32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Offset3D {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Rect2D {
    offset: Offset2D,
    extent: Extent2D,
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

impl From<(u32, u32)> for Extent2D {
    fn from((width, height): (u32, u32)) -> Self {
        Self { width, height }
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

impl Offset2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<(i32, i32)> for Offset2D {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<Offset2D> for ffi::Offset2D {
    fn from(offset: Offset2D) -> Self {
        Self {
            x: offset.x,
            y: offset.y,
        }
    }
}

impl From<ffi::Offset2D> for Offset2D {
    fn from(offset: ffi::Offset2D) -> Self {
        Self {
            x: offset.x,
            y: offset.y,
        }
    }
}

impl Rect2D {
    pub fn new(offset: Offset2D, extent: Extent2D) -> Self {
        Self { offset, extent }
    }
}

impl From<Rect2D> for ffi::Rect2D {
    fn from(rect: Rect2D) -> Self {
        Self {
            offset: rect.offset.into(),
            extent: rect.extent.into(),
        }
    }
}

impl From<ffi::Rect2D> for Rect2D {
    fn from(rect: ffi::Rect2D) -> Self {
        Self {
            offset: rect.offset.into(),
            extent: rect.extent.into(),
        }
    }
}
