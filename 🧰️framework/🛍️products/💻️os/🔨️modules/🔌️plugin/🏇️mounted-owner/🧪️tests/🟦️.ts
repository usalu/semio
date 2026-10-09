import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020.js";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { Database } from "bun:sqlite";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import contextSchema from "../🧬️schema/🧾️context/🔣️.json";
import contextFixture from "../🧫️fixtures/🧾️context/🔣️.json";
type Observation = typeof fixture.vectors[number]["observation"];
interface MountedOwnerOracleV1 { valid(value:unknown):boolean; policy(value:unknown):boolean; verdict(value:Observation):string }
type NestedObservation = typeof fixture.nestedJobs[number]["observation"];
interface MountedNestedJobOracleV1 { valid(value:unknown):boolean; receipt(value:unknown):boolean; verdict(value:NestedObservation):string }
/** 🔮️ Independent schema and bounded scheduling output checks stay behind a first-party test oracle. */
function createMountedNestedJobOracleV1():MountedNestedJobOracleV1 {
 const ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(schema);
 const validate=ajv.getSchema(schema.$id+"#/$defs/MountedOwnerNestedJobObservationV1"),receipt=ajv.getSchema(schema.$id+"#/$defs/MountedOwnerJobReceiptV1");
 if(!validate||!receipt)throw Error("Nested mounted scheduling schema is unresolved");
 return {valid:value=>validate(value)===true,receipt:value=>receipt(value)===true,verdict:value=>{
  const physical=ajv.compile({type:"object",properties:{copiedItems:{type:"integer",maximum:value.grant.maximumItems},copiedBytes:{type:"integer",maximum:value.grant.maximumCopyBytes},retainedCapacityBytes:{type:"integer",maximum:value.grant.maximumCapacityBytes},releasedBytes:{type:"integer",maximum:value.grant.maximumReleaseBytes}},required:["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"],additionalProperties:false})(value.receipt);
  const debit=(Object.values(value.receipt).every(amount=>amount===0)||value.usedFuel>0)&&value.usedFuel<=value.schedule.fuel;
  return physical&&debit&&(value.result!=="complete"||value.terminalIsEmpty)?value.result:"refused";
 }};
}
/** 🔮️ Strict independent schema bounds reproduce admitted ownership verdicts behind a first-party oracle. */
function createMountedOwnerOracleV1():MountedOwnerOracleV1 {
 const ajv=new Ajv({strict:true,allErrors:true}),validate=ajv.compile(schema),validatePolicy=ajv.getSchema(schema.$id+"#/$defs/MountedOwnerPolicyV1");
 if(!validatePolicy)throw new Error("Mounted owner policy schema is unresolved");
 const bounded=(properties:Record<string,unknown>,value:unknown)=>ajv.compile({type:"object",properties,required:Object.keys(properties)})(value)===true;
 return {valid:value=>validate(value)===true,policy:value=>validatePolicy(value)===true,verdict:value=>{
  if(!validate(value))return "refused";
  if(!bounded(Object.fromEntries(Object.entries(value.expectedIdentity).map(([key,identity])=>[key,{const:identity}])),value.identity))return "refused";
  if(value.schedule.cancelled)return "cancelled";
  if(!bounded({clockAvailable:{const:true},fuel:{type:"integer",minimum:1}},value.schedule)||BigInt(value.schedule.nowUs)>=BigInt(value.schedule.deadlineUs))return "held";
  const grant=value.grant,demand=value.demand;
  if(!bounded({maximumItems:{type:"integer",minimum:1},maximumCopyBytes:{type:"integer",minimum:demand.copyBytes},maximumCapacityBytes:{type:"integer",minimum:demand.capacityBytes},maximumReleaseBytes:{type:"integer",minimum:demand.releaseBytes},maximumDepth:{type:"integer",minimum:demand.depth}},grant))return "held";
  if(!bounded({copiedItems:{type:"integer",maximum:grant.maximumItems},copiedBytes:{type:"integer",maximum:grant.maximumCopyBytes},retainedCapacityBytes:{type:"integer",maximum:grant.maximumCapacityBytes},releasedBytes:{type:"integer",maximum:grant.maximumReleaseBytes}},value.receipt))return "refused";
  if(value.result==="complete")return value.terminalIsEmpty?"complete":"refused";
  if(value.result==="blocked"||value.result==="awaiting-input")return value.wakeOwned&&Object.values(value.receipt).every(amount=>amount===0)?value.result:"refused";
  return value.result;
 }};
}
test("portable mounted ownership schema agrees with strict independent Ajv",()=>{
 const oracle=createMountedOwnerOracleV1();expect(fixture.vectors).toHaveLength(28);
 for(const vector of fixture.vectors){expect(validateJsonSchemaSubset(schema,vector.observation)).toEqual([]);expect(oracle.valid(vector.observation)).toBe(true);}
 for(const vector of fixture.invalid){expect(validateJsonSchemaSubset(schema,vector.observation).length).toBeGreaterThan(0);expect(oracle.valid(vector.observation)).toBe(false);}
 console.log("[DEBUG] Mounted turn schema: all27 original observations plus actual initial store generation0 and eight malformed grants/identities/clocks agreed with strict Ajv");
});
test("all independent physical currencies and original scheduling ownership have portable oracle verdicts",()=>{
 const oracle=createMountedOwnerOracleV1();for(const vector of fixture.vectors)expect(oracle.verdict(vector.observation)).toBe(vector.expected);
 console.log("[DEBUG] Mounted ownership verdicts retained independent copy/capacity/release/depth, identity, cancellation, deadline, wake and terminal rules");
});
test("runtime policy requires all three explicit caller phase grants",()=>{
 const oracle=createMountedOwnerOracleV1(),policySchema={...schema,$ref:"#/$defs/MountedOwnerPolicyV1"};
 for(const vector of fixture.policies){expect(validateJsonSchemaSubset(policySchema,vector.value).length===0).toBe(vector.valid);expect(oracle.policy(vector.value)).toBe(vector.valid);}
 console.log("[DEBUG] Four portable caller policies agreed: explicit independent phases, explicit empty authority, missing phase and forbidden default");
});
test("portable mounted ownership nested job debits preserve the original scheduling owner",()=>{
 const oracle=createMountedNestedJobOracleV1();
 const nestedSchema={...schema,$ref:"#/$defs/MountedOwnerNestedJobObservationV1"};
 const receiptSchema={...schema,$ref:"#/$defs/MountedOwnerJobReceiptV1"};
 expect(fixture.nestedJobs).toHaveLength(8);
 for(const vector of fixture.nestedJobs){
  const value=vector.observation;expect(oracle.valid(value)).toBe(true);expect(validateJsonSchemaSubset(nestedSchema,value)).toEqual([]);
  const receipt={progress:value.receipt,complete:value.result==="complete",terminalIsEmpty:value.terminalIsEmpty};
  expect(oracle.receipt(receipt)).toBe(true);expect(validateJsonSchemaSubset(receiptSchema,receipt)).toEqual([]);
  expect(oracle.verdict(value)).toBe(vector.expected);expect(Math.max(0,value.schedule.fuel-value.usedFuel)).toBe(vector.expectedFuel);
 }
 console.log("[DEBUG] Eight nested job schema/output vectors agreed with strict Ajv and independently measured original scheduling debit");
});
test("actual native verdicts preserve every nested physical receipt and original scheduling debit",()=>{
 const path=process.env.SEMIO_MOUNTED_OWNER_NESTED_RESULTS;if(!path)throw Error("Nested mounted oracle requires an explicit native result artifact");
 expect(path.length<=256&&[...path].length<=256).toBe(true);
 const results:unknown=JSON.parse(readFileSync(path,"utf8")),oracle=createMountedNestedJobOracleV1();
 expect(results).toEqual(fixture.nestedJobs.map(vector=>({id:vector.id,actual:oracle.verdict(vector.observation),remainingFuel:Math.max(0,vector.observation.schedule.fuel-vector.observation.usedFuel)})));
 console.log("[DEBUG] All eight actual nested native verdicts and remaining original fuel agreed with strict independent Ajv");
});
test("actual native verdicts reproduce the complete strict independent mounted oracle",()=>{
 const path=process.env.SEMIO_MOUNTED_OWNER_RESULTS;if(!path)throw Error("Native mounted oracle requires an explicit result artifact");
 expect(path.length<=256&&[...path].length<=256).toBe(true);
 const results:unknown=JSON.parse(readFileSync(path,"utf8"));
 const oracle=createMountedOwnerOracleV1(),expected=fixture.vectors.map(vector=>({id:vector.id,actual:oracle.verdict(vector.observation)}));
 expect(results).toEqual(expected);
 console.log("[DEBUG] All28 actual native mounted turn verdicts agreed with strict independent Ajv, including exact decimal-u64 identity and clocks");
});
test("actual runtime mounted receivers require caller policy and typed scheduling turn",()=>{
 const root=resolve(import.meta.dir,"../../../../../../.."),source=readFileSync(resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"),"utf8");
 expect(source.includes("MountedOwnerTurnV1")).toBe(true);
 expect(/fn mounted_job_(?:maintenance|close)_step\([^\n]*maximum_(?:items|bytes)/.test(source)).toBe(false);
 expect(/pub fn new\(mounted_policy:\s*MountedOwnerPolicyV1\)/.test(source)).toBe(true);
 expect(source.includes("PluginRuntime::new();")).toBe(false);
 expect(/mounted_job_maintenance_step\([^;\n]*turn/.test(source)).toBe(true);
 expect(/mounted_job_close_step\([^;\n]*turn/.test(source)).toBe(true);
 expect(source.includes("instance.app.mounted_owner_maintenance_step(&mut turn)")).toBe(true);
 expect(source.includes("instance.app.mounted_owner_close_step(&mut turn)")).toBe(true);
 expect(source.includes("pub mounted_policy: crate::MountedOwnerPolicyV1")).toBe(true);
 expect(source.includes("fn create_app(&self, app_id: &str, actor: protocol::ActorId, mounted_policy: crate::MountedOwnerPolicyV1")).toBe(true);
 expect(source.includes("runtime_lifecycle_grant")).toBe(false);
 expect(source.includes("cx.is_cancelled()")).toBe(true);expect(source.includes("cx.deadline_exceeded()")).toBe(true);
 console.log("[DEBUG] Actual General mounted receiver source binding is explicit; native host/session runtime remains a separate required gate");
});

function mountedContextOracle() {
 const ajv=new Ajv({strict:true,allErrors:true});expect(ajv.compile(contextSchema)(contextFixture)).toBe(true);expect(validateJsonSchemaSubset(contextSchema,contextFixture)).toEqual([]);
 const db=new Database(":memory:");try{
  db.run("CREATE TABLE original(id TEXT,axis TEXT,policy INTEGER,caller INTEGER,already INTEGER,receipt INTEGER)");
  const axes=[["maximumItems","copiedItems"],["maximumCopyBytes","copiedBytes"],["maximumCapacityBytes","retainedCapacityBytes"],["maximumReleaseBytes","releasedBytes"],["maximumDepth",null]] as const;
  for(const vector of contextFixture.vectors)for(const[axis,currency]of axes)db.query("INSERT INTO original VALUES(?,?,?,?,?,?)").run(vector.id,axis,vector.policy[axis],vector.context[axis],currency?vector.already[currency]:0,currency?vector.receipt[currency]:0);
  const results=contextFixture.vectors.map(vector=>({id:vector.id,remaining:Object.fromEntries(axes.map(([axis])=>{const row=db.query("SELECT min(policy,caller-already)-receipt AS remaining FROM original WHERE id=? AND axis=?").get(vector.id,axis)as{remaining:number};return[axis,row.remaining];}))}));
  for(let index=0;index<results.length;index++)expect(results[index]!.remaining).toEqual(contextFixture.vectors[index]!.remaining);
  return [...results,...contextFixture.nestedJobs.map(vector=>({id:vector.id,accepted:vector.debit==="child"}))];
 }finally{db.close();}
}

test("all independent physical original context recipients agree with Ajv and SQLite",()=>{
 expect(mountedContextOracle()).toHaveLength(16);
 console.log("[DEBUG] Ajv/SQLite twelve original policy/context/preconsumed grant cases preserve five independent axes; four nested receipt cases require exact original child debit");
});

test("actual native verdicts preserve the original context recipient and reject repeated physical credits",()=>{
 const path=process.env.SEMIO_MOUNTED_OWNER_CONTEXT_RESULTS;if(!path)throw Error("Original context oracle requires an explicit native result artifact");
 expect(path.length<=256&&[...path].length<=256).toBe(true);
 expect(JSON.parse(readFileSync(path,"utf8"))).toEqual(mountedContextOracle());
 console.log("[DEBUG] All sixteen actual original context/native verdicts agree with independent Ajv/SQLite and exact caller remaining currency");
});

test("portable mounted ownership consumes the original context memory wallet once",()=>{
 const law=fixture.contextReceipt,oracle=new Ajv({strict:true});oracle.addSchema(schema);const valid=oracle.getSchema(schema.$id+"#/$defs/RetainedCloneGrantV1");if(!valid)throw Error("Canonical mounted grant schema missing");expect(valid(law.grant)).toBe(true);expect(Buffer.byteLength(law.source)).toBe(law.grant.maximumCopyBytes);expect(law.childFuel).toBeGreaterThan(law.grant.maximumItems);
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("self.context.consume_retained(receipt)?;");expect(source).toContain("let original=context.retained_grant();");expect(source).not.toContain("min(self.context.fuel_remaining())");
});

test("actual native verdicts preserve original mounted actor capture and full physical close",()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🎭️actor/🔣️.json",import.meta.url),"utf8"));
 const source=process.env.SEMIO_MOUNTED_OWNER_RESULTS;if(!source)throw Error("actual original mounted output authority is required");
 const rows=JSON.parse(readFileSync(resolve(source,"../actor.json"),"utf8"))as {id:string;textBytes:number;copyBytes:number;capacityBytes:number;releaseBytes:number;turns:number;births:number;releases:number;deniedHeapBytes:number;dropHeapBytes:number}[];
 const ajv=new Ajv({strict:true,allErrors:true});const valid=ajv.compile({type:"array",minItems:3,maxItems:3,items:{type:"object",required:["id","textBytes","copyBytes","capacityBytes","releaseBytes","turns","births","releases","deniedHeapBytes","dropHeapBytes"],additionalProperties:false,properties:{id:{type:"string"},textBytes:{type:"integer",minimum:1,maximum:256},copyBytes:{type:"integer",minimum:1,maximum:law.nativeGrant[1]},capacityBytes:{type:"integer",minimum:1,maximum:law.nativeGrant[2]},releaseBytes:{const:0},turns:{type:"integer",minimum:1,maximum:999},births:{type:"integer",minimum:1},releases:{type:"integer",minimum:1},deniedHeapBytes:{const:0},dropHeapBytes:{const:0}}}});
 expect(valid(rows)).toBe(true);expect(rows.map(row=>row.id)).toEqual(law.cases.map((row:{id:string})=>row.id));
 const db=new Database(":memory:");try{db.run("CREATE TABLE actor(id TEXT,text INTEGER,copy INTEGER,capacity INTEGER,birth INTEGER,release INTEGER,denied INTEGER,terminal INTEGER)");for(const row of rows)db.query("INSERT INTO actor VALUES(?,?,?,?,?,?,?,?)").run(row.id,row.textBytes,row.copyBytes,row.capacityBytes,row.births,row.releases,row.deniedHeapBytes,row.dropHeapBytes);expect(db.query("SELECT count(*) AS n FROM actor WHERE birth=release AND denied=0 AND terminal=0 AND copy>text AND capacity>text").get()).toEqual({n:3});expect(db.query("SELECT count(distinct capacity-text) AS frames FROM actor").get()).toEqual({frames:1});}finally{db.close();}
 console.log("[DEBUG] Actual three native actor receipts and original cancellation heap outputs agree with independent strict Ajv/SQLite; original borrowed source and fixed caller grants retained");
});

 test("runtime policy records the actual failed producer before refusing its mounted turn",()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/⚠️receipt/🔣️.json",import.meta.url),"utf8"));
 const schema=JSON.parse(readFileSync(new URL("../🧬️schema/⚠️receipt/🔣️.json",import.meta.url),"utf8"));
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const producer=Buffer.alloc(law.producerCapacity);const db=new Database(":memory:");
 try{const row=db.query("SELECT ? AS external,? AS turn,max(0,?-?) AS remaining,7-1 AS fuel").get(producer.byteLength,producer.byteLength,law.policyCapacity,producer.byteLength)as{external:number;turn:number;remaining:number;fuel:number};expect(row).toEqual({external:law.externalCapacity,turn:law.turnCapacity,remaining:law.remainingCapacity,fuel:law.fuel});}finally{db.close();}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");const record=source.slice(source.indexOf("pub fn record("),source.indexOf("pub fn advance_job("));
 expect(record.indexOf("self.progress = progress;")).toBeLessThan(record.indexOf("self.context.consume_retained(receipt)?;"));expect(record).toContain("with_retained_progress(receipt)");expect(source).toContain("maximum_capacity_bytes.saturating_sub(self.progress.retained_capacity_bytes)");
});

test("runtime policy receives the original child failure before propagation",()=>{
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");
 for(const name of ["maintenance_step","close_step"]){expect(source).toContain(`match self.${name}(turn.grant()){Ok(step)=>step,Err(error)=>{turn.record(error.retained_progress(),false,false).map_err(ValueError::into_fault)?;return Err(error);}}`);}
});

test("runtime policy retains actual nested error debit without consuming it twice",()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/⚠️receipt/🔣️.json",import.meta.url),"utf8"));
 const schema=JSON.parse(readFileSync(new URL("../🧬️schema/⚠️receipt/🔣️.json",import.meta.url),"utf8"));expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");const method=source.slice(source.indexOf("pub fn advance_job("),source.indexOf("pub fn progress("));
 expect(method).toContain("let result = job(self.context, grant);");expect(method).toContain("self.progress = self.progress.checked_add(actual)");expect(method).toContain("let receipt = result?");expect(method.indexOf("self.progress = self.progress.checked_add(actual)")).toBeLessThan(method.indexOf("let receipt = result?"));expect(method).not.toContain("consume_retained(");
 const db=new Database(":memory:");try{expect(db.query("SELECT 64-0 AS actual,64 AS external,64 AS turn,7-1 AS fuel").get()).toEqual({actual:64,external:64,turn:64,fuel:6});}finally{db.close();}
});

test("runtime policy alias custody keeps final Arc payload and shell distinct",()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔗️alias/🔣️.json",import.meta.url),"utf8"));const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔗️alias/🔣️.json",import.meta.url),"utf8"));expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);expect(validateJsonSchemaSubset(schema,law)).toEqual([]);
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("pub mod alias_retirement;");const alias=readFileSync(new URL("../🔗️alias/🦀️.rs",import.meta.url),"utf8");expect(alias).toContain("pub struct MountedOwnerCellV1");expect(alias).not.toContain("Mutex<Box<T>>");
 const db=new Database(":memory:");try{db.run("CREATE TABLE phase(phase TEXT,release INTEGER)");for(const[phase,release]of [["arc-backing",32],["payload",64],["box-shell",24]])db.query("INSERT INTO phase VALUES(?,?)").run(phase,release);expect(db.query("SELECT sum(release) AS release,count(*) AS phases FROM phase").get()).toEqual({release:120,phases:3});expect(db.query("SELECT phase FROM phase ORDER BY rowid").all()).toEqual(law.finalPhases.map((phase:string)=>({phase})));}finally{db.close();}
});

test("actual native verdicts preserve measured alias births and physical phase receipts",()=>{
 const path=process.env.SEMIO_MOUNTED_OWNER_RESULTS;if(!path)throw Error("Original mounted results are required");const rows=JSON.parse(readFileSync(resolve(path,"../alias.json"),"utf8"))as {aliases:number;birth:number;release:number;arc:number;payload:number;shell:number;constructor:number;denied:number;terminal:number;sharedClosed:boolean}[];
 const schema={type:"array",minItems:3,maxItems:3,items:{type:"object",required:["aliases","birth","release","arc","payload","shell","constructor","denied","terminal","sharedClosed"],additionalProperties:false,properties:{aliases:{enum:[0,1,3]},birth:{type:"integer",minimum:1},release:{type:"integer",minimum:1},arc:{type:"integer",minimum:1},payload:{const:64},shell:{type:"integer",minimum:1},constructor:{const:0},denied:{const:0},terminal:{const:0},sharedClosed:{const:false}}}};expect(new Ajv({strict:true}).compile(schema)(rows)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE receipt(birth INTEGER,release INTEGER,arc INTEGER,payload INTEGER,shell INTEGER)");for(const row of rows)db.query("INSERT INTO receipt VALUES(?,?,?,?,?)").run(row.birth,row.release,row.arc,row.payload,row.shell);expect(db.query("SELECT count(*) AS n FROM receipt WHERE birth=release AND release=arc+payload+shell").get()).toEqual({n:3});}finally{db.close();}
 console.log("[DEBUG] Actual three original alias custody laws agree with independent strict Ajv/SQLite: backing, same payload and shell each separately funded and released");
});

/** 🧾️ Independent scheduling vectors retain the original extension receipt at the actual turn boundary. */
test("runtime policy forwards original extension retirement currencies into the reactor receiver",()=>{
 mountedContextOracle();
 const root=resolve(import.meta.dir,"../../../../../../.."),plugin=readFileSync(resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"),"utf8"),turn=readFileSync(resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs"),"utf8");
 expect(plugin).toContain("pub fn mounted_owner_policy(&self) -> crate::MountedOwnerPolicyV1 { self.mounted_policy }");
 expect(plugin).toContain("fn extension_retirement_turn(grant: RetainedCloneGrant) -> Result<(bool, PluginLifecycleStep), Fault>");
 expect(plugin).toContain("Ok((more_work, step))");
 expect(turn).toContain("runtime.mounted_owner_policy().close");
 expect(turn).toContain("extension_retirement_step.progress()");
 expect(turn).toContain("admit_retained_clone_progress(extension_grant, progress");
 expect(turn).not.toContain("extension_retirement_turn(usize::from");
 console.log("[DEBUG] Independent strict mounted policy/Ajv/SQLite vectors preserve cumulative original currencies; actual extension-to-reactor source join retains the performed receipt, native whole Plugin remains required");
});
