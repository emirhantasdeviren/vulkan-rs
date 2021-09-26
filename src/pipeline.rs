use crate::shaders::ShaderModule;

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
