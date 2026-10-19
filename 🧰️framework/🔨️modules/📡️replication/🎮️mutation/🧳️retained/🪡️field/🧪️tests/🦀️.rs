use semio_framework_test_fixtures::fixture_text;
use super::*;
use semio_framework_value::retained_clone::RetainedCloneSource;

fn observe<T>(operation: impl FnOnce() -> T) -> (T, (usize,usize)) {
    let (result,requested,released)=crate::test_allocation::observe_backing(operation);
    (result,(requested,released))
}
fn quote(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }
}

fn source(text: String) -> RetainedCloneSource<String> {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 4096 };
    match RetainedCloneSource::admit_owned(text, (), grant) { Ok((source, _)) => source, Err((error, _, _)) => panic!("original field source refused: {error}") }
}

fn close(field: &mut RetainedFieldPreparation<String>, source: &mut RetainedCloneSource<String>) {
    field.begin_close();
    while !field.terminal_is_empty() {
        let demand = field.close_demand(4096).unwrap();
        let (step, heap) = observe(|| field.close_step(Default::default()).unwrap());
        assert_eq!(step.progress(), Default::default()); assert_eq!(heap,(0,0));
        let grant = quote(demand);
        let (step, heap) = observe(|| field.close_step(grant).unwrap());
        assert!(step.progress().fits(grant));
        assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
    }
    while !source.terminal_is_empty() {
        let grant = quote(source.next_close_demand(4096).unwrap());
        assert!(source.close_step(grant).unwrap().progress().fits(grant));
    }
}

/// 🪡️ SQLite-verified Unicode and empty setters use bounded original field custody and owned exchange.
#[test]
fn retained_field_exchange_matches_neutral_strings_and_physical_receipts() {
    let vectors: serde_json::Value = serde_json::from_str(fixture_text!("../../🪡️field/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for vector in vectors.as_array().unwrap() {
        let mut original = source(vector["authored"].as_str().unwrap().into());
        let mut field = RetainedFieldPreparation::<String>::default();
        let mut unpublished = vector["before"].as_str().unwrap().to_owned();
        loop {
            let demand = field.demand(original.borrow(), 4).unwrap();
            let grant = quote(demand);
            let (denied,heap) = observe(|| field.advance(original.borrow(), 4, Default::default()).unwrap());
            assert_eq!(denied.progress(),Default::default()); assert_eq!(heap,(0,0));
            let (step,heap) = observe(|| field.advance(original.borrow(),4,grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
            if matches!(step,RetainedCloneStep::Complete(_)) { break; }
        }
        assert_eq!(unpublished,vector["before"].as_str().unwrap());
        assert_eq!(field.value().unwrap(),vector["authored"].as_str().unwrap());
        let grant = quote(field.exchange_demand().unwrap());
        let (denied,heap) = observe(|| field.install(&mut unpublished,super::super::super::mint_capability(),Default::default()).unwrap());
        assert_eq!(denied,Default::default()); assert_eq!(heap,(0,0));
        let (receipt,heap) = observe(|| field.install(&mut unpublished,super::super::super::mint_capability(),grant).unwrap());
        assert!(receipt.fits(grant)); assert_eq!(heap,(0,0));
        assert_eq!(unpublished,vector["expected"].as_str().unwrap());
        close(&mut field,&mut original);
        eprintln!("[DEBUG] retained field {} matches original authored setter; candidate/displaced/source receipts settle exactly",vector["id"]);
    }
}

/// 🛑️ Cancelling before each text preparation frontier returns every source alias and staged field.
#[test]
fn retained_field_cancellation_closes_every_typed_frontier() {
    for frontier in 0..9 {
        let mut original = source("Feldgröße 🧱️".into());
        let mut field = RetainedFieldPreparation::<String>::default();
        for _ in 0..frontier {
            let grant = quote(field.demand(original.borrow(),4).unwrap());
            let _ = field.advance(original.borrow(),4,grant).unwrap();
            if field.value().is_some() { break; }
        }
        close(&mut field,&mut original);
        assert!(field.terminal_is_empty()); assert!(original.terminal_is_empty());
    }
}

/// 📏️ Original logical quantum survives an expanded physical field header grant on every real copy turn.
#[test]
fn retained_field_logical_body_survives_expanded_physical_headers() {
    let vectors:serde_json::Value=serde_json::from_str(fixture_text!("../../📏️body/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in vectors["rows"].as_array().unwrap() {
        let body=row["body"].as_u64().unwrap() as usize;
        let expected="original field payload ".repeat(32);
        let mut original=source(expected.clone());
        let mut field=RetainedFieldPreparation::<String>::default();
        let mut turns=0;
        while field.value().is_none() {
            let demand=field.demand(original.borrow(),body).unwrap();
            let exact=quote(demand);
            let grant=RetainedCloneGrant{maximum_copy_bytes:row["physical"].as_u64().unwrap() as usize,..exact};
            let (denied,heap)=observe(||field.advance(original.borrow(),body,RetainedCloneGrant{maximum_items:0,..grant}).unwrap());
            assert_eq!(denied.progress(),Default::default());assert_eq!(heap,(0,0));
            let (step,heap)=observe(||field.advance(original.borrow(),body,grant).unwrap());
            assert!(step.progress().fits(exact));
            assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
            turns+=1;assert!(turns<10000);
        }
        assert_eq!(field.value().unwrap(),&expected);
        assert_eq!(original.borrow().get(),&expected);
        close(&mut field,&mut original);
        assert_eq!(observe(||drop(field)).1,(0,0));
        eprintln!("[DEBUG] Original field body={body} is preserved independently of physical header grant");
    }
}
