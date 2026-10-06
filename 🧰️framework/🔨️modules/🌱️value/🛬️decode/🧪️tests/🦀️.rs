use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_value::native_decoding::{NativeDecodeControl, NativeDecodeProgress};

#[test]
fn sqlite_snapshot_nested_native_stages_preserve_parent_and_cumulative_ownership(){
    use std::{cell::Cell,io::Write,process::{Command,Stdio}};
    fn label(error:&ValueError)->&str{match error.kind{ValueRefusalKind::WorkLimit=>"overrun",ValueRefusalKind::Canceled=>"canceled",ValueRefusalKind::OwnershipLimit=>"limit",ValueRefusalKind::InvalidValue=>"rejected",_=>panic!("unexpected decode error: {error}")}}
    fn run(control:&mut NativeDecodeControl<'_>,operations:&serde_json::Value,errors:&mut Vec<String>)->Result<(),ValueError>{
        for op in operations.as_array().unwrap(){match op["kind"].as_str().unwrap(){
            "begin"=>control.begin_stage(op["units"].as_u64().unwrap() as usize)?,
            "advance"=>control.advance(op["units"].as_u64().unwrap() as usize)?,
            "charge"=>control.charge(op["units"].as_u64().unwrap() as usize)?,
            "reject"=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned child rejected")),
            "scope"=>{if let Err(error)=control.scoped_stage(|child|run(child,&op["operations"],errors)){errors.push(label(&error).to_owned());}},
            kind=>panic!("unexpected stage operation: {kind}")
        }}Ok(())
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🪆️stage/🔣️.json")).unwrap();
    
    let mut actual=Vec::new();
    for case in fixture["cases"].as_array().unwrap(){
        let progress=Cell::new(NativeDecodeProgress{completed:0,total:0,owned_bytes:0});
        let mut callback=|event:NativeDecodeProgress|{progress.set(event);case["cancel"].is_null()||event.total!=case["cancel"]["total"].as_u64().unwrap() as usize||event.completed<(case["cancel"]["at"].as_u64().unwrap() as usize)};
        let mut control=NativeDecodeControl::new(case["maximumBytes"].as_u64().unwrap() as usize,&mut callback);let mut errors=Vec::new();
        if let Err(error)=run(&mut control,&case["operations"],&mut errors){errors.push(label(&error).to_owned());}control.checkpoint().unwrap();
        let event=progress.get();let result=serde_json::json!({"progress":{"completed":event.completed,"total":event.total,"ownedBytes":event.owned_bytes},"errors":errors});
        assert_eq!(result,case["expected"],"{}",case["id"]);actual.push(result);
    }
    let script=format!("{}\nconst input=JSON.parse(await Bun.stdin.text());await Bun.write(Bun.stdout,JSON.stringify(input.fixture.cases.map(stageOracle)));",include_str!("🪆️stage/🟦️.ts"));
    let mut child=Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(actual,serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout).unwrap());
}

#[test]
fn sqlite_snapshot_native_materialization_preserves_caller_budget_and_cancellation() {
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    
    let mut actual=Vec::new();
    for case in fixture["cases"].as_array().unwrap() {
        let mut progress=|_:NativeDecodeProgress|true;
        let mut control=NativeDecodeControl::new(case["maximumBytes"].as_u64().unwrap() as usize,&mut progress);
        let mut accepted=0;
        for bytes in case["charges"].as_array().unwrap() { if control.charge(bytes.as_u64().unwrap() as usize).is_err(){break} accepted+=1; }
        assert_eq!(accepted,case["acceptedCharges"].as_u64().unwrap());assert_eq!(control.owned_bytes(),case["ownedBytes"].as_u64().unwrap() as usize);
        actual.push(serde_json::json!({"acceptedCharges":accepted,"ownedBytes":control.owned_bytes()}));
    }
    let script="import {Database} from 'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const db=new Database(':memory:');db.run('CREATE TABLE charge(position INTEGER PRIMARY KEY,bytes INTEGER NOT NULL)');const actual=x.fixture.cases.map(c=>{db.run('DELETE FROM charge');let acceptedCharges=0,ownedBytes=0;for(const bytes of c.charges){db.run('INSERT INTO charge VALUES(?,?)',acceptedCharges+1,bytes);const n=db.query('SELECT SUM(bytes) AS n FROM charge').get().n;if(n>c.maximumBytes)break;ownedBytes=n;acceptedCharges++;}return {acceptedCharges,ownedBytes};});db.close();await Bun.write(Bun.stdout,JSON.stringify(actual));";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(actual,serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout).unwrap());
    let copy_total=fixture["copyWorkload"]["total"].as_u64().unwrap() as usize;let copy_cancel=fixture["copyWorkload"]["cancelAt"].as_u64().unwrap() as usize;
    for text in [false,true]{let mut reached=Vec::new();let mut progress=|event:NativeDecodeProgress|{if event.total!=copy_total{return true;}reached.push(event.completed);event.completed<copy_cancel};let mut control=NativeDecodeControl::new(1_000_000,&mut progress);if text{let input=fixture["copyWorkload"]["text"].as_str().unwrap().repeat(fixture["copyWorkload"]["repetitions"].as_u64().unwrap() as usize);assert_eq!(input.len(),copy_total);assert!(control.copy_text(&input).is_err());}else{assert!(control.copy_bytes(&vec![1;copy_total]).is_err());}assert_eq!(reached,vec![0,copy_cancel]);assert!(copy_cancel<copy_total);}
    let mut canceled=|_:NativeDecodeProgress|false;
    let mut control=NativeDecodeControl::new(1_000_000,&mut canceled);
    assert!(control.copy_bytes(&vec![1;65537]).is_err());assert_eq!(control.owned_bytes(),0);
    assert!(control.copy_bytes(&[1]).is_err());assert_eq!(control.owned_bytes(),0);
    let total=fixture["workload"]["total"].as_u64().unwrap() as usize;let cancel_at=fixture["workload"]["cancelAt"].as_u64().unwrap() as usize;
    let mut reached=Vec::new();let mut progress=|event:NativeDecodeProgress|{assert_eq!(event.total,total);reached.push(event.completed);event.completed<cancel_at};
    let mut control=NativeDecodeControl::new(1_000_000,&mut progress);
    control.begin_stage(total).unwrap();for _ in 0..cancel_at-1 {control.step().unwrap();}assert!(control.step().is_err());assert!(cancel_at<total);assert_eq!(serde_json::json!(reached),fixture["workload"]["checkpoints"]);
    let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(16,&mut accepted);
    assert!(control.allocate_vec::<Vec<u8>>(2048).is_err());assert_eq!(control.owned_bytes(),0);
    control.charge(12).unwrap();control.begin_stage(4).unwrap();assert!(control.charge(8).is_err());assert_eq!(control.owned_bytes(),12);
    let text="🧬".repeat(copy_total/4);let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(0,&mut accepted);assert_eq!(control.borrow_text(text.as_bytes()).unwrap(),text);assert_eq!(control.owned_bytes(),0);for bytes in [&[0xc0,0x80][..],&[0xed,0xa0,0x80][..],&[0xf4,0x90,0x80,0x80][..],&[0xf0,0x9f][..]]{assert!(control.borrow_text(bytes).is_err());}
    let mut interior=false;let mut cancel=|event:NativeDecodeProgress|{if event.completed==copy_cancel&&event.total==copy_total{interior=true;false}else{true}};let mut control=NativeDecodeControl::new(0,&mut cancel);assert!(control.borrow_text(text.as_bytes()).is_err());assert!(interior);
    let n=&fixture["nestedStage"];let mut control=NativeDecodeControl::new(64,&mut accepted);control.begin_stage(n["parentTotal"].as_u64().unwrap() as usize).unwrap();control.advance(n["parentBefore"].as_u64().unwrap() as usize).unwrap();control.scoped_stage(|child|->Result<(),ValueError>{child.begin_stage(n["childTotal"].as_u64().unwrap() as usize)?;child.charge(n["childBytes"].as_u64().unwrap() as usize)?;child.advance(n["childTotal"].as_u64().unwrap() as usize)}).unwrap();control.advance(n["parentAfter"].as_u64().unwrap() as usize).unwrap();assert_eq!(control.owned_bytes(),n["childBytes"].as_u64().unwrap() as usize);assert!(control.scoped_stage(|child|->Result<(),ValueError>{child.begin_stage(2)?;Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned child failed"))}).is_err());control.advance((n["parentTotal"].as_u64().unwrap()-n["parentBefore"].as_u64().unwrap()-n["parentAfter"].as_u64().unwrap()) as usize).unwrap();assert!(control.step().is_err());
    let mut control=NativeDecodeControl::new(0,&mut accepted);for case in fixture["utf8"].as_array().unwrap(){let bytes:Vec<u8>=serde_json::from_value(case["bytes"].clone()).unwrap();assert_eq!(control.borrow_text(&bytes).is_ok(),case["accepted"].as_bool().unwrap());assert_eq!(control.owned_bytes(),0);}
}

#[test]
fn native_decode_explicit_one_slot_preserves_success_and_refusal_physical_owners(){
    use semio_framework_value::{ErasedSnapshotRetirement,SnapshotRetirementStep};
    use semio_framework_value::native_decoding::NativeDecodeRetirementRecipient;
    use std::{cell::Cell,io::Write,process::{Command,Stdio}};
    #[repr(C)]
    struct Pages{pages:[Option<Box<[u8;4096]>>;3],next:usize}
    impl ErasedSnapshotRetirement for Pages{
        fn close_step(&mut self,items:usize,bytes:usize)->Result<SnapshotRetirementStep,ValueError>{
            if items==0||bytes<4096{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}
            if self.next==3{return Ok(SnapshotRetirementStep::Complete);}
            self.pages[self.next].take();self.next+=1;Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:4096})
        }
        fn terminal_is_empty(&self)->bool{self.pages.iter().all(Option::is_none)}
        fn next_close_byte_demand(&self)->usize{if self.terminal_is_empty(){0}else{4096}}
    }
    impl Drop for Pages{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"actual page owner was dropped");}}
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🫴️recipient.json")).unwrap();
    
    let wrapper=std::mem::size_of::<Pages>();assert_eq!(wrapper,4*std::mem::size_of::<usize>());
    let mut actual=Vec::new();
    for row in fixture["cases"].as_array().unwrap(){
        let mode=row["mode"].as_str().unwrap();let entered=Cell::new(0);
        let checkpoints=Cell::new(0);let mut callback=|progress:NativeDecodeProgress|{checkpoints.set(checkpoints.get()+1);mode!="canceled"&&(mode!="canceled-after-wrapper"||checkpoints.get()<2)&&(mode!="interior-canceled"||progress.completed<256)};
        let mut recipient=NativeDecodeRetirementRecipient::new();
        let mut control=NativeDecodeControl::new(if mode=="over-limit"{wrapper-1}else if mode=="initial-4096"{4096}else{65536},&mut callback);
        if mode!="missing"{control.install_retirement_recipient(&mut recipient).unwrap();}
        let operation=|control:&mut NativeDecodeControl<'_>|{
            entered.set(entered.get()+1);
            if let Err(error)=control.charge(3*4096){return(Err(error),None);}
            let owner=Pages{pages:[Some(Box::new([1;4096])),Some(Box::new([2;4096])),Some(Box::new([3;4096]))],next:0};
            let result=if mode=="interior-canceled"{control.begin_stage(256).and_then(|_|control.advance(256))}else if mode=="refusal"{Err(ValueError::new(ValueRefusalKind::InvalidValue,"actual decoder refusal"))}else{Ok(())};
            (result,Some(Box::new(owner)as Box<dyn ErasedSnapshotRetirement>))
        };
        let mut result=control.with_retirement_owner(wrapper,operation);
        if mode=="occupied"{result=control.with_retirement_owner(wrapper,|_|{entered.set(entered.get()+1);(Ok(()),None)});}
        let error=result.err().map(|error|match error.kind{ValueRefusalKind::OwnershipLimit=>"ownership",ValueRefusalKind::Canceled=>"canceled",ValueRefusalKind::InvalidValue=>"invalid",_=>panic!("unexpected recipient error")});
        let owned=control.owned_bytes();drop(control);
        let retained=recipient.has_owner();
        assert_eq!(entered.get(),row["entered"].as_u64().unwrap());assert_eq!(error.map(serde_json::Value::from).unwrap_or(serde_json::Value::Null),row["error"]);assert_eq!(retained,row["retained"].as_bool().unwrap());
        if retained{
            for grant in [(0,4096),(1,4095)]{assert_eq!(recipient.close_step(grant.0,grant.1).unwrap(),SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});assert!(!recipient.terminal_is_empty());}
            let mut bytes=0;for _ in 0..8{match recipient.close_step(1,4096).unwrap(){SnapshotRetirementStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);bytes+=released_bytes;},SnapshotRetirementStep::Complete=>break,SnapshotRetirementStep::Blocked=>panic!("actual paid page owner blocked")}}
            assert!(recipient.terminal_is_empty());assert_eq!(bytes,3*4096+wrapper);assert_eq!(owned,3*4096+wrapper);
        }else{assert!(recipient.terminal_is_empty());assert_eq!(owned,if matches!(mode,"initial-4096"|"canceled-after-wrapper"){wrapper}else{0});}
        actual.push(serde_json::json!({"entered":entered.get(),"error":error,"retained":retained}));
    }
    let script="import {Database} from 'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const db=new Database(':memory:');db.run('CREATE TABLE recipient(slot INTEGER PRIMARY KEY CHECK(slot=1))');const out=x.fixture.cases.map(c=>{db.run('DELETE FROM recipient');const before=['missing','canceled','canceled-after-wrapper','over-limit'].includes(c.mode);let entered=0,error=before?(c.mode.startsWith('canceled')?'canceled':'ownership'):c.mode==='refusal'?'invalid':c.mode==='interior-canceled'?'canceled':c.mode==='initial-4096'?'ownership':null;if(!before){if(c.mode!=='initial-4096')db.run('INSERT INTO recipient VALUES(1)');entered++;if(c.mode==='occupied'){try{db.run('INSERT INTO recipient VALUES(1)');throw Error('duplicate admitted');}catch(e){if(String(e).includes('duplicate admitted'))throw e;error='ownership';}}}return {entered,error,retained:db.query('SELECT COUNT(*) AS n FROM recipient').get().n===1};});db.close();await Bun.write(Bun.stdout,JSON.stringify(out));";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(actual,serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout).unwrap());
    println!("[DEBUG] explicit one-slot decoder recipient preserves success/refusal, absent/full/canceled/over-limit and actual initial4096 retain zero partial pages, actual three4096 pages plus wrapper physically drain under1/4096");
}

#[test]
fn paged_semantic_text_keeps_actual8194_success_and_partial_cancellation_owners_under4096(){
    use semio_framework_value::{paged_text::PagedText,native_decoding::NativeDecodeRetirementRecipient,ErasedSnapshotRetirement,SnapshotRetirementStep};
    use std::{cell::Cell,io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📝️paged-text.json")).unwrap();
    
    let ascii=vec![fixture["ascii"]["byte"].as_u64().unwrap()as u8;fixture["ascii"]["repetitions"].as_u64().unwrap()as usize];
    let unicode=format!("{}{}{}{}","x".repeat(4095),fixture["unicode"]["middle"].as_str().unwrap(),"x".repeat(4093),fixture["unicode"]["tail"].as_str().unwrap()).into_bytes();
    assert_eq!(ascii.len(),8194);assert_eq!(unicode.len(),8194);
    let wrapper=std::mem::size_of::<PagedText<131072>>();let mut actual=Vec::new();
    let drain=|recipient:&mut NativeDecodeRetirementRecipient,expected:usize|{
        assert_eq!(recipient.close_step(0,4096).unwrap(),SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});
        let mut released=0;for _ in 0..8194+128{match recipient.close_step(1,4096).unwrap(){SnapshotRetirementStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);released+=released_bytes;},SnapshotRetirementStep::Complete=>break,SnapshotRetirementStep::Blocked=>panic!("paged semantic owner blocked")}}
        assert!(recipient.terminal_is_empty());assert_eq!(released,expected);
    };
    for row in fixture["cases"].as_array().unwrap(){
        let input=if row["source"]=="unicode"{&unicode}else{&ascii};let pointer=input.as_ptr();let mode=row["mode"].as_str().unwrap();let phase=Cell::new(0);
        let mut callback=|progress:NativeDecodeProgress|{
            if progress.total==8194&&progress.completed==0{phase.set(phase.get()+1);}
            !(progress.completed>=256&&((mode=="validate-cancel"&&phase.get()==1)||(mode=="materialize-cancel"&&phase.get()>=2)))
        };
        let mut recipient=NativeDecodeRetirementRecipient::new();let mut control=NativeDecodeControl::new(if mode=="initial"{4096}else{131072},&mut callback);control.install_retirement_recipient(&mut recipient).unwrap();
        let logical=Cell::new(0);let allocated=Cell::new(0);let output=std::cell::RefCell::new(Vec::new());
        let result=control.with_retirement_owner(wrapper,|control|{
            let mut text=PagedText::<131072>::empty();let result=text.read_from_source(&input.as_slice(),control);logical.set(text.byte_len());allocated.set(text.allocated_bytes());
            if result.is_ok(){let span=text.borrow().unwrap();assert_eq!(span.byte_len(),8194);assert!(span.bytes().eq(input.iter().copied()));let chars=span.chars().collect::<String>();assert_eq!(chars.as_bytes(),input.as_slice());*output.borrow_mut()=span.bytes().collect();}
            else{assert!(text.borrow().is_err());}
            (result,Some(Box::new(text)as Box<dyn ErasedSnapshotRetirement>))
        });
        let owned=control.owned_bytes();drop(control);
        let error=result.err().map(|error|match error.kind{ValueRefusalKind::OwnershipLimit=>"ownership",ValueRefusalKind::Canceled=>"canceled",_=>panic!("unexpected text refusal")});
        assert_eq!(logical.get(),row["logical"].as_u64().unwrap()as usize);assert_eq!(error.map(serde_json::Value::from).unwrap_or(serde_json::Value::Null),row["error"]);assert_eq!(owned,allocated.get()+wrapper);assert_eq!(input.as_ptr(),pointer);assert_eq!(input.len(),8194);
        if mode=="initial"{assert!(allocated.get()>0&&allocated.get()+wrapper<=4096);}if mode=="materialize-cancel"{assert!(allocated.get()>=4096);}
        drain(&mut recipient,allocated.get()+wrapper);
        actual.push(serde_json::json!({"id":row["id"],"error":error,"logical":logical.get(),"octets":output.into_inner()}));
    }
    for bytes in fixture["invalidUtf8"].as_array().unwrap(){let input:Vec<u8>=serde_json::from_value(bytes.clone()).unwrap();let mut text=PagedText::<131072>::empty();let mut accept=|_|true;let mut control=NativeDecodeControl::new(131072,&mut accept);assert_eq!(text.read_from_source(&input.as_slice(),&mut control).unwrap_err().kind,ValueRefusalKind::InvalidValue);assert_eq!(control.owned_bytes(),0);assert!(text.terminal_is_empty());}
    let script="const x=JSON.parse(await Bun.stdin.text());const f=x.fixture,ascii=Uint8Array.from({length:f.ascii.repetitions},()=>f.ascii.byte),unicode=new TextEncoder().encode(String.fromCharCode(f.unicode.prefixByte).repeat(f.unicode.prefixRepetitions)+f.unicode.middle+String.fromCharCode(f.unicode.suffixByte).repeat(f.unicode.suffixRepetitions)+f.unicode.tail);if(ascii.length!==8194||unicode.length!==8194)throw Error('independent exact extents');for(const v of f.invalidUtf8){let refused=false;try{new TextDecoder('utf-8',{fatal:true}).decode(Uint8Array.from(v));}catch{refused=true;}if(!refused)throw Error('invalid UTF8 accepted');}await Bun.write(Bun.stdout,JSON.stringify(f.cases.map(c=>({id:c.id,error:c.error,logical:c.logical,octets:c.error===null?Array.from(c.source==='ascii'?ascii:unicode):[]}))));";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(actual,serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout).unwrap());
    println!("[DEBUG] actual8194 semantic UTF8, Unicode across4096 boundary, initial4096 metadata refusal and interior partial-page cancellation preserve exact owned bytes and physically retire every allocation at1/4096");
}


#[test]
fn paged_semantic_text_encoding_source_retains_full8194_and_partial4096(){
    use crate::{NativeEncodeControl,ErasedSnapshotRetirement,SnapshotRetirementStep,paged_text::PagedText};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧾️semantic-text-retirement/🔣️.json")).unwrap();
    let text=fixture["text"].as_str().unwrap().repeat(fixture["textBytes"].as_u64().unwrap()as usize);assert_eq!(text.len(),8194);assert_eq!(fixture["maximumBytes"],4096);assert_eq!(fixture["maximumItems"],1);
    fn close(owner:&mut PagedText<20000>)->usize{let mut bytes=0;for _ in 0..8194+128{match owner.close_step(1,4096).unwrap(){SnapshotRetirementStep::Complete=>break,SnapshotRetirementStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1);assert!(released_bytes<=4096);bytes+=released_bytes;},step=>panic!("real paged text cannot retire its funded original page: {step:?}")}}assert!(owner.terminal_is_empty());bytes}
    let mut owner=PagedText::<20000>::empty();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);owner.read_from_encoding_source(&text.as_str(),&mut encoding).unwrap();assert!(owner.borrow().unwrap().bytes().eq(text.bytes()));assert_eq!(encoding.owned_bytes(),owner.allocated_bytes());let admitted=encoding.owned_bytes();assert_eq!(close(&mut owner),admitted);
    let mut owner=PagedText::<20000>::empty();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(4096,&mut allow);assert!(owner.read_from_encoding_source(&text.as_str(),&mut encoding).is_err());assert_eq!(owner.byte_len(),0);assert!(owner.allocated_bytes()>0);assert_eq!(encoding.owned_bytes(),owner.allocated_bytes());let admitted=encoding.owned_bytes();assert!(admitted<=4096);assert_eq!(close(&mut owner),admitted);
    let mut owner=PagedText::<20000>::empty();let mut stages=0;let mut cancel=|progress:crate::native_encoding::NativeEncodeProgress|{if progress.total==8194&&progress.completed==0{stages+=1;}!(stages>=2&&progress.completed>=256)};let mut encoding=NativeEncodeControl::new(65536,&mut cancel);assert!(owner.read_from_encoding_source(&text.as_str(),&mut encoding).is_err());assert_eq!(owner.byte_len(),256);assert!(owner.allocated_bytes()>=4096);assert!(owner.borrow().is_err());assert_eq!(encoding.owned_bytes(),owner.allocated_bytes());let admitted=encoding.owned_bytes();assert_eq!(close(&mut owner),admitted);
    println!("[DEBUG] paged semantic encoding owns full8194 source directly, retains real initial4096 metadata refusal and actual partial256 text pages, and disposes all actual backing1/4096");
}

#[test]
fn paged_semantic_text_parent_retains_original8194_actual_allocations_until4096_disposal(){
    use crate::{NativeDecodeControl,ErasedSnapshotRetirement,paged_text::PagedText,retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️text-parent/🔣️.json")).unwrap();let text=fixture["text"].as_str().unwrap().repeat(fixture["textBytes"].as_u64().unwrap()as usize);assert_eq!(text.len(),8194);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
    let mut owner=PagedText::<{isize::MAX as usize}>::empty();let mut allow=|_|true;let mut decoding=NativeDecodeControl::new(65536,&mut allow);owner.read_from_source(&text.as_str(),&mut decoding).unwrap();assert!(owner.borrow().unwrap().bytes().eq(text.bytes()));let actual=owner.allocated_bytes();assert_eq!(decoding.owned_bytes(),actual);let mut parent=ParentAllocationReturn::<64>::try_new(4096,actual).unwrap();assert!(!owner.return_one(&mut parent,0).unwrap().progressed);assert_eq!(owner.byte_len(),8194);assert!(owner.borrow().is_ok());assert_eq!(parent.retained_bytes(),0);let mut returned=0;
    for _ in 0..8194+128{if owner.terminal_is_empty(){break;}let before=owner.allocated_bytes();let progress=owner.return_one(&mut parent,1).unwrap();assert!(progress.progressed);assert_eq!(before-owner.allocated_bytes(),progress.returned_allocation_bytes);returned+=progress.returned_allocation_bytes;assert!(owner.borrow().is_err());}assert!(owner.terminal_is_empty());assert_eq!(returned,actual);assert_eq!(parent.retained_bytes(),actual);assert!(!parent.terminal_is_empty());let mut physical=0;for _ in 0..128{match parent.close_step(1,4096){AllocationReturnStep::Complete=>break,AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1);assert!(released_bytes<=4096);physical+=released_bytes;}}}assert!(parent.terminal_is_empty());assert_eq!(physical,actual);
    println!("[DEBUG] original8194 paged semantic text returns actual source backing to preadmitted real parent with zero child physical credit, denies zero-item handoff without changing text borrow, and parent alone physically releases1/4096");
}

#[test]
fn paged_semantic_text_stream_formats_full8194_and_retains_every_refusal(){
    use crate::{paged_text::{PagedText,write_encoding_format},NativeEncodeControl,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,SnapshotRetirementStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🌊️text-stream/🔣️.json")).unwrap();assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["initialAllocationBytes"],4096);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);let payload=std::iter::repeat_n('x',8181).collect::<String>();
    fn drain<const N:usize>(owner:&mut PagedText<N>)->usize{let mut released=0;for _ in 0..8194+128{match owner.close_step(1,4096).unwrap(){SnapshotRetirementStep::Complete=>break,SnapshotRetirementStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);released+=released_bytes;},SnapshotRetirementStep::Blocked=>panic!("stream owner blocked")}}assert!(owner.terminal_is_empty());released}
    let mut owner=PagedText::<{isize::MAX as usize}>::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);owner.encode_with(8194,&mut control,|output|write_encoding_format(output,format_args!("Set label to {}",payload))).unwrap();let view=owner.borrow().unwrap();assert_eq!(view.byte_len(),8194);assert!(view.bytes().eq(b"Set label to ".iter().copied().chain(std::iter::repeat_n(b'x',8181))));assert_eq!(owner.allocated_bytes(),control.owned_bytes());let allocated=owner.allocated_bytes();assert_eq!(drain(&mut owner),allocated);
    for mode in["initial","interior","ignored","producer","segment"]{let mut owner=PagedText::<{isize::MAX as usize}>::empty();let mut callback=|progress:crate::native_encoding::NativeEncodeProgress|mode!="interior"||progress.completed<256;let mut control=NativeEncodeControl::new(if mode=="initial"{4096}else{65536},&mut callback);let result=owner.encode_with(if mode=="segment"{128}else{8194},&mut control,|output|{
        if mode=="ignored"{assert!(output.write_text(&payload).is_ok());assert!(output.write_text("too many octets after real prefix").is_err());return Ok(())}
        write_encoding_format(output,format_args!("Set label to {}",payload))?;if mode=="producer"{return Err(ValueError::new(ValueRefusalKind::Canceled,"whole semantic producer refused after full output"))}Ok(())
    });assert!(result.is_err(),"{mode}");assert!(owner.borrow().is_err(),"{mode}");assert_eq!(owner.allocated_bytes(),control.owned_bytes());if mode=="initial"{assert_eq!(owner.byte_len(),0);assert!(owner.allocated_bytes()>0&&owner.allocated_bytes()<=4096);}if mode=="interior"{assert!(owner.byte_len()>0&&owner.byte_len()<8194);}if mode=="producer"{assert_eq!(owner.byte_len(),8194);}if mode=="segment"{assert!(owner.byte_len()<=128);}let allocated=owner.allocated_bytes();assert_eq!(drain(&mut owner),allocated);}
    eprintln!("[DEBUG] semantic authored formatting owns full8194 directly at64k; actual initial4096, interior cancellation, ignored write refusal, producer refusal and segment refusal retain actual pages until1/4096 physical close");
}

#[test]
fn paged_semantic_text_inline_stream_preserves128_and_full8194_without_shadow(){
    use crate::{paged_text::{PagedText,InlineTextBuffer,write_encoding_format},NativeEncodeControl,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,SnapshotRetirementStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🤏️inline-text-stream/🔣️.json")).unwrap();assert_eq!(fixture["inlineBytes"],128);assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["initialAllocationBytes"],4096);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);let payload=std::iter::repeat_n('x',8181).collect::<String>();
    fn drain<const N:usize>(paged:&mut PagedText<N>,inline:&mut InlineTextBuffer)->usize{if !inline.terminal_is_empty(){assert!(!inline.close_one(0));assert!(inline.close_one(1));assert!(inline.borrow().is_err());}let mut released=0;for _ in 0..8194+128{match paged.close_step(1,4096).unwrap(){SnapshotRetirementStep::Complete=>break,SnapshotRetirementStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);released+=released_bytes;},SnapshotRetirementStep::Blocked=>panic!("intrinsic owner blocked")}}assert!(inline.terminal_is_empty()&&paged.terminal_is_empty());released}
    let mut known=InlineTextBuffer::empty();let mut allow=|_|true;let mut decoding=crate::NativeDecodeControl::new(4096,&mut allow);known.read_from_source(&fixture["knownShortText"].as_str().unwrap(),&mut decoding).unwrap();assert_eq!(known.borrow().unwrap(),fixture["knownShortText"].as_str().unwrap());assert_eq!(decoding.owned_bytes(),0);assert!(known.close_one(1));assert!(known.borrow().is_err());let mut known=InlineTextBuffer::empty();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(4096,&mut allow);known.read_from_encoding_source(&fixture["knownShortText"].as_str().unwrap(),&mut encoding).unwrap();assert_eq!(known.borrow().unwrap(),fixture["knownShortText"].as_str().unwrap());assert_eq!(encoding.owned_bytes(),0);assert!(known.close_one(1));assert!(known.borrow().is_err());
    let mut paged=PagedText::<{isize::MAX as usize}>::empty();let mut inline=InlineTextBuffer::empty();let mut active=false;let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);paged.encode_with_inline(&mut inline,&mut active,8194,&mut control,|output|write_encoding_format(output,format_args!("{}","first"))).unwrap();assert!(!active);assert_eq!(inline.borrow().unwrap(),"first");assert!(paged.borrow().is_err());assert_eq!(paged.byte_len(),0);assert_eq!(control.owned_bytes(),0);assert_eq!(drain(&mut paged,&mut inline),0);
    let mut paged=PagedText::<{isize::MAX as usize}>::empty();let mut inline=InlineTextBuffer::empty();let mut active=false;let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);paged.encode_with_inline(&mut inline,&mut active,8194,&mut control,|output|write_encoding_format(output,format_args!("Set label to {}",payload))).unwrap();assert!(active&&inline.terminal_is_empty());assert!(inline.borrow().is_err());assert!(paged.borrow().unwrap().bytes().eq(b"Set label to ".iter().copied().chain(std::iter::repeat_n(b'x',8181))));let allocated=paged.allocated_bytes();assert_eq!(allocated,control.owned_bytes());assert_eq!(drain(&mut paged,&mut inline),allocated);
    for mode in["initial","interior","ignored","short"]{let mut paged=PagedText::<{isize::MAX as usize}>::empty();let mut inline=InlineTextBuffer::empty();let mut active=false;let mut callback=|progress:crate::native_encoding::NativeEncodeProgress|mode!="interior"||progress.completed<256;let mut control=NativeEncodeControl::new(if mode=="initial"{4096}else{65536},&mut callback);let result=paged.encode_with_inline(&mut inline,&mut active,8194,&mut control,|output|{if mode=="short"{output.write_text("first")?;return Err(ValueError::new(ValueRefusalKind::Canceled,"intrinsic producer refused after short source"))}if mode=="ignored"{output.write_text("Set label to ")?;assert!(output.write_text(&payload).is_ok());assert!(output.write_text("too much after full8194").is_err());return Ok(())}write_encoding_format(output,format_args!("Set label to {}",payload))});assert!(result.is_err(),"{mode}");assert!(paged.borrow().is_err(),"{mode}");assert!(inline.borrow().is_err(),"{mode}");assert_eq!(paged.allocated_bytes(),control.owned_bytes());if mode=="initial"{assert!(active);assert_eq!(inline.as_bytes(),b"Set label to ");assert_eq!(paged.byte_len(),0);assert!(paged.allocated_bytes()>0&&paged.allocated_bytes()<=4096);}if mode=="interior"{assert!(active&&paged.byte_len()>0&&paged.byte_len()<8194);}if mode=="short"{assert!(!active);assert_eq!(inline.as_bytes(),b"first");assert_eq!(paged.allocated_bytes(),0);}let allocated=paged.allocated_bytes();assert_eq!(drain(&mut paged,&mut inline),allocated);}
    eprintln!("[DEBUG] intrinsic128 semantic formatter owns short text without heap, transfers bounded prefix into exact full8194 pages, retains actual inline+metadata at initial4096 and every partial/ignored/whole refusal until explicit1/4096 close");
}
