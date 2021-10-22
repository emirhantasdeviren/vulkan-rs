#![warn(missing_debug_implementations)]

use std::fmt;

use crate::core::Rect2D;
use crate::format::Format;
use crate::shaders::ShaderModule;

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
