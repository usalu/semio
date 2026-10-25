import{expect,test}from"bun:test";
import {testPythonV1} from "../../../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{createHash}from"node:crypto";
import{chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{uptime,tmpdir}from"node:os";
import{join,resolve}from"node:path";
import corpus from"./🔣️.json";
import schema from"../🧬️schema/🔣️.json";
import publicationSchema from"../../../../📤️publication/🧬️schema/🔣️.json";
import{appendNativeBootLedgerV1,bindNativeBackingIdentityV1,bindNativeIdentitiesV1,decideNativeBootFactV1,healNativeBootV1,loadNativeBootLedgerV1,nativeBootLedgerCommittedV1,nativeBootLedgerPathV1,parseNativeBootEpochV1,planNativeBootRecoveryV1,readNativeBootLedgerV1,recoverNativeBootV1,type NativeBootFactV1,type NativeBootObservationV1,type NativeBootPortV1}from"../🟦️.ts";
import{createNativeBootFsPortV1,type NativeBootEffectsV1}from"../🧯️act/🟦️.ts";
import type{NativeBootTargetV1}from"../📡️observe/🟦️.ts";

const oracle=resolve(import.meta.dir,"🐍️.py"),BOOT="1E4A0CBE-3710-481F-9614-F8F1FCBD61D0",DAY=86400000;
const scratch=()=>{const base=process.env.SEMIO_TEST_ARTIFACT_DIR?resolve(process.env.SEMIO_TEST_ARTIFACT_DIR):tmpdir();mkdirSync(base,{recursive:true});return mkdtempSync(join(base,"br-"));};
const sha=(text:string|Buffer)=>createHash("sha256").update(text).digest("hex");
const effects:NativeBootEffectsV1={retireTicket:async()=>"done",retireSettledClaims:async()=>"done",recoverDirectorySwap:async()=>"done",detachImage:async()=>"done"};
const never=(error:string)=>async()=>{throw Error(error);};

test("the corpus validates against the closed schema and Ajv rejects mutations",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema);const root=schema.$id+"#/definitions/";
 const corpusValid=ajv.getSchema(root+"Corpus")!;expect(corpusValid(corpus),JSON.stringify(corpusValid.errors?.slice(0,2))).toBe(true);
 expect(corpusValid({...corpus,extra:1})).toBe(false);
 const broken=structuredClone(corpus);(broken.plans[0]!.observation.facts[0] as Record<string,unknown>).unexpected=1;expect(corpusValid(broken)).toBe(false);
 const wrong=structuredClone(corpus);wrong.plans[0]!.observation.bootSession="not-a-boot";expect(corpusValid(wrong)).toBe(false);
 for(const row of corpus.plans){expect(ajv.getSchema(root+"Plan")!(row.plan),row.name).toBe(true);expect(ajv.getSchema(root+"Observation")!(row.observation),row.name).toBe(true);}
});

test("the planner reproduces every hand-authored plan of the corpus",()=>{
 for(const row of corpus.plans)expect(planNativeBootRecoveryV1(row.observation),row.name).toEqual(row.plan as never);
 expect(()=>planNativeBootRecoveryV1({...corpus.plans[0]!.observation,bootSession:"x"})).toThrow("Closed boot recovery observation");
});

test("an independent python implementation re-derives every plan, binding, epoch and ledger state",()=>{
 const python=testPythonV1();expect(python).toBeTruthy();
 const dir=scratch();try{const path=join(dir,"corpus.json");writeFileSync(path,JSON.stringify(corpus));
  const run=Bun.spawnSync([python!,"-I",oracle,path],{stdout:"pipe",stderr:"pipe"});
  expect(run.stdout.toString()+run.stderr.toString(),"oracle output").toContain('"ok": true');expect(run.exitCode).toBe(0);
  const mutated=structuredClone(corpus);mutated.plans[2]!.plan.steps=[];writeFileSync(path,JSON.stringify(mutated));
  expect(Bun.spawnSync([python!,"-I",oracle,path],{stdout:"pipe",stderr:"pipe"}).exitCode,"a wrong expectation must fail").toBe(1);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("an in-memory SQLite CASE table agrees with the publication and ticket decisions",()=>{
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE p(name TEXT,writer INT,dir TEXT,intent TEXT,receipt TEXT,unknown INT,isha TEXT,istate TEXT,ish TEXT)");
  const insert=db.prepare("INSERT INTO p VALUES(?,?,?,?,?,?,?,?,?)");
  const rows=corpus.plans.flatMap(row=>row.observation.facts.filter(f=>f.family==="publication-residue").map(f=>({name:row.name,f:f as Record<string,any>})));
  for(const{name,f}of rows)insert.run(name,Number(f.writerFree),f.directory,f.intent,f.receipt,f.unknownEntries,f.intentReceiptSha256,f.image.state,f.image.sha256);
  const reasons=db.query(`SELECT name, CASE
    WHEN writer=0 THEN 'live-writer'
    WHEN dir='absent' THEN 'settled'
    WHEN dir='unsafe' OR unknown>0 OR intent='unsafe' OR receipt='unsafe' THEN 'unknown-shape'
    WHEN intent='absent' AND receipt='absent' THEN 'empty-directory'
    WHEN intent='absent' THEN 'candidate-without-intent'
    WHEN intent='torn' AND receipt='absent' THEN 'torn-intent'
    WHEN intent='torn' THEN 'torn-intent-with-candidate'
    WHEN istate<>'valid' OR isha IS NULL OR ish IS NULL THEN 'receipt-unavailable'
    WHEN ish=isha THEN 'rolled-back'
    WHEN receipt='absent' THEN 'completed-before-crash'
    ELSE 'ambiguous-candidate' END AS reason FROM p`).all() as{name:string;reason:string}[];
  expect(reasons.length).toBe(rows.length);
  reasons.forEach((row,index)=>expect(decideNativeBootFactV1(rows[index]!.f as NativeBootFactV1,corpus.plans[0]!.observation as NativeBootObservationV1).reason,row.name).toBe(row.reason));
  db.exec("CREATE TABLE t(name TEXT,arrived INT,boot INT)");const add=db.prepare("INSERT INTO t VALUES(?,?,?)");
  const tickets=corpus.plans.flatMap(row=>row.observation.facts.filter(f=>f.family==="lease-ticket"&&(f as Record<string,any>).parsed).map(f=>({row,f:f as Record<string,any>})));
  for(const{row,f}of tickets)add.run(row.name,f.arrivedAtMs,row.observation.bootEpochMs);
  const stale=db.query("SELECT CASE WHEN arrived+5000<=boot THEN 'previous-boot' ELSE 'current-boot' END AS reason FROM t").all() as{reason:string}[];
  stale.forEach((row,index)=>expect(decideNativeBootFactV1(tickets[index]!.f as NativeBootFactV1,tickets[index]!.row.observation as NativeBootObservationV1).reason).toBe(row.reason));
 }finally{db.close();}
});

test("an in-memory SQLite CASE table agrees with the leaked image decisions",()=>{
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE l(name TEXT,backing TEXT,location TEXT,idle INT,lease TEXT)");const insert=db.prepare("INSERT INTO l VALUES(?,?,?,?,?)");
  const rows=corpus.plans.flatMap(row=>row.observation.facts.filter(f=>f.family==="leaked-image").map(f=>({name:row.name,f:f as Record<string,any>})));
  for(const{name,f}of rows)insert.run(name,f.backing,f.location,Number(f.idle),f.lease);
  const reasons=db.query(`SELECT name, CASE
    WHEN backing='absent' THEN 'backing-removed'
    WHEN location<>'scratch' THEN 'owned'
    WHEN idle=0 THEN 'live'
    WHEN lease='unavailable' THEN 'unleased'
    WHEN lease='free' THEN 'abandoned'
    ELSE 'live' END AS reason FROM l`).all() as{name:string;reason:string}[];
  expect(reasons.length).toBe(rows.length);expect(rows.length).toBeGreaterThanOrEqual(8);
  reasons.forEach((row,index)=>expect(decideNativeBootFactV1(rows[index]!.f as NativeBootFactV1,corpus.plans[0]!.observation as NativeBootObservationV1).reason,row.name).toBe(row.reason));
 }finally{db.close();}
});

test("identity and backing bindings follow the corpus",()=>{
 for(const row of corpus.identities)expect(bindNativeIdentitiesV1(row.recorded,row.current),row.name).toBe(row.expected as never);
 for(const row of corpus.backings)expect(bindNativeBackingIdentityV1(row.recorded,row.current),row.name).toBe(row.expected as never);
});

test("boot epoch parsing follows the corpus and the running kernel",()=>{
 for(const row of corpus.epochs)expect(parseNativeBootEpochV1(row.platform as never,row.text),row.name).toBe(row.expectedMs);
 const text=process.platform==="darwin"?Bun.spawnSync(["/usr/sbin/sysctl","-n","kern.boottime"]).stdout.toString():process.platform==="linux"?readFileSync("/proc/stat","utf8"):undefined;
 if(text!==undefined){const parsed=parseNativeBootEpochV1(process.platform as never,text)!;expect(Math.abs(parsed-(Date.now()-uptime()*1000))).toBeLessThan(5000);}
});

test("ledger text is classified like the corpus and torn or corrupt ledgers heal on append",()=>{
 for(const row of corpus.ledgers)expect(readNativeBootLedgerV1(row.text),row.name).toEqual(row.expected as never);
 const dir=scratch();try{
  const path=nativeBootLedgerPathV1(join(dir,"boot-recovery"),dir);
  appendNativeBootLedgerV1(path,{v:1,type:"begin",boot:BOOT,at:1,steps:0,plan:sha("a")});appendNativeBootLedgerV1(path,{v:1,type:"commit",boot:BOOT,at:2,done:0,skipped:0,retained:0});
  expect(nativeBootLedgerCommittedV1(path,BOOT)).toBe(true);expect(nativeBootLedgerCommittedV1(path,"7C3B9A52-0F8E-4D21-A6B4-93E1D5C07F28")).toBe(false);
  writeFileSync(path,readFileSync(path,"utf8")+'{"v":1,"type":"be');expect(loadNativeBootLedgerV1(path).state.tornTail).toBe(true);expect(nativeBootLedgerCommittedV1(path,BOOT)).toBe(false);
  appendNativeBootLedgerV1(path,{v:1,type:"begin",boot:BOOT,at:3,steps:1,plan:sha("b")});const healed=loadNativeBootLedgerV1(path);expect(healed.state).toEqual({state:"incomplete",boot:BOOT,corrupt:false,tornTail:false});
  writeFileSync(path,"garbage\n");appendNativeBootLedgerV1(path,{v:1,type:"begin",boot:BOOT,at:4,steps:1,plan:sha("c")});expect(loadNativeBootLedgerV1(path).state.corrupt).toBe(false);
  const filler=(size:number)=>JSON.stringify({v:1,type:"step",boot:BOOT,index:0,action:"remove-file",target:".publication/intent.json",outcome:"done",detail:"x".repeat(size)})+"\n",commit=JSON.stringify({v:1,type:"commit",boot:BOOT,at:9,done:0,skipped:0,retained:0})+"\n";
  let text=commit+commit+commit;while(Buffer.byteLength(text)+Buffer.byteLength(filler(150))<=65500)text+=filler(150);
  const rest=65500-Buffer.byteLength(text)-Buffer.byteLength(filler(0));if(rest>=0)text+=filler(rest);writeFileSync(path,text);expect(readFileSync(path).length).toBeGreaterThan(65400);expect(readFileSync(path).length).toBeLessThanOrEqual(65536);
  appendNativeBootLedgerV1(path,{v:1,type:"begin",boot:BOOT,at:5,steps:1,plan:sha("d")});
  const compacted=readFileSync(path,"utf8");expect(compacted.length).toBeLessThan(2000);expect(compacted.split("\n").filter(Boolean).map(row=>JSON.parse(row).type)).toEqual(["commit","commit","begin"]);
  expect(()=>appendNativeBootLedgerV1(path,{v:1,type:"step"})).toThrow("Closed boot recovery ledger event");
 }finally{rmSync(dir,{recursive:true,force:true});}
});

function intentOf(receipt:Buffer){return{version:1,purpose:"current-publication",initialState:"mounted",receiptSha256:sha(receipt),directory:{dev:"16777229",ino:"422334440"},backing:{dev:"16777229",ino:"422334443",size:8588886016,allocatedBytes:8588886016},root:{dev:"16777248",ino:"2"}};}
function partition(root:string,stage:"dir"|"intent"|"candidate"|"renamed"|"torn-intent"|"unknown"){
 mkdirSync(root,{recursive:true});const old=Buffer.from(JSON.stringify({version:1,mountDirectory:{dev:"16777229",ino:"422334441",mode:16877},bootSession:"7C3B9A52-0F8E-4D21-A6B4-93E1D5C07F28",dev:16777232,ino:422334443,size:8588886016,ancestry:["2"],filesystemBytes:8588886016,maximumPhysicalBytes:8589934592})+"\n");
 const fresh=Buffer.from(JSON.stringify({version:1,mountDirectory:{dev:"16777229",ino:"422334441",mode:16877},bootSession:BOOT,dev:16777229,ino:422334443,size:8588886016,ancestry:["2"],filesystemBytes:8588886016,maximumPhysicalBytes:8589934592})+"\n");
 const publication=join(root,".publication");mkdirSync(publication,{mode:0o700});chmodSync(publication,0o700);
 const put=(name:string,bytes:Buffer|string)=>{writeFileSync(join(publication,name),bytes,{mode:0o600});chmodSync(join(publication,name),0o600);};
 writeFileSync(join(root,"image.json"),stage==="renamed"?fresh:old,{mode:0o600});chmodSync(join(root,"image.json"),0o600);
 const intent=Buffer.from(JSON.stringify(intentOf(old))+"\n");
 if(stage==="intent"||stage==="candidate"||stage==="renamed"||stage==="unknown")put("intent.json",intent);
 if(stage==="torn-intent")put("intent.json",intent.subarray(0,40));
 if(stage==="candidate")put("receipt.json",fresh);
 if(stage==="unknown")put("stranger.txt","x");
 return{old,fresh};
}
const target=(root:string,writerFree=()=>true):NativeBootTargetV1=>({kind:"publication",scope:"p",root,writerFree});
const clock={bootSession:BOOT,bootEpochMs:Date.now()};

test("the real intent and receipt shapes validate against the production publication schema",()=>{
 const ajv=new Ajv({strict:false});const valid=ajv.compile({...publicationSchema.definitions.Request,definitions:publicationSchema.definitions});
 expect(valid(intentOf(Buffer.from("x"))),JSON.stringify(valid.errors)).toBe(true);
});

test("every crash state of the publication ceremony is settled to the old or the new receipt and a second run is clean",async()=>{
 for(const stage of["dir","intent","torn-intent","candidate","renamed"]as const){
  const dir=scratch();try{
   const root=join(dir,"p"),{old,fresh}=partition(root,stage),ledgerPath=nativeBootLedgerPathV1(join(dir,"boot-recovery"),root),progress:string[]=[];
   const port=createNativeBootFsPortV1({clock,targets:[target(root)],effects});
   const result=await recoverNativeBootV1({port,ledgerPath,now:Date.now,onProgress:event=>progress.push(event.stage)});
   expect(result.outcome,stage).toBe("recovered");expect(existsSync(join(root,".publication")),stage).toBe(false);
   expect(readFileSync(join(root,"image.json")).equals(stage==="renamed"?fresh:old),stage).toBe(true);
   expect(progress[0]).toBe("observe");expect(progress.at(-1)).toBe("commit");expect(nativeBootLedgerCommittedV1(ledgerPath,BOOT)).toBe(true);
   const again=await recoverNativeBootV1({port,ledgerPath,now:Date.now});expect(again.outcome,stage).toBe("clean");expect(again.done).toBe(0);
   expect(await healNativeBootV1({ledgerPath,bootSession:BOOT,residue:()=>existsSync(join(root,".publication")),recover:never("must not run")})).toBe("healthy");
  }finally{rmSync(dir,{recursive:true,force:true});}
 }
});

test("unknown publication entries are retained, reported and never deleted",async()=>{
 const dir=scratch();try{
  const root=join(dir,"p");partition(root,"unknown");const ledgerPath=nativeBootLedgerPathV1(join(dir,"boot-recovery"),root);
  const result=await recoverNativeBootV1({port:createNativeBootFsPortV1({clock,targets:[target(root)],effects}),ledgerPath,now:Date.now});
  expect(result.refused.map(row=>row.reason)).toEqual(["unknown-shape"]);expect(readdirSync(join(root,".publication")).sort()).toEqual(["intent.json","stranger.txt"]);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a live writer keeps its publication metadata untouched",async()=>{
 const dir=scratch();try{
  const root=join(dir,"p");partition(root,"candidate");
  const result=await recoverNativeBootV1({port:createNativeBootFsPortV1({clock,targets:[target(root,()=>false)],effects}),ledgerPath:nativeBootLedgerPathV1(join(dir,"boot-recovery"),root),now:Date.now});
  expect(result.done).toBe(0);expect(readdirSync(join(root,".publication")).sort()).toEqual(["intent.json","receipt.json"]);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a crash or cancel between any two steps leaves a plan the next run completes",async()=>{
 for(let crashAt=0;crashAt<4;crashAt++){
  const dir=scratch();try{
   const root=join(dir,"p");partition(root,"candidate");const ledgerPath=nativeBootLedgerPathV1(join(dir,"boot-recovery"),root),real=createNativeBootFsPortV1({clock,targets:[target(root)],effects});
   let executed=0;const crashing:NativeBootPortV1={observe:()=>real.observe(),observeScope:scope=>real.observeScope(scope),execute:async step=>{if(executed++===crashAt)throw Error("injected crash");return real.execute(step);}};
   if(crashAt<3){await expect(recoverNativeBootV1({port:crashing,ledgerPath,now:Date.now})).rejects.toThrow("injected crash");expect(loadNativeBootLedgerV1(ledgerPath).state.state).toBe("incomplete");}
   const result=await recoverNativeBootV1({port:real,ledgerPath,now:Date.now});
   expect(existsSync(join(root,".publication")),String(crashAt)).toBe(false);expect(nativeBootLedgerCommittedV1(ledgerPath,BOOT)).toBe(true);expect(result.done+(crashAt<3?crashAt:0)).toBe(3);
  }finally{rmSync(dir,{recursive:true,force:true});}
 }
 const dir=scratch();try{
  const root=join(dir,"p");partition(root,"candidate");const ledgerPath=nativeBootLedgerPathV1(join(dir,"boot-recovery"),root),controller=new AbortController();
  await expect(recoverNativeBootV1({port:createNativeBootFsPortV1({clock,targets:[target(root)],effects}),ledgerPath,now:Date.now,signal:controller.signal,onProgress:event=>{if(event.stage==="step"&&event.done===1)controller.abort(Error("stop"));}})).rejects.toThrow("stop");
  expect(readdirSync(join(root,".publication"))).toEqual(["intent.json"]);expect(loadNativeBootLedgerV1(ledgerPath).state.state).toBe("incomplete");
  await recoverNativeBootV1({port:createNativeBootFsPortV1({clock,targets:[target(root)],effects}),ledgerPath,now:Date.now});expect(existsSync(join(root,".publication"))).toBe(false);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a step whose facts changed since the plan is skipped instead of applied",async()=>{
 const dir=scratch();try{
  const root=join(dir,"p");partition(root,"candidate");const real=createNativeBootFsPortV1({clock,targets:[target(root)],effects});
  const racing:NativeBootPortV1={observe:()=>real.observe(),observeScope:async scope=>{rmSync(join(root,".publication"),{recursive:true,force:true});return real.observeScope(scope);},execute:real.execute};
  const result=await recoverNativeBootV1({port:racing,ledgerPath:nativeBootLedgerPathV1(join(dir,"boot-recovery"),root),now:Date.now});
  expect(result.skipped).toBe(3);expect(result.done).toBe(0);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("stale build server files of a dead boot are removed and files of this boot are kept",async()=>{
 const dir=scratch();try{
  const base=join(dir,"bazel","base");mkdirSync(join(base,"server"),{recursive:true});
  for(const name of["server.pid.txt","request_cookie","response_cookie"])writeFileSync(join(base,"server",name),"1\n");
  const old=(Date.now()-2*DAY)/1000;utimesSync(join(base,"server","server.pid.txt"),old,old);utimesSync(join(base,"server","request_cookie"),old,old);
  const paths=["server/server.pid.txt","server/request_cookie","server/response_cookie"],ledgerPath=nativeBootLedgerPathV1(join(dir,"boot-recovery"),base);
  const port=createNativeBootFsPortV1({clock,targets:[{kind:"server",scope:"bazel",root:base,paths}],effects});
  const result=await recoverNativeBootV1({port,ledgerPath,now:Date.now});
  expect(result.done).toBe(2);expect(readdirSync(join(base,"server"))).toEqual(["response_cookie"]);
  await expect(port.execute({index:0,phase:50,family:"tool-server",scope:"bazel",action:"remove-file",target:"../escape",reason:"previous-boot"})).rejects.toThrow("inside its root");
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("dead queue tickets are decoded from production names and delegated one by one",async()=>{
 const dir=scratch();try{
  const lease=join(dir,"c"),stem="a".repeat(64),queue=join(lease,stem+".queue");mkdirSync(queue,{recursive:true});mkdirSync(join(lease,"claims"));
  const b36=(n:number,w:number)=>n.toString(36).padStart(w,"0"),dead=`${b36(Date.now()-3*DAY,10)}-${b36(8903,7)}-Qk3v8XhT2mZpLw0aYcR1dg`,live=`${b36(Date.now()+1000,10)}-${b36(4410,7)}-n1HfT9sLd0QwYb7xKa2VcA`;
  writeFileSync(join(queue,dead),"{}");writeFileSync(join(queue,live),"{}");writeFileSync(join(queue,"README"),"x");
  const retired:string[]=[],claims:string[]=[];
  const probe:NativeBootEffectsV1={retireTicket:async(scope,q,name)=>{retired.push(q+"/"+name);rmSync(join(lease,q,name));return"done";},retireSettledClaims:async scope=>{claims.push(scope);for(const name of readdirSync(join(lease,"claims")))rmSync(join(lease,"claims",name));return"done";},recoverDirectorySwap:async()=>"done",detachImage:async()=>"done"};
  const old=(Date.now()-3*DAY)/1000;writeFileSync(join(lease,"claims","b".repeat(64)+".sqlite"),"x");utimesSync(join(lease,"claims","b".repeat(64)+".sqlite"),old,old);
  const result=await recoverNativeBootV1({port:createNativeBootFsPortV1({clock,targets:[{kind:"lease-catalog",scope:"c",directory:lease}],effects:probe}),ledgerPath:nativeBootLedgerPathV1(join(dir,"boot-recovery"),lease),now:Date.now});
  expect(retired).toEqual([stem+".queue/"+dead]);expect(claims).toEqual(["c"]);expect(result.refused.map(row=>row.reason)).toEqual(["unknown-name"]);
  expect(readdirSync(queue).sort()).toEqual([live,"README"].sort());
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("kernel locks of a killed process vanish at once so no lease database needs recovery",async()=>{
 const dir=scratch();try{
  const path=join(dir,"lease.sqlite"),db=new Database(path);db.exec("CREATE TABLE t(x)");db.close();
  const holder=Bun.spawn([process.execPath,"-e",`const{Database}=require("bun:sqlite");const d=new Database(${JSON.stringify(path)});d.exec("PRAGMA busy_timeout=0;BEGIN EXCLUSIVE");console.log("held");setInterval(()=>{},1000);`],{stdout:"pipe",stderr:"pipe"});
  const reader=holder.stdout.getReader();expect(new TextDecoder().decode((await reader.read()).value)).toContain("held");
  const contender=new Database(path);contender.exec("PRAGMA busy_timeout=0");expect(()=>contender.exec("BEGIN EXCLUSIVE")).toThrow();
  holder.kill("SIGKILL");await holder.exited;
  contender.exec("BEGIN EXCLUSIVE");contender.exec("COMMIT");contender.close();
 }finally{rmSync(dir,{recursive:true,force:true});}
});
