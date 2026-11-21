//! 🏗️ Jack composition: the authored native installation that selects members, live bindings, genesis and format assembly for a snapshot, and the scene and genesis reads that go through it.

use crate::standards::v1::subsets::any::schema::content::{jack_content_for_handle,jack_working_scene_from_read,JackContentFactory,JackWorkingScene};
use crate::standards::v1::subsets::any::schema::content_read::{JackContentRead,JackOwnedContent};
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use crate::standards::v1::subsets::any::schema::graph::GraphEffect;
use semio_plugin::{Fault,app::{ChildContentView,child_read::ChildReadRegistration,child_editor::registration::ChildEditorRegistration}};
use semio_framework_value::{RetireOwned, ValueError, ValueRefusalKind};

#[path="🪆️child/🦀️.rs"]
mod child;
pub use child::JackChildQualification;

/// 🏗️ The authored native installation selects members, live bindings, genesis and format assembly.
pub trait JackComposition:JackContentFactory+Default+Clone+Send+Sync+'static{
    type Members:store::ReferenceSpaceMember+store::MemberFactory+Send+'static;
    type Read<'a>:JackContentRead+'a;
    type ContentSnapshot: RetireOwned + store::StoreState;
    type ContentMutation: protocol::SemanticMutation<Self::ContentSnapshot> + protocol::OpBinary + RetireOwned;
    fn child_editor() -> Option<(&'static ChildReadRegistration<Self::ContentSnapshot>, &'static ChildEditorRegistration<Self::ContentSnapshot>)> { None }
    fn child_mutations(_: &Self::ContentSnapshot, _: &[GraphEffect]) -> Result<Vec<Self::ContentMutation>, ValueError> {
        Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "Jack composition has no content mutation producer"))
    }
    fn initial_snapshot()->JackSnapshot;
    fn example_snapshot(id:&str)->Option<JackSnapshot>;
    fn initial_viewer_snapshot()->JackSnapshot;
    fn bind<'a>(snapshot:&JackSnapshot,children:&'a ChildContentView)->Result<Self::Read<'a>,Fault>;
    fn genesis_child_pack(snapshot:&JackSnapshot,slot:&str,child_id:&str)->Result<Option<Vec<u8>>,ValueError>;
    fn examples()->&'static[semio_plugin::ExampleSource];
    fn io_entries()->&'static[store::io::io_mechanism::IoEntry];
}

/// 🧱️ Projects the composed content of a snapshot through its authored read behavior.
pub fn jack_scene_from_children<C:JackComposition>(snapshot:&JackSnapshot,children:&semio_plugin::app::ChildContentView)->Result<JackWorkingScene,semio_plugin::Fault>{let read=C::bind(snapshot,children)?;jack_working_scene_from_read(&read).map_err(|error|semio_plugin::Fault::from(error.into_message()))}

/// 🧬️ The genesis pack of the composed content child.
pub fn genesis_jack_child_pack(snapshot:&JackSnapshot,slot:&str,child_id:&str)->Result<Option<Vec<u8>>,ValueError>{if slot!="content"||child_id!=snapshot.content.child_id{return Ok(None)}jack_content_for_handle(&snapshot.content)?.read().genesis_pack().map(Some)}
