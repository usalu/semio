//! 🗣️ The GUEST half of the resolved host context, driven by the SAME language-agnostic
//! `🪟️view-context/🧫️fixtures/🪟️resolved-host-context/🔣️.json` the TypeScript admission twin
//! (`🪟️view-context/🧪️tests/🪟️resolved-host-context/🟦️.ts`) reads.
//!
//! The fixture's `guestDecode` rows pin the exact fault a plugin raises for a context that never
//! passed through `parseResolvedPluginViewState`. Before this law only the host side was pinned, so
//! a host path that dispatched a raw `ActiveSession.viewState` (`{ activeModeId }`, no preferences)
//! produced the anonymous `missing field `locale`` app fault with nothing naming the crossing —
//! every deferred `flowEvalTick` re-arm in the browser
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use super::*;
#[path="../../../🌱️value/♻️retirement/📦️owner/🧪️work/effect.rs"]
mod effect;

/// 🗑️ Owns one decoded value whose collections must be retired through paid turns before it drops.
pub(crate) struct Retiring<T:semio_framework_value::retirement::RetireOwned>(Option<T>);
impl<T:semio_framework_value::retirement::RetireOwned> Retiring<T>{
    pub(crate) fn new(value:T)->Self{Self(Some(value))}
}
impl<T:semio_framework_value::retirement::RetireOwned> std::ops::Deref for Retiring<T>{type Target=T;fn deref(&self)->&T{self.0.as_ref().expect("the value is retired only on drop")}}
impl<T:semio_framework_value::retirement::RetireOwned+std::fmt::Debug> std::fmt::Debug for Retiring<T>{fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{self.0.fmt(formatter)}}
impl<T:semio_framework_value::retirement::RetireOwned> Drop for Retiring<T>{
    fn drop(&mut self){
        let Some(value)=self.0.take()else{return};
        if std::thread::panicking(){return}
        use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
        let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("the decoded value requires controlled retirement"));
        while !owner.terminal_is_empty(){
            let copy=owner.next_copy_byte_demand().expect("retirement copy quote");let release=owner.next_release_byte_demand().expect("retirement release quote");
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release}).expect("retirement capacity quote"),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().expect("retirement depth quote")};
            let work=owner.next_computational_work(copy).expect("retirement work quote").max(1);
            let poll=effect::with_effect(grant,work,&mut None,|effect|owner.step(grant.maximum_copy_bytes,effect));
            effect::settled(poll).result.unwrap_or_else(|_|panic!("the decoded value retires under its own quotes"));
        }
    }
}

#[test]
fn extension_inputs_match_the_independent_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🪟️view-context/🧪️tests/🧫️fixtures/🧩️extension-input/🔣️.json")).expect("extension input vectors");
    for row in fixture["cases"].as_array().expect("cases") {
        let input = &row["context"];
        let view = Retiring::new(semio_framework_pack_json::from_json_str::<ViewModel>(&input.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned context decoder"));
        let oracle = Retiring::new(serde_json::from_value::<ViewModel>(input.clone()).expect("independent context decoder"));
        assert_eq!(*view, *oracle);
        assert_eq!(view.extension_input_json.as_deref(), input["extensionInputJson"].as_str());
        let panel = Retiring::new(view.for_panel());
        assert_eq!(panel.extension_input_json, view.extension_input_json);
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct HostContextFixture {
    valid: serde_json::Value,
    invalid: Vec<InvalidRow>,
    guest_decode: Vec<GuestDecodeRow>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct InvalidRow {
    name: String,
    #[serde(default)]
    remove: Vec<String>,
    #[serde(default)]
    set: serde_json::Map<String, serde_json::Value>,
    #[serde(default)]
    repeat: Option<RepeatRow>,
}

/// 📏️ A capacity row the fixture states rather than inlines — a 65 537-character literal would make
/// the shared fixture unreadable in both twins.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RepeatRow {
    field: String,
    character: String,
    count: usize,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GuestDecodeRow {
    name: String,
    fault: String,
}

fn fixture() -> HostContextFixture {
    serde_json::from_str(include_str!("../../🪟️view-context/🧪️tests/🧫️fixtures/🪟️resolved-host-context/🔣️.json")).expect("the resolved-host-context fixture must parse")
}

fn row_json(valid: &serde_json::Value, row: &InvalidRow) -> String {
    let mut object = valid.as_object().expect("the valid context is an object").clone();
    for key in &row.remove {
        object.remove(key);
    }
    for (key, value) in &row.set {
        object.insert(key.clone(), value.clone());
    }
    if let Some(repeat) = &row.repeat {
        object.insert(repeat.field.clone(), serde_json::Value::String(repeat.character.repeat(repeat.count)));
    }
    serde_json::Value::Object(object).to_string()
}

#[semio_framework_async_macros::async_test]
async fn the_resolved_context_decodes_in_the_guest() {
    let fixture = fixture();
    let view = Retiring::new(semio_framework_pack_json::from_json_str::<ViewModel>(&fixture.valid.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("a resolved host context decodes"));
    assert_eq!(view.locale, Locale::De);
    assert_eq!(view.terminology, Terminology::Reuse);
    assert_eq!(view.window_instances.len(), 2);
    assert_eq!(view.tool_run_trace_cursor_by_window_id.get("right"), Some(&semio_framework_tool_run::ToolRunTraceCursor { run: 3, generation: 1, page: 12 }));
    let right = Retiring::new(view.for_window_instance("right").expect("right window"));
    assert_eq!(right.tool_run_trace_cursor_by_window_id, view.tool_run_trace_cursor_by_window_id, "a per-window projection keeps every echoed cursor");
}

#[semio_framework_async_macros::async_test]
async fn an_unresolved_context_names_the_field_it_lost() {
    let fixture = fixture();
    assert!(!fixture.guest_decode.is_empty(), "the fixture must pin at least one guest-side rejection");
    for expected in &fixture.guest_decode {
        let row = fixture.invalid.iter().find(|candidate| candidate.name == expected.name).unwrap_or_else(|| panic!("guestDecode row {} must also be an `invalid` row the host admission rejects", expected.name));
        let error = semio_framework_pack_json::from_json_str::<ViewModel>(&row_json(&fixture.valid, row), semio_framework_pack_json::JsonMemberPolicy::Reject).map(Retiring::new).err().unwrap_or_else(|| panic!("{} must not decode in the guest", expected.name));
        assert_eq!(error.to_string(), expected.fault, "{}", expected.name);
    }
}

/// ⚖️ LAW: contributions never reach a guest through a view context. The host publisher installs
/// them by the paged `setContributions` run (`🛠️ShellHelpers/🧩️contributions/🟦️.ts`) and the guest
/// folds them into its own registry (`🌊️flow/📔️registry/🦀️.rs`), so even a context that smuggles
/// the field decodes into a `ViewModel` that cannot expose it — there is no reader to route.
#[semio_framework_async_macros::async_test]
async fn a_smuggled_contributions_field_reaches_no_guest_reader() {
    let fixture = fixture();
    let row = fixture.invalid.iter().find(|candidate| candidate.name == "contributions-in-view-state").expect("the fixture must pin a contributions row");
    assert_eq!(row.set.keys().collect::<Vec<_>>(), vec!["contributionsJson"], "the row must set exactly the refused field");
    let view = Retiring::new(semio_framework_pack_json::from_json_str::<ViewModel>(&row_json(&fixture.valid, row), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("an unknown field is ignored, never routed"));
    let reprojected = serde_json::to_string(&*view).expect("the guest projection encodes");
    assert!(!reprojected.contains("contributionsJson"), "no guest reader may re-emit contributions from a view context: {reprojected}");
}

/// 🧬️ The one long field the contract keeps, pinned at the schema capacity the TypeScript twin
/// asserts against Ajv — `panelJson` at capacity decodes, and one character past it is the row the
/// host admission refuses.
#[semio_framework_async_macros::async_test]
async fn the_panel_capacity_row_is_the_only_long_field() {
    let fixture = fixture();
    let row = fixture.invalid.iter().find(|candidate| candidate.name == "oversized-panel").expect("the fixture must pin a panel capacity row");
    let repeat = row.repeat.as_ref().expect("the capacity row is stated, never inlined");
    assert_eq!(repeat.field, "panelJson");
    assert_eq!(repeat.count, VIEW_CONTEXT_LONG_STRING_CHARS + 1, "the refused row is exactly one character past the schema bound");
    let view = Retiring::new(semio_framework_pack_json::from_json_str::<ViewModel>(&row_json(&fixture.valid, row), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the guest decodes what the host admission refuses"));
    assert_eq!(view.panel_json.as_ref().map(|json| json.chars().count()), Some(repeat.count));
}

#[test]
fn view_model_original_retirement_preserves_every_field_and_physical_backing(){
 use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🪟️view-context/🧪️tests/🧫️fixtures/♻️original/🔣️.json")).unwrap();
 for context in fixture["contexts"].as_array().unwrap(){
  let input=context.to_string();let oracle=Retiring::new(serde_json::from_value::<ViewModel>(context.clone()).unwrap());
  let(original,heap)=observe(||semio_framework_pack_json::from_json_str::<ViewModel>(&input,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
  assert_eq!(original,*oracle);assert!(!heap.overflowed);let original_bytes=heap.requested_bytes-heap.released_bytes;
  let pointer=original.active_mode_id.as_ref().unwrap().as_ptr();
  let(mut owner,heap)=observe(||ControlledRetirement::new(original).unwrap_or_else(|_|panic!("original view context refused")));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  let(poll,heap)=effect::with_effect(RetainedCloneGrant::default(),1<<20,&mut None,|effect|observe(||owner.step(0,effect)));let step=effect::settled(poll).result.unwrap();assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.original().unwrap().active_mode_id.as_ref().unwrap().as_ptr(),pointer);
  let mut allocated=0;let mut released=0;let mut turns=0;
  for _ in 0..100000{
   if owner.terminal_is_empty(){break;}
   let body=fixture["copyGrant"].as_u64().unwrap()as usize;let copy=owner.next_copy_byte_demand().unwrap().max(body);let release=owner.next_release_byte_demand().unwrap();
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if body>0{body}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};
   let units=owner.next_computational_work(body).unwrap().max(1);let(poll,heap)=effect::with_effect(grant,units,&mut None,|effect|observe(||owner.step(body,effect)));let step=effect::settled(poll).result.unwrap();let progress=step.progress();assert!(progress.fits(grant));assert!(!heap.overflowed);assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);assert_eq!(heap.released_bytes,progress.released_bytes);allocated+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;
  }
  assert!(owner.terminal_is_empty());assert_eq!(original_bytes+allocated,released);let(_,heap)=observe(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }
}