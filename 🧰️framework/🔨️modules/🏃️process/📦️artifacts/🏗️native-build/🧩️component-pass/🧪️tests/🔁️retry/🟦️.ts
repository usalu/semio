import { sha256 } from "@noble/hashes/sha2.js";
import { blake3 } from "@noble/hashes/blake3.js";
import warmGraphFixture from "../../🔁️failure-churn/🧪️tests/🔣️.json" with {type:"json"};
import { registeredToolExecutableV1 } from "../../../../../🧰️tools/🏃️run/🟦️.ts";
import { registerIsolatedBuildTestV1 } from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import { expect } from "bun:test";
import { spawnSync } from "node:child_process";
import { Database } from "bun:sqlite";
import { decodeCargoProvenanceV1 } from "../../../🧾️receipt/🟦️.ts";
import { existsSync, mkdtempSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { nativeBuildComponentStagePathV1, NATIVE_BUILD_DRIVER_POLICY_V1 } from "../../🟦️.ts";
import { ConcurrentEditErrorV1, concurrentEditOfV1, installRestartableFailureHandlerV1 } from "../../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import { artifacts, header, stubWorkspace, runStub } from "../🎭️stub/🟦️.ts";

for(const correction of warmGraphFixture.secondSourceCorrections) registerIsolatedBuildTestV1(import.meta.path,correction.name,{bodyMilliseconds:120_000,scope:{kind:"body"}},async checkpoint=>{
  installRestartableFailureHandlerV1(() => undefined);
  const vector=warmGraphFixture.driverReplacement,directSecond=correction.source==="diagnostic",growth=correction.source==="growth";
  const root=mkdtempSync(join(artifacts,"wr-")), original=stubWorkspace(root,"app",[vector.application,vector.secondary,vector.prior,vector.replacement],"release");
  const pass={...original,packages:original.packages.slice(0,2)}, workspace=dirname(pass.manifest);
  const path=(name:string,leaf:string)=>join(workspace,name,leaf);
  const manifest=(name:string,dependency:string)=>[
    "[package]",'name = "'+name+'"','version = "0.1.0"','edition = "2021"',"[lib]",'path = "lib.rs"','crate-type = ["cdylib"]',
    "[dependencies]",...([vector.prior,vector.replacement].map(name=>name+' = { path = "../'+name+'", optional = true }')),
    "[features]",'default = ["dep:'+dependency+'"'+(growth&&dependency===vector.replacement?', "dep:'+vector.prior+'"':'')+']',"",
  ].join("\n");
  for (const name of [vector.prior,vector.replacement]) {
    const file=path(name,"Cargo.toml");
    writeFileSync(file,readFileSync(file,"utf8").replace('"cdylib"','"rlib"'));
    writeFileSync(path(name,"lib.rs"),vector.source);
  }
  for (const row of pass.packages) writeFileSync(row.manifest,manifest(row.name,growth?vector.replacement:vector.prior));
  expect(spawnSync(registeredToolExecutableV1("cargo"),["generate-lockfile","--offline","--manifest-path",pass.manifest],{encoding:"utf8"}).status).toBe(0);
  const command=join(root,"warm-graph.ts"),calls=join(root,"calls.jsonl"),target=pass.target;
  if(directSecond)writeFileSync(path(vector.application,"lib.rs"),vector.source);
  const config={directSecond,growth,cargo:registeredToolExecutableV1("cargo"),secondCode:correction.code,vector,workspace,calls,target,priorChecksum:Buffer.from(blake3(Buffer.from(vector.source))).toString("hex"),manifests:pass.packages.map(row=>[row.manifest,manifest(row.name,vector.replacement)]),header:[...header]};
  writeFileSync(command, String.raw`import {appendFileSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import {pathToFileURL} from "node:url";
const {directSecond,growth,cargo,secondCode,vector:v,workspace:w,calls,target,priorChecksum,manifests,header}=${JSON.stringify(config)};
if(process.argv[2]!=="build"){
 const result=spawnSync(cargo,process.argv.slice(2),{stdio:"inherit"});
 if(result.error)throw result.error;
 process.exit(result.status??1);
}
const path=(name,leaf)=>join(w,name,leaf),app=path(v.application,"Cargo.toml");
const selection=readFileSync(app,"utf8"),larger=growth&&readFileSync(path(v.prior,"lib.rs"),"utf8")!==v.source,first=!larger&&(growth||selection.includes("dep:"+v.prior)),child=first?v.prior:v.replacement;
const unit=(name,kind,features,dependencies)=>({pkg_id:"path+"+pathToFileURL(join(w,name)).href+"#"+name+"@0.1.0",target:{name,kind:[kind],crate_types:[kind],src_path:path(name,"lib.rs")},platform:target,mode:"build",features,dependencies:dependencies.map(index=>({index}))});
const edges=larger?[1,3]:[1],units=[unit(v.application,"cdylib",["default"],edges),unit(child,"rlib",[],[]),unit(v.secondary,"cdylib",["default"],edges)];
if(larger)units.push(unit(v.prior,"rlib",[],[]));
if(process.argv.includes("--unit-graph")){console.log(JSON.stringify({version:1,units,roots:[0,2]}));process.exit(0);}
const directory=join(process.env.CARGO_TARGET_DIR,target,"release");
appendFileSync(calls,JSON.stringify({child,directory,build:process.env.CARGO_BUILD_BUILD_DIR})+"\n");
mkdirSync(directory,{recursive:true});
const announce=(unit,file,checksum)=>{
 const source=unit.target.src_path,dep=file.replace(/\.(rlib|wasm)$/u,".d");
 writeFileSync(file,Buffer.concat([Buffer.from(header),Buffer.from(unit.target.name)]));
 writeFileSync(dep,file+": "+source.replaceAll(" ","\\ ")+"\n"+(checksum?"# checksum:blake3="+checksum+" file_len:"+Buffer.byteLength(v.source)+" "+source+"\n":""));
 console.log(JSON.stringify({reason:"compiler-artifact",package_id:unit.pkg_id,manifest_path:path(unit.target.name,"Cargo.toml"),target:unit.target,features:unit.features,profile:{test:false},filenames:[file],executable:null,fresh:false}));
};
const diagnostic=directSecond&&!first,edited=path(diagnostic?v.application:child,"lib.rs");
const changed=readFileSync(edited,"utf8")!==v.source;
announce(units[1],join(directory,"lib"+child+"-abc.rlib"),changed?undefined:priorChecksum);
if(!changed){
 writeFileSync(edited,v.editedSource);
 if(first&&!growth) for (const [path, manifest] of manifests) writeFileSync(path,manifest);
 console.log(JSON.stringify({reason:"compiler-message",package_id:units[0].pkg_id,manifest_path:app,target:units[0].target,message:{level:"error",code:{code:first?"E0599":secondCode},message:"selected source changed",spans:diagnostic?[{file_name:edited,is_primary:true}]:[]}}));
 process.exit(v.originalExit);
}
for (const index of [0,2]) announce(units[index],join(directory,units[index].target.name.replaceAll("-","_")+".wasm"),undefined);
process.exit(v.finalExit);`);
  if(growth&&"heldBytes" in correction&&"replacementBytes" in correction){
    let report="";
    const retry=runStub(root,{version:1,passes:[pass]},{command:process.execPath,args:[command],calls},
      {storageBytes:vector.storageBytes},"observe-linked",({held})=>{
        const generations=join(held.root,"cache",NATIVE_BUILD_DRIVER_POLICY_V1.warmRoot);
        const run=join(generations,readdirSync(generations)[0]!);
        report=readFileSync(join(run,NATIVE_BUILD_DRIVER_POLICY_V1.stores.report,"report.md"),"utf8");
        expect(existsSync(join(run,NATIVE_BUILD_DRIVER_POLICY_V1.stores.report,NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory,"cargo-unit-provenance-"+pass.id+".json"))).toBe(false);
      });
    const failure=await retry.catch(error=>error);
    expect(failure).toBeInstanceOf(ConcurrentEditErrorV1);
    const message=String(failure);
    const bounds=message.match(/held ([0-9]+) bytes, replacement ([0-9]+) bytes, admissible (-?[0-9]+) bytes, available (-?[0-9]+) bytes, floor ([0-9]+) bytes/u);
    expect<unknown>(bounds?.slice(1,3).map(Number)).toEqual([correction.heldBytes,correction.replacementBytes]);
    const [,held,,admissible,available,floor]=bounds!;
    using independent=new Database(":memory:");
    const predicted=independent.query("select min(?1,?2-?3) admissible").get(Number(held),Number(available),Number(floor));
    expect(predicted).toEqual({admissible:Number(admissible)});
    expect(report).toContain(bounds![0]);
    expect(readFileSync(calls,"utf8").trim().split("\\n").map(line=>JSON.parse(line).child)).toEqual([vector.prior]);
    checkpoint();
    return;
  }
  const seen:string[]=[];
  const outcome=await runStub(root,{version:1,passes:[pass]},{command:process.execPath,args:[command],calls}, {
    log:line=>seen.push(line),
    storageBytes:vector.storageBytes,
    stages:()=>[{name:"inspect",after:[],capacity:"io",async run(context){expect((await context.provenance()).units).toBe(vector.artifacts);}}],
  },"observe-linked",({outcome})=>{
    if(!outcome)return;
    const receipt=readFileSync(join(outcome.generation.logs,"unit-graph-"+pass.id+"-retry-2.json"),"utf8");
    expect(JSON.parse(receipt).units.map((unit:{target:{name:string}})=>unit.target.name)).toEqual([vector.application,vector.replacement,vector.secondary]);
    const directory=join(outcome.generation.report,NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory);
    const file=readdirSync(directory).find(name=>name.startsWith("cargo-unit-provenance-"))!;
    const sources=decodeCargoProvenanceV1(readFileSync(join(directory,file),"utf8")).units.flatMap(unit=>unit.inputs);
    const paths=sources.map(input=>input.path);
    if(directSecond)expect(sources.find(input=>input.path===path(vector.application,"lib.rs"))?.sha256).toBe(Buffer.from(sha256(Buffer.from(vector.editedSource))).toString("hex"));
    expect(paths).toContain(path(vector.replacement,"lib.rs"));
    expect(paths).not.toContain(path(vector.prior,"lib.rs"));
  });
  expect([...outcome.staged].sort()).toEqual([vector.application+":release",vector.secondary+":release"]);
  const rows=readFileSync(calls,"utf8").trim().split("\n").map(line=>JSON.parse(line));
  expect(rows.map(row=>row.child)).toEqual([vector.prior,vector.replacement,vector.replacement]);
  expect(new Set(rows.map(row=>row.directory)).size).toBe(1);
  expect(new Set(rows.map(row=>row.build)).size).toBe(1);
  expect(seen.filter(line=>line.includes("recompiling in the held lane"))).toHaveLength(2);
  checkpoint();
},correction.source==="growth"?/^Replacement Cargo unit graph does not fit the held component plan \(held /u:undefined);

for(const scenario of [...fixture.mixedWarmDepInfo.cases,...fixture.mixedWarmDepInfo.missingWitnessCases]) registerIsolatedBuildTestV1(import.meta.path,scenario.name,{bodyMilliseconds:120_000,scope:{kind:"body"}},async checkpoint=>{
  installRestartableFailureHandlerV1(() => undefined);
  const vector=fixture.mixedWarmDepInfo;
  const root=mkdtempSync(join(artifacts,"wm-")),workspace=stubWorkspace(root,"app",[vector.application,vector.dependency],"release");
  const [application,dependency]=workspace.packages;
  const pass={...workspace,packages:[application!]};
  writeFileSync(application!.manifest,readFileSync(application!.manifest,"utf8")+`\n[dependencies]\n${vector.dependency} = { path = "../${vector.dependency}" }\n`);
  writeFileSync(dependency!.manifest,readFileSync(dependency!.manifest,"utf8").replace('"cdylib"','"rlib"'));
  writeFileSync(join(dirname(dependency!.manifest),"lib.rs"),`include!("../${vector.application}/lib.rs");\n`);
  const source=join(dirname(application!.manifest),"lib.rs");
  const initialRevision="initialRevision" in scenario&&typeof scenario.initialRevision==="number"?scenario.initialRevision:0;
  const liveRevisions="liveRevisions" in scenario?scenario.liveRevisions:undefined;
  writeFileSync(source,vector.sourceRevisions[initialRevision]!);
  expect(spawnSync(registeredToolExecutableV1("cargo"),["generate-lockfile","--offline","--manifest-path",pass.manifest],{encoding:"utf8"}).status).toBe(0);
  const command=join(root,"mixed.ts"),calls=join(root,"mixed.jsonl");
  writeFileSync(command,String.raw`import {appendFileSync,existsSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {dirname,join} from "node:path";
import {pathToFileURL} from "node:url";
const {cargo,calls,source,units:packages,target,profile,header,claims,revisions,liveRevisions}=${JSON.stringify({
    cargo:registeredToolExecutableV1("cargo"),calls,source,units:[dependency,application],target:pass.target,profile:pass.profile,header:[...header],claims:scenario.claims,liveRevisions,
    revisions:vector.sourceRevisions.map(body=>({body,hash:Buffer.from(blake3(Buffer.from(body))).toString("hex"),length:Buffer.byteLength(body)})),
  })};
const args=process.argv.slice(2);
if(args[0]!=="build"){
  const result=spawnSync(cargo,args,{stdio:"inherit"});
  if(result.error)throw result.error;
  process.exit(result.status??1);
}
const units=packages.map(({name,manifest},index)=>{
  const pkg_id="path+"+pathToFileURL(dirname(manifest)).href+"#"+name+"@0.1.0";
  return {pkg_id,target:{name,kind:[index===0?"lib":"cdylib"],crate_types:[index===0?"rlib":"cdylib"],src_path:join(dirname(manifest),"lib.rs")},platform:target,mode:"build",features:[],dependencies:index===0?[]:[{index:0}]};
});
if(process.argv.includes("--unit-graph")){console.log(JSON.stringify({version:1,units,roots:[1]}));process.exit(0);}
const attempt=existsSync(calls)?readFileSync(calls,"utf8").trim().split("\n").length:0;
const directory=join(process.env.CARGO_TARGET_DIR,target,profile);
appendFileSync(calls,JSON.stringify({directory,build:process.env.CARGO_BUILD_BUILD_DIR})+"\n");
mkdirSync(directory,{recursive:true});
writeFileSync(source,revisions[claims[attempt][1]].body);
for(const [index,unit] of units.entries()){
  const file=join(directory,index===0?"libmixed_dep-abc.rlib":"mixed_app.wasm");
  const {hash,length}=revisions[claims[attempt][index]];
  const dep=file.replace(/\.(?:rlib|wasm)$/u,".d"),inputs=[...new Set([unit.target.src_path,source])];
  writeFileSync(file,Buffer.concat([Buffer.from(header),Buffer.from([attempt+1])]));
  writeFileSync(dep,file+": "+inputs.map(path=>path.replaceAll(" ","\\ ")).join(" ")+"\n# checksum:blake3="+hash+" file_len:"+length+" "+source+"\n");
   if(index===0&&liveRevisions)writeFileSync(source,revisions[liveRevisions[attempt]].body);
  console.log(JSON.stringify({reason:"compiler-artifact",package_id:unit.pkg_id,manifest_path:packages[index].manifest,target:unit.target,features:[],profile:{test:false},filenames:[file],executable:null,fresh:attempt>0||index===0}));
}
`);
  const planned:number[]=[],staged:number[]=[],log:string[]=[];
  const result=await runStub(root,{version:1,passes:[pass]},{command:process.execPath,args:[command],calls},{
    storageBytes:warmGraphFixture.driverReplacement.storageBytes,
    log:line=>log.push(line),
    stages:()=>{
      planned.push(readFileSync(calls,"utf8").trim().split("\n").length);
      return [{name:"inspect",after:[],capacity:"io",async run(context){
        expect((await context.provenance()).units).toBe(2);
      }}];
    },
  },"observe-mixed",({outcome})=>{
    if(!outcome)return;
    const stage=nativeBuildComponentStagePathV1(outcome.generation,pass,application!);
    if(existsSync(stage))staged.push(readFileSync(stage).at(-1)!);
  }).then(outcome=>({outcome}),error=>({error}));
  if(!existsSync(calls)&&"error"in result)throw result.error;
  const attempts=readFileSync(calls,"utf8").trim().split("\n").map(line=>JSON.parse(line));
  expect(attempts).toHaveLength(scenario.attempts);
  expect(new Set(attempts.map(row=>row.directory)).size).toBe(1);
  expect(new Set(attempts.map(row=>row.build)).size).toBe(1);
  expect(log.filter(line=>line.includes("recompiling in the held lane"))).toHaveLength(scenario.attempts-1);
  expect(planned).toEqual(scenario.staged===null?[]:[scenario.staged]);
  expect(staged).toEqual(scenario.staged===null?[]:[scenario.staged]);
  if(scenario.staged===null){
    expect("error"in result&&concurrentEditOfV1(result.error)).toMatchObject({reason:"bytes",path:source});
  }else expect(result).toHaveProperty("outcome");
  checkpoint();
},scenario.staged===null?(fixture.mixedWarmDepInfo.missingWitnessCases.some(row=>row.name===scenario.name)?/^Compiler consumed bytes have no matching retained source witness \(BLAKE3 and byte lengths /u:/^Compiler consumed inconsistent source versions \(BLAKE3 and byte lengths /u):undefined);
