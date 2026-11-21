//! 🪆️ Jack retains an explicit original graph capability owner and its retirement authority.
use crate::standards::v1::subsets::any::schema::content_read::JackOwnedContent;
use crate::standards::v1::subsets::any::schema::graph::{Edge,Node};
use semio_framework_value::{OriginalRetirementFailure, RetainedAllocationEffect, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{OriginalEffectReply, OriginalEffectReplyComplete};
use semio_framework_value::retirement::{RetireOwned, RetirementCursor, RetirementStep, original_box::BoxedOwner};
use std::sync::{Arc, Mutex};

/// 🧷️ Type-level marker of the selected graph capability, independent of its artifact kind.
pub enum JackContentGraph{}
pub type JackContentChild=store::os_store::artifact_child::ArtifactChild<JackContentGraph>;

/// 🏭️ The native installation's constructor of the graph capability child, the one composition behavior a snapshot needs to hold a working scene.
pub trait JackContentFactory{
    fn child_from_working(nodes:Vec<Node>,edges:Vec<Edge>)->JackContentChild;
}

/// 🧳️ The original rich content behavior and address retire together under the original owner quote.
#[derive(semio_framework_value::RetireOwned)]
pub struct JackContentOwner{identity:JackContentChild,source:JackContentSource}
impl JackContentOwner{
 pub fn new<T: JackOwnedContent + RetireOwned>(identity:JackContentChild,source:Box<T>)->Result<Self,JackContentAdmissionFailure<T>>{
    if let Err(error) = check_content_source(&identity, source.as_ref()) {
        return Err(JackContentAdmissionFailure { error, identity, source });
    }
    Ok(Self::from_admitted(identity, source))
 }
 fn from_admitted<T: JackOwnedContent + RetireOwned>(identity: JackContentChild, source: Box<T>) -> Self {
    Self { identity, source: JackContentSource(Box::new(JackContentFrame { original: Some(source), cursor: Mutex::new(None) })) }
 }
 pub fn read(&self)->&dyn JackOwnedContent{self.source.0.read()}
 /// 🔬️ Borrows the concrete rich source when a composition selected `T`, so it can read its own representation without a copy.
 pub fn source_as<T:JackOwnedContent>(&self)->Option<&T>{let any:&dyn std::any::Any=self.read();any.downcast_ref::<T>()}
}
/// 🎟️ Refusal returns the actual source and selected address under their original retirement authority.
#[derive(semio_framework_value::RetireOwned)]
pub struct JackContentAdmissionFailure<T: JackOwnedContent + RetireOwned>{error:ValueError,identity:JackContentChild,source:Box<T>}
impl<T: JackOwnedContent + RetireOwned> JackContentAdmissionFailure<T>{
 pub fn error(&self)->&ValueError{&self.error}
 pub fn into_parts(self)->(ValueError,JackContentChild,Box<T>){(self.error,self.identity,self.source)}
}
impl<T: JackOwnedContent + RetireOwned> std::fmt::Debug for JackContentAdmissionFailure<T>{fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{formatter.debug_struct("JackContentAdmissionFailure").field("error",&self.error).finish_non_exhaustive()}}

trait RetainedJackContent: Send + Sync {
    fn read(&self) -> &dyn JackOwnedContent;
    fn into_cursor(self: Box<Self>) -> Box<dyn RetirementCursor>;
}

struct JackContentSource(Box<dyn RetainedJackContent>);
impl RetireOwned for JackContentSource {
    type RetirementCursor = Box<dyn RetirementCursor>;
    fn retirement_seed(self) -> Self::RetirementCursor { self.0.into_cursor() }
    fn controlled_retirement_supported() -> bool { true }
}

struct JackContentFrame<T: RetireOwned> {
    original: Option<Box<T>>,
    cursor: Mutex<Option<BoxedOwner<T>>>,
}

impl<T: JackOwnedContent + RetireOwned> RetainedJackContent for JackContentFrame<T> {
    fn read(&self) -> &dyn JackOwnedContent { self.original.as_deref().expect("live Jack content") }
    fn into_cursor(mut self: Box<Self>) -> Box<dyn RetirementCursor> {
        let original = self.original.take().expect("live Jack content");
        *self.cursor.get_mut().expect("exclusive Jack content close") = Some(original.retirement_seed());
        self
    }
}

impl<T: RetireOwned> RetirementCursor for JackContentFrame<T> {
    fn next_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.cursor.lock().expect("Jack content close").as_ref().expect("selected Jack content close").next_demand(body)
    }
    fn next_computational_work(&self, body: usize) -> Result<u64, ValueError> {
        self.cursor.lock().expect("Jack content close").as_ref().expect("selected Jack content close").next_computational_work(body)
    }
    fn terminal_is_empty(&self) -> bool {
        self.cursor.lock().expect("Jack content close").as_ref().is_some_and(RetirementCursor::terminal_is_empty)
    }
    fn terminal_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>() + size_of::<Box<dyn RetirementCursor>>())
    }
    fn close_into<'r>(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>, reply: OriginalEffectReply<'r, RetirementStep, OriginalRetirementFailure>) -> Result<OriginalEffectReplyComplete<'r, RetirementStep, OriginalRetirementFailure>, ValueError> {
        self.cursor.get_mut().expect("exclusive Jack content close").as_mut().expect("selected Jack content close").close_into(body, effect, reply)
    }
    fn recover_close_into<'r>(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>, reply: OriginalEffectReply<'r, RetirementStep, OriginalRetirementFailure>) -> Result<OriginalEffectReplyComplete<'r, RetirementStep, OriginalRetirementFailure>, ValueError> {
        self.cursor.get_mut().expect("exclusive Jack content close").as_mut().expect("selected Jack content close").recover_close_into(body, effect, reply)
    }
}
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

fn check_content_source<T: JackOwnedContent + RetireOwned>(handle: &JackContentChild, source: &T) -> Result<(), ValueError> {
    semio_s_artifact_trinity_jack_graph_query::retained_jack_graph(jack_content_identity(handle), source)?;
    if !T::controlled_retirement_supported() || !source.retirement_ready() {
        return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "Jack content has no ready typed retirement owner"));
    }
    Ok(())
}

/// 🪪️ Borrows every selected identity coordinate without allocating a replacement address.
pub fn jack_content_identity(handle:&JackContentChild)->semio_framework_schema_composition::ChildRefFields<'_>{semio_framework_schema_composition::ChildRefFields{child_id:&handle.child_id,artifact_id:&handle.target.artifact_id,artifact_kind:&handle.target.dialect.artifact_kind,standard:&handle.target.dialect.standard,subset:&handle.target.dialect.subset}}

/// 🧩️ Attaches one explicitly selected rich owner without replacing its original source representation.
pub fn jack_content_with_owner<T: JackOwnedContent + RetireOwned>(handle: JackContentChild, source: Box<T>) -> Result<JackContentChild, JackContentAdmissionFailure<T>> {
    if let Err(error) = check_content_source(&handle, source.as_ref()) {
        return Err(JackContentAdmissionFailure { error, identity: handle, source });
    }
    let identity = JackContentChild::new(handle.child_id.clone(), handle.target.clone());
    Ok(handle.with_local_owner(Arc::new(JackContentOwner::from_admitted(identity, source))))
}

/// 🔎️ Admits the retained owner by all five selected coordinates before an entity is read.
pub fn jack_content_for_handle(handle:&JackContentChild)->Result<Arc<JackContentOwner>,ValueError>{let owner=handle.require_local_owner::<JackContentOwner>().map_err(|error|invalid(error.to_string()))?;semio_s_artifact_trinity_jack_graph_query::retained_jack_graph(jack_content_identity(handle),owner.read())?;Ok(owner)}
pub(crate) fn jack_local_graph_query(owner:&JackContentOwner)->&dyn semio_s_artifact_trinity_jack_graph_query::JackGraphQuery{owner.read()}

#[derive(Clone,Debug)]
pub struct JackWorkingScene{pub nodes:Vec<Node>,pub edges:Vec<Edge>}

/// 🧱️ A query workspace projects admitted entities while the selected source remains retained.
pub fn jack_working_scene_from_read(source:&dyn crate::standards::v1::subsets::any::schema::content_read::JackContentProjection)->Result<JackWorkingScene,ValueError>{let(nodes,edges)=source.projection_counts();if nodes>16_384||edges>16_384{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query scene exceeds its entity admission"))}let mut scene=JackWorkingScene{nodes:Vec::new(),edges:Vec::new()};for index in 0..nodes{scene.nodes.push(source.query_node(index,4_096)?.ok_or_else(||invalid("selected query node is not retained"))?);}for index in 0..edges{scene.edges.push(source.query_edge(index,4_096)?.ok_or_else(||invalid("selected query edge is not retained"))?);}Ok(scene)}
pub fn jack_working_scene_for_handle(handle:&JackContentChild)->Result<JackWorkingScene,ValueError>{let owner=jack_content_for_handle(handle)?;jack_working_scene_from_read(owner.read())}

/// 🧮️ The copy-byte body every self-funded Jack retirement turn quotes against.
pub const JACK_SELF_FUNDED_BODY_BYTES: usize = 65_536;

/// 🏇️ The mounted-owner policy every Trinity component declares for its preparation, maintenance and close lanes.
pub fn trinity_mounted_owner_policy() -> semio_plugin::MountedOwnerPolicyV1 {
    semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT
}

/// 🎟️ Funds exactly one turn from the owner's own published quote, for callers no scheduler grants (standalone stores, fixtures and local scratch owners).
pub fn jack_self_funded_grant(demand: semio_framework_value::RetirementDemand) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(JACK_SELF_FUNDED_BODY_BYTES), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

/// ♻️ Drives an admitted retirement cursor through demand-quoted retained turns until its terminal-empty witness.
pub fn drive_retirement_to_terminal(mut owner: Box<dyn semio_framework_value::ErasedSnapshotRetirement>) -> Result<(), ValueError> {
    use semio_framework_value::RetainedWorkPoll;
    let pump = semio_framework_job::OriginalRetirementPump::new();
    const IDLE_TURNS: usize = 4;
    let mut idle = 0;
    while !owner.terminal_is_empty() {
        let demand = owner.next_demand(JACK_SELF_FUNDED_BODY_BYTES)?;
        let work = owner.next_computational_work(JACK_SELF_FUNDED_BODY_BYTES)?;
        let (poll, _) = pump.turn(work.max(1), u64::MAX, || Some(0), jack_self_funded_grant(demand))?
            .with_effect(|effect| owner.close_step(JACK_SELF_FUNDED_BODY_BYTES, effect))?
            .ok_or_else(|| invalid("Jack retirement turn was not admitted"))?;
        let progressed = match poll {
            RetainedWorkPoll::Pending { work } => work > 0,
            RetainedWorkPoll::Complete(turn) => {
                let step = turn.result.map_err(OriginalRetirementFailure::into_value_error)?;
                turn.work > 0 || step.progress() != Default::default()
            }
        };
        idle = if progressed { 0 } else { idle + 1 };
        if idle > IDLE_TURNS {
            return Err(invalid("Jack retirement stalled under its own demand quote"));
        }
    }
    Ok(())
}

/// ♻️ Admits an owned value's retirement frame under its own quote, then drives it to the terminal-empty witness.
pub fn retire_owned_to_terminal<T: RetireOwned>(value: T) -> Result<(), ValueError> {
    use semio_framework_value::retirement::{admit_owned_retirement, controlled::controlled_retirement_birth_demand};
    let birth = jack_self_funded_grant(controlled_retirement_birth_demand(&value)?);
    let mut original = Some(value);
    let (turn, _) = semio_framework_job::OriginalRetirementPump::new().turn(8, u64::MAX, || Some(0), birth)?
        .with_effect(|effect| admit_owned_retirement(&mut original, effect))?
        .ok_or_else(|| invalid("Jack retirement birth turn was not admitted"))?;
    let (owner, _) = turn.result?.ok_or_else(|| invalid("Jack retirement frame was not admitted"))?;
    drive_retirement_to_terminal(owner)
}
/// 📢️ The localized notices of jack's own refusal codes (design §20.12).
pub fn jack_fault_notices() -> &'static [(&'static str, semio_ui_locale::LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<[(&str, semio_ui_locale::LocalizedLabel); 5]> = std::sync::LazyLock::new(|| {
        [
            ("trinity.jack.content-missing", semio_ui_locale::LocalizedLabel::native("The component graph of this Jack document is not loaded.", "Der Bauteilgraph dieses Jack-Dokuments ist nicht geladen.")),
            ("trinity.jack.content-dialect", semio_ui_locale::LocalizedLabel::native("The component graph of this Jack document is not supported by its graph composition.", "Der Bauteilgraph dieses Jack-Dokuments ist wird von seiner Graph-Komposition nicht unterstützt.")),
            ("trinity.jack.content-unreadable", semio_ui_locale::LocalizedLabel::native("The component graph holds a value Jack cannot read.", "Der Bauteilgraph enthält einen Wert, den Jack nicht lesen kann.")),
            ("trinity.jack.layout-run.start", semio_ui_locale::LocalizedLabel::native("The reorganize run could not start.", "Der Neuanordnungslauf konnte nicht starten.")),
            ("trinity.jack.retained-capacity", semio_ui_locale::LocalizedLabel::native("This edit is too large to apply in one step.", "Diese Änderung ist zu groß, um sie in einem Schritt anzuwenden.")),
        ]
    });
    NOTICES.as_slice()
}

