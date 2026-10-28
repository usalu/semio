use super::*;
use semio_framework_value::{NativeDecodeControl, RetainedCloneGrant};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcComplexType};
use std::cell::Cell;
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1_000_000, maximum_copy_bytes: 64 * 1048576, maximum_capacity_bytes: 64 * 1048576, maximum_release_bytes: 64 * 1048576, maximum_depth: 128 }
}
fn source() -> IfcSnapshot {
    IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCSTRUCTURALLOADSINGLEFORCE".into(), args: vec![IfcValue::String("load".into()), IfcValue::Real(-0.), IfcValue::Unset, IfcValue::Unset, IfcValue::Unset, IfcValue::Unset, IfcValue::Unset], complex: vec![] }], ..Default::default() }
}
fn close(native: &mut NativeDecodeControl<'_>) {
    native.with_retirement(grant(), |native| {
        while native.has_retirement_owner() { native.close_retirement_recipient(grant())?; }
        Ok(())
    }).unwrap();
}
#[test]
fn fixed_declared_component_names_and_present_words_follow_the_neutral_import_contract() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("import-cases.json")).unwrap();
    let mut source=source();
    let mut observe=|_|true;
    let mut recipient=NativeDecodeRetirementRecipient::new();
    let mut native=NativeDecodeControl::new(64*1048576,&mut observe);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut original=Owner::new(&mut native,grant());
    for (rank,component) in Component::ALL.into_iter().enumerate() {
        assert_eq!(component.name(),cases["components"][rank].as_str().unwrap());
        for bits in [0,(-0f64).to_bits(),3f64.to_bits()] {
            source.entities[0].args[rank+1]=IfcValue::Real(f64::from_bits(bits));
            assert_eq!(receive(&source,0,component,71,&mut original).unwrap().unwrap().to_bits(),bits);
            source.entities[0].args[rank+1]=IfcValue::Unset;
            assert!(receive(&source,0,component,71,&mut original).unwrap().is_none());
        }
    }
    assert!(original.progress().copied_items>0);
    drop(original);
    assert!(!native.has_retirement_owner());
}
#[test]
fn absent_serialized_missing_ambiguous_and_late_unknown_refuse_without_unset_coercion() {
    for defect in 0..5 {
        let mut source=source();
        match defect {
            0=>source.entities[0].args.truncate(1),
            1=>{source.entities[0].name="IFCSTRUCTURALLOAD".into();source.entities[0].args.truncate(1);}
            2|3=>{
                source.entities[0].complex.push(IfcComplexType{name:"IFCSTRUCTURALLOADSINGLEFORCE".into(),args:vec![]});
                if defect==3 {source.entities[0].complex.push(IfcComplexType{name:"SEMIO_UNKNOWN".into(),args:vec![]});}
            }
            _=>source.entities[0].args[1]=IfcValue::Derived,
        }
        let mut observe=|_|true;
        let mut recipient=NativeDecodeRetirementRecipient::new();
        let mut native=NativeDecodeControl::new(64*1048576,&mut observe);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let error=receive(&source,0,Component::ForceX,71,&mut Owner::new(&mut native,grant())).unwrap_err();
        assert_eq!(error.kind,if defect==3{ValueRefusalKind::UnsupportedOwner}else{ValueRefusalKind::InvalidValue});
        close(&mut native);
    }
}
#[test]
fn original_query_denial_and_every_observed_prefix_close_original_metadata() {
    let source=source();
    let total=Cell::new(0);
    for stop in std::iter::once(usize::MAX).chain(1..10000) {
        if stop!=usize::MAX&&stop>total.get(){break;}
        let calls=Cell::new(0);
        let mut observe=|_|{calls.set(calls.get()+1);calls.get()!=stop};
        let mut recipient=NativeDecodeRetirementRecipient::new();
        let mut native=NativeDecodeControl::new(64*1048576,&mut observe);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let result=receive(&source,0,Component::ForceX,71,&mut Owner::new(&mut native,grant()));
        if stop==usize::MAX{result.unwrap();total.set(calls.get());assert!(total.get()<10000);}
        else{assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);}
        close(&mut native);
    }
    for g in [
        RetainedCloneGrant{maximum_items:0,..grant()},RetainedCloneGrant{maximum_copy_bytes:0,..grant()},
        RetainedCloneGrant{maximum_capacity_bytes:0,..grant()},RetainedCloneGrant{maximum_release_bytes:0,..grant()},
        RetainedCloneGrant{maximum_depth:0,..grant()},
    ] {
        let mut observe=|_|true;
        let mut recipient=NativeDecodeRetirementRecipient::new();
        let mut native=NativeDecodeControl::new(64*1048576,&mut observe);
        native.install_retirement_recipient(&mut recipient).unwrap();
        assert!(receive(&source,0,Component::ForceX,71,&mut Owner::new(&mut native,g)).is_err());
        close(&mut native);
    }
}
