//! 📤️ The SDK's own plugin fault constructor.
use semio_framework::{Fault, FaultCode, FaultOrigin};



// 🗣️ Shared two-axis (locale × terminology) label resolution — replaces the ~25x hand-rolled
// `struct XLabels { .. }` + `const X_LABELS_EN/DE` + `fn x_labels(view_state) -> &'static XLabels`
// pattern duplicated per app, AND the 4-crate hand-rolled `NATIVE_EN/NATIVE_DE/REUSE_EN/REUSE_DE` +
// non-exhaustive `match (terminology, is_de) { ..., (_, true) => ... }` terminology resolvers this
// SDK never covered. `app_labels!` now declares all four cells per field and resolves them via an
// exhaustive match on the generated `Locale`/`Terminology` enums — no catch-all arm is possible, so
// adding a locale or terminology to `🖱️ui/🎚️axes/🔣️.json` breaks every invocation until it supplies that
// cell. See ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND.

// 🚫️async: E1 pure error constructor consumed pervasively by sync closures
// (`.map_err`/`.ok_or_else`) — see R9.
pub fn plugin_sdk_fault(message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), message)
}

/// 🚨️ Typed failure at the plugin assembly boundary.
#[derive(Clone, Debug, PartialEq,semio_framework_value::RetireOwned)]
pub struct PluginAssemblyError {
    pub code: String,
    pub message: String,
    pub original:Option<Fault>,
}

/// 🧱️ A render keeps its native assembly source or complete already-paid original failure.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum OriginalRenderFailure{Assembly(PluginAssemblyError),Original(semio_framework_job::OriginalTurnFailure)}
/// 🎬️ A pending render publishes no replacement tree while its original source remains retained.
pub type OriginalRenderResult<T>=Result<Option<T>,OriginalRenderFailure>;
impl From<Fault> for OriginalRenderFailure{fn from(source:Fault)->Self{Self::Original(source.into())}}
impl From<PluginAssemblyError> for OriginalRenderFailure{fn from(source:PluginAssemblyError)->Self{Self::Assembly(source)}}
impl From<semio_framework_job::OriginalTurnFailure> for OriginalRenderFailure{fn from(source:semio_framework_job::OriginalTurnFailure)->Self{Self::Original(source)}}
impl From<semio_framework_value::ValueError> for OriginalRenderFailure{fn from(source:semio_framework_value::ValueError)->Self{Self::Original(source.into())}}
impl semio_framework_job::OriginalDomainFailure for OriginalRenderFailure{
 fn domain_code(&self)->&str{match self{Self::Assembly(source)=>&source.code,Self::Original(_)=>"render.original"}}
 fn domain_message(&self)->&str{match self{Self::Assembly(source)=>&source.message,Self::Original(source)=>source.primary.message()}}
 fn domain_retained_progress(&self)->semio_framework_value::RetainedCloneProgress{Default::default()}
}
impl semio_framework_job::OriginalDomainProjection for OriginalRenderFailure{
 fn projection_view(&self,path:&[usize])->Result<semio_framework_job::OriginalDomainView<'_>,semio_framework_value::ValueError>{
  use semio_framework_job::OriginalDomainView as V;match(self,path){(_,[])=>Ok(V::IntrinsicObject(2)),(Self::Assembly(_),[0])=>Ok(V::Text("assembly")),(Self::Original(_),[0])=>Ok(V::Text("original")),(Self::Assembly(source),[1])=>Ok(V::IntrinsicObject(if source.original.is_some(){3}else{2})),(Self::Assembly(source),[1,0])=>Ok(V::Text(&source.code)),(Self::Assembly(source),[1,1])=>Ok(V::Text(&source.message)),(Self::Assembly(source),[1,2,rest @ ..])=>semio_framework_job::project_original_fault(source.original.as_ref().ok_or_else(render_projection_refusal)?,rest),(Self::Original(source),[1,rest @ ..])=>semio_framework_job::project_original_turn_failure(source,rest),_=>Err(render_projection_refusal())}
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,semio_framework_value::ValueError>{match(self,path){(_,[])=>["kind","source"].get(index).copied().ok_or_else(render_projection_refusal),(Self::Assembly(source),[1])=>["code","message","original"].get(index).filter(|_|index<if source.original.is_some(){3}else{2}).copied().ok_or_else(render_projection_refusal),(Self::Assembly(source),[1,2,rest @ ..])=>semio_framework_job::key_original_fault(source.original.as_ref().ok_or_else(render_projection_refusal)?,rest,index),(Self::Original(source),[1,rest @ ..])=>semio_framework_job::key_original_turn_failure(source,rest,index),_=>Err(render_projection_refusal())}}
}
fn render_projection_refusal()->semio_framework_value::ValueError{semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original render failure projection ordinal changed")}

// 🚫️async: E1 pure error constructor consumed pervasively by sync closures
// (`.map_err`/`.ok_or_else`) — see R9.
impl PluginAssemblyError {
    /// 🧬️ Keeps the complete producer diagnostic at the static assembly boundary.
    pub fn from_fault(fault:Fault)->Self{Self{code:fault.code.0.clone(),message:fault.message.clone(),original:Some(fault)}}
    /// 📤️ Startup receives the same original diagnostic instead of a text reconstruction.
    pub fn into_fault(self)->Fault{self.original.unwrap_or_else(||Fault::new(FaultOrigin::Plugin,FaultCode::new(self.code),self.message))}
    /// 🏗️ Creates a codebase-owned assembly failure.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self { code: code.into(), message: message.into(),original:None }
    }

    /// 🚫️async: E1 pure error constructor consumed pervasively by `.map_err(PluginAssemblyError::definition)` — see R9.
    pub fn definition(error: ArtifactDefinitionError) -> Self {
        Self::new(error.code, error.message)
    }
}

impl std::fmt::Display for PluginAssemblyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for PluginAssemblyError {}




// 🗣️ Shared two-axis (locale × terminology) label resolution — replaces the ~25x hand-rolled
// `struct XLabels { .. }` + `const X_LABELS_EN/DE` + `fn x_labels(view_state) -> &'static XLabels`
// pattern duplicated per app, AND the 4-crate hand-rolled `NATIVE_EN/NATIVE_DE/REUSE_EN/REUSE_DE` +
// non-exhaustive `match (terminology, is_de) { ..., (_, true) => ... }` terminology resolvers this
// SDK never covered. `app_labels!` now declares all four cells per field and resolves them via an
// exhaustive match on the generated `Locale`/`Terminology` enums — no catch-all arm is possible, so
// adding a locale or terminology to `🖱️ui/🎚️axes/🔣️.json` breaks every invocation until it supplies that
// cell. See ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND.

/// 🧾️ A validation failure in the artifact-definition contract.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactDefinitionError {
    code: &'static str,
    message: String,
}

// 🚫️async: E1 pure error constructor consumed pervasively by sync closures
// (`.map_err`/`.ok_or_else`) — see R9.
impl ArtifactDefinitionError {
    /// 🧾️ Creates a validation error at an artifact-definition boundary.
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self { code, message: message.into() }
    }

    /// 🏷️ Stable machine-readable failure identity.
    pub fn code(&self) -> &'static str {
        self.code
    }

    /// 📝 Human-readable failure detail.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ArtifactDefinitionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ArtifactDefinitionError {}
