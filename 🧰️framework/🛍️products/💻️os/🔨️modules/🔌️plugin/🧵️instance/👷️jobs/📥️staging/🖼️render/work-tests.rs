use semio_framework_plugin_instance_jobs::{jobs_instance_root::OriginalRenderTreeIdentity, jobs_render::work::OriginalRenderWork};
use semio_framework_plugin_testing_laws::OriginalWorkFixture;
use semio_framework_value::retained_clone::RetainedCloneGrant;

fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 65536, maximum_release_bytes: 65536, maximum_depth: 64 }
}

#[test]
fn render_work_preserves_original_owner_until_funded_retirement() {
    let mut caller = OriginalWorkFixture::new();
    let mut identity = caller.turn(grant(), |cx| OriginalRenderTreeIdentity::capture(cx).map_err(Into::into)).unwrap();
    let mut work = OriginalRenderWork::retain(String::from("original retained render query"));
    let pointer = caller.turn(grant(), |cx| Ok(work.original(&mut identity, cx)?.unwrap().as_ptr())).unwrap();
    for axis in 0..5 {
        let mut denied = grant();
        match axis { 0 => denied.maximum_items = 0, 1 => denied.maximum_copy_bytes = 0, 2 => denied.maximum_capacity_bytes = 0, 3 => denied.maximum_release_bytes = 0, _ => denied.maximum_depth = 0 }
        let before = work.demand(4096).unwrap();
        let step = caller.turn(denied, |cx| work.close(&mut identity, cx)).unwrap();
        if before.capacity_bytes > denied.maximum_capacity_bytes || before.copy_bytes > denied.maximum_copy_bytes || before.release_bytes > denied.maximum_release_bytes || denied.maximum_items == 0 || before.depth > denied.maximum_depth {
            assert_eq!(step, None);
        }
        if let Ok(actual) = caller.turn(grant(), |cx| Ok(work.original(&mut identity, cx)?.unwrap().as_ptr())) { assert_eq!(actual, pointer); }
    }
    for _ in 0..1024 {
        if caller.turn(grant(), |cx| work.close(&mut identity, cx)).unwrap() == Some(true) {
            assert!(work.terminal_is_empty());
            caller.finish();
            return;
        }
    }
    panic!("original retained render query did not retire");
}

#[test]
fn render_work_rejects_another_original_before_borrow_or_close() {
    let mut caller = OriginalWorkFixture::new();
    let mut other = OriginalWorkFixture::new();
    let mut identity = caller.turn(grant(), |cx| OriginalRenderTreeIdentity::capture(cx).map_err(Into::into)).unwrap();
    let mut work = OriginalRenderWork::retain(vec![String::from("owned exact choice")]);
    assert!(other.turn(grant(), |cx| work.original(&mut identity, cx).map(|_| ())).is_err());
    assert!(other.turn(grant(), |cx| work.close(&mut identity, cx)).is_err());
    assert_eq!(caller.turn(grant(), |cx| Ok(work.original(&mut identity, cx)?.unwrap().len())).unwrap(), 1);
    for _ in 0..1024 {
        if caller.turn(grant(), |cx| work.close(&mut identity, cx)).unwrap() == Some(true) { caller.finish(); other.finish(); return; }
    }
    panic!("unchanged original could not close after rejected substitute");
}

#[test]
fn render_snapshot_loan_obeys_neutral_original_identity_cases() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("source-loan-cases.json")).unwrap();
    for case in cases["cases"].as_array().unwrap(){
        let mut caller=OriginalWorkFixture::new();
        let mut other=OriginalWorkFixture::new();
        let mut identity=caller.turn(grant(),|cx|OriginalRenderTreeIdentity::capture(cx).map_err(Into::into)).unwrap();
        let source=std::sync::Arc::new(String::from("exact original source"));
        let substitute=std::sync::Arc::new(String::from("exact original source"));
        let mut work=OriginalRenderWork::retain(std::sync::Arc::clone(&source));
        if case["cancelled"].as_bool().unwrap(){caller.cancel().cancel_now();}
        let expected=if case["source"]=="original"{source.as_ref()}else{substitute.as_ref()};
        let selected=if case["caller"]=="original"{&mut caller}else{&mut other};
        let result=selected.turn(grant(),|cx|Ok(work.snapshot_loan(&mut identity,expected,cx)?.as_ptr()));
        assert_eq!(result.is_ok(),case["admitted"].as_bool().unwrap(),"{}",case["id"]);
        if let Ok(pointer)=result{assert_eq!(pointer,source.as_ptr());}
        assert_eq!(std::sync::Arc::strong_count(&source),2);
        for _ in 0..1024{if caller.turn(grant(),|cx|cx.with_original_retirement(|cx|work.close(&mut identity,cx))).unwrap()==Some(true){break;}}
        assert!(work.terminal_is_empty());
        assert!(caller.turn(grant(),|cx|work.snapshot_loan(&mut identity,source.as_ref(),cx).map(|_|())).is_err());
        assert_eq!(std::sync::Arc::strong_count(&source),1);
        caller.finish();other.finish();
    }
}
