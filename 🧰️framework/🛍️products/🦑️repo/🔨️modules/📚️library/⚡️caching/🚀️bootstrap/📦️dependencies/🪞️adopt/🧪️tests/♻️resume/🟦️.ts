import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import * as fs from "node:fs";
import {join,resolve} from "node:path";
import * as core from "../../🟦️.ts";
import * as storage from "../../💾️storage/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import vectors from "./🔣️.json" with {type:"json"};

test("sealed current generation reuse obeys neutral completion and refusal cases",async()=>{
 const run=(storage as unknown as Record<string,unknown>).reuseJavascriptDependencyInstalledGenerationV1 as undefined|((request:Record<string,unknown>)=>Promise<boolean>);
 if(!run)throw Error("Actual current-generation reuse owner required");
 const exercise=run;
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Owned resume artifacts required");fs.mkdirSync(output,{recursive:true});
 const checkCase=new Ajv({strict:true}).compile(schema.$defs.ResumeCase),db=new Database(":memory:");
 try{for(const row of vectors){
  expect(checkCase(row),row.id).toBe(true);
  const root=fs.mkdtempSync(join(output,"reuse-")),generation="b".repeat(64),directory=join(root,"j",generation.slice(0,20)),files=[{path:join(directory,"package.json"),bytes:Buffer.from('{"name":"resume"}')}],runtime={bun:"1.4.2",platform:"win32"as const,arch:"x64",recipeSha256:"a".repeat(64)},installed="d".repeat(64),shape={entries:3,directories:1,files:1,links:1,maximumPathUnits:100,sha256:"e".repeat(64)},body={version:1 as const,purpose:"dependency-adopt-receipt"as const,generation,projectionSha256:storage.adoptProjectionOfV1(directory,files),runtime,installed},changed=structuredClone(body),primary=Object.freeze({canceled:row.id});
  let verifications=0,scans=0,reused=false,error:unknown,finalAccepted=false;
  if(row.mode==="generation")changed.generation="c".repeat(64);
  if(row.mode==="projection"||row.mode==="immutable-input")changed.projectionSha256="c".repeat(64);
  if(row.mode==="bun")changed.runtime.bun="other";
  if(row.mode==="platform")changed.runtime.platform="linux"as "win32";
  if(row.mode==="arch")changed.runtime.arch="other";
  if(row.mode==="recipe")changed.runtime.recipeSha256="c".repeat(64);
  const receipt=core.sealAdoptReceiptV1(changed);
  fs.mkdirSync(directory,{recursive:true});fs.writeFileSync(files[0]!.path,files[0]!.bytes);
  fs.mkdirSync(join(root,"generation"),{recursive:true});
  if(row.mode!=="partial")fs.writeFileSync(join(root,"generation",generation.slice(0,20)+"."+receipt.sha256+core.adoptProtocolV1.adopt.receiptSuffix),JSON.stringify(row.mode==="forged"?{...receipt,installed:"c".repeat(64)}:receipt));
  try{reused=await exercise({storeRoot:root,generation,directory,files,runtime,check:()=>{if(row.canceled)throw primary;},verifyInstalled:async()=>{verifications++;return row.mode==="installed-missing"?undefined:{sha256:row.mode==="installed-digest"?"c".repeat(64):installed,shape};},verifyExports:async()=>{scans++;if(row.mode==="retarget-race")throw Error("Original installed dependency entry changed");return !["shape","root-alias","projected-root","projected-subpath"].includes(row.mode);}});finalAccepted=reused&&row.mode!=="provider-facts";}catch(failure){error=failure;}
  expect(reused,row.id).toBe(row.reused);
  expect(finalAccepted,row.id).toBe(row.finalAccepted);
  if(row.canceled)expect(error).toBe(primary);
  if(row.mode==="retarget-race")expect(String(error)).toContain("entry changed");
  const oracle=db.query("SELECT (?1 IN ('completed','unrelated-file-body','provider-facts')) reused, (?1 IN ('completed','unrelated-file-body')) accepted").get(row.mode)as{reused:number;accepted:number};
  expect(Boolean(oracle.reused),row.id).toBe(reused);expect(Boolean(oracle.accepted),row.id).toBe(finalAccepted);
  if(["completed","unrelated-file-body","provider-facts"].includes(row.mode)){expect(verifications).toBe(1);expect(scans).toBe(1);}
  fs.rmSync(root,{recursive:true,force:true});
 }}finally{db.close();}
});

import ts from "typescript";
import {createHash} from "node:crypto";
import {dirname,sep} from "node:path";
import exportVectors from "./📤️.json" with {type:"json"};

test("completed generation reuse reobserves actual no-follow export aliases and physical shape",async()=>{
 const sourceUrl=new URL("../../../💾️storage/🟦️.ts",import.meta.url),text=fs.readFileSync(sourceUrl,"utf8"),tree=ts.createSourceFile("installed.ts",text,ts.ScriptTarget.Latest,true),names=["scanJavascriptDependencyInstalledPathsV1","javascriptDependencyWorkspaceMappingsV1","verifyJavascriptDependencyCompletedExportsV1"],nodes=names.flatMap(name=>{const node=tree.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text===name);return node?[node]:[];});
 const descriptor=tree.statements.find((node):node is ts.ImportDeclaration=>ts.isImportDeclaration(node)&&node.importClause?.namedBindings!==undefined&&ts.isNamedImports(node.importClause.namedBindings)&&node.importClause.namedBindings.elements.some(item=>item.name.text==="withArtifactReadDirectoryV1"));if(!descriptor||!ts.isStringLiteral(descriptor.moduleSpecifier))throw Error("Actual readonly descriptor port required");
 const {withArtifactReadDirectoryV1}=await import(new URL(descriptor.moduleSpecifier.text,sourceUrl).href),policy=JSON.parse(fs.readFileSync(new URL("../../../💾️storage/🧬️schema/📥️install.json",import.meta.url),"utf8")),validate=new Ajv({strict:true}).compile(schema.$defs.ExportResumeCase),output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Owned export artifacts required");
 for(const row of exportVectors){
  expect(validate(row),row.mode).toBe(true);
  const root=fs.mkdtempSync(join(output,"ex-")),workspace=join(root,"s"),source=join(workspace,"m"),generation="b".repeat(64),directory=join(root,"j",generation.slice(0,20)),alias=join(directory,"node_modules/sample"),projected=join(directory,"w/0"),other=join(root,"other"),stop=new AbortController(),primary=Object.freeze({cancel:row.mode}),sourceBytes=Buffer.from('{"name":"sample","version":"1.0.0"}'),inputBytes=Buffer.from('{"name":"root"}');
  for(const path of[source,other,dirname(alias),projected]){if(path.length>256)throw Error("Bounded export fixture required");fs.mkdirSync(path,{recursive:true});}
  fs.writeFileSync(join(source,"package.json"),sourceBytes);fs.writeFileSync(join(directory,"package.json"),inputBytes);fs.writeFileSync(join(directory,"payload.js"),"original");
  const link=(target:string,path:string)=>fs.symlinkSync(target,path,process.platform==="win32"?"junction":"dir");
  if(row.mode!=="root-missing"&&row.mode!=="root-file")link(row.mode==="root-retarget"?other:source,alias);
  if(row.mode==="root-file")fs.writeFileSync(alias,"physical");
  if(["projected-root","projected-subpath","projected-node-modules","projected-node-modules-descendant"].includes(row.mode)){const target=row.mode==="projected-root"?projected:join(projected,row.mode==="projected-subpath"?"src":row.mode==="projected-node-modules-descendant"?"node_modules/child":"node_modules");fs.mkdirSync(target,{recursive:true});link(target,join(directory,"extra"));}
  if(row.mode==="provider-facts")fs.writeFileSync(join(source,"package.json"),'{"name":"sample","version":"2.0.0"}');
  if(row.mode==="immutable-input")fs.writeFileSync(join(directory,"package.json"),'{"name":"mutated"}');
  if(row.mode==="unrelated-file-body")fs.writeFileSync(join(directory,"payload.js"),"changed");
  let changed=false,error:unknown,accepted=false;
  const control={signal:stop.signal,publish:async()=>{},yieldContinuation:async()=>{}},check=()=>stop.signal.throwIfAborted(),ports={Buffer,performance,createHash,lstatSync:fs.lstatSync,readProducerBindingLinkV1:(path:string)=>{const value=fs.readlinkSync(path);if(path===alias&&!changed&&(row.mode==="retarget-race"||row.canceled)){changed=true;if(row.canceled)stop.abort(primary);else{fs.unlinkSync(path);link(other,path);}}return value;},join,resolve,dirname,sep,installSchema:policy,withArtifactReadDirectoryV1,requireIssuedResourceControlV1:(actual:unknown)=>{expect(actual).toBe(control);check();},checkResourceLeaseControl:(actual:unknown)=>{expect(actual).toBe(control);check();}};
  const api=new Function(...Object.keys(ports),ts.transpileModule(nodes.map(node=>node.getText(tree).replace(/^export\s/u,"")).join("\n")+";return {scan:scanJavascriptDependencyInstalledPathsV1,verify:typeof verifyJavascriptDependencyCompletedExportsV1==='function'?verifyJavascriptDependencyCompletedExportsV1:undefined};",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText)(...Object.values(ports));
  const plan={scope:".",plan:{generation,directory,members:["m"],sources:[{path:join(source,"package.json"),kind:"file",bytes:sourceBytes}]}};
  try{accepted=Boolean(await api.verify(workspace,plan,control,check));}catch(failure){error=failure;}
  try{
   expect(accepted,row.mode).toBe(row.accepted);
   const oracle=new Database(":memory:");
   try{const target=fs.existsSync(alias)&&fs.lstatSync(alias).isSymbolicLink()?resolve(dirname(alias),fs.readlinkSync(alias)):null,targets=[alias,join(directory,"extra")].filter(path=>fs.existsSync(path)&&fs.lstatSync(path).isSymbolicLink()).map(path=>resolve(dirname(path),fs.readlinkSync(path))),result=oracle.query("SELECT ((?1=?2) IS TRUE) AND NOT ?7 AND NOT EXISTS (SELECT 1 FROM json_each(?6) WHERE value=?3 OR (substr(value,1,length(?3||?5))=?3||?5 AND value<>?4 AND substr(value,1,length(?4||?5))<>?4||?5)) valid").get(target,source,projected,join(projected,"node_modules"),sep,JSON.stringify(targets),row.canceled?1:0)as{valid:number};expect(Boolean(result.valid),row.mode).toBe(accepted);}finally{oracle.close();}

   const finalAccepted=accepted&&fs.readFileSync(join(source,"package.json")).equals(sourceBytes)&&fs.readFileSync(join(directory,"package.json")).equals(inputBytes);
   expect(finalAccepted,row.mode).toBe(row.finalAccepted);
   if(row.canceled)expect(error).toBe(primary);
   if(row.mode==="retarget-race")expect(String(error)).toContain("entry changed");
   if(row.canceled||row.mode==="retarget-race")expect(changed).toBe(true);
  }finally{fs.rmSync(root,{recursive:true,force:true});}
 }
});

import productionVectors from "./🏭️.json" with {type:"json"};
test("actual producer fresh loop skips only completed install and export work",async()=>{
 const source=fs.readFileSync(new URL("../../../💾️storage/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("producer.ts",source,ts.ScriptTarget.Latest,true),owner=tree.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text==="runJavascriptDependencyStoreProducerV1");if(!owner)throw Error("Actual fixed producer required");let loop:ts.ForOfStatement|undefined;const find=(node:ts.Node)=>{if(ts.isForOfStatement(node)&&node.expression.getText(tree)==="fresh"&&node.statement.getText(tree).includes("adopter.adoptJavascriptDependencyInstalledTreeV1"))loop??=node;ts.forEachChild(node,find);};find(owner);if(!loop)throw Error("Actual current generation loop required");
 for(const row of productionVectors){
  expect(new Ajv({strict:true}).validate(schema.$defs.ResumeProductionCase,row),row.mode).toBe(true);
  const work:string[]=[],events:string[]=[],stop=new AbortController(),primary=Object.freeze({cancel:"production"}),check=()=>stop.signal.throwIfAborted(),control={signal:stop.signal,publish:async()=>{events.push("publish");},yieldContinuation:async()=>{events.push("yield");}},store={root:resolve("store")},current={scope:".",plan:{generation:"b".repeat(64),directory:resolve("store/j/current"),files:[],members:[]}},frame={built:new Map<string,number>([[".",37]])},policy=JSON.parse(fs.readFileSync(new URL("../../../💾️storage/🧬️schema/📥️install.json",import.meta.url),"utf8"));
  const ports={started:performance.now(),performance,workspace:resolve("."),store,control,fresh:[current],frame,anchors:{},environment:{},args:[],adoptRuntime:{},context:{root:store.root},limits:{maximumOutputBytes:4096,maximumOutputLines:100},installSchema:policy,process:{execPath:"bun",env:{}},checkpoint:check,fence:async()=>{check();events.push("fence");const count=events.filter(event=>event==="fence").length;if(row.mode==="input-drift"&&count===1||row.mode==="source-drift"&&count===2)throw primary;},admitJavascriptDependencyGenerationV1:async()=>{check();events.push("admit");},concurrentFreeBytesV1:async()=>100,prepareJavascriptDependencyStoreEnvironmentV1:async()=>{check();events.push("environment");},adopter:{reuseJavascriptDependencyInstalledGenerationV1:async()=>{events.push("reuse");if(row.canceled){stop.abort(primary);throw primary;}if(row.mode==="verification-error")throw primary;return row.reuse;},adoptJavascriptDependencyInstalledTreeV1:async()=>{work.push("donor");return undefined;}},incremental:{},withRequestedIssuedResourceControlV1:async(_control:unknown,_milliseconds:number,body:(control:unknown)=>Promise<unknown>)=>body(control),captureOwnedCommandResultV1:async()=>{work.push("installer");return{status:0,signal:null};},withArtifactRelativeDirectoryV1:async(_root:string,_check:()=>void,body:(base:unknown)=>Promise<void>)=>body({}),publishJavascriptDependencyExportsDescriptorV1:async()=>{work.push("exports");},requireJavascriptDependencyInstalledPathsIncrementalV1:async()=>{work.push("installed");},adoptSeal:async()=>{work.push("seal");}};
  const run=new Function(...Object.keys(ports),ts.transpileModule("return (async()=>{let purges=0;"+loop.getText(tree)+"})();",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText);
  let error:unknown;try{await run(...Object.values(ports));}catch(failure){error=failure;}
  expect(work,row.mode).toEqual(row.work);
  expect(events,row.mode).toEqual(row.events);expect(frame.built.get(".")).toBe(row.allocated);if(row.error)expect(error).toBe(primary);else expect(error).toBeUndefined();
 }
});
