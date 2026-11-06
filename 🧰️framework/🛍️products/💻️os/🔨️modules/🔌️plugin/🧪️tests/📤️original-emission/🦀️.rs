use semio_plugin_job::original_emission::OriginalEmissionInput;
use semio_plugin_testing_laws::artifact_app_laws;
use semio_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{contract_app_raw,meta,TestApp};
use semio_plugin_decl_emission::Emit;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_plugin_decl_app::tool_contract::TypedOperationResultLane;
use semio_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::{TestMutation,set_count::SetCount};
use semio_plugin_operation::ArtifactDocumentAuthority;
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::DslValue;

#[track_caller]
fn original_emission_turn<T>(scope:&mut artifact_app_laws::OriginalFixtureScope,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{
    let cx=scope.cx();
    let before=cx.retained_progress();
    let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||run(cx));
    let paid=semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress()).unwrap();
    assert_eq!((heap.requested_bytes,heap.released_bytes),(paid.retained_capacity_bytes,paid.released_bytes),"actual intake/publication allocations require the same caller receipt");
    result.unwrap()
}

#[semio_framework_async_macros::async_test]
async fn genuine_original_emission_intake_publishes_with_borrowed_caller_and_retires_real_ack(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let value=fixture["mutationValue"].as_i64().unwrap()as i32;
    for row in fixture["modes"].as_array().unwrap().iter().filter(|row|row["mode"]=="borrowed"&&row["cancellationScope"]!="context"){
        let cancellation=row["cancellationScope"].as_str().unwrap();
        let policy=artifact_app_laws::fixture_mounted_policy();
        let mut identity=artifact_app_laws::fixture_identity();
        let mut app=contract_app_raw(policy,&mut identity).await;
        let baseline=app.stores.store.content_revision_now();
        let mut scope=artifact_app_laws::OriginalFixtureScope::admit();
        original_emission_turn(&mut scope,|cx|artifact_app_laws::ensure_fixture_cancellation_issuer(&mut app,cx));
        let mut input=Some(OriginalEmissionInput::<TestApp>{verb:"increment".into(),meta:meta(),emit:Emit::mutations(vec![TestMutation::SetCount(SetCount{value})])});
        let mut receipt=None;
        let mut admitted=false;
        for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
            admitted=original_emission_turn(&mut scope,|cx|{
                let mut task=std::task::Context::from_waker(std::task::Waker::noop());
                let mut dispatch=app.dispatch_ctx();
                let mut future=std::pin::pin!(dispatch.dispatch_original_emission(&mut input,&mut receipt,cx));
                match std::future::Future::poll(future.as_mut(),&mut task){std::task::Poll::Ready(result)=>result,std::task::Poll::Pending=>panic!("original emission intake suspended outside its source turn")}
            });
            if admitted{break}
        }
        assert!(admitted&&input.is_none(),"the actual original input moves exactly once");
        let operation=match &receipt.as_ref().expect("actual admission receipt").output{
            DslValue::Object(fields)=>fields.iter().find_map(|(key,value)|match(key.as_str(),value){("operationId",DslValue::String(id))=>id.parse::<u64>().ok(),_=>None}).unwrap(),
            _=>panic!("actual original emission receipt lacks its operation identity"),
        };
        {
            let mounted=app.tools.tool_operations.get(operation).unwrap();
            assert!(mounted.borrows_original_emission_authority());
            assert!(mounted.completion.is_none()&&mounted.identity.is_none());
            mounted.validate_original_emission_authoring(scope.cx()).unwrap();
            if cancellation=="token"{mounted.cancellation_lease.as_ref().unwrap().cancel();}
        }
        if cancellation=="document"{assert!(app.tool_cancellation_handle().cancel_document(ArtifactDocumentAuthority(meta().instance_id)).unwrap());}
        if cancellation=="instance"{app.tool_cancellation_handle().cancel_scope_generation();}
        let mut retirement=None;
        assert!(artifact_app_laws::retire_fixture_owned(&mut receipt,&mut retirement,policy.close,scope.cx()).unwrap());
        assert!(receipt.is_none()&&retirement.is_none());
        let mut terminal=false;
        let mut pages=0;
        let mut awaiting_producer=None;
        for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
            original_emission_turn(&mut scope,|cx|{
                let mut task=std::task::Context::from_waker(std::task::Waker::noop());
                let mut future=std::pin::pin!(app.advance_typed_operation_publication(&mut identity,cx));
                match std::future::Future::poll(future.as_mut(),&mut task){std::task::Poll::Ready(result)=>result.map_err(Into::into),std::task::Poll::Pending=>panic!("original publication suspended outside its source turn")}
            });
            if let Some((id,grant,progress))=app.take_typed_operation_preparation_receipt(){assert_eq!(id.0,operation);assert!(progress.fits(grant)&&progress.fits(policy.preparation));}
            if let Some((id,grant,progress))=app.take_typed_operation_retirement_receipt(){assert_eq!(id.0,operation);assert!(progress.fits(grant)&&progress.fits(policy.close));}
            assert!(app.take_typed_operation_effect().is_none(),"simple original count mutation has no external effect");
            assert!(app.take_typed_operation_event().is_none(),"simple original count mutation has no app event");
            while app.take_typed_operation_ui_scope().is_some(){}
            while let Some(progress)=app.take_typed_operation_ui_progress(){assert!(progress.leftover.is_none()&&progress.history_patch.is_none(),"this original count mutation has no retained host payload");}
            if let Some(token)=awaiting_producer.take(){
                if let Some(mounted)=app.tools.tool_operations.get(operation){
                    assert!(mounted.result_page.as_ref().is_none_or(|page|page.token!=token)||mounted.result_page_retirement_pending());
                    assert!(mounted.identity.is_none()&&mounted.completion.is_none(),"borrowed producer never installs a replacement authoring receipt");
                    if mounted.result_page_retirement_pending(){awaiting_producer=Some(token);continue}
                }
                if terminal{break}
            }
            let Some(mounted)=app.tools.tool_operations.get_mut(operation)else{break};
            let Some(page)=artifact_app_laws::present_fixture_mounted_page(mounted,policy.preparation,scope.cx()).unwrap()else{continue};
            pages+=1;
            let token=page.token;
            let lane=page.lane;
            let mut wrong=token;wrong.sequence=wrong.sequence.wrapping_add(1);
            assert!(!mounted.acknowledge_result_page(wrong).unwrap());
            let producer=mounted.result_page.as_ref().unwrap().token;
            assert_eq!(producer,token);
            let mut page=page;
            for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
                if page.terminal_is_empty(){break}
                original_emission_turn(&mut scope,|cx|{
                    let demand=page.next_close_demand(policy.close.maximum_copy_bytes)?;
                    cx.with_retained_loan(policy.close,|same|semio_framework_job::close_original_retained_step(same,demand,|issued|semio_framework_value::RetainedWorkOutcome{result:page.close_step(issued),work:1})).map(|_|())
                });
            }
            assert!(page.terminal_is_empty(),"consumer alias physically closes before its exact ACK");
            let before=scope.cx().retained_progress();
            assert!(mounted.acknowledge_result_page(token).unwrap());
            assert_eq!(scope.cx().retained_progress(),before,"ACK only records delivery; the next actual pump pays producer retirement");
            assert!(mounted.result_page_retirement_pending());
            assert!(!mounted.acknowledge_result_page(token).unwrap(),"the exact ACK is accepted once");
            awaiting_producer=Some(token);
            terminal=matches!(lane,TypedOperationResultLane::Terminal|TypedOperationResultLane::Fault);
            if terminal&&cancellation=="none"{assert_ne!(lane,TypedOperationResultLane::Fault);}
            if terminal&&cancellation!="none"{assert_eq!(lane,TypedOperationResultLane::Fault);}
        }
        assert!(terminal&&pages>0,"genuine original emission reaches an actual terminal page");
        if cancellation=="none"{assert_ne!(app.stores.store.content_revision_now(),baseline);assert_eq!(app.stores.store.snapshot_ref().count,value);assert_eq!(app.history.command_log.len(),1);}
        else{assert_eq!(app.stores.store.content_revision_now(),baseline);assert_eq!(app.history.command_log.len(),0);}
        for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
            if app.close_terminal_is_empty(){break}
            original_emission_turn(&mut scope,|cx|{
                let demand=app.close_retirement_demands(policy.close.maximum_copy_bytes)?;
                semio_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,policy.close,demand,|same,grant|Ok((PluginApp::close_step(&mut app,same,grant)?,app.close_terminal_is_empty()))).map(|_|()).map_err(Into::into)
            });
        }
        assert!(app.close_terminal_is_empty(),"all original source, mounted and cancellation owners physically close");
        scope.retire();
    }
}
