//! 🧪️ Intrinsic owned source laws use an explicit independent fixture composition.
use crate::standards::v1::subsets::any::schema::content::{jack_content_for_handle, jack_content_identity, retire_owned_to_terminal, JackContentChild};
use crate::standards::v1::subsets::any::schema::snapshot::{empty_trinity_graph_snapshot, jack_working_scene};
use semio_s_artifact_trinity_jack_graph_query::JackGraphQuery;
use semio_os_store_test_support::FixtureComposition;
#[test]fn intrinsic_owner_retains_every_selected_identity_coordinate(){let snapshot=empty_trinity_graph_snapshot::<FixtureComposition>();let owner=jack_content_for_handle(&snapshot.content).unwrap();assert_eq!(owner.read().identity(),jack_content_identity(&snapshot.content));assert_eq!((owner.read().node_count(),owner.read().edge_count()),(0,0));drop(owner);retire_owned_to_terminal(snapshot).unwrap();}
#[test]fn intrinsic_unpublished_source_is_refused(){let mut snapshot=empty_trinity_graph_snapshot::<FixtureComposition>();snapshot.content=JackContentChild::new(snapshot.content.child_id.clone(),snapshot.content.target.clone());assert!(jack_working_scene(&snapshot).is_err());retire_owned_to_terminal(snapshot).unwrap();}
/// ⚖️ LAW: the canonical empty example id `""` loads the blank graph document, and an unknown id loads nothing.
#[test]fn set_active_example_empty_id_loads_the_blank_graph_document(){
 use crate::standards::v1::subsets::any::schema::composition::JackComposition;
 let emit=crate::editor::jack::commands::set_active_example::set_active_example::<FixtureComposition>("");
 let mut blank=FixtureComposition::initial_viewer_snapshot();blank.query=crate::standards::v1::subsets::any::schema::graph::TRINITY_JACK_DEFAULT_QUERY.into();
 assert_eq!(emit.effects.len(),1);
 assert!(format!("{:?}",emit.effects[0])==format!("{:?}",crate::editor::jack::reset_document_effect(&blank)));
 assert!(crate::editor::jack::commands::set_active_example::set_active_example::<FixtureComposition>("not-an-example").effects.is_empty());
}
