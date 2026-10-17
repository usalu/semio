//! 🪆️ Jack retains an explicit original graph capability owner and its retirement authority.
use crate::standards::v1::subsets::any::schema::content_read::JackOwnedContent;
use crate::standards::v1::subsets::any::schema::graph::{Edge,Node};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor};
use std::sync::Arc;

/// 🧷️ Type-level marker of the selected graph capability, independent of its artifact kind.
pub enum JackContentGraph{}
pub type JackContentChild=store::os_store::artifact_child::ArtifactChild<JackContentGraph>;

/// 🏭️ The native installation's constructor of the graph capability child, the one composition behavior a snapshot needs to hold a working scene.
pub trait JackContentFactory{
    fn child_from_working(nodes:Vec<Node>,edges:Vec<Edge>)->JackContentChild;
}

/// 🧳️ The original rich content behavior and address retire together under the original owner quote.
pub struct JackContentOwner{identity:JackContentChild,source:Box<dyn JackOwnedContent>}
impl JackContentOwner{
 pub fn new(identity:JackContentChild,source:Box<dyn JackOwnedContent>)->Result<Self,JackContentAdmissionFailure>{if let Err(error)=semio_s_artifact_trinity_jack_graph_query::retained_jack_graph(jack_content_identity(&identity),source.as_ref()){return Err(JackContentAdmissionFailure{error,identity,source})}Ok(Self{identity,source})}
 pub fn read(&self)->&dyn JackOwnedContent{self.source.as_ref()}
 /// 🔬️ Borrows the concrete rich source when a composition selected `T`, so it can read its own representation without a copy.
 pub fn source_as<T:JackOwnedContent>(&self)->Option<&T>{let any:&dyn std::any::Any=self.source.as_ref();any.downcast_ref::<T>()}
}
/// 🎟️ Refusal returns the actual source and selected address under their original retirement authority.
pub struct JackContentAdmissionFailure{error:ValueError,identity:JackContentChild,source:Box<dyn JackOwnedContent>}
impl JackContentAdmissionFailure{
 pub fn error(&self)->&ValueError{&self.error}
 pub fn into_parts(self)->(ValueError,JackContentChild,Box<dyn JackOwnedContent>){(self.error,self.identity,self.source)}
}
impl std::fmt::Debug for JackContentAdmissionFailure{fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{formatter.debug_struct("JackContentAdmissionFailure").field("error",&self.error).finish_non_exhaustive()}}
impl RetireOwned for JackContentAdmissionFailure{
 fn retirement(self)->Box<dyn RetirementCursor>{semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.error),semio_framework_value::retirement::deferred(self.identity),self.source.into_retirement()])}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.error),semio_framework_value::retirement::deferred_birth_bytes_for(&self.identity),self.source.retirement_birth_bytes()?])}
 fn controlled_retirement_supported()->bool{true}
}
impl RetireOwned for JackContentOwner{
 fn retirement(self)->Box<dyn RetirementCursor>{semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.identity),self.source.into_retirement()])}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.identity),self.source.retirement_birth_bytes()?])}
 fn controlled_retirement_supported()->bool{true}
}
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

/// 🪪️ Borrows every selected identity coordinate without allocating a replacement address.
pub fn jack_content_identity(handle:&JackContentChild)->semio_framework_schema_composition::ChildRefFields<'_>{semio_framework_schema_composition::ChildRefFields{child_id:&handle.child_id,artifact_id:&handle.target.artifact_id,artifact_kind:&handle.target.dialect.artifact_kind,standard:&handle.target.dialect.standard,subset:&handle.target.dialect.subset}}

/// 🧩️ Attaches one explicitly selected rich owner without replacing its original source representation.
pub fn jack_content_with_owner(handle:JackContentChild,source:Box<dyn JackOwnedContent>)->Result<JackContentChild,JackContentAdmissionFailure>{if let Err(error)=semio_s_artifact_trinity_jack_graph_query::retained_jack_graph(jack_content_identity(&handle),source.as_ref()){return Err(JackContentAdmissionFailure{error,identity:handle,source})}let identity=JackContentChild::new(handle.child_id.clone(),handle.target.clone());Ok(handle.with_local_owner(Arc::new(JackContentOwner{identity,source})))}

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
pub fn trinity_mounted_owner_policy() -> semio_framework_plugin::MountedOwnerPolicyV1 {
    semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT
}

/// 🎟️ Funds exactly one turn from the owner's own published quote, for callers no scheduler grants (standalone stores, fixtures and local scratch owners).
pub fn jack_self_funded_grant(demand: semio_framework_value::RetirementDemand) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(JACK_SELF_FUNDED_BODY_BYTES), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

/// ♻️ Drives an admitted retirement cursor through demand-quoted retained turns until its terminal-empty witness.
pub fn drive_retirement_to_terminal(mut owner: Box<dyn semio_framework_value::ErasedSnapshotRetirement>) -> Result<(), ValueError> {
    use semio_framework_value::retained_clone::RetainedCloneProgress;
    const IDLE_TURNS: usize = 4;
    let mut idle = 0;
    while !owner.terminal_is_empty() {
        let demand = owner.next_demand(JACK_SELF_FUNDED_BODY_BYTES)?;
        let progress = owner.close_step(jack_self_funded_grant(demand))?.progress();
        idle = if progress == RetainedCloneProgress::default() { idle + 1 } else { 0 };
        if idle > IDLE_TURNS {
            return Err(invalid("Jack retirement stalled under its own demand quote"));
        }
    }
    Ok(())
}

/// ♻️ Admits an owned value's retirement frame under its own quote, then drives it to the terminal-empty witness.
pub fn retire_owned_to_terminal<T: RetireOwned>(value: T) -> Result<(), ValueError> {
    let birth = jack_self_funded_grant(semio_framework_value::RetirementDemand { capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(), ..Default::default() });
    let (owner, _) = semio_framework_value::retirement::admit_owned_retirement(value, birth).map_err(|(error, _)| error)?;
    drive_retirement_to_terminal(owner)
}
/// 📢️ The localized notices of jack's own refusal codes (design §20.12).
pub fn jack_fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<[(&str, semio_framework_ui_locale::LocalizedLabel); 5]> = std::sync::LazyLock::new(|| {
        [
            ("trinity.jack.content-missing", semio_framework_ui_locale::LocalizedLabel::native("The component graph of this Jack document is not loaded.", "Der Bauteilgraph dieses Jack-Dokuments ist nicht geladen.")),
            ("trinity.jack.content-dialect", semio_framework_ui_locale::LocalizedLabel::native("The component graph of this Jack document is not supported by its graph composition.", "Der Bauteilgraph dieses Jack-Dokuments ist wird von seiner Graph-Komposition nicht unterstützt.")),
            ("trinity.jack.content-unreadable", semio_framework_ui_locale::LocalizedLabel::native("The component graph holds a value Jack cannot read.", "Der Bauteilgraph enthält einen Wert, den Jack nicht lesen kann.")),
            ("trinity.jack.layout-run.start", semio_framework_ui_locale::LocalizedLabel::native("The reorganize run could not start.", "Der Neuanordnungslauf konnte nicht starten.")),
            ("trinity.jack.retained-capacity", semio_framework_ui_locale::LocalizedLabel::native("This edit is too large to apply in one step.", "Diese Änderung ist zu groß, um sie in einem Schritt anzuwenden.")),
        ]
    });
    NOTICES.as_slice()
}

