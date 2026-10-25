//! 🚧️ Acceptance error, limits and verdicts: the acceptance error with its conversions, the per-leaf limits, the acceptance case, the verdict and the seed fault. See [`super`].

#[derive(Debug,semio_framework_value::RetireOwned)]
#[value(send_only)]
pub(super) struct AcceptanceError{pub message:String,pub original:Option<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>}

impl AcceptanceError{pub fn context(mut self,message:String)->Self{self.message=if self.message.is_empty(){message}else{format!("{message}: {}",self.message)};self}}

impl From<&str> for AcceptanceError{fn from(message:&str)->Self{message.to_owned().into()}}

impl From<String> for AcceptanceError{fn from(message:String)->Self{Self{message,original:None}}}

impl From<semio_framework_job::OriginalTurnFailure> for AcceptanceError{fn from(original:semio_framework_job::OriginalTurnFailure)->Self{semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from(original).into()}}

impl From<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> for AcceptanceError{fn from(original:semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure)->Self{Self{message:String::new(),original:Some(original)}}}

impl From<semio_framework::Fault> for AcceptanceError{fn from(original:semio_framework::Fault)->Self{semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from(original).into()}}

impl From<semio_framework_value::ValueError> for AcceptanceError{fn from(original:semio_framework_value::ValueError)->Self{semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from(original).into()}}

impl std::fmt::Display for AcceptanceError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match &self.original{Some(original)=>write!(f,"{}: {original:?}",self.message),None=>f.write_str(&self.message)}}}

/// 🧑‍💻️ The actor every acceptance instance authors and dispatches as.
pub(super) const ACCEPTANCE_ACTOR: &str = "history-edit-acceptance";

/// 🌿️ The alternative a new-alternative finalize names.
pub(super) const ACCEPTANCE_ALTERNATIVE: &str = "History edit acceptance";

/// 🔢️ Changes tried per leaf before it counts as having no derivable schema-valid change.
pub(super) const ACCEPTANCE_CHANGES_PER_LEAF: usize = 16;

/// 🌊️ Downstream operations tried per leaf before the leaf runs alone.
pub(super) const ACCEPTANCE_DOWNSTREAM_PER_LEAF: usize = 2;

/// 🧰️ The input-control kinds a change is derived for, in the order of [`acceptance_change_buckets`].
pub(super) const ACCEPTANCE_CONTROL_KINDS: [&str; 5] = ["number", "boolean", "option", "vector", "text"];

/// 🗃️ Committed cases the census tries per leaf before the leaf counts as not exercised.
pub(super) const ACCEPTANCE_CASES_PER_LEAF: usize = 3;

/// 🪙️ Further cases tried for one control kind nobody proved in the leaf sweep.
pub(super) const ACCEPTANCE_KIND_ATTEMPTS: usize = 12;

/// 🌋️ Seeded histories the conflict law tries before the editor counts as having no dependents.
pub(super) const ACCEPTANCE_CASCADE_CASES: usize = 12;

/// 🪜️ Blockers resolved in turn before a blocked review counts as never reaching ready.
pub(super) const ACCEPTANCE_CASCADE_BLOCKERS: usize = 8;

/// 🧱️ Cases whose history cannot be built that the conflict law passes over before it stops looking.
pub(super) const ACCEPTANCE_CASCADE_UNBUILT: usize = 60;

/// 🧫️ One case the law can seed: where it comes from, the DSL text of the document before it, the key of that document (cases
/// sharing it can follow each other downstream) and its operation.
pub struct AcceptanceCase<Mu> {
    pub directory: String,
    pub base: String,
    pub base_key: String,
    pub op: Mu,
}

/// 🏁️ The outcome of one acceptance scenario: it passed (with its summary), its data cannot exercise the flow (`Skip`: the next
/// variant is tried; `SkipCase`: the case's own operation does not seed, so no variant of it can; `SkipBase`: its document does not
/// load, so no case on it can) or the generic mechanism broke (`Fail`, the law fails at once).
pub(super) enum AcceptanceVerdict {
    Pass(String),
    Skip(String),
    SkipCase(String),
    SkipBase(String),
    Fail(AcceptanceError),
}

/// 🌱️ A refused seed input or the complete original failure of its document operation.
pub(super) enum AcceptanceSeedFault {
    Operation(usize, String),
    Original(AcceptanceError),
}

impl AcceptanceSeedFault {
    /// 🎒️ Moves the complete seed cause into its acceptance result.
    pub(super) fn into_error(self) -> AcceptanceError {
        match self {
            Self::Operation(index, reason) => format!("seed operation #{index}: {reason}").into(),
            Self::Original(reason) => reason,
        }
    }

}

impl AcceptanceVerdict {
    /// 🗯️ The verdict's reason or summary in words.
    pub fn text(&self) -> String {
        match self {
            Self::Pass(text) | Self::Skip(text) | Self::SkipCase(text) | Self::SkipBase(text) => text.clone(),
            Self::Fail(error) => error.to_string(),
        }
    }
}
