import Ajv from "ajv";
import{Database}from"bun:sqlite";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️.json";
/** 🪦️ Contract: a holder killed between its body and its own retention (SIGKILL, out of memory, watchdog) leaves its receipts in the lane, and the pool trimmer that removes that idle lane publishes them into the shared ledger the holder recorded first, so a finished build never loses its provenance to the trimmer. Unresolved destinations or conflicts retain the lane until its custody can be settled. The oracle is the shared ledger on disk after a real process was killed. */
import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,truncateSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {UNIT_CACHE_POLICY_V1} from "../../../../🔏️key/🟦️.ts";
import {acquirePooledLaneV1,readPoolV1,trimPoolV1} from "../../../🏊️pool/🟦️.ts";
import {retainLaneOutputsV1,withLaneRetentionV1} from "../../🟦️.ts";

const rules=UNIT_CACHE_POLICY_V1.retention,layout=UNIT_CACHE_POLICY_V1.layout,here=dirname(fileURLToPath(import.meta.url)),worker=join(here,"📥️worker/🟦️.ts"),R="cargo-unit-provenance-";
const scratch=()=>realpathSync(mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"dead-holder-"))),everything={maximumLanes:0,maximumLiveLanes:32,quotaBytes:0};
async function killedHolder(root:string,shared:string):Promise<string>{
 const child=Bun.spawn([process.execPath,worker,root,shared],{stdout:"pipe",stderr:"inherit"}),id=(await new Response(child.stdout).text()).trim();
 expect(await child.exited,"the holder dies of SIGKILL").not.toBe(0);expect(id).toMatch(/^[a-z0-9]{1,12}$/u);
 const claim=join(root,"u",layout.locks,"lane-"+id+".lock"),past=new Date(Date.now()-UNIT_CACHE_POLICY_V1.lock.staleMilliseconds*3);utimesSync(claim,past,past);
 return id;
}

test("the receipts a killed holder left in its lane are published by the trimmer that removes the lane",async()=>{
 const root=scratch(),cache=join(root,"u"),shared=join(root,"b","p");
 try{
  const id=await killedHolder(root,shared);
  expect(existsSync(join(shared,rules.ledger)),"the holder never reached its retention").toBe(false);
  expect(readPoolV1(cache,Date.now()).map(lane=>[lane.id,lane.claimed])).toEqual([[id,false]]);
  const trimmed=trimPoolV1(cache,everything,Date.now());
  expect(trimmed.removed).toBe(1);expect(readPoolV1(cache,Date.now())).toEqual([]);
  expect(readFileSync(join(shared,rules.ledger,R+"killed.json"),"utf8")).toBe('{"killed":true}');
  expect(readdirSync(join(shared,rules.ledger)).filter(name=>name.startsWith(".")||name.endsWith(".tmp"))).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);

test("a conflicting receipt keeps its original lane and is reported without replacement",async()=>{
 const root=scratch(),cache=join(root,"u"),shared=join(root,"b","p");
 try{
  mkdirSync(join(shared,rules.ledger),{recursive:true});writeFileSync(join(shared,rules.ledger,R+"killed.json"),"first writer");
  const id=await killedHolder(root,shared),reports:[string,Record<string,string|number>][]=[];
  expect(trimPoolV1(cache,everything,Date.now(),(kind,parameters)=>reports.push([kind,parameters])).removed).toBe(0);expect(existsSync(join(cache,layout.lanes,id))).toBe(true);
  expect(readFileSync(join(shared,rules.ledger,R+"killed.json"),"utf8")).toBe("first writer");
  expect(reports).toEqual([["retentionConflict",{lane:id,count:1,names:R+"killed.json"}]]);
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);

test("an unresolved ledger without a destination keeps its lane",async()=>{
 const root=scratch(),cache=join(root,"u"),shared=join(root,"b","p");
 try{
  const lane=await acquirePooledLaneV1(cache,{crates:["x"],quotaBytes:Number.MAX_SAFE_INTEGER});
  mkdirSync(join(lane.paths.build,rules.ledger),{recursive:true});writeFileSync(join(lane.paths.build,rules.ledger,R+"x.json"),"x");lane.release();
  expect(trimPoolV1(cache,everything,Date.now()).removed).toBe(0);expect(existsSync(lane.paths.base)).toBe(true);
  expect(existsSync(shared)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a holder records its shared ledger beside the lane before its body runs, and an invalid destination keeps its lane",async()=>{
 const root=scratch(),cache=join(root,"u"),shared=join(root,"b","p");
 try{
  const lane=await acquirePooledLaneV1(cache,{crates:["x"],quotaBytes:Number.MAX_SAFE_INTEGER});
  await withLaneRetentionV1({lane,shared,language:"en"},async()=>{expect(JSON.parse(readFileSync(join(lane.paths.target,layout.destination),"utf8"))).toEqual({version:1,targetDirectory:lane.paths.target.replaceAll("\\","/").normalize("NFC"),sharedBuild:shared});});
  const again=await acquirePooledLaneV1(cache,{crates:["x"],quotaBytes:Number.MAX_SAFE_INTEGER});
  writeFileSync(join(again.paths.target,layout.destination),"relative/shared");mkdirSync(join(again.paths.build,rules.ledger),{recursive:true});writeFileSync(join(again.paths.build,rules.ledger,R+"y.json"),"y");again.release();
  expect(trimPoolV1(cache,everything,Date.now()).removed).toBe(0);expect(existsSync(again.paths.base)).toBe(true);expect(existsSync(join(root,"relative"))).toBe(false);expect(existsSync(join(shared,rules.ledger,R+"y.json"))).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the retention is a synchronous step under the claim of the trimmer: its result is a value, not a promise",()=>{
 const root=scratch();
 try{
  const lane=join(root,"lane"),ledger=join(lane,rules.ledger);mkdirSync(ledger,{recursive:true});writeFileSync(join(ledger,R+"a.json"),"a");
  const summary=retainLaneOutputsV1({lane,shared:join(root,"shared"),laneId:"ab12",language:"en"});
  expect(typeof (summary as unknown as {then?:unknown}).then).toBe("undefined");expect(summary).toMatchObject({published:1,conflicts:[]});
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("authenticated target evidence and ledgers settle before trimming",async()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("create table verdict(mode text, removed integer)");for(const row of fixture.cases){const root=scratch(),cache=join(root,"u"),shared=join(root,"b","p");try{const lane=await acquirePooledLaneV1(cache,{crates:["x"],quotaBytes:Number.MAX_SAFE_INTEGER});await withLaneRetentionV1({lane,shared,language:"en"},async()=>{});const oldBuild=join(lane.paths.base,"b"),oldTarget=join(lane.paths.base,"t"),current=lane.paths.target;mkdirSync(oldBuild,{recursive:true});mkdirSync(oldTarget,{recursive:true});const bindings=[{version:1,buildDirectory:oldBuild,targetDirectory:oldTarget},{version:1,buildDirectory:lane.paths.build,targetDirectory:current}].map(value=>JSON.stringify(value)+"\n");writeFileSync(join(oldTarget,".cargo-store.json"),row.mode==="malformed"?"broken":bindings[0]!);writeFileSync(join(current,".cargo-store.json"),bindings[1]!);writeFileSync(join(oldTarget,layout.destination),JSON.stringify({version:1,targetDirectory:oldTarget.replaceAll("\\","/").normalize("NFC"),sharedBuild:shared})+"\n");for(const [dir,name] of [[oldBuild,"old"],[lane.paths.build,"new"]]){mkdirSync(join(dir,rules.ledger),{recursive:true});writeFileSync(join(dir,rules.ledger,R+name+".json"),name);}if(row.mode==="conflict"){mkdirSync(join(shared,rules.ledger),{recursive:true});writeFileSync(join(shared,rules.ledger,R+"old.json"),"foreign");}if(row.mode==="missing")rmSync(join(oldTarget,layout.destination));if(row.mode==="blocked"){mkdirSync(shared,{recursive:true});writeFileSync(join(shared,rules.ledger),"blocked");}if(row.mode==="unbound"){const dir=join(lane.paths.base,"unbound",rules.ledger);mkdirSync(dir,{recursive:true});writeFileSync(join(dir,R+"unbound.json"),"unbound");}if(row.mode==="limit")for(let i=0;i<=rules.maximumFiles;i++)mkdirSync(join(lane.paths.base,"d"+i));if(row.mode==="oversized")truncateSync(join(oldBuild,rules.ledger,R+"old.json"),rules.maximumFileBytes+1);const result=trimPoolV1(cache,everything,Date.now());db.query("insert into verdict values(?,?)").run(row.mode,result.removed);expect(result.removed,row.id).toBe(row.removed);if(row.removed){for(const text of bindings){const hash=new Bun.CryptoHasher("sha256").update(text).digest("hex");expect(readFileSync(join(shared,rules.ledger,rules.bindingPrefix+hash.slice(0,32)+".json"),"utf8")).toBe(text);const binding=JSON.parse(text),marker=JSON.stringify({version:1,targetDirectory:binding.targetDirectory.replaceAll("\\","/").normalize("NFC"),sharedBuild:shared})+"\n",markerHash=new Bun.CryptoHasher("sha256").update(marker).digest("hex");expect(readFileSync(join(shared,rules.ledger,rules.destinationPrefix+markerHash.slice(0,32)+".json"),"utf8")).toBe(marker);}expect(readFileSync(join(shared,rules.ledger,R+"old.json"),"utf8")).toBe("old");expect(readFileSync(join(shared,rules.ledger,R+"new.json"),"utf8")).toBe("new");}else{expect(existsSync(lane.paths.base),row.id).toBe(true);expect(readFileSync(join(oldTarget,".cargo-store.json"),"utf8")).toBe(row.mode==="malformed"?"broken":bindings[0]!);expect(readFileSync(join(current,".cargo-store.json"),"utf8")).toBe(bindings[1]!);if(row.mode==="conflict")expect(readFileSync(join(oldBuild,rules.ledger,R+"old.json"),"utf8")).toBe("old");}}finally{rmSync(root,{recursive:true,force:true});}}expect((db.query("select count(*) as n from verdict where removed != (mode = ?)").get("valid")as{n:number}).n).toBe(0);}finally{db.close();}});
