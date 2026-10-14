import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {Command} from "commander";
import {spawnSync} from "node:child_process";
import {existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {availableParallelism} from "node:os";
import {readRepositoryCargoLaunchV1,withRepositoryCargoIsolatedStoreV1,REPOSITORY_CARGO_LAUNCH_POLICY_V1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {receiveScriptProcessInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {openNativeImageStorageV1,retireNativeImageStorageV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts";
import {acquireNativeStorageV1,nativeStorageRetirementPortV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";

const ajv=new Ajv({strict:true,allErrors:true}).addSchema(schema,"launch"),leases=new URL("../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts",import.meta.url).href;

test("repository Cargo launch arguments agree with commander, Ajv and an independent SQLite subcommand classifier",()=>{
 expect(ajv.validate("launch#/definitions/Fixture",fixture)).toBe(true);expect(ajv.validate("launch#/definitions/Policy",REPOSITORY_CARGO_LAUNCH_POLICY_V1)).toBe(true);
 const oracle=new Database(":memory:");
 try{
  for(const row of fixture.arguments){
   if(row.request===null){expect(()=>readRepositoryCargoLaunchV1(row.argv,row.environment),row.name).toThrow();continue;}
   const request=readRepositoryCargoLaunchV1(row.argv,row.environment),independent=new Command().exitOverride().option("--store <store>").option("--configuration <configuration>").option("--parallelism <cores>").argument("[cargo...]").parse([...row.argv],{from:"user"}),selected=independent.opts<{store:string;configuration:string;parallelism?:string}>();
   expect(request,row.name).toEqual({...row.request,parallelism:row.request.parallelism==="available"?availableParallelism():row.request.parallelism} as never);expect(ajv.validate("launch#/definitions/Request",request),row.name).toBe(true);
   expect([selected.store,selected.configuration,independent.args],row.name).toEqual([request.store,request.configuration,[...request.cargo]]);expect(selected.parallelism===undefined?availableParallelism():Number(selected.parallelism),row.name).toBe(request.parallelism);
   const kind=oracle.query("SELECT subcommands.key AS kind FROM json_each(json_extract(?, '$.subcommands')) AS subcommands, json_each(subcommands.value) AS names WHERE names.value = ?").get(JSON.stringify(REPOSITORY_CARGO_LAUNCH_POLICY_V1),request.cargo[0]!) as {kind:string};expect(kind.kind,row.name).toBe(request.kind);
  }
  console.log(`[DEBUG] ${fixture.arguments.length} repository Cargo launch argument cases agree with commander, Ajv and SQLite`);
 }finally{oracle.close();}
});

test("isolated Cargo stores are removed on every owner settlement and swept only after their kernel claim is released",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original test artifact directory required");
 const parent=mkdtempSync(join(artifacts,"cl-")),directory=join(parent,"v"),leaseDirectory=join(process.cwd(),".cache/b/l/m/data");
 await receiveScriptProcessInvocation(process.env,async original=>{
  const control=scriptCompilerAdmissionControlV1(original,"repo:cargo:isolated-law",60000),image=await openNativeImageStorageV1({directory,leaseDirectory,filesystemBytes:33554432,maximumPhysicalBytes:34603008,control}),policy={version:1 as const,root:image.root,leaseDirectory,capacityBytes:34603008,maximumWorkspaceBytes:8388608,maximumArtifactBytes:8388608};
  const storage=await acquireNativeStorageV1({policy,control,port:nativeStorageRetirementPortV1(image.port,async()=>{}),owner:"repo:cargo:isolated-law",kind:"compiler-workspace",bytes:8388608}),base=join(image.root,"cargo","prototype"),isolated=join(base,REPOSITORY_CARGO_LAUNCH_POLICY_V1.isolatedDirectory);
  const db=new Database(":memory:");db.exec("CREATE TABLE expected(name TEXT PRIMARY KEY, removed_after_owner INTEGER NOT NULL, removed_by_sweep INTEGER NOT NULL)");
  const present=(root:string)=>{const result=spawnSync("python3",["-c","import os,sys;print(int(os.path.isdir(sys.argv[1])))",root],{encoding:"utf8"});expect(result.status,result.stderr).toBe(0);return result.stdout.trim()==="1";};
  const owner=()=>{const nonce=Array.from(crypto.getRandomValues(new Uint8Array(6)),byte=>byte.toString(16).padStart(2,"0")).join(""),marker=join(parent,nonce+".ready");const child=Bun.spawn([process.execPath,"--eval",`import {acquireResourceLease} from ${JSON.stringify(leases)};import {mkdirSync,writeFileSync} from "node:fs";await acquireResourceLease({directory:${JSON.stringify(isolated)},resource:${JSON.stringify(nonce)},mode:"exclusive",signal:new AbortController().signal,timeoutMs:0});mkdirSync(${JSON.stringify(join(isolated,nonce,"target"))},{recursive:true});writeFileSync(${JSON.stringify(marker)},"ready");setInterval(()=>{},10000);`],{stdout:"ignore",stderr:"pipe"});return{nonce,marker,child,root:join(isolated,nonce)};};
  try{
   for(const row of fixture.lifecycle){
    db.query("INSERT INTO expected VALUES(?,?,?)").run(row.name,Number(row.removedAfterOwner),Number(row.removedBySweep));
    if(row.outcome==="killed"||row.outcome==="live"){
     const peer=owner();while(!existsSync(peer.marker)){if(peer.child.exitCode!==null)throw Error(await new Response(peer.child.stderr).text());await Bun.sleep(10);}
     if(row.outcome==="killed"){peer.child.kill("SIGKILL");await peer.child.exited;}
     expect(present(peer.root),row.name).toBe(!row.removedAfterOwner);
     await withRepositoryCargoIsolatedStoreV1({storage,base,control},async store=>{expect(present(store.root)).toBe(true);});
     const expected=db.query("SELECT removed_by_sweep FROM expected WHERE name=?").get(row.name) as {removed_by_sweep:number};expect(present(peer.root),row.name).toBe(!expected.removed_by_sweep);
     if(row.outcome==="live"){peer.child.kill("SIGKILL");await peer.child.exited;await withRepositoryCargoIsolatedStoreV1({storage,base,control},async()=>{});expect(present(peer.root),row.name).toBe(false);}
     continue;
    }
    let root="";const cancel=new AbortController();
    const settlement=withRepositoryCargoIsolatedStoreV1({storage,base,control},async store=>{root=store.root;writeFileSync(join(store.target,"unit.rmeta"),"unit");expect(present(store.build)&&present(store.temporary)).toBe(true);if(row.outcome==="throw")throw Error("fixture compiler failure");if(row.outcome==="abort"){const stopped=new Promise<never>((_,reject)=>cancel.signal.addEventListener("abort",()=>reject(cancel.signal.reason),{once:true}));cancel.abort(Error("fixture cancellation"));await stopped;}return row.name;});
    if(row.outcome==="return")expect(await settlement).toBe(row.name);else await expect(settlement).rejects.toThrow();
    expect(present(root),row.name).toBe(!row.removedAfterOwner);
   }
   expect(spawnSync("python3",["-c","import os,sys;print(len([n for n in os.listdir(sys.argv[1])]))",isolated],{encoding:"utf8"}).stdout.trim()).toBe("0");
   console.log(`[DEBUG] ${fixture.lifecycle.length} isolated store lifecycles: settled owners removed, killed owner swept, live peer kept until released; Python and SQLite agree`);
  }finally{db.close();await storage.retire();await image.close();await retireNativeImageStorageV1({directory,leaseDirectory,control});rmSync(parent,{recursive:true,force:true});}
 });
},120000);

test("isolated store refuses an invented reservation before any filesystem effect",async()=>{
 const control={signal:new AbortController().signal,remainingMilliseconds:()=>1000,publish:async()=>{},yieldContinuation:async()=>{}},base=join(process.env.SEMIO_TEST_ARTIFACT_DIR??process.cwd(),"invented-isolated");
 await expect(withRepositoryCargoIsolatedStoreV1({storage:{} as never,base,control},async()=>{})).rejects.toThrow();expect(existsSync(base)).toBe(false);expect(readFileSync(new URL("../🔣️policy.json",import.meta.url),"utf8")).toContain("isolatedDirectory");
});
