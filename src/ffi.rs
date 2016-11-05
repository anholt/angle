use libc::*;

// ShShaderSpec
pub const SH_GLES2_SPEC: c_int = 0x8B40;
pub const SH_WEBGL_SPEC: c_int = 0x8B41;
pub const SH_GLES3_SPEC: c_int = 0x8B86;
pub const SH_WEBGL2_SPEC: c_int = 0x8B87;
pub const SH_CSS_SHADERS_SPEC: c_int = 0x8B42;

// ShShaderOutput
pub const SH_ESSL_OUTPUT: c_int = 0x8B45;
pub const SH_GLSL_OUTPUT: c_int = 0x8B46;
pub const SH_GLSL_COMPATIBILITY_OUTPUT: c_int = 0x8B46;
pub const SH_GLSL_CORE_OUTPUT: c_int = 0x8B47;
pub const SH_GLSL_130_OUTPUT: c_int = 0x8B47;
pub const SH_GLSL_140_OUTPUT: c_int = 0x8B80;
pub const SH_GLSL_150_CORE_OUTPUT: c_int = 0x8B81;
pub const SH_GLSL_330_CORE_OUTPUT: c_int = 0x8B82;
pub const SH_GLSL_400_CORE_OUTPUT: c_int = 0x8B83;
pub const SH_GLSL_410_CORE_OUTPUT: c_int = 0x8B84;
pub const SH_GLSL_420_CORE_OUTPUT: c_int = 0x8B85;
pub const SH_GLSL_430_CORE_OUTPUT: c_int = 0x8B86;
pub const SH_GLSL_440_CORE_OUTPUT: c_int = 0x8B87;
pub const SH_GLSL_450_CORE_OUTPUT: c_int = 0x8B88;
pub const SH_HLSL_OUTPUT: c_int = 0x8B48;
pub const SH_HLSL9_OUTPUT: c_int = 0x8B48;
pub const SH_HLSL11_OUTPUT: c_int = 0x8B49;

// ShCompileOptions
pub const SH_VALIDATE: c_int = 0;
pub const SH_VALIDATE_LOOP_INDEXING: c_int = 0x0001;
pub const SH_INTERMEDIATE_TREE: c_int = 0x0002;
pub const SH_OBJECT_CODE: c_int = 0x0004;
pub const SH_VARIABLES: c_int = 0x0008;
pub const SH_LINE_DIRECTIVES: c_int = 0x0010;
pub const SH_SOURCE_PATH: c_int = 0x0020;
pub const SH_UNROLL_FOR_LOOP_WITH_INTEGER_INDEX: c_int = 0x0040;
pub const SH_UNROLL_FOR_LOOP_WITH_SAMPLER_ARRAY_INDEX: c_int = 0x0080;
pub const SH_EMULATE_BUILT_IN_FUNCTIONS: c_int = 0x0100;
pub const SH_TIMING_RESTRICTIONS: c_int = 0x0200;
pub const SH_DEPENDENCY_GRAPH: c_int = 0x0400;
pub const SH_ENFORCE_PACKING_RESTRICTIONS: c_int = 0x0800;
pub const SH_CLAMP_INDIRECT_ARRAY_BOUNDS: c_int = 0x1000;
pub const SH_LIMIT_EXPRESSION_COMPLEXITY: c_int = 0x2000;
pub const SH_LIMIT_CALL_STACK_DEPTH: c_int = 0x4000;
pub const SH_INIT_GL_POSITION: c_int = 0x8000;
pub const SH_UNFOLD_SHORT_CIRCUIT: c_int = 0x10000;
pub const SH_INIT_VARYINGS_WITHOUT_STATIC_USE: c_int = 0x20000;
pub const SH_SCALARIZE_VEC_AND_MAT_CONSTRUCTOR_ARGS: c_int = 0x40000;
pub const SH_REGENERATE_STRUCT_NAMES: c_int = 0x80000;
pub const SH_DONT_PRUNE_UNUSED_FUNCTIONS: c_int = 0x100000;
pub const SH_REMOVE_POW_WITH_CONSTANT_EXPONENT: c_int = 0x200000;

// ShArrayIndexClampingStrategy;
pub const SH_CLAMP_WITH_CLAMP_INTRINSIC: c_int = 1;
pub const SH_CLAMP_WITH_USER_DEFINED_INT_CLAMP_FUNCTION: c_int = 2;

#[allow(non_snake_case)]
#[repr(C)]
pub struct ShBuiltInResources {
    pub MaxVertexAttribs: c_int,
    pub MaxVertexUniformVectors: c_int,
    pub MaxVaryingVectors: c_int,
    pub MaxVertexTextureImageUnits: c_int,
    pub MaxCombinedTextureImageUnits: c_int,
    pub MaxTextureImageUnits: c_int,
    pub MaxFragmentUniformVectors: c_int,
    pub MaxDrawBuffers: c_int,
    pub OES_standard_derivatives: c_int,
    pub OES_EGL_image_external: c_int,
    pub ARB_texture_rectangle: c_int,
    pub EXT_blend_func_extended: c_int,
    pub EXT_draw_buffers: c_int,
    pub EXT_frag_depth: c_int,
    pub EXT_shader_texture_lod: c_int,
    pub WEBGL_debug_shader_precision: c_int,
    pub EXT_shader_framebuffer_fetch: c_int,
    pub NV_shader_framebuffer_fetch: c_int,
    pub ARM_shader_framebuffer_fetch: c_int,
    pub NV_draw_buffers: c_int,
    pub FragmentPrecisionHigh: c_int,
    pub MaxVertexOutputVectors: c_int,
    pub MaxFragmentInputVectors: c_int,
    pub MinProgramTexelOffset: c_int,
    pub MaxProgramTexelOffset: c_int,
    pub MaxDualSourceDrawBuffers: c_int,
    pub HashFunction: *const c_void,
    pub ArrayIndexClampingStrategy: c_int,
    pub MaxExpressionComplexity: c_int,
    pub MaxCallStackDepth: c_int,
}

#[repr(C)]
pub struct GLSLangActiveInfo {
    pub gl_type: c_int,
    pub size: c_int,
    pub name: *const c_char,
}

pub type ShHandle = *mut c_void;

extern {
    pub fn GLSLangInitialize() -> c_int;
    pub fn GLSLangFinalize() -> c_int;
    pub fn GLSLangInitBuiltInResources(res: *mut ShBuiltInResources);
    pub fn GLSLangConstructCompiler(_type: c_uint, spec: c_int, output: c_int, resources_ptr: *const ShBuiltInResources) -> ShHandle;
    pub fn GLSLangDestructCompiler(handle: ShHandle);
    pub fn GLSLangCompile(handle: ShHandle, strings: *const *const c_char, num_strings: size_t, compile_options: c_int) -> c_int;
    pub fn GLSLangClearResults(handle: ShHandle);
    pub fn GLSLangGetShaderVersion(handle: ShHandle) -> c_int;
    pub fn GLSLangGetShaderOutputType(handle: ShHandle) -> c_int;
    pub fn GLSLangGetObjectCode(handle: ShHandle) -> *const c_char;
    pub fn GLSLangGetInfoLog(handle: ShHandle) -> *const c_char;
    pub fn GLSLangGetNumActiveUniforms(handle: ShHandle) -> c_int;
    pub fn GLSLangGetActiveUniform(handle: ShHandle, i: c_int) -> GLSLangActiveInfo;
}

