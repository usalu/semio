import { blake3 } from "@noble/hashes/blake3.js";
import { registeredToolExecutableV1 } from "../../../../../🧰️tools/🏃️run/🟦️.ts";
import { registerIsolatedBuildTestV1 } from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import { expect } from "bun:test";
import { spawnSync } from "node:child_process";
import { Database } from "bun:sqlite";
import { mkdirSync, mkdtempSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { NATIVE_BUILD_DRIVER_POLICY_V1 } from "../../🟦️.ts";
import { installRestartableFailureHandlerV1 } from "../../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import { artifacts, header, stubWorkspace, runStub } from "../🎭️stub/🟦️.ts";

/** 🧩️ Proves that a failed pass only observes the complete closure of the components it did link. */
for (const scenario of fixture.partialProvenance.rows) {
  const refusal = scenario.exit === 0 ? undefined : scenario.missing !== null
    ? /Native build failed: stdio .*\(provenance\)/u : scenario.accepted
      ? /^Native build failed: stdio \(release-app\);/u : /Compiler consumed bytes have no matching retained source witness: .*common[\\/]lib\.rs/u;
  registerIsolatedBuildTestV1(import.meta.path, "closure-scoped partial Cargo provenance: " + scenario.name,
    {bodyMilliseconds:120_000,scope:{kind:"body"}}, async checkpoint => {
    if (refusal) installRestartableFailureHandlerV1(() => undefined);
    const vector=fixture.partialProvenance, root=mkdtempSync(join(artifacts,"p-"));
    const pass=stubWorkspace(root,"app",vector.components,"release"), workspace=dirname(pass.manifest);
    const path=(name:string,file:string)=>join(workspace,name,file);
    for(const dependency of vector.dependencies){
      mkdirSync(join(workspace,dependency));
      writeFileSync(path(dependency,"Cargo.toml"),`[package]\nname="${dependency}"\nversion="0.1.0"\nedition="2021"\n[lib]\npath="lib.rs"\n`);
      writeFileSync(path(dependency,"lib.rs"),vector.edited);
      writeFileSync(path(dependency,"build.rs"),"fn main() {}\n");
    }
    const work=readFileSync(pass.manifest,"utf8");
    writeFileSync(pass.manifest,work.replace(JSON.stringify(vector.components),JSON.stringify([...vector.components,...vector.dependencies])));
    for(const [index,row] of pass.packages.entries()){
      const deps=vector.edges.filter(([source])=>source===index).map(([,dep])=>vector.nodes[dep!]![0]);
      writeFileSync(row.manifest,readFileSync(row.manifest,"utf8")+"\n[dependencies]\n"+deps.map(name=>`${name} = { path = "../${name}" }\n`).join(""));
    }
    expect(spawnSync(registeredToolExecutableV1("cargo"),["generate-lockfile","--offline","--manifest-path",pass.manifest],{encoding:"utf8"}).status).toBe(0);
    const command=join(root,"partial.ts"),calls=join(root,"calls"), config={workspace,vector,scenario,calls,target:pass.target,header:[...header],staleHash:Buffer.from(blake3(Buffer.from(vector.original))).toString("hex")};
    writeFileSync(command,String.raw`import {appendFileSync,mkdirSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {pathToFileURL} from "node:url";
import {spawnSync} from "node:child_process";
const args=process.argv.slice(2);
if (["locate-project","update","metadata"].includes(args[0])) {
  const cargo=spawnSync(${JSON.stringify(registeredToolExecutableV1("cargo"))},args,{stdio:"inherit"});
  process.exit(cargo.status??1);
}
const {workspace,vector,scenario,target,calls,header,staleHash}=${JSON.stringify(config)};
const path=(name,file)=>join(workspace,name,file);
const units=vector.nodes.map(([packageName,kind],index)=>{
  const script=kind==="run-custom-build"||kind==="custom-build";
  return {pkg_id:"path+"+pathToFileURL(join(workspace,packageName)).href+"#"+packageName+"@0.1.0",
    target:{name:script?"build-script-build":packageName,kind:[script?"custom-build":kind],crate_types:[script?"bin":kind],src_path:path(packageName,script?"build.rs":"lib.rs")},
    platform:kind==="custom-build"?null:target,mode:kind==="run-custom-build"?kind:"build",features:[],
    dependencies:vector.edges.filter(([from])=>from===index).map(([,child])=>({index:child})),
  };
});
if(process.argv.includes("--unit-graph")){console.log(JSON.stringify({version:1,units,roots:vector.roots}));process.exit(0);}
appendFileSync(calls,"build\n");
const emit=(message)=>console.log(JSON.stringify(message));
for(const [index,unit] of units.entries()){
  if(scenario.missing===index||vector.roots.includes(index)&&!scenario.linked.includes(index))continue;
  const name=vector.nodes[index][0],pkg=unit.pkg_id;
  if(unit.mode==="run-custom-build"){
    const out=join(process.env.CARGO_BUILD_BUILD_DIR,target,name,"out");
    mkdirSync(out,{recursive:true});
    writeFileSync(join(out,"semio-runtime-resource-inputs.jsonl"),index===scenario.runStale?"unparseable unrelated journal\n":JSON.stringify({kind:"read",path:path(name,"build.rs")})+"\n");
    emit({reason:"build-script-executed",package_id:pkg,out_dir:out,linked_libs:[],linked_paths:[],cfgs:[],env:[]});
    continue;
  }
  const out=join(process.env.CARGO_TARGET_DIR,...(unit.platform?[unit.platform]:[]),"release"),kind=unit.target.kind[0],source=unit.target.src_path;
  mkdirSync(out,{recursive:true});
  const stem=kind==="custom-build"?name+"-build":name,file=join(out,stem+(kind==="cdylib"?".wasm":".rlib"));
  writeFileSync(file,Buffer.concat([Buffer.from(header),Buffer.from(name)]));
  writeFileSync(join(out,stem+".d"),file+": "+source.replaceAll(" ","\\ ")+"\n"+
   (scenario.stale===index?"# checksum:blake3="+staleHash+" file_len:"+Buffer.byteLength(vector.original)+" "+source+"\n":""));
  emit({reason:"compiler-artifact",package_id:pkg,manifest_path:path(name,"Cargo.toml"),target:unit.target,
    features:unit.features,profile:{test:false},filenames:[file],executable:null,fresh:false});
}
if(scenario.exit!==0){
  const failed=units[2];
  emit({reason:"compiler-message",package_id:failed.pkg_id,manifest_path:path("stdio","Cargo.toml"),target:failed.target,
    message:{level:"error",code:{code:"E0599"},message:vector.cargoError,rendered:vector.cargoError,spans:[]}});
}
process.exit(scenario.exit);`);
    const db=new Database(":memory:");
    const expected=new Map<string,number>();
    try{
      db.exec("create table edges(src integer,dst integer); create table nodes(id integer primary key)");
      for(const [src,dst] of vector.edges)db.query("insert into edges values(?,?)").run(src,dst);
      for(const i of vector.nodes.keys())db.query("insert into nodes values(?)").run(i);
      for(const node of scenario.linked){
        const {count}=db.query<{count:number},[number]>("with recursive closure(id) as (select ? union select dst from edges join closure on src=id) select count(*) as count from nodes join closure using(id)").get(node)!;
        expected.set(vector.nodes[node]![0]!,count);
      }
    }finally{db.close();}
    const seen=new Map<string,number>();
    const logs:string[]=[];
    let finish!:()=>void;
    const passFinished=new Promise<void>(resolve=>{finish=resolve;});
    const run=runStub(root,{version:1,passes:[pass]},{command:process.execPath,args:[command],calls},{
      storageBytes:vector.storageBytes,
      log:line=>{logs.push(line);if(line.startsWith("[native-build] "+pass.id)&&line.includes("component(s) linked"))finish();},
      stages:linked=>[{name:"proof",capacity:"io",after:[],async run(context){
        await passFinished;
        const provenance=await context.provenance();
        seen.set(linked.package.name,provenance.units);
      }}],
    },"observe-partial",({outcome,held})=>{
      expect(readFileSync(calls,"utf8").trim().split("\n")).toHaveLength(1);
      const runs=join(held.root,"cache",NATIVE_BUILD_DRIVER_POLICY_V1.warmRoot);
      const report=outcome?.generation.report??join(runs,readdirSync(runs)[0]!,NATIVE_BUILD_DRIVER_POLICY_V1.stores.report);
      const provenance=join(report,NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory);
      expect(readdirSync(provenance).some(name=>name.startsWith("cargo-unit-provenance-"))).toBe(scenario.exit===0);
      if(scenario.exit!==0)expect(logs.join("\n")).toContain(vector.cargoError);
    });
    if(scenario.exit===0)expect((await run).staged).toHaveLength(scenario.linked.length);
    else await expect(run).rejects.toThrow(refusal);
    expect(seen).toEqual(scenario.accepted?expected:new Map());
    checkpoint();
  }, refusal);
}
