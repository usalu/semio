use super::*;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_value::RetainedCloneGrant;
#[path="guided-type-tests.rs"]
mod type_guidance;
#[path="guided-type-multiple-tests.rs"]
mod type_multiple_guidance;
#[path="guided-type-scalar-tests.rs"]
mod type_scalar_guidance;
#[path="guided-type-enum-tests.rs"]
mod type_enum_guidance;
#[path="guided-type-number-binary-tests.rs"]
mod type_number_binary_guidance;
#[path="guided-type-list-tests.rs"]
mod type_list_guidance;
#[path="guided-type-select-tests.rs"]
mod type_select_guidance;
#[path="guided-type-reference-select-tests.rs"]
mod type_reference_select_guidance;
#[path="guided-type-mixed-select-tests.rs"]
mod type_mixed_select_guidance;
#[path="guided-type-set-select-tests.rs"]
mod type_set_select_guidance;
#[path="guided-type-measure-select-tests.rs"]
mod type_measure_select_guidance;

#[test]
fn contextual_details_body_uses_original_retained_work_only() {
    assert!(Ifc4x3AnyEditor::uses_render_work(editing::SNAPSHOT_DETAILS_BODY_KEY));
    assert!(!Ifc4x3AnyEditor::uses_render_work(MeshWindowKit::KIND_ID));
    let mut caller = OriginalWorkFixture::new();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65536, maximum_capacity_bytes: 65536, maximum_release_bytes: 65536, maximum_depth: 64 };
    assert!(caller.turn(RetainedCloneGrant { maximum_items: 0, ..grant }, |cx| Ifc4x3AnyEditor::begin_render_work(editing::SNAPSHOT_DETAILS_BODY_KEY,cx)).unwrap().is_none());
    let work=caller.turn(grant, |cx| Ifc4x3AnyEditor::begin_render_work(editing::SNAPSHOT_DETAILS_BODY_KEY,cx)).unwrap().unwrap();
    let mut close=semio_framework_value::retirement::controlled::ControlledRetirement::new(work).ok().unwrap();
    for _ in 0..1024 {
        if close.terminal_is_empty(){caller.finish();return}
        caller.turn(grant, |cx| {let demand=close.next_demand(4096)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|close.step(grant))?;Ok(())}).unwrap();
    }
    panic!("original contextual render owner did not retire");
}

fn staged_grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:256}}
fn retire_staged<T:semio_framework_value::retirement::RetireOwned>(caller:&mut OriginalWorkFixture,owner:&mut Option<T>){let mut close=semio_framework_value::retained_clone::RetainedCloneClose::default();for _ in 0..1_000_000{if owner.is_none()&&close.is_empty(){return;}caller.turn(staged_grant(),|cx|cx.with_original_retirement(|cx|{let demand=close.next_owner_demands::<T>(owner.is_some(),&None,32768)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|if close.is_empty(){close.begin_granted(owner,grant)?.ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"missing staged original"))}else{close.step_granted(grant)})?;Ok(())})).unwrap();}panic!("staged original did not retire");}
fn staged_view()->semio_framework_plugin::ViewModel{let mut view=semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::De,semio_framework_ui_locale::Terminology::Native);view.window_id=Some("actual-details".into());view}
fn staged_request(application:&serde_json::Value)->semio_framework_plugin::action_staging::StagedFormRequest{let parsed=semio_framework_pack_json::parse(&application.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();semio_framework_plugin::action_staging::StagedFormRequest{token:semio_framework_plugin::action_staging::StagedFormToken{action:APPLY_TEMPLATE_ACTION.into(),generation:"1".into(),revision:"0".repeat(64),window:"actual-details".into()},args:semio_framework_pack_json::to_dsl_value(&parsed),locale:semio_framework_ui_locale::Locale::De,page:None}}
#[test]
fn guided_controller_actual_hook_keeps_denied_and_wrong_context_donors_stationary(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧬️schema/🏷️templates/✍️apply/native-cases.json")).unwrap();let mut caller=OriginalWorkFixture::new();let view=staged_view();let mut work=caller.work_turn(staged_grant(),1000,|cx|Ifc4x3AnyEditor::begin_render_work(semio_framework_plugin::action_staging::STAGED_FORM_BODY_KEY,cx)).unwrap().unwrap();
    for key in ["window","locale","generation","action"]{let mut request=staged_request(&law["cases"][0]["application"]);match key{"window"=>request.token.window="other".into(),"locale"=>request.locale=semio_framework_ui_locale::Locale::En,"action"=>request.token.action="caller-producer".into(),_=>request.token.generation="18446744073709551616".into()}let mut donor=Some(request);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.work_turn(staged_grant(),1000,|cx|Ifc4x3AnyEditor::receive_render_work_request(&mut work,&mut donor,&view,cx)));assert!(result.is_err());assert!(donor.is_some());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
    let mut donor=Some(staged_request(&law["cases"][0]["application"]));for axis in 0..4{let stock=staged_grant();let grant=match axis{0=>RetainedCloneGrant{maximum_items:0,..stock},1=>RetainedCloneGrant{maximum_copy_bytes:0,..stock},2=>RetainedCloneGrant{maximum_capacity_bytes:0,..stock},_=>RetainedCloneGrant{maximum_depth:0,..stock}};let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.work_turn(grant,1000,|cx|Ifc4x3AnyEditor::receive_render_work_request(&mut work,&mut donor,&view,cx)));assert!(result.unwrap().is_none());assert!(donor.is_some());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
    retire_staged(&mut caller,&mut donor);retire_staged(&mut caller,&mut Some(work));caller.finish();
}
#[test]
fn guided_controller_builds_actual_owned_ready_and_incomplete_forms_without_publication(){
    use semio_framework_plugin::{AppRenderOperationContext,ChildContentView};use semio_framework_ui_contract::{Component,ContainerRole,OriginalBuiltTreeRetirement};
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧬️schema/🏷️templates/✍️apply/native-cases.json")).unwrap();let row=&law["cases"][0];let snapshot=std::sync::Arc::new(crate::io::parse_ifc4x3_step(row["source"].as_str().unwrap()).unwrap());let history=semio_framework_plugin::HistoryView::empty();let view=staged_view();
    for mode in ["ready","empty","partial","wrong-revision"]{let mut application=row["application"].clone();if mode=="empty"{application["identities"]=serde_json::json!([]);}else if mode=="partial"{application["identities"][0]["id"]="-".into();}let operation=AppRenderOperationContext{app_instance_id:17,base_revision:semio_framework_job::RevisionId(0),generation:semio_framework_job::Generation(1),canonical_base_revision:if mode=="wrong-revision"{[1;32]}else{[0;32]},mounted_policy:semio_framework_plugin::app_testing::artifact_app_laws::fixture_mounted_policy()};let doc=semio_framework_async::poll::resolve_ready(ArtifactView::with_render_context(snapshot.as_ref(),&history,ChildContentView::EMPTY,operation,"actual-source",None));let mut caller=OriginalWorkFixture::new();let mut work=caller.work_turn(staged_grant(),1000,|cx|Ifc4x3AnyEditor::begin_render_work(semio_framework_plugin::action_staging::STAGED_FORM_BODY_KEY,cx)).unwrap().unwrap();let mut donor=Some(staged_request(&application));let mut accepted=false;for _ in 0..16{if caller.work_turn(staged_grant(),1000,|cx|Ifc4x3AnyEditor::receive_render_work_request(&mut work,&mut donor,&view,cx)).unwrap().is_some(){accepted=true;break;}}assert!(accepted&&donor.is_none());let mut output=None;let mut refusal=None;
        let substitute=std::sync::Arc::new(IfcSnapshot::default());let count=std::sync::Arc::strong_count(&snapshot);
        let (mismatch,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.work_turn(staged_grant(),1000,|cx|Ok(work.render(semio_framework_plugin::action_staging::STAGED_FORM_BODY_KEY,&substitute,&doc,&view,cx))).unwrap());
        assert!(mismatch.is_err());assert_eq!(std::sync::Arc::strong_count(&snapshot),count);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut mismatch=Some(mismatch.err().unwrap());retire_staged(&mut caller,&mut mismatch);
        for _ in 0..100000{match caller.work_turn(staged_grant(),1000,|cx|Ok(work.render(semio_framework_plugin::action_staging::STAGED_FORM_BODY_KEY,&snapshot,&doc,&view,cx))).unwrap(){Ok(Some(tree))=>{output=Some(tree);break;},Ok(None)=>{},Err(failure)=>{refusal=Some(failure);break;}}}
        if mode=="wrong-revision"{assert!(output.is_none()&&refusal.is_some());retire_staged(&mut caller,&mut refusal);}else{let tree=output.take().expect("private donated controller must build a body");let Component::Container(form)=&tree.root.component else{panic!("form missing")};assert_eq!(form.role,ContainerRole::Form);assert!(tree.root.children.len()<=19);let Component::Extension(status)=&tree.root.children.get(0).unwrap().component else{panic!("source status missing")};assert_eq!(status.extension.as_ref(),semio_framework_plugin::action_staging::STAGED_FORM_STATUS_EXTENSION);let status=serde_json::to_value(&status.props).unwrap();assert_eq!(status["ready"].as_bool(),Some(mode=="ready"));assert_eq!(status["token"]["revision"],"0".repeat(64));let mut close=OriginalBuiltTreeRetirement::new(tree.root);for _ in 0..100000{if close.terminal_is_empty(){break;}caller.turn(staged_grant(),|cx|cx.with_original_retirement(|cx|{let demand=close.next_demand(32768)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|close.step(grant))?;Ok(())})).unwrap();}assert!(close.terminal_is_empty());}
        caller.cancel().cancel_now();retire_staged(&mut caller,&mut Some(work));caller.finish();
    }
    assert!(!crate::templates::application::work::staging_native::NATIVE_PREPARED_TEMPLATE_MOUNTED);assert!(!crate::templates::application::work::NATIVE_TEMPLATE_SEMANTIC_ADMISSION);
}

#[path="guided-type-value-select-tests.rs"]
mod type_value_select_guidance;
