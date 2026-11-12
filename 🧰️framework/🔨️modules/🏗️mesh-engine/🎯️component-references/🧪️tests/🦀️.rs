use super::*;
use semio_framework_value_testing_allocator::observe_retirement_allocations;
use semio_framework_value::{retained_clone::{RetainedCloneGrant,RetainedCloneStep},retirement::controlled::ControlledRetirement};
#[path="../../../🌱️value/♻️retirement/📦️owner/🧪️work/effect.rs"]
mod effect;

#[test]
fn component_reference_table_preserves_original_labels_and_wire_map_then_retires_exact_backings(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../🧬️schema/🔣️.json")).unwrap();
    use std::io::Write;let mut oracle=std::process::Command::new("bun").args(["-e","import Ajv from 'ajv';const x=JSON.parse(await Bun.stdin.text());const valid=new Ajv({strict:true}).compile(x.schema);for(const table of x.fixture.tables)if(!valid(table))throw Error('schema');console.log(JSON.stringify(x.fixture.tables.map(table=>Object.entries(table).sort(([a],[b])=>a<b?-1:a>b?1:0))));"]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();oracle.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture}).to_string().as_bytes()).unwrap();let output=oracle.wait_with_output().unwrap();assert!(output.status.success());let reference:Vec<Vec<(String,Vec<String>)>>=serde_json::from_slice(&output.stdout).unwrap();
    for row in reference{for work in fixture["copyGrants"].as_array().unwrap(){let work=work.as_u64().unwrap()as usize;
        let entries=row.clone();let original=entries.capacity()*std::mem::size_of::<(String,Vec<String>)>()+entries.iter().map(|(key,labels)|key.capacity()+labels.capacity()*std::mem::size_of::<String>()+labels.iter().map(String::capacity).sum::<usize>()).sum::<usize>();let pointers=entries.iter().map(|(_,labels)|labels.iter().map(|s|s.as_ptr()).collect::<Vec<_>>()).collect::<Vec<_>>();
        let(table,heap)=observe_retirement_allocations(||ComponentReferenceTable::from_entries(entries));assert_eq!(heap,(0,0));assert_eq!(table.iter().map(|(key,labels)|(key.clone(),labels.clone())).collect::<Vec<_>>(),row);for((_,labels),pointers)in table.iter().zip(pointers){assert_eq!(labels.iter().map(|s|s.as_ptr()).collect::<Vec<_>>(),pointers);}
        let third_party=row.clone().into_iter().collect::<std::collections::BTreeMap<_,_>>();let wire=semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&table));let actual:serde_json::Value=serde_json::from_str(&wire.to_string()).unwrap();assert_eq!(actual,serde_json::to_value(&third_party).unwrap());
        let(mut owner,heap)=observe_retirement_allocations(||ControlledRetirement::new(table).unwrap_or_else(|_|panic!("component table typed authority")));assert_eq!(heap,(0,0));let mut born=0;let mut physical=0;
        for _ in 0..100000{if owner.terminal_is_empty(){break;}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:owner.next_copy_byte_demand().unwrap().max(work),maximum_capacity_bytes:owner.next_capacity_byte_demand(work).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let units=owner.next_computational_work(work).unwrap().max(1);let(poll,heap)=effect::with_effect(grant,units,&mut None,|effect|observe_retirement_allocations(||owner.step(work,effect)));let step=effect::settled(poll).result.unwrap();let p=step.progress();assert!(p.fits(grant));assert!(p!=Default::default()||owner.terminal_is_empty(),"retirement made no progress under {grant:?}: {step:?} demand {:?}",owner.next_demand(work));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;physical+=heap.1;assert!(!matches!(step,RetainedCloneStep::Complete(_))||owner.terminal_is_empty());}
        assert!(owner.terminal_is_empty());assert_eq!(original+born,physical);
    }}
}

