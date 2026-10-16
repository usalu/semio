use crate::{FactoryRetirement,FactoryAuthority,factory_arc_birth_bytes};
use std::sync::Arc;
use semio_framework_value_factory::original_factory_fixture::*;
#[test]
fn factory_authority_keeps_original_typed_source_until_admitted_close_and_reports_all_physical_frees(){
    use crate::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneSource};
    let fixture:serde_json::Value=serde_json::from_str(crate::text("♻️retirement/🏭️factory/🔗️authority/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    use std::io::Write;let mut oracle=std::process::Command::new("bun").args(["-e","const x=JSON.parse(await Bun.stdin.text());console.log(JSON.stringify([...new TextEncoder().encode(x.fixture.payload)]));"]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();oracle.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture}).to_string().as_bytes()).unwrap();let output=oracle.wait_with_output().unwrap();assert!(output.status.success());let reference:Vec<u8>=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(reference,fixture["payload"].as_str().unwrap().as_bytes());
    for work in fixture["copyGrants"].as_array().unwrap(){let work=work.as_u64().unwrap()as usize;
        let mut payload=Vec::with_capacity(fixture["payloadCapacity"].as_u64().unwrap()as usize);payload.extend_from_slice(&reference);let payload_capacity=payload.capacity();let factory=Arc::new(OwnedFactory{payload});let factory_extent=arc_extent::<OwnedFactory>();
        let original=String::from("source");let original_bytes=original.capacity();
        let authority=FactoryAuthority::new(factory.clone());let pointer=Arc::as_ptr(&factory);
        let (refused,heap)=crate::observe_retirement_allocations(||RetainedCloneSource::admit_owned(original,authority,Default::default()));assert_eq!(heap,(0,0));let (_,original,authority)=refused.err().unwrap();assert_eq!(authority.factory_address(),pointer as*const()as usize);
        let capacity=RetainedCloneSource::<String>::constructor_capacity_bytes::<FactoryAuthority>();let (admitted,heap)=crate::observe_retirement_allocations(||RetainedCloneSource::admit_owned(original,authority,grant(capacity,0,1)));let(mut source,receipt)=admitted.unwrap_or_else(|_|panic!("source authority admission"));assert_eq!(heap,(receipt.retained_capacity_bytes,0));let mut born=receipt.retained_capacity_bytes;let mut physical=0;
        let mut cursor=String::retained_clone_cursor();cursor.advance(source.borrow(),RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1}).unwrap();drop(factory);cursor.begin_close();
        for _ in 0..100000{if source.terminal_is_empty()&&cursor.terminal_is_empty(){break;}let grant=if !source.terminal_is_empty(){RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:source.next_close_capacity_byte_demand(work).unwrap(),maximum_release_bytes:source.next_close_release_byte_demand().unwrap(),maximum_depth:source.next_close_depth_demand().unwrap()}}else{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(work).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),maximum_depth:64}};let(step,heap)=crate::observe_retirement_allocations(||if !source.terminal_is_empty(){source.close_step(grant).unwrap()}else{cursor.close_step(grant).unwrap()});let p=step.progress();assert!(p.fits(grant));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;physical+=heap.1;}
        assert!(source.terminal_is_empty()&&cursor.terminal_is_empty());assert_eq!(original_bytes+factory_extent+payload_capacity+born,physical);println!("[DEBUG] Factory source authority work={work} original={} born={born} physical={physical}",original_bytes+factory_extent+payload_capacity);
    }
}

