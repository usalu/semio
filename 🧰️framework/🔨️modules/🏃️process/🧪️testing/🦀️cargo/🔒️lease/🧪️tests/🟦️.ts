import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {parseArgs} from "node:util";
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync,existsSync,rmSync} from "node:fs";
import {join,resolve} from "node:path";
import corpus from "./🧫️fixtures/🔣️.json";
import policySchema from "../../🧬️schema/🔣️.json";
import leaseSchema from "../../../../📦️artifacts/🏗️native-build/🔒️lease/🧬️schema/🔣️.json";
import {acquireCargoBuildLeaseV1,acquireCargoProfileMaintenanceLeaseV1,cargoBuildLeaseIdentityV1} from "../../../../📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import * as api from "../../🟦️.ts";

const base=JSON.parse(readFileSync(new URL("../../🧪️tests/🧫️fixtures/🔣️.json",import.meta.url),"utf8")).policies[0];
const leaseStores=new Map<string,string>();
const artifact=()=>{const path=process.env.SEMIO_TEST_ARTIFACT_DIR;expect(path).toBeTruthy();mkdirSync(path!,{recursive:true});const root=mkdtempSync(join(path!,"t-")),store=join(process.cwd(),".cache/b/l/m/data");mkdirSync(store,{recursive:true});leaseStores.set(root,mkdtempSync(join(store,"t-")));return root;};
const retire=(root:string)=>{rmSync(root,{recursive:true,force:true});const leases=leaseStores.get(root);if(leases)rmSync(leases,{recursive:true,force:true});leaseStores.delete(root);};
const policy=(root:string)=>({...base,manifestPath:join(root,"Cargo.toml"),targetDirectory:join(root,"target"),buildDirectory:join(root,"build"),leaseDirectory:leaseStores.get(root)!,configPath:join(root,"nextest.toml"),artifactDirectory:root,level:"long",coverageEnabled:false,coveragePath:null,retainArtifacts:false,buildBudgetMs:corpus.maximumProcessMs});
const request=(root:string,current:any)=>({manifestPath:current.manifestPath,packages:["owner-package"],cwd:root,environment:{...process.env,LEASE_ROOT:root},extraArgs:[]});
const until=async(condition:()=>boolean)=>{const deadline=performance.now()+corpus.readinessMs;while(!condition()){if(performance.now()>deadline)throw Error("Native lease readiness expired");await Bun.sleep(10);}};
const worker="const fs=require('node:fs'),path=require('node:path'),root=process.env.LEASE_ROOT,args=process.argv.slice(1),phase=args[1];fs.appendFileSync(path.join(root,'calls.jsonl'),JSON.stringify(args)+'\\n');fs.writeFileSync(path.join(root,phase+'.started'),'');const timer=setInterval(()=>{if(fs.existsSync(path.join(root,phase+'.release'))){clearInterval(timer);if(phase==='list')process.stdout.write(JSON.stringify({rust_build_meta:{},rust_binaries:{}}));}},10);";
const port={command:"node",args:["-e",worker,"--"]};

test("Cargo test profile lease follows schema and independent Node CLI selection",()=>{
 const root=artifact(),validate=new Ajv({strict:true}).compile(policySchema),identityValidate=new Ajv({strict:true}).compile(leaseSchema);
 try{for(const row of corpus.identities){
  const current={...policy(root),mode:row.mode as "prototype"|"release",nextest:row.nextest,coverageEnabled:row.coverage,coveragePath:row.coverage?join(root,"coverage.lcov"):null},input={...request(root,current),extraArgs:row.args};
  expect(validate(current)).toBe(true);
  const plan=api.cargoTestPlanV1(input,current,join(root,"metadata.json")),option=row.nextest?"cargo-profile":"profile",compiler=plan[0]!.args,at=compiler.indexOf("--"+option),parsed=parseArgs({args:compiler.slice(at,at+2),options:{[option]:{type:"string"}},strict:true});
  expect(parsed.values[option]).toBe(row.profile);
  const identity=(api as any).cargoTestBuildLeaseIdentityV1(input,current);expect(identityValidate(identity)).toBe(true);expect(identity.profile).toBe(row.profile);expect(identity.buildDirectory).toBe(cargoBuildLeaseIdentityV1(current.buildDirectory,[]).buildDirectory);
  console.log(`[DEBUG] Native test compiler lease ${row.id} actualProfile=${identity.profile}`);
 }
 for(const args of corpus.invalid)expect(()=>(api as any).cargoTestBuildLeaseIdentityV1({...request(root,policy(root)),extraArgs:args},policy(root))).toThrow();
 for(const key of ["buildDirectory","leaseDirectory"]){const invalid={...policy(root)};delete invalid[key];expect(validate(invalid)).toBe(false);}
 }finally{retire(root);}
});

test("Cargo tests hold the shared compiler lease through list and assertion termination",async()=>{
 const root=artifact(),current=policy(root),controller=new AbortController();let held:any,run:Promise<void>|undefined;
 try{
  held=await acquireCargoProfileMaintenanceLeaseV1({directory:current.leaseDirectory,buildDirectory:current.buildDirectory,args:["--profile",current.mode],signal:controller.signal});
  run=api.runCargoTestsV1(request(root,current),current,port);await Bun.sleep(corpus.cancelAfterMs);expect(existsSync(join(root,"calls.jsonl"))).toBe(false);held.release();held=undefined;
  const identity=cargoBuildLeaseIdentityV1(current.buildDirectory,["--profile",current.mode]),resource=JSON.stringify([leaseSchema.definitions.Protocol.const.namespace,identity.buildDirectory,identity.profile]),databasePath=join(current.leaseDirectory,createHash("sha256").update(resource).digest("hex")+".sqlite");
  for(const phase of corpus.phases){await until(()=>existsSync(join(root,phase+".started")));const database=new Database(databasePath);try{database.exec("PRAGMA busy_timeout=0");expect(()=>database.exec("BEGIN EXCLUSIVE")).toThrow();}finally{database.close();}writeFileSync(join(root,phase+".release"),"");}
  await run;run=undefined;const database=new Database(databasePath);try{database.exec("BEGIN EXCLUSIVE");database.exec("ROLLBACK");}finally{database.close();}
  const calls=readFileSync(join(root,"calls.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));expect(calls.map(row=>row[1])).toEqual(corpus.phases);expect(calls[0]).toContain("long");expect(calls[1]).toContain("--binaries-metadata");
  console.log("[DEBUG] Native test shared compiler lease list/run SQLite exclusion=true terminalRelease=true exactArgs=true");
 }finally{controller.abort();held?.release();for(const phase of corpus.phases)writeFileSync(join(root,phase+".release"),"");await run?.catch(()=>{});retire(root);}
},30000);

test("Cargo lease cancellation and process timeout release exact ownership",async()=>{
 const root=artifact(),current=policy(root),controller=new AbortController(),held=await acquireCargoProfileMaintenanceLeaseV1({directory:current.leaseDirectory,buildDirectory:current.buildDirectory,args:["--profile",current.mode],signal:new AbortController().signal});
 try{const run=api.runCargoTestsV1({...request(root,current),signal:controller.signal},current,port);const timer=setTimeout(()=>controller.abort(),corpus.cancelAfterMs);try{await expect(run).rejects.toThrow();}finally{clearTimeout(timer);}expect(existsSync(join(root,"calls.jsonl"))).toBe(false);}finally{held.release();}
 try{
  await expect(api.runCargoTestsV1(request(root,current),{...current,buildBudgetMs:corpus.timeoutMs},port)).rejects.toThrow("budget");
  const reacquired=await acquireCargoBuildLeaseV1({directory:current.leaseDirectory,buildDirectory:current.buildDirectory,targetDirectory:current.targetDirectory,args:["--profile",current.mode],signal:new AbortController().signal});reacquired.release();
  console.log("[DEBUG] Native test compiler lease queuedCancel=noSpawn timeout=terminated ownership=reacquired");
 }finally{retire(root);}
},30000);
