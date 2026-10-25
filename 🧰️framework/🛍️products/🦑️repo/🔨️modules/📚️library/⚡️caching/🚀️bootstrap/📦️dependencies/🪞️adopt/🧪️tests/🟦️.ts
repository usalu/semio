import {test,expect} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {dirname,join} from "node:path";
import * as core from "../🟦️.ts";
import * as glue from "../💾️storage/🟦️.ts";
import {generationSha256V1} from "../../🧾️generation/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import installSchema from "../../💾️storage/🧬️schema/📥️install.json" with {type:"json"};

const protocol=core.adoptProtocolV1;
const oracle=(args:string[],input?:string,raw=false):any=>{const run=spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),...args],{encoding:"utf8",input,maxBuffer:1<<28});if(run.status!==0)throw Error("oracle failed: "+run.stderr);return raw?run.stdout.trim():JSON.parse(run.stdout);};
const scratch=()=>realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"adopt-")));
const bytes=(text:string)=>Buffer.from(text);
const toFiles=(files:Record<string,string>):core.AdoptFileV1[]=>Object.entries(files).map(([path,text])=>({path,bytes:bytes(text)}));
const cloneOf=(files:Record<string,string>)=>JSON.parse(JSON.stringify(files))as Record<string,string>;
function mutateFiles(files:Record<string,string>,row:(typeof fixture.projection)[number]):Record<string,string>{
 const out=cloneOf(files),edit=row.edit as {set?:Record<string,unknown>;unset?:string[];text?:string;remove?:boolean;reorder?:boolean};
 if(edit.text!==undefined)out[row.path]=edit.text;
 else if(edit.remove)delete out[row.path];
 else{
  const document=JSON.parse(out[row.path]??"{}")as Record<string,unknown>;
  if(edit.reorder)out[row.path]=JSON.stringify(Object.fromEntries(Object.entries(document).reverse()));
  else{for(const key of edit.unset??[])delete document[key];Object.assign(document,edit.set??{});out[row.path]=JSON.stringify(document);}
 }
 return out;
}
const projectionByOracle=(files:Record<string,string>)=>oracle(["projection"],JSON.stringify({files,inert:fixture.inert}),true);

/** 🧬️ Pins the neutral contract: Ajv accepts a sealed receipt, the inert field list equals the fixture and the independent Python seal equals ours. */
test("adopt contract: Ajv accepts sealed receipts and the inert list equals the fixture",()=>{
 const ajv=new Ajv({strict:false}).addSchema(schema),runtime={bun:"1.3.14",platform:"darwin"as const,arch:"arm64",recipeSha256:"a".repeat(64)};
 const body={version:1 as const,purpose:"dependency-adopt-receipt" as const,generation:"b".repeat(64),projectionSha256:"c".repeat(64),runtime,installed:"d".repeat(64)};
 const receipt=core.sealAdoptReceiptV1(body),check=ajv.getSchema(schema.$id+"#/$defs/AdoptReceipt")!;
 expect(check(JSON.parse(JSON.stringify(receipt))),JSON.stringify(check.errors)).toBe(true);
 expect(core.parseAdoptReceiptV1(Buffer.from(JSON.stringify(receipt)))).toEqual(receipt);
 expect(receipt.sha256).toBe(oracle(["seal",JSON.stringify(body)],undefined,true));
 expect(()=>core.parseAdoptReceiptV1(Buffer.from(JSON.stringify({...receipt,installed:"e".repeat(64)})))).toThrow();
 expect([...protocol.adopt.inertManifestFields]).toEqual(fixture.inert);
 expect(core.adoptCloneCommandV1("darwin","/s","/t")).toEqual([...protocol.adopt.clone.darwin,"/s","/t"]);
 expect(core.adoptCloneCommandV1("win32","/s","/t")).toBeUndefined();
 expect(core.adoptClonedRootsV1(2)).toEqual(["node_modules","w/0/node_modules","w/1/node_modules"]);
});

/** 🔑️ Moves the projection key exactly with install-relevant inputs: inert manifest edits keep it, dependency, lock, patch, config and membership edits change it, and the independent Python walk computes the same digest for every case. */
test("adopt projection: inert edits keep the key, install inputs move it, Python agrees",()=>{
 const base=fixture.base as Record<string,string>,baseKey=core.installProjectionSha256V1(toFiles(base));
 expect(baseKey).toBe(projectionByOracle(base));
 expect(core.installProjectionSha256V1(toFiles(base).reverse())).toBe(baseKey);
 for(const row of fixture.projection){
  const changed=mutateFiles(base,row),key=core.installProjectionSha256V1(toFiles(changed));
  expect(key===baseKey?"same":"different",row.id).toBe(row.expect);
  expect(key,row.id).toBe(projectionByOracle(changed));
 }
 expect(()=>core.installProjectionSha256V1([{path:"a",bytes:bytes("1")},{path:"a",bytes:bytes("2")}])).toThrow();
});

const installToy=(root:string,variant:"base"|"inert"|"control")=>{
 const cache=join(root,"cache"),workspace=join(root,variant),leaf=join(root,"pk");
 if(!existsSync(join(root,"leaf.tgz"))){
  for(const[path,text]of Object.entries(fixture.bun.leaf)){mkdirSync(dirname(join(leaf,path)),{recursive:true});writeFileSync(join(leaf,path),text);}
  expect(spawnSync("tar",["czf",join(root,"leaf.tgz"),"-C",leaf,"package"]).status).toBe(0);
 }
 for(const[path,text]of Object.entries(fixture.bun.workspace)){
  mkdirSync(dirname(join(workspace,path)),{recursive:true});
  let content=text;
  if(variant==="inert"&&path.endsWith("package.json")&&path!=="package.json"){
   const document=JSON.parse(text)as Record<string,unknown>;
   for(const field of fixture.inert)document[field]=(fixture.inertValues as Record<string,unknown[]>)[field]![1];
   content=JSON.stringify(document);
  }
  writeFileSync(join(workspace,path),content);
 }
 return{cache,workspace};
};
const bunInstall=(workspace:string,cache:string,root:string,frozen:boolean)=>spawnSync(process.execPath,["install",...(frozen?["--frozen-lockfile"]:[]),"--linker=isolated","--backend=copyfile","--cache-dir",cache,"--ignore-scripts"],{cwd:workspace,encoding:"utf8",env:{...process.env,HOME:root}});

/** 🔬️ Proves with Bun itself that every inert field is install-inert: the frozen install of a workspace whose members carry different values for all of them yields the identical installed tree, while an install-relevant edit does not. */
test("adopt bun probe: all inert manifest fields leave the installed tree and the frozen lock unchanged",()=>{
 const root=scratch();try{
  const base=installToy(root,"base"),first=bunInstall(base.workspace,base.cache,root,false);expect(first.status,first.stderr).toBe(0);
  const members=["a","b","c"],baseTree=oracle(["modules",base.workspace,...members]);
  const inert=installToy(root,"inert");writeFileSync(join(inert.workspace,"bun.lock"),readFileSync(join(base.workspace,"bun.lock")));
  const second=bunInstall(inert.workspace,inert.cache,root,true);expect(second.status,second.stderr).toBe(0);
  expect(readFileSync(join(inert.workspace,"bun.lock"),"utf8")).toBe(readFileSync(join(base.workspace,"bun.lock"),"utf8"));
  expect(oracle(["modules",inert.workspace,...members])).toEqual(baseTree);
  const control=installToy(root,"control");writeFileSync(join(control.workspace,"bun.lock"),readFileSync(join(base.workspace,"bun.lock")));
  writeFileSync(join(control.workspace,fixture.bun.controlRemove),fixture.bun.controlManifest);
  const third=bunInstall(control.workspace,control.cache,root,true);
  const moved=third.status!==0||JSON.stringify(oracle(["modules",control.workspace,...members]))!==JSON.stringify(baseTree);
  expect(moved,"an install-relevant edit must change the outcome").toBe(true);
  const keys=fixture.inert.map(field=>core.installProjectionSha256V1([{path:"w/0/package.json",bytes:bytes(JSON.stringify({name:"a",[field]:(fixture.inertValues as Record<string,unknown[]>)[field]![0]}))}]));
  const other=fixture.inert.map(field=>core.installProjectionSha256V1([{path:"w/0/package.json",bytes:bytes(JSON.stringify({name:"a",[field]:(fixture.inertValues as Record<string,unknown[]>)[field]![1]}))}]));
  expect(keys).toEqual(other);
 }finally{rmSync(root,{recursive:true,force:true});}
},240000);

const cloneTree=(source:string,target:string)=>{
 const argv=core.adoptCloneCommandV1(process.platform,source,target);
 if(argv){const run=spawnSync(argv[0]!,argv.slice(1),{encoding:"utf8"});return run.status===0;}
 return false;
};

/** 🪞️ Clones a real Bun installation by the platform's copy-on-write command and proves with Python that the clone is an exact structural copy sharing no inode with its donor. */
test("adopt clone: the platform command yields an exact copy without shared inodes",()=>{
 if(!core.adoptCloneCommandV1(process.platform,"s","t"))return;
 const root=scratch();try{
  const base=installToy(root,"base"),first=bunInstall(base.workspace,base.cache,root,false);expect(first.status,first.stderr).toBe(0);
  const source=join(base.workspace,"node_modules"),target=join(root,"clone","node_modules");mkdirSync(dirname(target));
  expect(cloneTree(source,target)).toBe(true);
  expect(oracle(["tree",target])).toEqual(oracle(["tree",source]));
  expect(Number(oracle(["shared",source,target],undefined,true))).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});}
},240000);

/** 🔄️ Runs the adoption flow over a real store directory and real clones: the newest qualifying donor with the identical projection and runtime whose ledger verifies and whose clone has the donor's exact shape donates its trees; every other case reports that an installation is needed and leaves no partial clone. */
test("adopt flow: donors are chosen by projection, runtime, verification and shape",async()=>{
 for(const row of fixture.donors){
  const store=scratch();try{
   const base=fixture.base as Record<string,string>,other={...base,"bun.lock":"{\"lockfileVersion\": 2}\n"},runtime={bun:"1.3.14",platform:process.platform as "darwin",arch:process.arch,recipeSha256:"a".repeat(64)};
   const generation="f".repeat(64),target=join(store,"j",generation.slice(0,20));mkdirSync(join(target,"w","0"),{recursive:true});
   const absolute=toFiles(base).map(file=>({path:join(target,...file.path.split("/")),bytes:file.bytes}));
   const roots=["node_modules","w/0/node_modules"],walk=(directory:string)=>{
    const lines:string[]=[];let files=0,links=0,directories=0;
    for(const root of roots){const start=join(directory,...root.split("/"));if(!existsSync(start))continue;
     const visit=(path:string,portable:string)=>{const facts=lstatSync(path);const kind=facts.isSymbolicLink()?"link":facts.isDirectory()?"directory":"file";lines.push(JSON.stringify([portable,kind]));if(kind==="file")files++;else if(kind==="link")links++;else{directories++;for(const name of readdirSync(path).sort())visit(join(path,name),portable+"/"+name);}};
     visit(start,root);}
    return{entries:lines.length,directories,files,links,sha256:generationSha256V1(lines.join("\n"))};
   };
   const ids:string[]=[],installed:string[]=[];
   for(const[index,spec]of row.donors.entries()){
    const id=String(index+1).repeat(64),dir=join(store,"j",id.slice(0,20));
    mkdirSync(join(dir,"node_modules","pkg"),{recursive:true});mkdirSync(join(dir,"w","0","node_modules"),{recursive:true});
    writeFileSync(join(dir,"node_modules","pkg","index.js"),"module.exports="+index);writeFileSync(join(dir,"w","0","node_modules","x"),"x");
    ids.push(id);installed.push(generationSha256V1("installed"+index));
    await glue.sealJavascriptDependencyAdoptReceiptV1(store,{version:1,purpose:"dependency-adopt-receipt",generation:id,projectionSha256:core.installProjectionSha256V1(toFiles(spec.projection==="same"?base:other)),runtime:{...runtime,bun:spec.bun},installed:installed[index]!},()=>{},async()=>{});
    const at=new Date(Date.now()-spec.age*3600000);
    for(const name of readdirSync(join(store,"generation")))if(name.startsWith(id.slice(0,20)+"."))utimesSync(join(store,"generation",name),at,at);
   }
   const pinned:string[]=[];
   const adopted=await glue.adoptJavascriptDependencyInstalledTreeV1({storeRoot:store,generation,directory:target,files:absolute,members:1,runtime,check:()=>{},
    verifyDonor:async donor=>{const index=ids.indexOf(donor),spec=row.donors[index]!;if(spec.verify==="missing")return undefined;const shape=walk(join(store,"j",donor.slice(0,20)));return{sha256:spec.verify==="mismatch"?"0".repeat(64):installed[index]!,shape:spec.scan==="differs"?{...shape,sha256:"9".repeat(64)}:shape};},
    pin:async donor=>{pinned.push(donor);},
    run:async argv=>spawnSync(argv[0]!,argv.slice(1)).status===0,
    scan:async()=>walk(target),
    remove:async path=>{rmSync(path,{recursive:true,force:true});}});
   expect(adopted===undefined?null:ids.indexOf(adopted),row.id).toBe(row.expectDonor);
   if(row.expectDonor===null){expect(existsSync(join(target,"node_modules")),row.id).toBe(false);expect(existsSync(join(target,"w","0","node_modules")),row.id).toBe(false);}
   else{
    expect(readFileSync(join(target,"node_modules","pkg","index.js"),"utf8"),row.id).toBe("module.exports="+row.expectDonor);
    expect(oracle(["tree",join(target,"node_modules")]),row.id).toEqual(oracle(["tree",join(store,"j",ids[row.expectDonor]!.slice(0,20),"node_modules")]));
    expect(Number(oracle(["shared",join(target,"node_modules"),join(store,"j",ids[row.expectDonor]!.slice(0,20),"node_modules")],undefined,true)),row.id).toBe(0);
    expect(pinned).toContain(ids[row.expectDonor]);
   }
   const fresh=await glue.sealJavascriptDependencyAdoptReceiptV1(store,{version:1,purpose:"dependency-adopt-receipt",generation,projectionSha256:core.installProjectionSha256V1(toFiles(base)),runtime,installed:generationSha256V1("target")},()=>{},async()=>{});
   expect(fresh).toMatch(/^[a-f0-9]{64}$/u);expect(glue.hasJavascriptDependencyAdoptReceiptV1(store,generation)).toBe(true);
   rmSync(join(store,"j",ids[0]?.slice(0,20)??"none"),{recursive:true,force:true});
   await glue.sealJavascriptDependencyAdoptReceiptV1(store,{version:1,purpose:"dependency-adopt-receipt",generation,projectionSha256:core.installProjectionSha256V1(toFiles(base)),runtime,installed:generationSha256V1("target2")},()=>{},async()=>{});
   if(ids[0])expect(glue.hasJavascriptDependencyAdoptReceiptV1(store,ids[0]),row.id+" retires the receipt of a removed generation").toBe(false);
  }finally{rmSync(store,{recursive:true,force:true});}
 }
},180000);

/** 🛑️ Propagates cancellation raised while cloning and removes the partial clone first. */
test("adopt flow: cancellation during the clone leaves no partial tree",async()=>{
 const store=scratch();try{
  const base=fixture.base as Record<string,string>,runtime={bun:"1.3.14",platform:process.platform as "darwin",arch:process.arch,recipeSha256:"a".repeat(64)},generation="f".repeat(64),target=join(store,"j",generation.slice(0,20)),id="1".repeat(64),dir=join(store,"j",id.slice(0,20));
  mkdirSync(target,{recursive:true});mkdirSync(join(dir,"node_modules","pkg"),{recursive:true});writeFileSync(join(dir,"node_modules","pkg","index.js"),"x");
  await glue.sealJavascriptDependencyAdoptReceiptV1(store,{version:1,purpose:"dependency-adopt-receipt",generation:id,projectionSha256:core.installProjectionSha256V1(toFiles(base)),runtime,installed:"a".repeat(64)},()=>{},async()=>{});
  let calls=0;const absolute=toFiles(base).map(file=>({path:join(target,...file.path.split("/")),bytes:file.bytes}));
  await expect(glue.adoptJavascriptDependencyInstalledTreeV1({storeRoot:store,generation,directory:target,files:absolute,members:0,runtime,check:()=>{if(++calls>2)throw Error("cancelled");},
   verifyDonor:async()=>({sha256:"a".repeat(64),shape:{entries:1,directories:1,files:0,links:0,sha256:"b".repeat(64)}}),pin:async()=>{},
   run:async argv=>{const status=spawnSync(argv[0]!,argv.slice(1)).status===0;return status;},scan:async()=>{throw Error("scan refused");},remove:async path=>{rmSync(path,{recursive:true,force:true});}})).rejects.toThrow("cancelled");
  expect(existsSync(join(target,"node_modules"))).toBe(false);
 }finally{rmSync(store,{recursive:true,force:true});}
},120000);

/** ⚖️ Pins the producer wiring by source law: a donor is tried before the installer, the receipt is sealed after the installed-paths ledger and once for existing generations. */
test("adopt wiring laws over the storage module",async()=>{
 const ts=await import("typescript"),path=new URL("../../💾️storage/🟦️.ts",import.meta.url),source=readFileSync(path,"utf8"),tree=ts.createSourceFile(path.pathname,source,ts.ScriptTarget.Latest,true);
 const node=tree.statements.find(value=>ts.isFunctionDeclaration(value)&&value.name?.text==="runJavascriptDependencyStoreProducerV1");expect(node).toBeDefined();
 const producer=node!.getText(tree);
 expect(producer.indexOf("adoptJavascriptDependencyInstalledTreeV1(")).toBeGreaterThan(0);
 expect(producer.indexOf("adoptJavascriptDependencyInstalledTreeV1(")).toBeLessThan(producer.indexOf("let terminal=adoptedFrom?"));
 expect(producer.indexOf("let terminal=adoptedFrom?")).toBeLessThan(producer.indexOf("await adoptSeal(row);frame.built.set("));
 expect(producer.indexOf("requireJavascriptDependencyInstalledPathsIncrementalV1(workspace,context,control,row.plan.generation,checkpoint);await adoptSeal(row)")).toBeGreaterThan(0);
 expect(producer).toContain("hasJavascriptDependencyAdoptReceiptV1(store.root,row.plan.generation))await adoptSeal(row)");
 expect(producer).toContain("pin:donor=>pinJavascriptDependencyGenerationsV1(frame,[donor])");
 expect(producer).toContain("current-dependency-generation-adopted");
 expect(existsSync(fileURLToPath(new URL("../🟦️.ts",import.meta.url)))).toBe(true);
 expect(lstatSync(fileURLToPath(new URL("../💾️storage/🟦️.ts",import.meta.url))).isFile()).toBe(true);
});

/** 🧾️ Pins the generation key and the ready receipt by source and schema law: staged manifests drop the inert fields before they are hashed and written, the ready receipt is version 3 without any execution fingerprint, and the donor is pinned before it is verified. */
test("adopt key and ready laws: the generation key is install-relevant and the ready receipt carries no execution",async()=>{
 const ts=await import("typescript"),path=new URL("../../💾️storage/🟦️.ts",import.meta.url),source=readFileSync(path,"utf8"),tree=ts.createSourceFile(path.pathname,source,ts.ScriptTarget.Latest,true);
 const text=(name:string)=>{const node=tree.statements.find(value=>ts.isFunctionDeclaration(value)&&value.name?.text===name);expect(node,name).toBeDefined();return node!.getText(tree);};
 const inputs=text("inspectJavascriptDependencyInputsReadV1");
 expect(inputs).toContain('key!=="workspaces"&&!javascriptDependencyInertManifestFieldsV1.has(key)');
 expect(inputs).toContain("manifest=Object.fromEntries(Object.entries(JSON.parse(capture(rootPath)");
 expect(inputs.indexOf("javascriptDependencyInertManifestFieldsV1.has(key)")).toBeLessThan(inputs.indexOf("const hash=createHash"));
 expect(source).toContain("new Set(adoptProtocolV1.adopt.inertManifestFields)");
 const ready=text("javascriptDependencySourceOwnerReadyReceiptV1");expect(ready).toContain('version:3,purpose:"current-dependency-source-owner-ready"');expect(ready).not.toContain("executionSha256");
 expect(text("readJavascriptDependencySourceOwnerReadyV1")).not.toContain("executionSha256");
 const definition=(installSchema.$defs as Record<string,any>).SourceOwnerReady;
 expect(definition.required).not.toContain("executionSha256");expect(Object.keys(definition.properties)).not.toContain("executionSha256");expect(definition.properties.version.const).toBe(3);
 expect(definition.required).toEqual(expect.arrayContaining(["scope","generation","bindings","members","ownerReferences"]));
 const ajv=new Ajv({strict:false}).addSchema(installSchema,"install.json"),validate=ajv.compile({$ref:"install.json#/$defs/SourceOwnerReady"}),sha="a".repeat(64),ok={version:3,purpose:"current-dependency-source-owner-ready",scope:".",generation:sha,ownerRelationSha256:sha,groupSha256:sha,recipeSha256:sha,members:[],bindings:[{sourceDirectory:"/w/node_modules",targetDirectory:"/s/j/"+sha.slice(0,20)+"/node_modules"}],ownerReferences:[]};
 expect(validate(ok),JSON.stringify(validate.errors)).toBe(true);
 expect(validate({...ok,executionSha256:sha})).toBe(false);expect(validate({...ok,version:2})).toBe(false);
 const glueSource=readFileSync(new URL("../💾️storage/🟦️.ts",import.meta.url),"utf8");
 expect(glueSource.indexOf("await request.pin(donor.generation)")).toBeLessThan(glueSource.indexOf("await request.verifyDonor(donor.generation)"));
 const producer=text("runJavascriptDependencyStoreProducerV1");
 expect(producer.indexOf("pinJavascriptDependencyGenerationsV1(frame,selected")).toBeLessThan(producer.indexOf("captureOwnedCommandResultV1("));
 expect(producer.indexOf("admitJavascriptDependencyGenerationV1(workspace,store,control,row.scope)")).toBeLessThan(producer.indexOf("adoptJavascriptDependencyInstalledTreeV1("));
});
