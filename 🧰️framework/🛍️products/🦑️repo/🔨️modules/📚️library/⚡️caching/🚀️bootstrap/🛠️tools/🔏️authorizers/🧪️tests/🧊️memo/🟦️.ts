/** 🧊️ Tests for tools authorizers memo. */
import{test,expect}from"bun:test";
import{mkdtemp,mkdir,writeFile,rm,rename,link,truncate,utimes,readFile,copyFile}from"node:fs/promises";
import{readFileSync}from"node:fs";
import{dirname,join,resolve}from"node:path";
import ts from"typescript";
import{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedImportsV1,nxAuthorizerCapturedBytesV1}from"../../🟦️.ts";
import{NX_AUTHORIZER_MEMO_IMPLEMENTATION_FILES_V1,nxAuthorizerMemoBaseV1,nxAuthorizerMemoFileV1,readNxAuthorizerMemoSwitchV1}from"../../🧊️memo/🟦️.ts";
import{withBuildTestInvocationV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{withAuthorizerFixtureArtifactsV1}from"../../🧪️testing/📁️artifacts/🟦️.ts";
import{privateRepositoryWorkspaceV1}from"../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1();

const tree:Readonly<Record<string,string>>={
 "package.json":'{"name":"memo-fixture","private":true}',
 "entry.ts":'import{a}from"./a.ts";import data from"./data.json";import{pkg}from"memo-pkg";export const result=[a,data,pkg];',
 "a.ts":'import{b}from"./sub/b.ts";const optional=require("./optional.js");export const a=[b,optional];',
 "sub/b.ts":'export const b=1;',
 "data.json":'{"data":true}',
 "node_modules/memo-pkg/package.json":'{"name":"memo-pkg","version":"1.0.0","main":"index.js"}',
 "node_modules/memo-pkg/index.js":'exports.pkg=1;',
};
const mutations:readonly Readonly<{name:string;apply:(area:string)=>Promise<void>;served:"memo"|"capture";rejects?:boolean}>[]=[
 {name:"unrelated sibling file created in a closure directory",served:"memo",apply:async area=>writeFile(join(area,"sub","notes.txt"),"unrelated")},
 {name:"mtime changed, bytes identical",served:"memo",apply:async area=>utimes(join(area,"sub","b.ts"),new Date(1e12),new Date(1e12))},
 {name:"closure file edited",served:"capture",apply:async area=>writeFile(join(area,"sub","b.ts"),"export const b=2;")},
 {name:"data directory created beside a code file",served:"capture",apply:async area=>{await mkdir(join(area,"sub","🧬️schema"));await writeFile(join(area,"sub","🧬️schema","policy.json"),'{"policy":1}');}},
 {name:"file added to an existing data directory",served:"capture",apply:async area=>writeFile(join(area,"sub","🧬️schema","more.json"),'{"more":1}')},
 {name:"remembered lenient absence filled",served:"capture",apply:async area=>writeFile(join(area,"optional.js"),"module.exports=1;")},
 {name:"installed package version moved",served:"capture",apply:async area=>writeFile(join(area,"node_modules","memo-pkg","package.json"),'{"name":"memo-pkg","version":"1.1.0","main":"index.js"}')},
 {name:"closure directory replaced by an identical copy",served:"capture",apply:async area=>{await rename(join(area,"sub"),join(area,"sub.old"));await mkdir(join(area,"sub"));for(const name of["b.ts"])await writeFile(join(area,"sub",name),await readFile(join(area,"sub.old",name)));await mkdir(join(area,"sub","🧬️schema"));await writeFile(join(area,"sub","🧬️schema","policy.json"),'{"policy":1}');await writeFile(join(area,"sub","🧬️schema","more.json"),'{"more":1}');}},
 {name:"closure file hard linked",served:"capture",rejects:true,apply:async area=>link(join(area,"sub","b.ts"),join(area,"sub","b-link.ts"))},
];

async function plainV1(area:string,entry:string,installed:boolean){return captureNxAuthorizerSourcesV1(area,entry,new AbortController().signal,{installed});}

test("the source memo answers exactly what a full capture answers after every kind of edit",()=>withBuildTestInvocationV1(async original=>withAuthorizerFixtureArtifactsV1(original,createIssuedResourceControlV1(original,"test:source-memo-artifacts",0),async artifacts=>{
 const area=await mkdtemp(join(artifacts,"sm-")),base=await mkdtemp(join(artifacts,"sb-"));
 let failed=false,primary:unknown;
 try{
  for(const[name,text]of Object.entries(tree)){await mkdir(join(area,name,".."),{recursive:true});await writeFile(join(area,name),text);}
  const file=nxAuthorizerMemoFileV1(base,area,"entry.ts",true),served:string[]=[],run=async(entry="entry.ts",installed=true,signal=new AbortController().signal)=>captureNxAuthorizerSourcesV1(area,entry,signal,{installed,memo:{file:nxAuthorizerMemoFileV1(base,area,entry,installed),served:value=>served.push(value)}});
  const cold=await run(),warm=await run(),plain=await plainV1(area,"entry.ts",true);
  expect(served).toEqual(["capture","memo"]);
  expect(warm.digest).toBe(cold.digest);
  expect(warm.digest).toBe(plain.digest);
  const closure=ts.preProcessFile(tree["entry.ts"]!,true,true).importedFiles.map(row=>row.fileName).filter(name=>name.startsWith("./"));
  for(const name of closure)expect(warm.manifest.sources.some(row=>row.path===name.slice(2)),name).toBe(true);
  for(const row of warm.manifest.sources)expect(row.sha256,row.path).toBe(Buffer.from(await crypto.subtle.digest("SHA-256",await readFile(join(area,row.path)))).toString("hex"));
  expect(nxAuthorizerCapturedImportsV1(area,warm)).toEqual(nxAuthorizerCapturedImportsV1(area,plain));
  expect(Object.fromEntries(nxAuthorizerCapturedBytesV1(warm))).toEqual(Object.fromEntries(nxAuthorizerCapturedBytesV1(plain)));
  for(const mutation of mutations){
   served.length=0;await mutation.apply(area);
   if(mutation.rejects){await expect(plainV1(area,"entry.ts",true),mutation.name).rejects.toThrow();await expect(run(),mutation.name).rejects.toThrow();await rm(join(area,"sub","b-link.ts"));continue;}
   const first=await run(),oracle=await plainV1(area,"entry.ts",true),second=await run();
   expect(served[0],mutation.name).toBe(mutation.served);
   expect(first.digest,mutation.name).toBe(oracle.digest);
   expect(second.digest,mutation.name).toBe(oracle.digest);
   expect(served[1],mutation.name).toBe("memo");
   expect(nxAuthorizerCapturedImportsV1(area,second),mutation.name).toEqual(nxAuthorizerCapturedImportsV1(area,oracle));
  }
  served.length=0;await writeFile(file,'{"version":1,"truncated"');
  expect((await run()).digest).toBe((await plainV1(area,"entry.ts",true)).digest);
  expect(served).toEqual(["capture"]);
  served.length=0;await truncate(file,0);await run();await run();
  expect(served).toEqual(["capture","memo"]);
  const stored=JSON.parse(await readFile(file,"utf8"));
  for(const[field,value]of[["implementation","0".repeat(64)],["root",area+"-other"]]as const){
   served.length=0;await writeFile(file,JSON.stringify({...stored,[field]:value}));
   expect((await run()).digest,field).toBe((await plainV1(area,"entry.ts",true)).digest);
   expect(served,field).toEqual(["capture"]);
  }
  served.length=0;const aborted=new AbortController();aborted.abort(Error("cancelled by the caller"));
  await expect(run("entry.ts",true,aborted.signal)).rejects.toThrow("cancelled by the caller");
  served.length=0;await run("a.ts",false);await run("a.ts",false);
  expect(served).toEqual(["capture","memo"]);
  served.length=0;await run("a.ts",true);
  expect(served).toEqual(["capture"]);
  for(const[source,entry,installed]of[["entry.ts","a.ts",true],["a.ts","a.ts",false]]as const){
   const target=nxAuthorizerMemoFileV1(base,area,entry,installed);served.length=0;await rm(target,{force:true});await copyFile(nxAuthorizerMemoFileV1(base,area,source,true),target);
   expect((await run(entry,installed)).digest,entry).toBe((await plainV1(area,entry,installed)).digest);
   expect(served,entry+installed).toEqual(["capture"]);
  }
 }catch(error){failed=true;primary=error;}
 finally{for(const path of[area,base])try{await rm(path,{recursive:true,force:true});}catch(error){if(!failed){failed=true;primary=error;}}}
 if(failed)throw primary;
})),120000);

test("the memo switch and the cache location are closed and per platform",()=>{
 expect(readNxAuthorizerMemoSwitchV1({})).toBe(true);
 expect(readNxAuthorizerMemoSwitchV1({SEMIO_REPO_SOURCE_MEMO:"off"})).toBe(false);
 expect(readNxAuthorizerMemoSwitchV1({SEMIO_REPO_SOURCE_MEMO:"ON"})).toBe(true);
 expect(()=>readNxAuthorizerMemoSwitchV1({SEMIO_REPO_SOURCE_MEMO:"maybe"})).toThrow(/Expected SEMIO_REPO_SOURCE_MEMO=on\|off.*Erwartet SEMIO_REPO_SOURCE_MEMO=on\|off/su);
 expect(nxAuthorizerMemoBaseV1({},"darwin","/home/u")).toBe("/home/u/Library/Caches/semio-repo");
 expect(nxAuthorizerMemoBaseV1({XDG_CACHE_HOME:"/xdg"},"linux","/home/u")).toBe("/xdg/semio-repo");
 expect(nxAuthorizerMemoBaseV1({},"linux","/home/u")).toBe("/home/u/.cache/semio-repo");
 expect(nxAuthorizerMemoBaseV1({XDG_CACHE_HOME:""},"linux","/home/u")).toBe("/home/u/.cache/semio-repo");
 expect(nxAuthorizerMemoBaseV1({XDG_CACHE_HOME:"relative/cache"},"linux","/home/u")).toBe("/home/u/.cache/semio-repo");
 expect(nxAuthorizerMemoBaseV1({LOCALAPPDATA:"C:\\L"},"win32","C:\\U").replaceAll("\\","/")).toBe("C:/L/semio-repo/cache");
 expect(nxAuthorizerMemoFileV1("/b","/w","e.ts",true)).not.toBe(nxAuthorizerMemoFileV1("/b","/w","e.ts",false));
 expect(nxAuthorizerMemoFileV1("/b","/w","e.ts",true)).not.toBe(nxAuthorizerMemoFileV1("/b","/v","e.ts",true));
});

test("the memo implementation digest covers every local module and JSON policy the capture modules import",()=>{
 const base=resolve(import.meta.dir,"../../🧊️memo"),covered=new Set(NX_AUTHORIZER_MEMO_IMPLEMENTATION_FILES_V1.map(name=>resolve(base,name))),missing:string[]=[];
 for(const file of covered){
  const source=readFileSync(file,"utf8");
  for(const match of source.matchAll(/(?:^|\n|;)\s*import[^"'\n]*?from\s*["'](\.{1,2}\/[^"']+\.(?:ts|json|mjs))["']/gu)){
   const target=resolve(dirname(file),match[1]!);
   if(!target.includes("🔏️authorizers"))continue;
   if(!covered.has(target))missing.push(target.slice(target.indexOf("🔏️authorizers")));
  }
 }
 expect(missing).toEqual([]);
});
