import ts from"typescript";
import fixtureSchema from"./🧬️schema/🔣️.json";
import{physicalSourceOriginV1}from"../../../../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{closureCaptureWithinBudgetV1,closureRetirementWindowDueV1}from"../../🟦️.ts";
import storage from"../../../../../../../../../🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json"with{type:"json"};
import protocol from"../../🧬️schema/🔣️.json"with{type:"json"};
import cases from"./🧫️cases.json"with{type:"json"};

const policy=storage.definitions.OptionalQuota.policy,sources=new URL("../../🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL));

test("the retirement window is a required, finite policy key and the corpus uses the declared value",()=>{
 expect(policy.retirementWindowMilliseconds).toBe(cases.windowMilliseconds);
 const validate=new Ajv({strict:false}).compile(storage.definitions.OptionalQuota.definitions.OptionalQuotaPolicy);
 expect(validate(policy)).toBe(true);
 const{retirementWindowMilliseconds:_,...without}=policy;
 expect(validate(without)).toBe(false);
 expect(validate({...policy,retirementWindowMilliseconds:0})).toBe(false);
 expect(validate({...policy,retirementWindowMilliseconds:60001})).toBe(false);
 expect(validate({...policy,retirementWindowMilliseconds:1.5})).toBe(false);
},60000);

test("the window is due after at least one step once the declared time elapsed, equal to an independent SQLite computation",()=>{
 const database=new Database(":memory:");
 try{
  for(const row of cases.window){
   const actual=closureRetirementWindowDueV1(row.startedMs,row.nowMs,row.iterations,cases.windowMilliseconds),oracle=database.query("SELECT (?1>0 AND ?2-?3>=?4) AS due").get(row.iterations,row.nowMs,row.startedMs,cases.windowMilliseconds)as{due:number};
   expect(actual,row.id).toBe(row.due);expect(Boolean(oracle.due),row.id).toBe(row.due);
  }
 }finally{database.close();}
},60000);

test("every capture is charged against the proof cap on its own, where the old cumulative budget would fail a long retirement",()=>{
 const database=new Database(":memory:"),cap=cases.budget.maximumProofBytes;
 expect(protocol.definitions.Protocol.const.maximumProofBytes).toBe(cap);
 try{
  for(const row of cases.budget.captures){
   const perCapture=!closureCaptureWithinBudgetV1(row.bytes,cap),cumulative=row.bytes.reduce((sum,value)=>sum+value,0)>cap||row.bytes.some(value=>value<0);
   database.exec("DROP TABLE IF EXISTS capture;CREATE TABLE capture(bytes INTEGER)");
   for(const bytes of row.bytes)database.query("INSERT INTO capture VALUES(?)").run(bytes);
   const oracle=database.query("SELECT COUNT(*) FILTER(WHERE bytes>?1 OR bytes<0)>0 AS each,COALESCE(SUM(bytes),0)>?1 OR COUNT(*) FILTER(WHERE bytes<0)>0 AS total FROM capture").get(cap)as{each:number;total:number};
   expect(perCapture,row.id).toBe(row.perCaptureFails);expect(Boolean(oracle.each),row.id).toBe(row.perCaptureFails);
   expect(cumulative,row.id).toBe(row.cumulativeFails);expect(Boolean(oracle.total),row.id).toBe(row.cumulativeFails);
  }
 }finally{database.close();}
 expect(closureCaptureWithinBudgetV1([1.5])).toBe(false);
 expect(closureCaptureWithinBudgetV1([])).toBe(true);
},60000);

test("retirement windows preserve current control and exact primary failures",async()=>{
 expect(new Ajv({strict:true}).validate(fixtureSchema,cases)).toBe(true);
 const tree=ts.createSourceFile("closures.ts",readFileSync(sources,"utf8"),ts.ScriptTarget.Latest,true),take=(name:string)=>{const row=tree.statements.find(value=>ts.isFunctionDeclaration(value)&&value.name?.text===name);if(!row)throw Error("Actual retirement owner required: "+name);return row.getText(tree).replace(/^export /u,"");},compile=(code:string,ports:Record<string,unknown>,answer:string)=>Function(...Object.keys(ports),ts.transpileModule(code,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText+";return "+answer)(...Object.values(ports)),annotate=compile(take("annotateRetirementFailureV1"),{},"annotateRetirementFailureV1"),make=(kind:string):unknown=>{const error=Error("primary");if(kind==="frozen")return Object.freeze(error);if(kind==="sealed")return Object.seal(error);if(kind==="getter"){Object.defineProperty(error,"cause",{get(){throw Error("secondary");}});return error;}if(kind==="proxy")return new Proxy(error,{getOwnPropertyDescriptor(){throw Error("secondary");},defineProperty(){throw Error("secondary");}});if(kind==="existing"){Object.defineProperty(error,"cause",{value:Object.freeze({existing:true})});return error;}return kind==="undefined"?undefined:kind==="false"?false:kind==="null"?null:kind==="zero"?0:kind==="empty"?"":error;},db=new Database(":memory:");
 try{
  for(const row of cases.primary){const primary=make(row.kind),actual=annotate(primary,"graph-group-retirement-failed",7,2),oracle=db.query("select ?1='error' attached").get(row.kind)as{attached:number};expect(Object.is(actual,primary),row.id).toBe(true);expect(Boolean(oracle.attached)).toBe(row.attached);if(row.attached){const cause=Object.getOwnPropertyDescriptor(actual as object,"cause")?.value;expect(cause).toEqual({version:1,kind:"graph-group-retirement-failed",retiredBytes:7,retiredFiles:2});expect(Object.isFrozen(cause)).toBe(true);}if(row.kind==="existing")expect(Object.getOwnPropertyDescriptor(primary as object,"cause")?.value).toEqual({existing:true});}
  for(const row of cases.boundary){const primary=make(row.kind),image=Object.freeze({root:"controlled"}),custody=Object.freeze({kind:"kernel-ex",control:{}}),token={},proof={intents:new Map(),live:true};let now=0,stopped=false,captures=0,invalidations=0;
   const run=compile(take("retireBuildGraphOptionalGroupsHeadroomV1"),{requireNativePlatformImageWriterV1:(actualImage:unknown,actualCustody:unknown)=>{expect(actualImage).toBe(image);expect(actualCustody).toBe(custody);},schema:protocol,validateJsonSchemaSubset:()=>[],optionalPolicy:storage.definitions.OptionalQuota,checkResourceLeaseControl:(control:unknown)=>{expect(control).toBe(custody.control);if(stopped&&!row.liveControl)throw primary;},nativeImageSpaceV1:()=>({bavail:0,bsize:1}),closureRetirementWindowDueV1,Date:{now:()=>now},captureClosureProof:async()=>{captures++;return token;},closureProof:(actual:unknown)=>{expect(actual).toBe(token);return proof;},liveOptionalBytes:()=>0,invalidateGroup:async()=>{invalidations++;stopped=true;now=cases.windowMilliseconds;proof.live=false;return true;},annotateRetirementFailureV1:annotate},"retireBuildGraphOptionalGroupsHeadroomV1");
   let failed=false,failure:unknown,result:any;try{result=await run(image,custody,{minimumAvailableBytes:1});}catch(error){failed=true;failure=error;}
   const oracle=db.query("select ?1 partial").get(Number(row.liveControl))as{partial:number};expect(Boolean(oracle.partial)).toBe(row.partial);expect(!failed,row.id).toBe(row.partial);if(failed)expect(Object.is(failure,primary),row.id).toBe(true);else expect(result).toEqual({retiredBytes:0,retiredFiles:0,partial:true});expect(captures).toBe(1);expect(invalidations).toBe(1);
  }
 }finally{db.close();}
},60000);
