use super::ffi::*;
use super::ffi::ShShaderOutput::*;
use super::ffi::ShShaderSpec::*;

use std::default;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::sync::Mutex;
use std::os::raw::c_char;

lazy_static! {
    static ref CONSTRUCT_COMPILER_LOCK: Mutex<()> = {
        Mutex::new(())
    };
}


pub fn initialize() -> Result<(), &'static str> {
    if unsafe { GLSLangInitialize() } == 0 {
        Err("Couldn't initialize GLSLang")
    } else {
        Ok(())
    }
}

pub fn finalize() -> Result<(), &'static str> {
    if unsafe { GLSLangFinalize() } == 0 {
        Err("Couldn't finalize GLSLang")
    } else {
        Ok(())
    }
}

pub trait AsAngleEnum {
    fn as_angle_enum(&self) -> i32;
}

pub enum ShaderSpec {
    Gles2,
    WebGL,
    Gles3,
    WebGL2,
    WebGL3,
}

impl AsAngleEnum for ShaderSpec {
    #[inline]
    fn as_angle_enum(&self) -> i32 {
        (match *self {
            ShaderSpec::Gles2 => SH_GLES2_SPEC,
            ShaderSpec::WebGL => SH_WEBGL_SPEC,
            ShaderSpec::Gles3 => SH_GLES3_SPEC,
            ShaderSpec::WebGL2 => SH_WEBGL2_SPEC,
            ShaderSpec::WebGL3 => SH_WEBGL3_SPEC,
        }) as i32
    }
}

pub enum Output {
    Essl,
    Glsl,
    GlslCompat,
    GlslCore,
    Glsl130,
    Glsl140,
    Glsl150Core,
    Glsl330Core,
    Glsl400Core,
    Glsl410Core,
    Glsl420Core,
    Glsl430Core,
    Glsl440Core,
    Glsl450Core,
}

impl AsAngleEnum for Output {
    #[inline]
    fn as_angle_enum(&self) -> i32 {
        (match *self {
            Output::Essl => SH_ESSL_OUTPUT,
            Output::Glsl => SH_GLSL_COMPATIBILITY_OUTPUT,
            Output::GlslCompat => SH_GLSL_COMPATIBILITY_OUTPUT,
            Output::GlslCore => SH_GLSL_130_OUTPUT,
            Output::Glsl130 => SH_GLSL_130_OUTPUT,
            Output::Glsl140 => SH_GLSL_140_OUTPUT,
            Output::Glsl150Core => SH_GLSL_150_CORE_OUTPUT,
            Output::Glsl330Core => SH_GLSL_330_CORE_OUTPUT,
            Output::Glsl400Core => SH_GLSL_400_CORE_OUTPUT,
            Output::Glsl410Core => SH_GLSL_410_CORE_OUTPUT,
            Output::Glsl420Core => SH_GLSL_420_CORE_OUTPUT,
            Output::Glsl430Core => SH_GLSL_430_CORE_OUTPUT,
            Output::Glsl440Core => SH_GLSL_440_CORE_OUTPUT,
            Output::Glsl450Core => SH_GLSL_450_CORE_OUTPUT,
        }) as i32
    }
}

pub type BuiltInResources = ShBuiltInResources;

impl default::Default for BuiltInResources {
    fn default() -> BuiltInResources {
        unsafe {
            let mut ret: BuiltInResources = mem::zeroed();
            GLSLangInitBuiltInResources(&mut ret);
            ret
        }
    }
}

impl BuiltInResources {
    #[inline]
    pub fn empty() -> BuiltInResources {
        unsafe { mem::zeroed() }
    }
}


pub struct ShaderValidator {
    handle: ShHandle,
}

impl ShaderValidator {
    /// Create a new ShaderValidator instance
    /// NB: To call this you should have called first
    /// initialize()
    pub fn new(shader_type: u32,
               spec: ShaderSpec,
               output: Output,
               resources: &BuiltInResources) -> Option<ShaderValidator> {
        // GLSLangConstructCompiler is non-thread safe because it internally calls TCache::getType()
        // which writes/reads a std::map<T> with no locks.
        let _guard = CONSTRUCT_COMPILER_LOCK.lock().unwrap();
        let handle = unsafe {
            GLSLangConstructCompiler(shader_type, spec.as_angle_enum(), output.as_angle_enum(), resources)
        };

        if handle.is_null() {
            return None;
        }

        Some(ShaderValidator {
            handle: handle,
        })
    }

    #[inline]
    pub fn for_webgl(shader_type: u32,
                     output: Output,
                     resources: &BuiltInResources) -> Option<ShaderValidator> {
        Self::new(shader_type, ShaderSpec::WebGL, output, resources)
    }

    #[inline]
    pub fn for_webgl2(shader_type: u32,
                      output: Output,
                      resources: &BuiltInResources) -> Option<ShaderValidator> {
        Self::new(shader_type, ShaderSpec::WebGL2, output, resources)
    }

    pub fn compile(&self, strings: &[&str], options: ShCompileOptions) -> Result<(), &'static str> {
        let mut cstrings = Vec::with_capacity(strings.len());

        for s in strings.iter() {
            cstrings.push(try!(CString::new(*s).map_err(|_| "Found invalid characters")))
        }

        let cptrs: Vec<_> = cstrings.iter().map(|s| s.as_ptr()).collect();

        if unsafe { GLSLangCompile(self.handle,
                                   cptrs.as_ptr() as *const *const c_char,
                                   cstrings.len(),
                                   options) } == 0 {
            return Err("Couldn't compile shader")
        }
        Ok(())
    }

    pub fn object_code(&self) -> String {
        unsafe {
            let c_str = CStr::from_ptr(GLSLangGetObjectCode(self.handle));
            c_str.to_string_lossy().into_owned()
        }
    }

    pub fn info_log(&self) -> String {
        unsafe {
            let c_str = CStr::from_ptr(GLSLangGetInfoLog(self.handle));
            c_str.to_string_lossy().into_owned()
        }
    }

    pub fn compile_and_translate(&self, strings: &[&str]) -> Result<String, &'static str> {
        let options = SH_VALIDATE | SH_OBJECT_CODE |
                      SH_EMULATE_ABS_INT_FUNCTION | // To workaround drivers
                      SH_EMULATE_ISNAN_FLOAT_FUNCTION | // To workaround drivers
                      SH_EMULATE_ATAN2_FLOAT_FUNCTION | // To workaround drivers
                      SH_CLAMP_INDIRECT_ARRAY_BOUNDS |
                      SH_INIT_GL_POSITION |
                      SH_ENFORCE_PACKING_RESTRICTIONS |
                      SH_LIMIT_EXPRESSION_COMPLEXITY |
                      SH_LIMIT_CALL_STACK_DEPTH;

        // Todo(Mortimer): Add SH_TIMING_RESTRICTIONS to options when the implementations gets better
        // Right now SH_TIMING_RESTRICTIONS is experimental 
        // and doesn't support user callable functions in shaders

        try!(self.compile(strings, options));
        Ok(self.object_code())
    }
}

impl Drop for ShaderValidator {
    fn drop(&mut self) {
        unsafe {
            GLSLangDestructCompiler(self.handle)
        }
    }
}
