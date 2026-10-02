use semio_framework_os_kernel::native_decoding::*;

#[test]
fn sqlite_snapshot_nested_native_stages_preserve_parent_and_cumulative_ownership(){
    use std::{cell::Cell,io::Write,process::{Command,Stdio}};
    fn label(error:&str)->&str{match error{"native decoding exceeded declared stage workload"=>"overrun","native decoding canceled"=>"canceled","native decoding ownership exceeds caller limit"=>"limit","owned child rejected"=>"rejected",_=>panic!("unexpected decode error: {error}")}}
    fn run(control:&mut NativeDecodeControl<'_>,operations:&serde_json::Value,errors:&mut Vec<String>)->Result<(),String>{
        for op in operations.as_array().unwrap(){match op["kind"].as_str().unwrap(){
            "begin"=>control.begin_stage(op["units"].as_u64().unwrap() as usize)?,
            "advance"=>control.advance(op["units"].as_u64().unwrap() as usize)?,
            "charge"=>control.charge(op["units"].as_u64().unwrap() as usize)?,
            "reject"=>return Err("owned child rejected".into()),
            "scope"=>{if let Err(error)=control.scoped_stage(|child|run(child,&op["operations"],errors)){errors.push(label(&error).to_owned());}},
            kind=>panic!("unexpected stage operation: {kind}")
        }}Ok(())
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🪆️stage/🔣️.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../🧬️schema/🪆️stage/🔣️.json")).unwrap();
    let mut actual=Vec::new();
    for case in fixture["cases"].as_array().unwrap(){
        let progress=Cell::new(NativeDecodeProgress{completed:0,total:0,owned_bytes:0});
        let mut callback=|event:NativeDecodeProgress|{progress.set(event);case["cancel"].is_null()||event.total!=case["cancel"]["total"].as_u64().unwrap() as usize||event.completed<(case["cancel"]["at"].as_u64().unwrap() as usize)};
        let mut control=NativeDecodeControl::new(case["maximumBytes"].as_u64().unwrap() as usize,&mut callback);let mut errors=Vec::new();
        if let Err(error)=run(&mut control,&case["operations"],&mut errors){errors.push(label(&error).to_owned());}control.checkpoint().unwrap();
        let event=progress.get();let result=serde_json::json!({"progress":{"completed":event.completed,"total":event.total,"ownedBytes":event.owned_bytes},"errors":errors});
        assert_eq!(result,case["expected"],"{}",case["id"]);actual.push(result);
    }
    let script=format!("{}\nimport Ajv from 'ajv/dist/2020.js';const input=JSON.parse(await Bun.stdin.text());const ajv=new Ajv({{strict:true}});if(!ajv.validate(input.schema,input.fixture))throw Error('stage fixture');const admit=ajv.getSchema(input.schema.$id+'#/$defs/case');if(input.fixture.cases.some(c=>!admit(c))||input.fixture.refusals.some(c=>admit(c.value)))throw Error('stage admission');await Bun.write(Bun.stdout,JSON.stringify(input.fixture.cases.map(stageOracle)));",include_str!("🪆️stage/🟦️.ts"));
    let mut child=Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(actual,serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout).unwrap());
}

#[test]
fn sqlite_snapshot_native_materialization_preserves_caller_budget_and_cancellation() {
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../🧬️schema/🔣️.json")).unwrap();
    let mut actual=Vec::new();
    for case in fixture["cases"].as_array().unwrap() {
        let mut progress=|_:NativeDecodeProgress|true;
        let mut control=NativeDecodeControl::new(case["maximumBytes"].as_u64().unwrap() as usize,&mut progress);
        let mut accepted=0;
        for bytes in case["charges"].as_array().unwrap() { if control.charge(bytes.as_u64().unwrap() as usize).is_err(){break} accepted+=1; }
        assert_eq!(accepted,case["acceptedCharges"].as_u64().unwrap());assert_eq!(control.owned_bytes(),case["ownedBytes"].as_u64().unwrap() as usize);
        actual.push(serde_json::json!({"acceptedCharges":accepted,"ownedBytes":control.owned_bytes()}));
    }
    let script="import {Database} from 'bun:sqlite';import Ajv from 'ajv/dist/2020.js';const x=JSON.parse(await Bun.stdin.text());if(!new Ajv({strict:true}).validate(x.schema,x.fixture))throw Error('fixture');const db=new Database(':memory:');db.run('CREATE TABLE charge(position INTEGER PRIMARY KEY,bytes INTEGER NOT NULL)');const actual=x.fixture.cases.map(c=>{db.run('DELETE FROM charge');let acceptedCharges=0,ownedBytes=0;for(const bytes of c.charges){db.run('INSERT INTO charge VALUES(?,?)',acceptedCharges+1,bytes);const n=db.query('SELECT SUM(bytes) AS n FROM charge').get().n;if(n>c.maximumBytes)break;ownedBytes=n;acceptedCharges++;}return {acceptedCharges,ownedBytes};});db.close();await Bun.write(Bun.stdout,JSON.stringify(actual));";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(actual,serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout).unwrap());
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
    let n=&fixture["nestedStage"];let mut control=NativeDecodeControl::new(64,&mut accepted);control.begin_stage(n["parentTotal"].as_u64().unwrap() as usize).unwrap();control.advance(n["parentBefore"].as_u64().unwrap() as usize).unwrap();control.scoped_stage(|child|->Result<(),String>{child.begin_stage(n["childTotal"].as_u64().unwrap() as usize)?;child.charge(n["childBytes"].as_u64().unwrap() as usize)?;child.advance(n["childTotal"].as_u64().unwrap() as usize)}).unwrap();control.advance(n["parentAfter"].as_u64().unwrap() as usize).unwrap();assert_eq!(control.owned_bytes(),n["childBytes"].as_u64().unwrap() as usize);assert!(control.scoped_stage(|child|->Result<(),String>{child.begin_stage(2)?;Err("owned child failed".into())}).is_err());control.advance((n["parentTotal"].as_u64().unwrap()-n["parentBefore"].as_u64().unwrap()-n["parentAfter"].as_u64().unwrap()) as usize).unwrap();assert!(control.step().is_err());
    let mut control=NativeDecodeControl::new(0,&mut accepted);for case in fixture["utf8"].as_array().unwrap(){let bytes:Vec<u8>=serde_json::from_value(case["bytes"].clone()).unwrap();assert_eq!(control.borrow_text(&bytes).is_ok(),case["accepted"].as_bool().unwrap());assert_eq!(control.owned_bytes(),0);}
}
