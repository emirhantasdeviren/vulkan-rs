use crate::format::Format;
use crate::shaders::ShaderModule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexInputRate {
    Vertex,
    Instance,
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

impl Default for ShaderStage {
    fn default() -> Self {
        Self::Vertex
    }
}

impl std::fmt::Debug for PipelineShaderStageCreateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl std::fmt::Debug for PipelineVertexInputStateCreateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
