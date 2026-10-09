//! 🧾️ Schema diagnostics preserve the actual retained effects of refused close turns.
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::RetainedCloneProgress};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct SchemaDecodeDiagnostic<Path>{pub code:&'static str,pub offset:u64,pub line:u32,pub column:u32,pub path:Path,pub refusal_kind:ValueRefusalKind,pub retained_progress:RetainedCloneProgress}
impl<Path> SchemaDecodeDiagnostic<Path>{
    /// 🛑️ Declares a diagnostic before any admitted physical effect occurs.
    pub fn before(code:&'static str,path:Path)->Self{Self{code,offset:0,line:0,column:0,path,refusal_kind:ValueRefusalKind::InvariantViolated,retained_progress:Default::default()}}
    /// 📥️ Carries the original native refusal and every effect already completed by that turn.
    pub fn with_native(mut self,error:ValueError)->Self{self.refusal_kind=error.kind;self.retained_progress=error.retained_progress();self}
    /// 🧵️ Carries a completed producer step through a later diagnostic refusal.
    pub fn with_progress(mut self,progress:RetainedCloneProgress)->Self{self.retained_progress=progress;self}
}
