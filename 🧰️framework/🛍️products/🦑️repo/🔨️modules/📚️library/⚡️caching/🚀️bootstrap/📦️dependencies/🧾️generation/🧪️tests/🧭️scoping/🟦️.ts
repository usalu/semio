import {test,expect} from "bun:test";
import {chmodSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import {captureGenerationReceiptV1,generationProtocolV1 as protocol,generationSha256V1,sealGenerationReceiptV1,verifyGenerationReceiptV1,type GenerationControlV1,type GenerationReceiptV1,type GenerationRuntimeV1} from "../../🟦️.ts";
import {manifestProfileSha256V1} from "../../🪪️profile/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import * as glue from "../../💾️storage/🟦️.ts";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {publishBunWorkspaceOwnership,withCurrentBunWorkspaceOwnershipV1,readBunWorkspaceListingProjectionV1} from "../../../../../../🗂️workspaces/🟦️bun/🟦️.ts";

const runtime:GenerationRuntimeV1={manager:"0.0.0",revision:"fixture",node:"0.0.0",platform:"fixture",arch:"fixture",sha256:"a".repeat(64)};
const oracle=fileURLToPath(new URL("../🐍️.py",import.meta.url)),schemaPath=fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url)),python=()=>testPythonV1()??"python3";
const control=():GenerationControlV1=>({signal:new AbortController().signal,check:()=>{},tick:async()=>{}});
const manifest=(name:string,extra:object={})=>JSON.stringify({name,version:"1.0.0",exports:{".":"./a.js"},scripts:{build:"x"},...extra});
const scopeManifest=(name:string,members:string[])=>manifest(name,{semio:{workspace:{schemaVersion:1,members,owners:["*/package.json"]}}});
const READY="fingerprint";
/** 🌍️ A root scope over `lib`, scope `sa` over its own package and `lib`, scope `sb` over its own package, `lib` and the packages of `sa`. */
function world(){
 const root=realpathSync(mkdtempSync(join(tmpdir(),"gr-scope-"))),store=join(root,".store");
 const put=(path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
 put("package.json",manifest("root",{semio:{workspace:{schemaVersion:1,members:["lib/**"],owners:["*/package.json"]}}}));
 put("lib/p0/package.json",manifest("lib-p0"));put("sa/package.json",scopeManifest("sa",["**","../lib/**"]));put("sa/q0/package.json",manifest("sa-q0"));
 put("sb/package.json",scopeManifest("sb",["**","../lib/**","../sa/**"]));put("sb/q0/package.json",manifest("sb-q0"));
 put("bun.lock","{}");mkdirSync(store,{recursive:true});
 return{root,store,put};
}
const REQUIRED:Record<string,string[]>={".":["."],sa:[".","sa"],sb:[".","sa","sb"]};
const DIRECTORIES=[".","sa","sb"];
const run=(...args:string[])=>{const out=spawnSync(python(),["-I",oracle,...args],{encoding:"utf8"});if(out.status!==0)throw Error("oracle failed: "+out.stderr);return JSON.parse(out.stdout);};
async function receiptOf(root:string,store:string,scope:string,walked:Awaited<ReturnType<typeof walk>>){
 const required=REQUIRED[scope]!,ambient={env:{},home:join(root,".ambient/home"),config:join(root,".ambient/xdg")};
 const sources=(owner:string)=>walked.texts.filter(row=>walked.textOwners[row.path]===owner).map(row=>{const f=lstatSync(join(root,row.path));return{path:join(root,row.path),kind:"file" as const,dev:String(f.dev),ino:String(f.ino),mode:f.mode,bytes:readFileSync(join(root,row.path))};});
 const generation="e".repeat(64),observation="[]",scopes=required.map(name=>({scope:name,sha256:"f".repeat(64)})),receipts=required.map(name=>({scope:name,generation,fingerprint:READY}));
 const input={workspace:root,storeRoot:store,scope,startedAtMs:Date.now()+10000,runtime,producer:glue.observeJavascriptDependencyGenerationProducerV1(root,store,required,ambient),recipeSha256:"c".repeat(64),ownershipSha256:"d".repeat(64),required,
  inputs:DIRECTORIES.map(name=>({scope:name,sources:sources(name)})),
  membership:{root:walked.root,paths:walked.paths,texts:walked.texts.map(row=>({...row,owner:walked.textOwners[row.path]!,profileSha256:manifestProfileSha256V1(row.path,readFileSync(join(root,row.path)))}))},
  bindings:required.map(name=>({scope:name,pairs:[],observation})),ready:required.map(name=>({scope:name,generation,path:join(store,"j/g1/bindings.json"),fingerprint:READY})),
  execution:{scopes,runtimeSha256:runtime.sha256,observations:[]},scopeReadSha256:generationSha256V1(JSON.stringify([receipts,required.map(name=>[name,observation]),scopes])),control:control()};
 const result=await captureGenerationReceiptV1(input);expect(result.status).toBe("captured");if(result.status!=="captured")throw Error("deferred");
 return{receipt:result.receipt,files:new Map(result.files.map(file=>[file.name,file.bytes])),ambient};
}
const walk=async(root:string)=>{await publishBunWorkspaceOwnership(root,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}});return withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}},async custody=>custody.observed());};
const verify=(root:string,store:string,receipt:GenerationReceiptV1,files:Map<string,Uint8Array>,ambient:{env:Record<string,string>;home:string;config:string})=>verifyGenerationReceiptV1({
 workspace:root,storeRoot:store,receipt,readEvidence:name=>{const bytes=files.get(name);if(!bytes)throw Error("Sealed generation evidence required");return bytes;},
 producer:()=>glue.observeJavascriptDependencyGenerationProducerV1(root,store,receipt.required,ambient),runtime:()=>runtime,conditionsClosed:()=>true,
 project:path=>readBunWorkspaceListingProjectionV1(root,path),observeBindings:()=>"[]",readReady:()=>READY,control:control(),
});
const oracleVerify=(root:string,store:string,receipt:GenerationReceiptV1)=>{
 const file=root+".receipt.json";writeFileSync(file,JSON.stringify(receipt));
 try{return run("verify",file,root,store,JSON.stringify(runtime),schemaPath,JSON.stringify({env:{},policy:protocol.producer}))as{status:string;class?:string};}finally{rmSync(file,{force:true});}
};
const outcome=(result:{status:string;class?:string})=>result.status==="match"?"match":"mismatch:"+result.class;

test("ownership walk records the owning scope of every manifest and equals an independent Python implementation",async()=>{
 const{root}=world();
 try{
  const walked=await walk(root),paths=walked.texts.map(row=>row.path);
  expect(paths.sort()).toEqual(["lib/p0/package.json","package.json","sa/package.json","sa/q0/package.json","sb/package.json","sb/q0/package.json"]);
  expect(walked.textOwners).toEqual({"package.json":".","lib/p0/package.json":".","sa/package.json":"sa","sa/q0/package.json":"sa","sb/package.json":"sb","sb/q0/package.json":"sb"});
  expect(run("owners",JSON.stringify(DIRECTORIES),JSON.stringify(paths))).toEqual(walked.textOwners);
  expect(run("owners",JSON.stringify([".","a","a/b"]),JSON.stringify(["x/package.json","a/package.json","a/b/package.json","a/b/c/package.json","a/c/package.json","ab/package.json"]))).toEqual({"x/package.json":".","a/package.json":"a","a/b/package.json":"a/b","a/b/c/package.json":"a/b","a/c/package.json":"a","ab/package.json":"."});
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("each scope receipt keys manifests and inputs to its required scopes only, schema valid, equal to the Python selection",async()=>{
 const{root,store}=world();
 try{
  const walked=await walk(root),ajv=new Ajv({strict:false}).addSchema(schema),validate=ajv.getSchema(schema.$id+"#/$defs/Receipt")!;
  const expected:Record<string,string[]>={".":["lib/p0/package.json","package.json"],sa:["lib/p0/package.json","package.json","sa/package.json","sa/q0/package.json"],sb:["lib/p0/package.json","package.json","sa/package.json","sa/q0/package.json","sb/package.json","sb/q0/package.json"]};
  for(const scope of Object.keys(REQUIRED)){
   const{receipt}=await receiptOf(root,store,scope,walked);
   expect(validate(JSON.parse(JSON.stringify(receipt))),JSON.stringify(validate.errors)).toBe(true);
   const texts=receipt.membership.texts.map(row=>row.path).sort();
   expect(texts,scope).toEqual(expected[scope]);
   expect(run("scoped",JSON.stringify(walked.texts.map(row=>({path:row.path,owner:walked.textOwners[row.path]}))),JSON.stringify(REQUIRED[scope])).sort(),scope).toEqual(texts);
   expect(receipt.inputs.map(group=>group.scope).sort(),scope).toEqual([...REQUIRED[scope]!].sort());
   expect(receipt.walkStamp).toBe("semio.dependency-generation-walk/4");
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});

type Edit={name:string;path:string;change:(text:string)=>string;stale:string[];class:string};
const exportsEdit=(text:string)=>JSON.stringify({...JSON.parse(text),exports:{".":"./b.js"}});
const scriptsEdit=(text:string)=>JSON.stringify({...JSON.parse(text),scripts:{build:"y"}});
const versionEdit=(text:string)=>JSON.stringify({...JSON.parse(text),version:"2.0.0"});
const EDITS:Edit[]=[
 {name:"relevant edit of a shared root-scope manifest stales every scope",path:"lib/p0/package.json",change:exportsEdit,stale:[".","sa","sb"],class:"input-changed"},
 {name:"relevant edit in scope sa stales sa and the scope that requires sa, not the root",path:"sa/q0/package.json",change:exportsEdit,stale:["sa","sb"],class:"input-changed"},
 {name:"relevant edit in scope sb stales only sb",path:"sb/q0/package.json",change:versionEdit,stale:["sb"],class:"input-changed"},
 {name:"inert edit of any manifest stales nothing",path:"sa/q0/package.json",change:scriptsEdit,stale:[],class:"input-changed"},
 {name:"edit of the scope manifest of sb stales only sb",path:"sb/package.json",change:exportsEdit,stale:["sb"],class:"input-changed"},
];
for(const edit of EDITS)test("scope law: "+edit.name+" (TS equals Python)",async()=>{
 const{root,store,put}=world();
 try{
  const walked=await walk(root),receipts=new Map<string,Awaited<ReturnType<typeof receiptOf>>>();
  for(const scope of Object.keys(REQUIRED))receipts.set(scope,await receiptOf(root,store,scope,walked));
  put(edit.path,edit.change(readFileSync(join(root,edit.path),"utf8")));
  for(const scope of Object.keys(REQUIRED)){
   const row=receipts.get(scope)!,verdict=await verify(root,store,row.receipt,row.files,row.ambient),python=oracleVerify(root,store,row.receipt),stale=edit.stale.includes(scope);
   expect(outcome(verdict),scope).toBe(stale?"mismatch:"+edit.class:"match");
   expect(outcome(python),scope+" oracle").toBe(outcome(verdict));
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a manifest edit outside every required scope never touches the frontier: stat drift of a foreign manifest keeps the receipt matching",async()=>{
 const{root,store,put}=world();
 try{
  const walked=await walk(root),row=await receiptOf(root,store,".",walked);
  expect(row.receipt.membership.texts.some(text=>text.path.startsWith("sa/"))).toBe(false);
  put("sa/q0/package.json",exportsEdit(readFileSync(join(root,"sa/q0/package.json"),"utf8")));chmodSync(join(root,"sa/q0/package.json"),0o600);
  expect(outcome(await verify(root,store,row.receipt,row.files,row.ambient))).toBe("match");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a receipt that names a manifest or an input of a scope it does not require is receipt-invalid (TS equals Python)",async()=>{
 const{root,store}=world();
 try{
  const walked=await walk(root),row=await receiptOf(root,store,".",walked),{sha256:_,...body}=row.receipt;
  const foreignText=sealGenerationReceiptV1({...body,membership:{...body.membership,texts:[...body.membership.texts,{path:"sa/package.json",owner:"sa",sha256:"1".repeat(64),profileSha256:null}]}});
  const foreignInput=sealGenerationReceiptV1({...body,inputs:[...body.inputs,{scope:"sa",sources:[]}]});
  for(const receipt of [foreignText,foreignInput]){
   expect(outcome(await verify(root,store,receipt,row.files,row.ambient))).toBe("mismatch:receipt-invalid");
   expect(outcome(oracleVerify(root,store,receipt))).toBe("mismatch:receipt-invalid");
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});
