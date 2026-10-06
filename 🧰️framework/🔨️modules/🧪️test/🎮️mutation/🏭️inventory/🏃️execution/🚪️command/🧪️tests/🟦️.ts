import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {createRequire} from "node:module";
import {mkdir,writeFile} from "node:fs/promises";
import {join,isAbsolute} from "node:path";
import {setImmediate} from "node:timers/promises";
import {runMutationInventoryCargoProducer,runMutationInventoryCargoProducerCommand} from "../🟦️.ts";
import {MutationInventoryProcessWorkspace,readMutationInventoryCapture,mutationInventoryPolicyArguments,readMutationInventoryExecutionPolicy} from "../../🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import laws from "../🧬️laws/🔣️.json" with {type:"json"};
const require=createRequire(import.meta.url),Ajv=require("ajv"),execa=require("execa"),output=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!output||!isAbsolute(output))throw Error("Authored absolute command test storage required");
const policy={version:1 as const,maximumUnits:10000000,maximumOwnedBytes:1048576,maximumCaptureBytes:262144,budgetMs:60000,compilerStorage:{buildDirectory:join(output,"build"),leaseDirectory:join(output,"leases"),targetDirectory:join(output,"target"),captureDirectory:join(output,"captures")}},manifestPath=join(output,"native/Cargo.toml"),request={manifestPath,binary:"inventory-fixture",cwd:process.cwd()},environment={...process.env,CARGO_TARGET_DIR:policy.compilerStorage.targetDirectory,CARGO_BUILD_BUILD_DIR:policy.compilerStorage.buildDirectory,CARGO_BUILD_JOBS:"2"};
const operation=(workspace:MutationInventoryProcessWorkspace)=>({signal:new AbortController().signal,maximumUnits:policy.maximumUnits,maximumOwnedBytes:policy.maximumOwnedBytes,workspace,onProgress:async()=>{},yieldContinuation:()=>setImmediate()});
async function prepare(){await mkdir(join(output!,"native"),{recursive:true});await writeFile(manifestPath,'[workspace]\n[package]\nname = "inventory-fixture"\nversion = "0.1.0"\nedition = "2021"\n[features]\nmutation-inventory = []\n[[bin]]\nname = "inventory-fixture"\npath = "🦀️.rs"\n');await writeFile(join(output!,"native/🦀️.rs"),fixture.nativeSource);const result=await execa("cargo",["generate-lockfile","--offline","--manifest-path",manifestPath],{reject:false,env:environment,timeout:30000});expect(result.code).toBe(0);}
test("closed request and authored flags refuse before process admission",async()=>{
  expect(new Ajv({strict:false}).compile(laws)(fixture)).toBe(true);const valid=new Ajv({strict:false}).compile(schema);expect(valid(request)).toBe(true);
  for(const value of[{...request,manifestPath:"relative"},{...request,unknown:true}]){expect(valid(value)).toBe(false);const w=new MutationInventoryProcessWorkspace();await expect(runMutationInventoryCargoProducer(value,[...fixture.cases[0].argv,...mutationInventoryPolicyArguments(policy)],environment,policy,operation(w))).rejects.toThrow();expect(w.process).toBeUndefined();expect(w.directory).toBeUndefined();}
  const storageWorkspace=new MutationInventoryProcessWorkspace();await expect(runMutationInventoryCargoProducer(request,[...fixture.cases[0].argv,...mutationInventoryPolicyArguments(policy)],{...environment,CARGO_TARGET_DIR:"mismatched"},policy,operation(storageWorkspace))).rejects.toThrow();expect(storageWorkspace.process).toBeUndefined();expect(storageWorkspace.directory).toBeUndefined();
  const w=new MutationInventoryProcessWorkspace();await expect(runMutationInventoryCargoProducer(request,[...fixture.cases[0].argv,...mutationInventoryPolicyArguments({...policy,budgetMs:2})],environment,policy,operation(w))).rejects.toThrow();expect(w.process).toBeUndefined();expect(()=>readMutationInventoryExecutionPolicy({})).toThrow();
});
test("canonical selected Cargo command equals independent Execa and SQLite",async()=>{
  await prepare();const db=new Database(":memory:");db.exec("CREATE TABLE cases(id TEXT PRIMARY KEY,code INTEGER,stdout TEXT,stderr TEXT)");
  for(const row of fixture.cases){
    const argv=[...row.argv,...mutationInventoryPolicyArguments(policy)],expected={code:row.code,stdout:row.code===0?argv.join("\n")+"\n":"",stderr:row.code===0?"":"authored fixture refusal\n"};
    db.query("INSERT INTO cases VALUES(?,?,?,?)").run(row.id,expected.code,expected.stdout,expected.stderr);
    const oracle=db.query("SELECT code,stdout,stderr FROM cases WHERE id=?").get(row.id) as {code:number;stdout:string;stderr:string},independent=await execa("cargo",["run","--quiet","--offline","--locked","--manifest-path",manifestPath,"--bin",request.binary,"--features","mutation-inventory","--",...argv],{env:environment,reject:false,stripEof:false,timeout:60000});
    expect({code:independent.code,stdout:independent.stdout,stderr:independent.stderr}).toEqual(oracle);
    const w=new MutationInventoryProcessWorkspace(),op=operation(w),result=await runMutationInventoryCargoProducer(request,argv,environment,policy,op);
    expect(result.reason).toBe("exit");expect({code:result.code,stdout:await readMutationInventoryCapture(result.stdout,op),stderr:await readMutationInventoryCapture(result.stderr,op)}).toEqual(oracle);
    expect(w.process?.closed).toBe(true);expect(w.lease).toBeUndefined();
  }db.close();
},90000);
test("artifact host entry forwards exact stdout and actual native refusal code",async()=>{
  await prepare();
  const modulePath=new URL("../🟦️.ts",import.meta.url).href;
  for(const row of [fixture.cases[0],fixture.cases[3]]){
    const argv=[...row.argv,...mutationInventoryPolicyArguments(policy)],source="const {runMutationInventoryCargoProducerCommand}=await import("+JSON.stringify(modulePath)+"); await runMutationInventoryCargoProducerCommand("+JSON.stringify(request)+");";
    const entry=join(output!,"native/cli/📜️script.ts");await mkdir(join(output!,"native/cli"),{recursive:true});await writeFile(entry,source);const result=await execa(process.execPath,[entry,...argv],{env:{...environment,SEMIO_MUTATION_INVENTORY_POLICY:JSON.stringify(policy)},reject:false,stripEof:false,timeout:70000});
    expect(result.code).toBe(row.code);expect(result.stdout).toBe(row.code===0?argv.join("\n")+"\n":"");if(row.code)expect(result.stderr).toContain("capture=");
  }
},90000);

