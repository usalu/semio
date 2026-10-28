//! 🧭️ IFC4 profile for complete catalog-aware snapshot details.
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::catalog::{IfcCatalog,IfcAttributeDeclaration,IfcTypeExpression};
#[path="../../../../../../🔎️details/🦀️.rs"]mod shared;
pub type Ifc4DetailsProvider<'a>=shared::IfcDetailsProvider<'a,Ifc4DetailsProfile>;
pub struct Ifc4DetailsProfile;
impl shared::IfcDetailsProfile for Ifc4DetailsProfile{type CandidateRules=();const MAXIMUM_ATTRIBUTES:usize=semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::catalog::IFC4_MAXIMUM_ATTRIBUTES;fn catalog()->&'static IfcCatalog{semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::catalog::ifc4_catalog()}}

pub fn details_edit_is_admitted(event:&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent,snapshot:&semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot)->bool{shared::details_edit_is_admitted::<Ifc4DetailsProfile>(event,snapshot)}
pub fn details_edit_retained_path_is_admitted(event:&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent,snapshot:&semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot)->bool{shared::details_edit_retained_path_is_admitted::<Ifc4DetailsProfile>(event,snapshot)}
pub fn details_edit_special(event:&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent,snapshot:&semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot)->Result<Option<Vec<semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::IfcMutation>>,semio_framework_plugin::Fault>{shared::details_edit_special::<Ifc4DetailsProfile>(event,snapshot)}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

pub fn details_work_capacity()->semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity{shared::details_work_capacity::<Ifc4DetailsProfile>()}
pub fn details_edit_work<E>(tool:&'static str)->Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<E>>> where E:semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor<Snapshot=semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot,Mutation=semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::IfcMutation>{Box::new(shared::IfcDetailsWork::<E,Ifc4DetailsProfile>::new(tool))}

pub fn details_edit_special_controlled(event:&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent,snapshot:&semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot,tick:&mut dyn FnMut()->Result<(),String>)->Result<Option<Vec<semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::IfcMutation>>,semio_framework_plugin::Fault>{shared::details_edit_special_controlled::<Ifc4DetailsProfile>(event,snapshot,tick)}

#[cfg(test)]type DetailsTestEditor=crate::editor::ifc4_any::Ifc4AnyEditor;
#[cfg(test)]type DetailsTestProfile=Ifc4DetailsProfile;
#[cfg(test)]fn details_test_command(event:semio_s_artifact_stdio_contract::editing::SnapshotEditEvent)->crate::editor::ifc4_any::Ifc4AnyEditCommand{crate::editor::ifc4_any::Ifc4AnyEditCommand::EditSnapshot{event}}
