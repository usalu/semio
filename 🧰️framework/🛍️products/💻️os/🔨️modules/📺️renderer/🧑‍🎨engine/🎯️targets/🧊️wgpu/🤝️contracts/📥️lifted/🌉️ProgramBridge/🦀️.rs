//! 🧯️ The refused-program-call vocabulary every program consumer names: the guest's structured fault plus the one-line text channel.
//!
//! 🧭️ Lifted out of `🌉️ProgramBridge` into contracts: the kernel crates below the bridge raise it, the shell above reads it.

/// 🧯️ A refused program call: the guest's own structured `Fault` when it answered with one (its code, severity and the
/// `params` a localized notice fills its placeholders from, design §20.12), and the one-line `code: message …` text every
/// `String` error channel of the shell carries.
#[derive(Clone, Debug, PartialEq)]
pub struct ProgramFault {
    pub fault: Option<semio_framework::Fault>,
    pub text: String,
    #[cfg(not(target_arch = "wasm32"))]
    pub frame: Option<semio_framework_actor::pack::PackError>,
    #[cfg(not(target_arch = "wasm32"))]
    pub kernel:Option<semio_framework_actor::KernelError>,
    #[cfg(not(target_arch = "wasm32"))]
    pub actor_call:Option<semio_framework_plugin_host::original_actor_context::OriginalActorFixedCall>,
    #[cfg(not(target_arch = "wasm32"))]
    pub create_app_source:Option<CreateAppSourceRefusal>,
}

/// 🚪️ The rendered text of a refused CreateApp source open, captured while its request owner was still live; the shell reads only its presence and text.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateAppSourceRefusal(String);

#[cfg(not(target_arch = "wasm32"))]
impl CreateAppSourceRefusal {
    pub fn new(text: String) -> Self { Self(text) }
}

#[cfg(not(target_arch = "wasm32"))]
impl std::fmt::Display for CreateAppSourceRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str(&self.0) }
}

impl From<String> for ProgramFault {
    fn from(text: String) -> Self {
        Self { fault: None, text, #[cfg(not(target_arch = "wasm32"))] frame: None,#[cfg(not(target_arch = "wasm32"))] kernel:None,#[cfg(not(target_arch = "wasm32"))] actor_call:None,#[cfg(not(target_arch = "wasm32"))] create_app_source:None }
    }
}

impl From<&str> for ProgramFault {
    fn from(text: &str) -> Self {
        Self { fault: None, text: text.to_string(), #[cfg(not(target_arch = "wasm32"))] frame: None,#[cfg(not(target_arch = "wasm32"))] kernel:None,#[cfg(not(target_arch = "wasm32"))] actor_call:None,#[cfg(not(target_arch = "wasm32"))] create_app_source:None }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<semio_framework_actor::pack::PackError> for ProgramFault {
    fn from(error: semio_framework_actor::pack::PackError) -> Self { Self { fault: None, text: String::new(), frame: Some(error),kernel:None,actor_call:None,#[cfg(not(target_arch = "wasm32"))] create_app_source:None } }
}

#[cfg(not(target_arch="wasm32"))]
impl From<semio_framework_actor::KernelError> for ProgramFault{fn from(error:semio_framework_actor::KernelError)->Self{Self{fault:None,text:String::new(),frame:None,kernel:Some(error),actor_call:None,#[cfg(not(target_arch = "wasm32"))] create_app_source:None}}}
#[cfg(not(target_arch="wasm32"))]
impl From<semio_framework_plugin_host::original_actor_context::OriginalActorFixedCall> for ProgramFault{fn from(call:semio_framework_plugin_host::original_actor_context::OriginalActorFixedCall)->Self{Self{fault:None,text:String::new(),frame:None,kernel:None,actor_call:Some(call),create_app_source:None}}}
impl std::fmt::Display for ProgramFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(error) = &self.frame { return std::fmt::Display::fmt(error, formatter); }
        #[cfg(not(target_arch="wasm32"))]
        if let Some(error)=&self.kernel{return std::fmt::Display::fmt(error,formatter)}
        #[cfg(not(target_arch="wasm32"))]
        if let Some(call)=&self.actor_call{return write!(formatter,"original actor call {}/{}/{} {:?} after {:?}",call.input.operation,call.input.generation,call.input.epoch,call.kind,call.spent)}
        #[cfg(not(target_arch="wasm32"))]
        if let Some(error)=&self.create_app_source{return std::fmt::Display::fmt(error,formatter)}
        formatter.write_str(&self.text)
    }
}
