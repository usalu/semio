import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {parseArgs} from "node:util";
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync,existsSync,rmSync} from "node:fs";
import {join,resolve} from "node:path";
import corpus from "../🧫️fixtures/🔣️.json";
import policySchema from "../../🧬️schema/🔣️.json";
import leaseSchema from "../../../../📦️artifacts/🏗️native-build/🔒️lease/🧬️schema/🔣️.json";
import {acquireCargoBuildLeaseV1,cargoBuildLeaseIdentityV1} from "../../../../📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import * as api from "../../🟦️.ts";

const base=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json",import.meta.url),"utf8")).policies[0];
const artifact=()=>{const path=process.env.SEMIO_TEST_ARTIFACT_DIR;expect(path).toBeTruthy();mkdirSync(path!,{recursive:true});return mkdtempSync(join(path!,"cargo-test-lease-"));};
const policy=(root:string)=>({...base,manifestPath:join(root,"Cargo.toml"),targetDirectory:join(root,"target"),buildDirectory:join(root,"build"),leaseDirectory:join(root,"leases"),configPath:join(root,"nextest.toml"),artifactDirectory:root,level:"long",coverageEnabled:false,coveragePath:null,retainArtifacts:false,buildBudgetMs:corpus.maximumProcessMs});
const request=(root:string,current:any)=>({manifestPath:current.manifestPath,packages:["owner-package"],cwd:root,environment:{...process.env,LEASE_ROOT:root},extraArgs:[]});
const until=async(condition:()=>boolean)=>{const deadline=performance.now()+corpus.readinessMs;while(!condition()){if(performance.now()>deadline)throw Error("Native lease readiness expired");await Bun.sleep(10);}};
const worker="const fs=require('node:fs'),path=require('node:path'),root=process.env.LEASE_ROOT,args=process.argv.slice(1),phase=args[1];fs.appendFileSync(path.join(root,'calls.jsonl'),JSON.stringify(args)+'\\n');fs.writeFileSync(path.join(root,phase+'.started'),'');const timer=setInterval(()=>{if(fs.existsSync(path.join(root,phase+'.release'))){clearInterval(timer);if(phase==='list')process.stdout.write(JSON.stringify({rust_build_meta:{},rust_binaries:{}}));}},10);";
const port={command:"node",args:["-e",worker,"--"]};

test("Cargo test profile lease follows schema and independent Node CLI selection",()=>{
 const root=artifact(),validate=new Ajv({strict:true}).compile(policySchema),identityValidate=new Ajv({strict:true}).compile(leaseSchema);
 try{for(const row of corpus.identities){
  const current={...policy(root),nextest:row.nextest,coverageEnabled:row.coverage,coveragePath:row.coverage?join(root,"coverage.lcov"):null},input={...request(root,current),extraArgs:row.args};
  expect(validate(current)).toBe(true);
  const parsed=parseArgs({args:row.args,options:{release:{type:"boolean"},"cargo-profile":{type:"string"},profile:{type:"string"}},strict:true});
  const authored=row.coverage||parsed.values.release?"release":(row.nextest?parsed.values["cargo-profile"]:parsed.values.profile)??"dev",aliases=leaseSchema.definitions.Protocol.const.profileDirectories as Record<string,string>;
  expect(aliases[authored]??authored).toBe(row.profile);
  const identity=(api as any).cargoTestBuildLeaseIdentityV1(input,current);expect(identityValidate(identity)).toBe(true);expect(identity.profile).toBe(row.profile);expect(identity.buildDirectory).toBe(cargoBuildLeaseIdentityV1(current.buildDirectory,[]).buildDirectory);
  console.log(`[DEBUG] Native test compiler lease ${row.id} actualProfile=${identity.profile}`);
 }
 for(const args of corpus.invalid)expect(()=>(api as any).cargoTestBuildLeaseIdentityV1({...request(root,policy(root)),extraArgs:args},policy(root))).toThrow();
 for(const key of ["buildDirectory","leaseDirectory"]){const invalid={...policy(root)};delete invalid[key];expect(validate(invalid)).toBe(false);}
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("Cargo tests hold the shared compiler lease through list and assertion termination",async()=>{
 const root=artifact(),current=policy(root),controller=new AbortController();let held:any,run:Promise<void>|undefined;
 try{
  held=await acquireCargoBuildLeaseV1({directory:current.leaseDirectory,buildDirectory:current.buildDirectory,args:[],signal:controller.signal});
  run=api.runCargoTestsV1(request(root,current),current,port);await Bun.sleep(corpus.cancelAfterMs);expect(existsSync(join(root,"calls.jsonl"))).toBe(false);held.release();held=undefined;
  const identity=cargoBuildLeaseIdentityV1(current.buildDirectory,[]),resource=JSON.stringify([leaseSchema.definitions.Protocol.const.namespace,identity.buildDirectory,identity.profile]),databasePath=join(current.leaseDirectory,createHash("sha256").update(resource).digest("hex")+".sqlite");
  for(const phase of corpus.phases){await until(()=>existsSync(join(root,phase+".started")));const database=new Database(databasePath);try{database.exec("PRAGMA busy_timeout=0");expect(()=>database.exec("BEGIN IMMEDIATE")).toThrow();}finally{database.close();}writeFileSync(join(root,phase+".release"),"");}
  await run;run=undefined;const database=new Database(databasePath);try{database.exec("BEGIN IMMEDIATE");database.exec("ROLLBACK");}finally{database.close();}
  const calls=readFileSync(join(root,"calls.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));expect(calls.map(row=>row[1])).toEqual(corpus.phases);expect(calls[0]).toContain("long");expect(calls[1]).toContain("--binaries-metadata");
  console.log("[DEBUG] Native test shared compiler lease list/run SQLite exclusion=true terminalRelease=true exactArgs=true");
 }finally{controller.abort();held?.release();for(const phase of corpus.phases)writeFileSync(join(root,phase+".release"),"");await run?.catch(()=>{});rmSync(root,{recursive:true,force:true});}
},30000);

test("Cargo lease cancellation and process timeout release exact ownership",async()=>{
 const root=artifact(),current=policy(root),controller=new AbortController(),held=await acquireCargoBuildLeaseV1({directory:current.leaseDirectory,buildDirectory:current.buildDirectory,args:[],signal:new AbortController().signal});
 try{const run=api.runCargoTestsV1({...request(root,current),signal:controller.signal},current,port);const timer=setTimeout(()=>controller.abort(),corpus.cancelAfterMs);try{await expect(run).rejects.toThrow();}finally{clearTimeout(timer);}expect(existsSync(join(root,"calls.jsonl"))).toBe(false);}finally{held.release();}
 try{
  await expect(api.runCargoTestsV1(request(root,current),{...current,buildBudgetMs:corpus.timeoutMs},port)).rejects.toThrow("budget");
  const reacquired=await acquireCargoBuildLeaseV1({directory:current.leaseDirectory,buildDirectory:current.buildDirectory,args:[],signal:new AbortController().signal});reacquired.release();
  console.log("[DEBUG] Native test compiler lease queuedCancel=noSpawn timeout=terminated ownership=reacquired");
 }finally{rmSync(root,{recursive:true,force:true});}
},30000);
