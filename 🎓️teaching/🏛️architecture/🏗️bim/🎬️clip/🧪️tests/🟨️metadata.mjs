/** 🧪 Source discovery keeps browser runtime and provisioning outside the metadata path. */
import assert from "node:assert/strict";
import fs from "node:fs/promises";
import path from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";
import {createRequire} from "node:module";

const require=createRequire(import.meta.url);
const {parse}=require("@babel/parser");
const here=path.dirname(fileURLToPath(import.meta.url));
const root=process.cwd();
const fixture=JSON.parse(await fs.readFile(path.join(here,"../🧪️cases/🔣️.json"),"utf8")).browser_metadata;
const renderer=path.resolve(root,fixture.renderer);
const source=await fs.readFile(renderer,"utf8");
const imports=parse(source,{sourceType:"module"}).program.body.filter(node=>node.type==="ImportDeclaration");
let body=source;
for(const node of imports.reverse())body=body.slice(0,node.start)+body.slice(node.end);
body=body.replaceAll("import.meta.url",JSON.stringify(pathToFileURL(renderer).href));
const calls=[],writes=new Map(),output=[];
const owned=path.resolve(root,fixture.work_directory),input=path.join(owned,"input.json");
const protocol={schema:fixture.schema,language:"en",catalog:{courses:[]},frames:[],viewport:{width:640,height:360},work_directory:owned};
const adapter=path.resolve(root,fixture.adapter);
const opaque={mkdir:async()=>{},writeFile:async(file,value)=>writes.set(file,value),rm:async()=>{},readFile:async(file)=>{
  if(file===input)return JSON.stringify(protocol);
  if(file.endsWith("🖱️interaction/🔣️.json")||file.endsWith("🖱️interaction"+path.sep+"🔣️.json"))return JSON.stringify({icons:{},cursors:{}});
  return "";
}};
const fakeRequire=name=>{
  if(name==="esbuild")return{build:async options=>{
    const tool=options.entryPoints[0]===adapter;
    calls.push(tool?"adapter-bundle":"ui-bundle");
    return{outputFiles:[{contents:Buffer.from("opaque-adapter"),text:"opaque-ui"}],metafile:{inputs:tool?{[path.relative(root,adapter)]:{}}:{}}};
  }};
  if(name==="@tailwindcss/node")return{compile:async()=>({sources:[],build:()=>""})};
  if(name==="@tailwindcss/oxide")return{Scanner:class{files=[];scan(){return[]}}};
  calls.push("browser-runtime-load");
  throw Error("Metadata source discovery loaded a browser runtime");
};
const processFacade={argv:["node",renderer,"--sources","--input",input],pid:73,cwd:()=>root};
const AsyncFunction=Object.getPrototypeOf(async function(){}).constructor;
await new AsyncFunction("fs","path","fileURLToPath","pathToFileURL","createRequire","launchBrowser","process","console",body)(
  opaque,path,fileURLToPath,pathToFileURL,()=>fakeRequire,async()=>{calls.push("browser-launch");throw Error("Unexpected browser launch")},
  processFacade,{log:value=>output.push(value),error:()=>{}});
assert.deepEqual(calls,fixture.calls);
const result=JSON.parse(output.at(-1));
assert.equal(result.schema,fixture.schema);
assert.ok(result.source_files.includes(adapter),"Adapter dependency closure remains fingerprinted");
assert.equal(writes.size,1,"Only the adapter bundle is written");
console.log("BIM browser metadata: source discovery bundles dependencies without loading or provisioning Playwright.");
