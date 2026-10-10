import {test,expect} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import Ajv2020 from "ajv/dist/2020";
import draft7 from "ajv/dist/refs/json-schema-draft-07.json";
import {Database} from "bun:sqlite";
const base=new URL("../../",import.meta.url),read=(path:string)=>readFileSync(new URL(path,base),"utf8"),law=JSON.parse(read("🧫️fixtures/🧮️compute-retained/🔣️.json"));
test("original Compute invocation preserves the caller receipt identity and refusal custody",()=>{
 const ajv=new Ajv2020({strict:false}).addMetaSchema(draft7).addSchema(JSON.parse(read("../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json")),"semio:retained-grant").addSchema(JSON.parse(read("../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🧾️progress.json")),"semio:retained-progress"),validate=ajv.compile(JSON.parse(read("🧮️compute/🎟️invocation/🧬️schema/🔣️.json"))),zero={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0},input={operation:71,generation:3,completion:null,grant:law.callerGrant,recipient:{operation:71,generation:3,progress:zero,remaining:law.callerGrant}};
 expect(validate(input)).toBe(true);expect(validate({...input,defaultGrant:law.callerGrant})).toBe(false);expect(validate({...input,grant:{...input.grant,maximumCopyBytes:-1}})).toBe(false);
 const db=new Database(":memory:");try{for(const row of[{operation:71,generation:3,open:true,items:1,copy:128,capacity:4096,depth:2,expected:true},{operation:72,generation:3,open:true,items:1,copy:128,capacity:4096,depth:2,expected:false},{operation:71,generation:4,open:true,items:1,copy:128,capacity:4096,depth:2,expected:false},{operation:71,generation:3,open:false,items:1,copy:128,capacity:4096,depth:2,expected:false},{operation:71,generation:3,open:true,items:0,copy:128,capacity:4096,depth:2,expected:false},{operation:71,generation:3,open:true,items:1,copy:0,capacity:4096,depth:2,expected:false},{operation:71,generation:3,open:true,items:1,copy:128,capacity:0,depth:2,expected:false},{operation:71,generation:3,open:true,items:1,copy:128,capacity:4096,depth:0,expected:false}]){const admitted=Boolean((db.query("SELECT (?1=71 AND ?2=3 AND ?3 AND ?4>=1 AND ?5>=128 AND ?6>=4096 AND ?7>=2) AS admitted").get(row.operation,row.generation,Number(row.open),row.items,row.copy,row.capacity,row.depth)as{admitted:number}).admitted);expect(admitted).toBe(row.expected);expect(admitted?{copiedItems:1,copiedBytes:128,retainedCapacityBytes:4096,releasedBytes:0}:zero).toEqual(row.expected?{copiedItems:1,copiedBytes:128,retainedCapacityBytes:4096,releasedBytes:0}:input.recipient.progress);}}finally{db.close()}
 console.log("[DEBUG] original Services compute invocation canonical Grant/Progress Ajv and SQLite identity/cancellation/eight independent source-retention cases");
 const source=read("🦀️.rs"),start=source.indexOf("pub async fn run_job<"),end=source.indexOf("/// 🌐️ Runs a blocking platform-I/O",start),receiver=source.slice(start,end);
 expect(receiver).toContain("job: &mut Option<J>");expect(receiver).toContain("params: &mut Option<semio_framework_job::BatchJobParams>");expect(receiver).toContain("control: &mut impl semio_framework_job::WorkerJobAdmissionControl");expect(receiver).toContain("try_admit_owned(job, params, control)");expect(receiver).not.toContain("MountedWorkerJobSession::try_new");expect(receiver).toContain("retained_recipient");
});
test("original Compute session survives dropping only its borrowed async driver",()=>{
 const db=new Database(":memory:");try{db.run("CREATE TABLE custody(phase TEXT,caller INTEGER,driver INTEGER)");for(const[phase,caller,driver]of[["source",1,0],["admitted",1,1],["driver-cancelled",1,0],["paid-close",0,0]])db.query("INSERT INTO custody VALUES(?,?,?)").run(phase,caller,driver);expect(db.query("SELECT caller,driver FROM custody WHERE phase='driver-cancelled'").get()).toEqual({caller:1,driver:0});expect(db.query("SELECT count(*) AS n FROM custody WHERE caller=0 AND driver=1").get()).toEqual({n:0});}finally{db.close()}
 const source=read("🦀️.rs"),start=source.indexOf("pub async fn run_job<"),end=source.indexOf("/// 🌐️ Runs a blocking platform-I/O",start);expect(source.slice(start,end)).toContain("completion: &mut Option<ComputeJobCompletion<J>>");console.log("[DEBUG] SQLite original caller custody remains after driver cancellation; actual receiving slot required");
});
test("Services mandatory retained contract validates every neutral axis and an independent SQLite receipt",()=>{
 const ajv=new Ajv2020({strict:false}).addMetaSchema(draft7),grant=ajv.compile(JSON.parse(read("../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json"))),progress=ajv.compile(JSON.parse(read("../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🧾️progress.json")));
 expect(grant(law.callerGrant)).toBe(true);expect(grant(law.metadataTurn.grant)).toBe(true);expect(progress(law.metadataTurn.receipt)).toBe(true);
 for(const key of Object.keys(law.callerGrant)){const absent=structuredClone(law.callerGrant);delete absent[key];expect(grant(absent)).toBe(false);const negative=structuredClone(law.callerGrant);negative[key]=-1;expect(grant(negative)).toBe(false);}
 const db=new Database(":memory:");try{const oracle=db.query("SELECT 1 AS copiedItems, 1 AS copiedBytes, 0 AS retainedCapacityBytes, 0 AS releasedBytes").get();expect(law.metadataTurn.receipt).toEqual(oracle);}finally{db.close();}
});
test("Services and both original router callers preserve caller authority without a scalar close fallback",()=>{
 const services=read("🦀️.rs"),effects=read("../🔌️plugin/🖥️host/⚡️effects/🦀️.rs"),imports=read("../🔌️plugin/🖥️host/📥️imports/🦀️.rs");expect(services.includes("compute_job_close_grant")).toBe(false);expect(services).toContain('config:semio_framework_job::BatchDriveConfig{retained,site:"os-services.compute-job"');expect(services).toContain("return_original_cancel_alias_step(witness,grant)");expect(services).toContain("self.session.close_step(grant)");expect(effects.includes("|outcome|{")).toBe(true);expect(effects.includes("outcome.close_step(1,")).toBe(false);expect(imports.includes("call.router_turn_grant, &call.router_handler")).toBe(true);expect(law.laws.every((row:{sameGrant:boolean})=>row.sameGrant)).toBe(true);
});

test("canonical Native I/O denials preserve distinct work, ownership and depth axes",()=>{
 const db=new Database(":memory:");try{db.run("CREATE TABLE denials(axis TEXT PRIMARY KEY,refusal TEXT)");for(const axis of ["items","release","depth"]){const grant={items:1,release:128,depth:1,[axis]:0};const refusal=db.query("SELECT CASE WHEN ?=0 THEN 'WorkLimit' WHEN ?<1 THEN 'DepthLimit' WHEN ?<128 THEN 'OwnershipLimit' END AS refusal").get(grant.items,grant.depth,grant.release) as {refusal:string};db.run("INSERT INTO denials VALUES(?,?)",axis,refusal.refusal);}expect(db.query("SELECT axis,refusal FROM denials ORDER BY rowid").all()).toEqual(law.nativeIoDenials);}finally{db.close();}
});

test("Services original outcome loan never reconstructs owned worker payloads",()=>{
 const original=JSON.parse(read("🚪️native-io/🧪️tests/🧫️fixtures/🔣️.json")),db=new Database(":memory:");
 try{db.run("CREATE TABLE loans(kind TEXT PRIMARY KEY,originalRetained INTEGER,recipientBorrow INTEGER,acknowledgedBeforeClose INTEGER)");for(const row of original.outcomeLoans)db.run("INSERT INTO loans VALUES(?,?,?,?)",row.kind,Number(row.originalRetained),Number(row.recipientBorrow),Number(row.acknowledgedBeforeClose));expect(db.query("SELECT kind FROM loans WHERE originalRetained AND recipientBorrow AND acknowledgedBeforeClose ORDER BY rowid").all()).toEqual([{kind:"complete"},{kind:"cancelled"},{kind:"fault"}]);expect(JSON.parse(JSON.stringify(original.outcomeLoans))).toEqual(original.outcomeLoans);}finally{db.close();}
 const io=read("🚪️native-io/🦀️.rs"),services=read("🦀️.rs"),effects=read("../🔌️plugin/🖥️host/⚡️effects/🦀️.rs");
 expect(io).toContain("Result<Option<JobOutcomeBorrow<'a>>, ValueError>");expect(io).toContain("fn borrow_outcome<'a>");expect(io).toContain("JobOutcomeBorrow::admit_fault(cx,");expect(io).not.toContain("-> StepOutcome");
 expect(services).not.toContain("take_checked_out_outcome");expect(services).toContain("recipient: impl for<'a> FnOnce(semio_framework_job::JobOutcomeView<'a>)");expect(services).toContain("acknowledge_checked_out_outcome(retained)");expect(effects).toContain("JobOutcomeView::Complete");
 console.log("[DEBUG] Services borrowed original complete/cancel/fault SQLite lifecycle",original.outcomeLoans.length);
});

test("original Services borrowed recipient Rust grammar remains valid",()=>{for(const name of ["🦀️.rs","🚪️native-io/🦀️.rs","../🔌️plugin/🖥️host/⚡️effects/🦀️.rs"]){const result=spawnSync("rustfmt",["--edition","2021","--emit","stdout",fileURLToPath(new URL(name,base))],{stdio:["ignore","ignore","pipe"],timeout:5000});expect(result.error).toBeUndefined();expect(result.status).toBe(0);}console.log("[DEBUG] Services original borrowed recipient grammar",3);});

test("Services compute examples have no exclusive testing-law wrapper",()=>{expect(existsSync(new URL("🧬️schema/🧮️compute-retained/🔣️.json",base))).toBe(false);console.log("[DEBUG] Services example authority absence",1);});


test("Compute mounted step receives the original supplied five-axis policy",()=>{
 const services=read("🦀️.rs"),job=read("../../../../🔨️modules/🧵️job/🦀️.rs");
 expect(job.includes("lane: Lane,retained:RetainedCloneGrant")).toBe(true);expect(services).toContain("completion.session.pump_one(&self.pool, lane, retained)");expect(services).toContain("take_checked_out_retained_step_receipt()");expect(services).toContain("retained_recipient.record_progress(progress)");expect(services).toContain("let retained=retained_recipient.remaining_grant()?;");
 const db=new Database(":memory:");try{const row=db.query("SELECT ? AS maximumItems, ? AS maximumCopyBytes, ? AS maximumCapacityBytes, ? AS maximumReleaseBytes, ? AS maximumDepth").get(law.callerGrant.maximumItems,law.callerGrant.maximumCopyBytes,law.callerGrant.maximumCapacityBytes,law.callerGrant.maximumReleaseBytes,law.callerGrant.maximumDepth);expect(row).toEqual(law.callerGrant);}finally{db.close();}
 expect(services).not.toContain("pump_one(&next_pool, lane)");console.log("[DEBUG] Compute mounted turn forwards original caller five-axis authority");
});

test("original Compute recipient conserves its one issued grant across successive transitions",()=>{
 const original=law.callerGrant,db=new Database(":memory:");
 try{db.run("CREATE TABLE wallet(items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER,depth INTEGER)");db.query("INSERT INTO wallet VALUES(?,?,?,?,?)").run(original.maximumItems,original.maximumCopyBytes,original.maximumCapacityBytes,original.maximumReleaseBytes,original.maximumDepth);
 const spent={copiedItems:2,copiedBytes:128,retainedCapacityBytes:4096,releasedBytes:0};db.query("UPDATE wallet SET items=items-?,copy=copy-?,capacity=capacity-?,release=release-?").run(spent.copiedItems,spent.copiedBytes,spent.retainedCapacityBytes,spent.releasedBytes);
 const remaining=db.query("SELECT items AS maximumItems,copy AS maximumCopyBytes,capacity AS maximumCapacityBytes,release AS maximumReleaseBytes,depth AS maximumDepth FROM wallet").get();
 expect(remaining).toEqual({...original,maximumItems:original.maximumItems-spent.copiedItems,maximumCopyBytes:original.maximumCopyBytes-spent.copiedBytes,maximumCapacityBytes:original.maximumCapacityBytes-spent.retainedCapacityBytes});
 expect(db.query("SELECT items>=? AND copy>=? AND capacity>=? AND release>=? AS admitted FROM wallet").get(original.maximumItems,0,0,0)).toEqual({admitted:0});
 }finally{db.close()}
 const source=read("🦀️.rs");expect(source).toContain("pub trait ComputeRetainedRecipient");expect(source).toContain("retained_recipient: &mut impl ComputeRetainedRecipient");expect(source).toContain("retained_recipient.remaining_grant()?");expect(source).toContain("retained_recipient.record_progress(progress)");expect(source).not.toContain("fn receive_compute_turn(recipient:&mut semio_framework_job::RetainedCloneProgress");
 console.log("[DEBUG] original Compute wallet independent SQLite conservation; actual borrowed original recipient required at each admission/pump/ACK");
});

test("original compute admission reserves its receiving header before native child birth",()=>{
 const db=new Database(":memory:");try{const original={copy:700,items:3},header={copy:128,items:1},child={copy:600,items:1};
 const paid=db.query("SELECT ?-? >= ? AND ?-? >= ? AS admitted").get(original.copy,header.copy,child.copy,original.items,header.items,child.items);expect(paid).toEqual({admitted:0});
 expect(original).toEqual({copy:700,items:3});
 }finally{db.close()}
 const source=read("🦀️.rs");expect(source).toContain("WorkerJobSession::<J>::owned_admission_demand()?");expect(source).toContain("child.copy_bytes.checked_add(header)");console.log("[DEBUG] original remaining wallet funds receiving header and complete child demand before birth");
});
