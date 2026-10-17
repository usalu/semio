//! 🧪️ Intrinsic owned source laws use an explicit independent fixture composition.
use crate::standards::v1::subsets::any::schema::content::{jack_content_for_handle, jack_content_identity, retire_owned_to_terminal, JackContentChild};
use crate::standards::v1::subsets::any::schema::snapshot::{empty_trinity_graph_snapshot, jack_working_scene};
use semio_s_artifact_trinity_jack_graph_query::JackGraphQuery;
use crate::test_support::FixtureComposition;
#[test]fn intrinsic_owner_retains_every_selected_identity_coordinate(){let snapshot=empty_trinity_graph_snapshot::<FixtureComposition>();let owner=jack_content_for_handle(&snapshot.content).unwrap();assert_eq!(owner.read().identity(),jack_content_identity(&snapshot.content));assert_eq!((owner.read().node_count(),owner.read().edge_count()),(0,0));drop(owner);retire_owned_to_terminal(snapshot).unwrap();}
#[test]fn intrinsic_unpublished_source_is_refused(){let mut snapshot=empty_trinity_graph_snapshot::<FixtureComposition>();snapshot.content=JackContentChild::new(snapshot.content.child_id.clone(),snapshot.content.target.clone());assert!(jack_working_scene(&snapshot).is_err());retire_owned_to_terminal(snapshot).unwrap();}
