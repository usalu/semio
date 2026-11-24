//! 🧬️ Generic owners finance native implementations while cold declarations remain unconstrained.
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::{retained_clone::RetainedClone,retirement::RetireOwned};
#[derive(semio_framework_value_derive::RetireOwned,semio_framework_value_derive::RetainedClone,serde::Serialize,serde::Deserialize)]
struct Node<K,V>{entry:Option<(K,V)>,left:Option<usize>,right:Option<usize>}
use semio_framework_value_retained_clone::*;
#[path="../../../🧬️retained-clone/🧬️contract/🧪️tests/⏱️effect/🦀️.rs"]
mod effect;
fn owns<T:RetainedClone+RetireOwned>(){}
fn drain<T:RetireOwned>(value:T){
 let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();
 for turn in 0..2000{
  if owner.terminal_is_empty(){return}
  assert!(turn<1999,"retirement stalled; quote {:?}",owner.selected_demand(4096));
  let demand=owner.selected_demand(4096).unwrap();
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
  assert!(effect::driven(grant,|e|owner.step(4096,e)).0.result.unwrap().progress().fits(grant));
 }
 panic!("retirement did not terminate");
}
#[test]
fn retained_generic_composite_fields_bound_owned_parameters_without_restricting_cold_declaration(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🧬️generic-owner/🔣️.json")).unwrap();
 let source=serde_json::json!({"entry":law["entry"],"left":law["left"],"right":law["right"]});let native:Node<String,Vec<Box<u32>>>=serde_json::from_value(source.clone()).unwrap();owns::<Node<String,Vec<Box<u32>>>>();assert_eq!(serde_json::to_value(&native).unwrap(),source);
 struct ColdOnly(std::rc::Rc<()>);let cold:Node<ColdOnly,()>=Node{entry:None,left:None,right:None};assert!(cold.entry.is_none());let marker=ColdOnly(std::rc::Rc::new(()));assert_eq!(std::rc::Rc::strong_count(&marker.0),1);
 drain(native);
}

#[derive(semio_framework_value_derive::RetireOwned)]
#[value(send_only)]
struct ExclusiveCell{close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<u8>>}
#[derive(semio_framework_value_derive::RetireOwned)]
struct SharedPlain{value:u8}
#[test]
fn send_only_owners_derive_exclusive_retirement_and_plain_owners_keep_shared_original_custody(){
 fn require_send<T:Send>(){}fn require_shared<T:Send+Sync>(){}
 require_send::<ExclusiveCell>();require_shared::<SharedPlain>();let _:fn()=require_send::<ExclusiveCell>;
 assert!(<ExclusiveCell as RetireOwned>::controlled_retirement_supported());
 assert!(ExclusiveCell{close:None}.close.is_none());
 assert!(<SharedPlain as RetireOwned>::shared_original_demands(&SharedPlain{value:1}).unwrap().is_some());
 assert!(<ExclusiveCell as RetireOwned>::retirement_birth_bytes(&ExclusiveCell{close:None}).is_some());
}

trait Definition:Send+Sync+'static{type Value:RetireOwned+Sync;}
struct OriginalDefinition;
impl Definition for OriginalDefinition{type Value=String;}
#[derive(semio_framework_value_derive::RetireOwned)]
struct ActualFields<D:Definition>{value:D::Value}
#[derive(semio_framework_value_derive::RetireOwned)]
enum Completion<D:Definition>{Value(ActualFields<D>)}
#[test]
fn composite_retirement_requires_the_real_field_port_without_retiring_its_definition_parameter(){
 let source=Completion::<OriginalDefinition>::Value(ActualFields{value:"original field".into()});assert!(source.retirement_birth_bytes().unwrap()>0);drain(source);
}

#[derive(semio_framework_value_derive::RetireOwned)]
enum Uninhabited{}
#[test]
fn uninhabited_enum_owner_derives_a_retirement_cursor(){
 owns_retire::<Uninhabited>();
 assert!(<Uninhabited as RetireOwned>::controlled_retirement_supported());
}
fn owns_retire<T:RetireOwned>(){}

macro_rules! wide_retirement_record {
    ($($field:ident),+) => {
        #[derive(semio_framework_value_derive::RetireOwned)]
        struct WideRetirementRecord { $($field: String),+ }

        impl WideRetirementRecord {
            fn from_sizes(sizes: &[usize]) -> Self {
                let mut lengths = sizes.iter().copied();
                let owner = Self { $($field: "x".repeat(lengths.next().unwrap())),+ };
                assert_eq!([$(owner.$field.capacity()),+], sizes);
                owner
            }
        }
    };
}

wide_retirement_record!(
    field00, field01, field02, field03, field04, field05, field06, field07, field08, field09,
    field10, field11, field12, field13, field14, field15, field16, field17, field18, field19,
    field20, field21, field22, field23, field24, field25, field26, field27, field28, field29,
    field30, field31, field32, field33, field34, field35, field36, field37, field38, field39,
    field40, field41, field42, field43, field44, field45, field46, field47, field48, field49,
    field50, field51, field52, field53, field54, field55, field56, field57, field58, field59,
    field60, field61, field62, field63, field64, field65
);

#[test]
fn wide_record_retires_in_field_order_within_the_original_depth() {
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🧬️generic-owner/🔣️.json")).unwrap();
    let wide = &law["wideRetirement"];
    let amount = |field: &str| wide[field].as_u64().unwrap() as usize;
    let expected: Vec<_> = (amount("firstRelease")..=amount("lastRelease")).collect();
    assert_eq!(expected.len(), amount("fields"));
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(WideRetirementRecord::from_sizes(&expected)).map_err(|(error, _)| error).unwrap();
    let mut released = Vec::with_capacity(expected.len());
    let mut admitted_bytes = 0;
    let mut released_bytes = 0;
    for _ in 0..2000 {
        if owner.terminal_is_empty() { break; }
        let demand = owner.selected_demand(4096).unwrap();
        let grant = RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: demand.copy_bytes,
            maximum_capacity_bytes: demand.capacity_bytes,
            maximum_release_bytes: demand.release_bytes,
            maximum_depth: amount("maximumDepth"),
        };
        assert!(demand.depth <= grant.maximum_depth, "wide record retirement demand: {demand:?}");
        let progress = effect::driven(grant, |effect| owner.step(4096, effect)).0.result.unwrap().progress();
        assert!(progress.fits(grant));
        admitted_bytes += progress.retained_capacity_bytes;
        released_bytes += progress.released_bytes;
        if progress.released_bytes >= amount("firstRelease") {
            released.push(progress.released_bytes);
        }
    }
    assert!(owner.terminal_is_empty());
    assert_eq!(released, expected);
    assert_eq!(released_bytes - admitted_bytes, amount("totalReleased"));
}

#[derive(semio_framework_value_derive::RetireOwned)]
#[value(send_only)]
struct ErasedHolder{close:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>}
#[derive(semio_framework_value_derive::RetireOwned)]
#[value(send_only)]
struct ErasedOuter{inner:Option<ErasedHolder>,label:String}
#[test]
fn send_only_owners_holding_erased_retirement_derive_without_sync(){
 fn require_send<T:Send+RetireOwned>(){}
 require_send::<ErasedHolder>();require_send::<ErasedOuter>();
 assert!(ErasedOuter{inner:Some(ErasedHolder{close:None}),label:String::new()}.inner.is_some());
}
