#![warn(missing_debug_implementations)]

use std::fmt;

use crate::core::Rect2D;
use crate::ffi;
use crate::format::Format;
use crate::shaders::ShaderModule;

pub type SampleMask = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontFace {
    CounterClockwise,
    Clockwise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexInputRate {
    Vertex,
    Instance,
}

/// Primitive topology describes how consecutive vertices are organized into primitives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
    TriangleFan,
    LineListWithAdjacency,
    LineStripWithAdjacency,
    TriangleListWithAdjacency,
    TriangleStripWithAdjacency,
    PatchList,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    Fill,
    Line,
    Point,
    FillRectangleNv,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicOp {
    Clear,
    And,
    AndReverse,
    Copy,
    AndInverted,
    NoOp,
    Xor,
    Or,
    Nor,
    Equivalent,
    Invert,
    OrReverse,
    CopyInverted,
    OrInverted,
    Nand,
    Set,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderStage {
    Vertex,
    TessellationControl,
    TessellationEvaluation,
    Geometry,
    Fragment,
    Compute,
    AllGraphics,
    All,
    RaygenKhr,
    AnyHitKhr,
    ClosestHitKhr,
    MissKhr,
    IntersectionKhr,
    CallableKhr,
    TaskNv,
    MeshNv,
    SubpassShadingHuawei,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    None,
    Front,
    Back,
    FrondAndBack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleCount {
    OneBit,
    TwoBit,
    FourBit,
    EightBit,
    SixteenBit,
    ThirtytwoBit,
    SixtyfourBit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    ConstantColor,
    OneMinusConstantColor,
    ConstantAlpha,
    OneMinusConstantAlpha,
    SrcAlphaSaturate,
    Src1Color,
    OneMinusSrc1Color,
    Src1Alpha,
    OneMinusSrc1Alpha,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendOp {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
    ZeroExt,
    SrcExt,
    DstExt,
    SrcOverExt,
    DstOverExt,
    SrcInExt,
    DstInExt,
    SrcOutExt,
    DstOutExt,
    SrcAtopExt,
    DstAtopExt,
    XorExt,
    MultiplyExt,
    ScreenExt,
    OverlayExt,
    DarkenExt,
    LightenExt,
    ColordodgeExt,
    ColorburnExt,
    HardlightExt,
    SoftlightExt,
    DifferenceExt,
    ExclusionExt,
    InvertExt,
    InvertRgbExt,
    LineardodgeExt,
    LinearburnExt,
    VividlightExt,
    LinearlightExt,
    PinlightExt,
    HardmixExt,
    HslHueExt,
    HslSaturationExt,
    HslColorExt,
    HslLuminosityExt,
    PlusExt,
    PlusClampedExt,
    PlusClampedAlphaExt,
    PlusDarkerExt,
    MinusExt,
    MinusClampedExt,
    ContrastExt,
    InvertOvgExt,
    RedExt,
    GreenExt,
    BlueExt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorComponent {
    Red,
    Green,
    Blue,
    Alpha,
}
#[derive(Default)]
pub struct ColorComponentFlags(u32);

#[derive(Default)]
pub struct PipelineShaderStageCreateFlags(u32);

#[derive(Debug)]
pub struct PipelineShaderStageCreateInfo<'a> {
    flags: PipelineShaderStageCreateFlags,
    stage: ShaderStage,
    module: &'a ShaderModule<'a>,
    name: &'a str,
    specialization_info: Option<()>, // TODO: SpecializationInfo
}

#[derive(Default)]
pub struct PipelineVertexInputStateCreateFlags(u32);
#[derive(Default)]
pub struct PipelineInputAssemblyStateCreateFlags(u32);
#[derive(Default)]
pub struct PipelineViewportStateCreateFlags(u32);
#[derive(Default)]
pub struct PipelineRasterizationStateCreateFlags(u32);
#[derive(Default)]
pub struct PipelineMultisampleStateCreateFlags(u32);
#[derive(Default)]
pub struct PipelineColorBlendStateCreateFlags(u32);

#[derive(Debug)]
pub struct VertexInputBindingDescription {
    binding: u32,
    stride: u32,
    input_rate: VertexInputRate,
}

#[derive(Debug)]
pub struct VertexInputAttributeDescription {
    location: u32,
    binding: u32,
    format: Format,
    offset: u32,
}

#[derive(Debug, Default)]
pub struct PipelineVertexInputStateCreateInfo<'a> {
    flags: PipelineVertexInputStateCreateFlags,
    vertex_binding_descriptions: Option<&'a [VertexInputBindingDescription]>,
    vertex_attribute_descriptions: Option<&'a [VertexInputAttributeDescription]>,
}

/// Description of assembly of primitives
#[derive(Debug)]
pub struct PipelineInputAssemblyStateCreateInfo {
    flags: PipelineInputAssemblyStateCreateFlags,
    topology: PrimitiveTopology,
    primitive_restart_enable: bool,
}

/// A polygon viewing region
#[derive(Debug, Default)]
pub struct Viewport {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    min_depth: f32,
    max_depth: f32,
}

#[derive(Debug)]
pub struct PipelineViewportStateCreateInfo<'a> {
    flags: PipelineViewportStateCreateFlags,
    viewports: Option<&'a [Viewport]>,
    scissors: Option<&'a [Rect2D]>,
}

#[derive(Debug, Default)]
pub struct PipelineRasterizationStateCreateInfo {
    flags: PipelineRasterizationStateCreateFlags,
    depth_clamp_enable: bool,
    rasterizer_discard_enable: bool,
    polygon_mode: PolygonMode,
    cull_mode: CullMode,
    front_face: FrontFace,
    depth_bias_enable: bool,
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
    line_width: f32,
}

#[derive(Debug, Default)]
pub struct PipelineMultisampleStateCreateInfo<'a> {
    flags: PipelineMultisampleStateCreateFlags,
    rasterization_samples: SampleCount,
    min_sample_mask: Option<f32>,
    sample_mask: &'a [SampleMask],
    alpha_to_coverage_enable: bool,
    alpha_to_one_enable: bool,
}

#[derive(Debug, Default)]
pub struct PipelineColorBlendAttachmentState {
    blend_enable: bool,
    src_color_blend_factor: BlendFactor,
    dst_color_blend_factor: BlendFactor,
    color_blend_op: BlendOp,
    src_alpha_blend_factor: BlendFactor,
    dst_alpha_blend_factor: BlendFactor,
    alpha_blend_op: BlendOp,
    color_write_mask: ColorComponentFlags,
}

#[derive(Debug, Default)]
pub struct PipelineColorBlendStateCreateInfo<'a> {
    _flags: PipelineColorBlendStateCreateFlags,
    logic_op_enable: bool,
    logic_op: LogicOp,
    attachments: Option<&'a [PipelineColorBlendAttachmentState]>,
    blend_constants: [f64; 4],
}

impl Default for FrontFace {
    fn default() -> Self {
        Self::CounterClockwise
    }
}

impl Default for PrimitiveTopology {
    fn default() -> Self {
        Self::PointList
    }
}

impl Default for PolygonMode {
    fn default() -> Self {
        Self::Fill
    }
}

impl Default for LogicOp {
    fn default() -> Self {
        Self::Clear
    }
}

impl Default for ShaderStage {
    fn default() -> Self {
        Self::Vertex
    }
}

impl Default for CullMode {
    fn default() -> Self {
        Self::None
    }
}

impl Default for SampleCount {
    fn default() -> Self {
        Self::OneBit
    }
}

impl Default for BlendFactor {
    fn default() -> Self {
        Self::Zero
    }
}

impl Default for BlendOp {
    fn default() -> Self {
        Self::Add
    }
}

impl ColorComponentFlags {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_red(mut self, red: bool) -> Self {
        if red {
            self.0 |= ffi::ColorComponentFlagBits::Red as u32;
        } else {
            self.0 &= !(ffi::ColorComponentFlagBits::Red as u32);
        }
        self
    }

    pub fn with_green(mut self, green: bool) -> Self {
        if green {
            self.0 |= ffi::ColorComponentFlagBits::Green as u32;
        } else {
            self.0 &= !(ffi::ColorComponentFlagBits::Green as u32);
        }
        self
    }

    pub fn with_blue(mut self, blue: bool) -> Self {
        if blue {
            self.0 |= ffi::ColorComponentFlagBits::Blue as u32;
        } else {
            self.0 &= !(ffi::ColorComponentFlagBits::Blue as u32);
        }
        self
    }

    pub fn with_alpha(mut self, alpha: bool) -> Self {
        if alpha {
            self.0 |= ffi::ColorComponentFlagBits::Alpha as u32;
        } else {
            self.0 &= !(ffi::ColorComponentFlagBits::Alpha as u32);
        }
        self
    }
}

impl fmt::Debug for ColorComponentFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            f.write_str("()")
        } else {
            let mut first = true;

            if self.0 & ffi::ColorComponentFlagBits::Red as u32 != 0 {
                if first {
                    first = false;
                }
                f.write_str("RED")?;
            }
            if self.0 & ffi::ColorComponentFlagBits::Blue as u32 != 0 {
                if first {
                    first = false;
                } else {
                    f.write_str(" | ")?;
                }
                f.write_str("BLUE")?;
            }
            if self.0 & ffi::ColorComponentFlagBits::Green as u32 != 0 {
                if first {
                    first = false;
                } else {
                    f.write_str(" | ")?;
                }
                f.write_str("GREEN")?;
            }
            if self.0 & ffi::ColorComponentFlagBits::Alpha as u32 != 0 {
                if !first {
                    f.write_str(" | ")?;
                }
                f.write_str("ALPHA")?;
            }

            Ok(())
        }
    }
}

impl fmt::Debug for PipelineShaderStageCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            f.write_str("()")
        } else {
            f.write_str("non-empty")
        }
    }
}

impl<'a> PipelineShaderStageCreateInfo<'a> {
    pub fn new(module: &'a ShaderModule<'_>) -> Self {
        Self {
            flags: Default::default(),
            stage: Default::default(),
            module,
            name: "main",
            specialization_info: None,
        }
    }

    pub fn with_flags(mut self, flags: PipelineShaderStageCreateFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn with_stage(mut self, stage: ShaderStage) -> Self {
        self.stage = stage;
        self
    }

    pub fn with_name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }

    fn _with_specialization_info(self, _specialization_info: Option<()>) -> Self {
        todo!()
    }
}

impl fmt::Debug for PipelineVertexInputStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            f.write_str("()")
        } else {
            f.write_str("non-empty")
        }
    }
}

impl fmt::Debug for PipelineInputAssemblyStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            f.write_str("()")
        } else {
            f.write_str("non-empty")
        }
    }
}

impl fmt::Debug for PipelineViewportStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            f.write_str("()")
        } else {
            f.write_str("non-empty")
        }
    }
}

impl fmt::Debug for PipelineRasterizationStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            f.write_str("()")
        } else {
            f.write_str("non-empty")
        }
    }
}

impl fmt::Debug for PipelineMultisampleStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("()")
    }
}

impl fmt::Debug for PipelineColorBlendStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("()")
    }
}

impl<'a> PipelineVertexInputStateCreateInfo<'a> {
    pub fn with_vertex_binding_descriptions(
        mut self,
        vertex_binding_descriptions: &'a [VertexInputBindingDescription],
    ) -> Self {
        self.vertex_binding_descriptions = Some(vertex_binding_descriptions);
        self
    }

    pub fn with_vertex_attribute_descriptions(
        mut self,
        vertex_attribute_descriptions: &'a [VertexInputAttributeDescription],
    ) -> Self {
        self.vertex_attribute_descriptions = Some(vertex_attribute_descriptions);
        self
    }
}

impl PipelineInputAssemblyStateCreateInfo {
    pub fn new() -> Self {
        Self {
            flags: Default::default(),
            topology: Default::default(),
            primitive_restart_enable: Default::default(),
        }
    }

    pub fn with_topology(mut self, topology: PrimitiveTopology) -> Self {
        self.topology = topology;
        self
    }

    pub fn with_primitive_restart_enable(mut self, primitive_restart_enable: bool) -> Self {
        self.primitive_restart_enable = primitive_restart_enable;
        self
    }
}

impl Default for PipelineInputAssemblyStateCreateInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl Viewport {
    pub fn new(x: f32, y: f32, width: f32, height: f32, min_depth: f32, max_depth: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            min_depth,
            max_depth,
        }
    }
}

impl<'a> PipelineViewportStateCreateInfo<'a> {
    pub fn new() -> Self {
        Self {
            flags: Default::default(),
            viewports: Default::default(),
            scissors: Default::default(),
        }
    }

    pub fn with_viewports(mut self, viewports: &'a [Viewport]) -> Self {
        self.viewports = Some(viewports);
        self
    }

    pub fn with_scissors(mut self, scissors: &'a [Rect2D]) -> Self {
        self.scissors = Some(scissors);
        self
    }
}

impl Default for PipelineViewportStateCreateInfo<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineRasterizationStateCreateInfo {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_depth_clamp_enable(mut self, depth_clamp_enable: bool) -> Self {
        self.depth_clamp_enable = depth_clamp_enable;
        self
    }

    pub fn with_rasterizer_discard_enable(mut self, rasterizer_discard_enable: bool) -> Self {
        self.rasterizer_discard_enable = rasterizer_discard_enable;
        self
    }

    pub fn with_polygon_mode(mut self, polygon_mode: PolygonMode) -> Self {
        self.polygon_mode = polygon_mode;
        self
    }

    pub fn with_cull_mode(mut self, cull_mode: CullMode) -> Self {
        self.cull_mode = cull_mode;
        self
    }

    pub fn with_front_face(mut self, front_face: FrontFace) -> Self {
        self.front_face = front_face;
        self
    }

    pub fn with_depth_bias_enable(mut self, depth_bias_enable: bool) -> Self {
        self.depth_bias_enable = depth_bias_enable;
        self
    }

    pub fn with_depth_bias_constant_factor(mut self, depth_bias_constant_factor: f32) -> Self {
        self.depth_bias_constant_factor = depth_bias_constant_factor;
        self
    }

    pub fn with_depth_bias_clamp(mut self, depth_bias_clamp: f32) -> Self {
        self.depth_bias_clamp = depth_bias_clamp;
        self
    }

    pub fn with_depth_bias_slope_factor(mut self, depth_bias_slope_factor: f32) -> Self {
        self.depth_bias_slope_factor = depth_bias_slope_factor;
        self
    }

    pub fn with_line_width(mut self, line_width: f32) -> Self {
        self.line_width = line_width;
        self
    }
}

impl<'a> PipelineMultisampleStateCreateInfo<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_rasterization_samples(mut self, rasterization_samples: SampleCount) -> Self {
        self.rasterization_samples = rasterization_samples;
        self
    }

    pub fn with_min_sample_mask(mut self, min_sample_mask: f32) -> Self {
        self.min_sample_mask = Some(min_sample_mask);
        self
    }

    pub fn with_sample_mask(mut self, sample_mask: &'a [SampleMask]) -> Self {
        self.sample_mask = sample_mask;
        self
    }

    pub fn with_alpha_to_coverage_enable(mut self, alpha_to_coverage_enable: bool) -> Self {
        self.alpha_to_coverage_enable = alpha_to_coverage_enable;
        self
    }

    pub fn with_alpha_to_one_enable(mut self, alpha_to_one_enable: bool) -> Self {
        self.alpha_to_one_enable = alpha_to_one_enable;
        self
    }
}

impl PipelineColorBlendAttachmentState {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_blend_enable(mut self, blend_enable: bool) -> Self {
        self.blend_enable = blend_enable;
        self
    }

    pub fn with_src_color_blend_factor(mut self, src_color_blend_factor: BlendFactor) -> Self {
        self.src_color_blend_factor = src_color_blend_factor;
        self
    }

    pub fn with_dst_color_blend_factor(mut self, dst_color_blend_factor: BlendFactor) -> Self {
        self.dst_color_blend_factor = dst_color_blend_factor;
        self
    }

    pub fn with_color_blend_op(mut self, color_blend_op: BlendOp) -> Self {
        self.color_blend_op = color_blend_op;
        self
    }

    pub fn with_src_alpha_blend_factor(mut self, src_alpha_blend_factor: BlendFactor) -> Self {
        self.src_alpha_blend_factor = src_alpha_blend_factor;
        self
    }

    pub fn with_dst_alpha_blend_factor(mut self, dst_alpha_blend_factor: BlendFactor) -> Self {
        self.dst_alpha_blend_factor = dst_alpha_blend_factor;
        self
    }

    pub fn with_alpha_blend_op(mut self, alpha_blend_op: BlendOp) -> Self {
        self.alpha_blend_op = alpha_blend_op;
        self
    }

    pub fn with_color_write_mask(mut self, color_write_mask: ColorComponentFlags) -> Self {
        self.color_write_mask = color_write_mask;
        self
    }
}

impl<'a> PipelineColorBlendStateCreateInfo<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_logic_op_enable(mut self, logic_op_enable: bool) -> Self {
        self.logic_op_enable = logic_op_enable;
        self
    }

    pub fn with_logic_op(mut self, logic_op: LogicOp) -> Self {
        self.logic_op = logic_op;
        self
    }

    pub fn with_attachments(mut self, attachments: &'a [PipelineColorBlendAttachmentState]) -> Self {
        self.attachments = Some(attachments);
        self
    }

    pub fn with_blend_constants(mut self, blend_constants: [f64; 4]) -> Self {
        self.blend_constants = blend_constants;
        self
    }
}
