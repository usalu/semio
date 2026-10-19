//! 🎟️ Original physical grant, progress and typed refusal contracts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RetainedCloneGrant {
    pub maximum_items: usize,
    /// 🧮️ Bounds payload copying within one clone turn.
    pub maximum_copy_bytes: usize,
    pub maximum_capacity_bytes: usize,
    /// ♻️ Bounds physical owner and scaffold release independently of copying.
    pub maximum_release_bytes: usize,
    pub maximum_depth: usize,
}
impl RetainedCloneGrant {
    pub fn wire_fields(self)->[(&'static str,usize);5]{[("maximumItems",self.maximum_items),("maximumCopyBytes",self.maximum_copy_bytes),("maximumCapacityBytes",self.maximum_capacity_bytes),("maximumReleaseBytes",self.maximum_release_bytes),("maximumDepth",self.maximum_depth)]}
}
impl RetainedCloneGrant {
    /// 🎟️ Admits one structural allocation while reserving no payload-copy credit.
    pub fn one_capacity_turn(maximum_bytes: usize, maximum_depth: usize) -> Self {
        Self { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: maximum_bytes, maximum_release_bytes: 0, maximum_depth }
    }

    /// 🎟️ Admits one payload turn while reserving no allocation-capacity credit.
    pub fn one_payload_turn(maximum_bytes: usize, maximum_depth: usize) -> Self {
        Self { maximum_items: 1, maximum_copy_bytes: maximum_bytes, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth }
    }
    /// ♻️ Admits one release turn without copy or allocation credit.
    pub fn one_release_turn(maximum_bytes: usize, maximum_depth: usize) -> Self {
        Self { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: maximum_bytes, maximum_depth }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RetainedCloneProgress {
    pub copied_items: usize,
    pub copied_bytes: usize,
    pub retained_capacity_bytes: usize,
    pub released_bytes: usize,
}
impl RetainedCloneProgress {
    pub fn wire_fields(self)->[(&'static str,usize);4]{[("copiedItems",self.copied_items),("copiedBytes",self.copied_bytes),("retainedCapacityBytes",self.retained_capacity_bytes),("releasedBytes",self.released_bytes)]}
}
impl RetainedCloneProgress {
    pub fn checked_add(self, other: Self) -> Result<Self, crate::ValueError> {
        Ok(Self {
            copied_items: self.copied_items.checked_add(other.copied_items).ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit, "retained clone item progress overflow"))?,
            copied_bytes: self.copied_bytes.checked_add(other.copied_bytes).ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit, "retained clone byte progress overflow"))?,
            released_bytes: self.released_bytes.checked_add(other.released_bytes).ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit, "retained clone release progress overflow"))?,
            retained_capacity_bytes: self.retained_capacity_bytes.checked_add(other.retained_capacity_bytes).ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "retained clone capacity progress overflow"))?,
        })
    }

    pub fn fits(self, grant: RetainedCloneGrant) -> bool {
        self.copied_items <= grant.maximum_items && self.copied_bytes <= grant.maximum_copy_bytes && self.retained_capacity_bytes <= grant.maximum_capacity_bytes && self.released_bytes <= grant.maximum_release_bytes
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedCloneStep {
    Progress(RetainedCloneProgress),
    Complete(RetainedCloneProgress),
}
impl RetainedCloneStep {
    pub fn progress(self) -> RetainedCloneProgress {
        match self {
            Self::Progress(progress) | Self::Complete(progress) => progress,
        }
    }
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct RetainedCloneBirthDemand {pub capacity_bytes:usize,pub depth:usize}
impl RetainedCloneBirthDemand {
    pub fn admit(self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,crate::ValueError>{
        if grant.maximum_items==0{return Err(crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit,"constructor requires one admitted item"));}
        if grant.maximum_depth<self.depth{return Err(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"constructor exceeds admitted depth"));}
        if grant.maximum_capacity_bytes<self.capacity_bytes{return Err(crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"constructor exceeds admitted capacity"));}
        Ok(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:self.capacity_bytes,..Default::default()})
    }
}
pub const RETAINED_CLONE_GRANT_SCHEMA: &str = include_str!("../../🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json");
pub const RETAINED_CLONE_PROGRESS_SCHEMA:&str=include_str!("../../🧬️retained-clone/🌐️wire/🧬️schema/🧾️progress.json");
pub fn admit_retained_clone_progress(grant: RetainedCloneGrant, progress: RetainedCloneProgress, _scope: &str) -> Result<RetainedCloneProgress, crate::ValueError> {
    if progress.fits(grant) { Ok(progress) } else { Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "original retained turn exceeded its item, copy, capacity, or release grant").with_retained_progress(progress)) }
}
pub fn admit_retained_clone_close(grant: RetainedCloneGrant, step: RetainedCloneStep, terminal: bool, scope: &str) -> Result<RetainedCloneStep, crate::ValueError> {
    admit_retained_clone_progress(grant, step.progress(), scope)?;
    if matches!(step, RetainedCloneStep::Complete(_)) && !terminal {
        return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "original retained turn completed with retained owners").with_retained_progress(step.progress()));
    }
    Ok(step)
}
/// ⚠️ Refusal identity and explicit prose ownership survive controlled codec boundaries.
/// 🏷️ Stable semantic failure identity declared by the refusal schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ValueRefusalKind { InvalidValue, Canceled, OwnershipLimit, AllocationFailed, WorkLimit, DepthLimit, UnsupportedOwner, InvariantViolated }

impl ValueRefusalKind {
    /// 🔎️ Reads only the original closed schema spelling without allocating or classifying prose.
    pub fn from_wire(value:&str)->Option<Self>{match value{"invalidValue"=>Some(Self::InvalidValue),"canceled"=>Some(Self::Canceled),"ownershipLimit"=>Some(Self::OwnershipLimit),"allocationFailed"=>Some(Self::AllocationFailed),"workLimit"=>Some(Self::WorkLimit),"depthLimit"=>Some(Self::DepthLimit),"unsupportedOwner"=>Some(Self::UnsupportedOwner),"invariantViolated"=>Some(Self::InvariantViolated),_=>None}}
    /// 🔤️ Returns the schema spelling without allocating or inspecting error prose.
    pub const fn as_str(self) -> &'static str {
        match self { Self::InvalidValue => "invalidValue", Self::Canceled => "canceled", Self::OwnershipLimit => "ownershipLimit", Self::AllocationFailed => "allocationFailed", Self::WorkLimit => "workLimit", Self::DepthLimit => "depthLimit", Self::UnsupportedOwner => "unsupportedOwner", Self::InvariantViolated => "invariantViolated" }
    }
}
/// 🚨️ An owned typed refusal with its complete dotted display path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueError { pub kind: ValueRefusalKind, pub message: std::borrow::Cow<'static, str>, retained_progress: crate::RetainedCloneProgress }
impl ValueError {
    /// 🧭️ Constructs one refusal with an explicit authority at its actual failure site.
    pub fn new(kind: ValueRefusalKind, message: impl Into<String>) -> Self { Self { kind, message: std::borrow::Cow::Owned(message.into()), retained_progress: Default::default() } }
    /// 🧱️ Returns immutable refusal prose without spending denied allocation credit.
    pub const fn literal(kind: ValueRefusalKind, message: &'static str) -> Self { Self { kind, message: std::borrow::Cow::Borrowed(message), retained_progress: crate::RetainedCloneProgress { copied_items: 0, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: 0 } } }
    /// 🧾️ Borrows actual retained effects carried by the original failed turn.
    pub fn retained_progress(&self) -> crate::RetainedCloneProgress { self.retained_progress }
    /// 📥️ Attaches the original producer receipt before its failure leaves custody.
    pub fn with_retained_progress(mut self, progress: crate::RetainedCloneProgress) -> Self { self.retained_progress = progress; self }
    /// 🪆️ Adds a parent field or ordinal while retaining the original refusal authority.
    pub fn under(self, segment: impl std::fmt::Display) -> Self { Self::new(self.kind, format!("{segment}.{}", self.message)).with_retained_progress(self.retained_progress) }
    /// 🗣️ Moves the prose at a declared terminal text-only boundary.
    pub fn into_message(self) -> String { self.message.into_owned() }
}
impl std::fmt::Display for ValueError { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str(&self.message) } }
impl std::error::Error for ValueError {}


#[path="../../⚠️refusal/🔤️utf8/🦀️.rs"]
mod utf8;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetirementDemand {
    pub copy_bytes: usize,
    pub capacity_bytes: usize,
    pub release_bytes: usize,
    pub depth: usize,
}

/// 🎟️ Preserves the exact admitted original action across its enclosing clone frame.
pub fn retained_clone_grant_admits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool {
    grant.maximum_items>0 && grant.maximum_copy_bytes>=demand.copy_bytes && grant.maximum_capacity_bytes>=demand.capacity_bytes && grant.maximum_release_bytes>=demand.release_bytes && grant.maximum_depth>=demand.depth
}
