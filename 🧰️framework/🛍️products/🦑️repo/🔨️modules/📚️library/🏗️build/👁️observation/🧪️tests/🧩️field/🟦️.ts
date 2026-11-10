import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import corpus from"./🔣️.json";
import{fieldFactV1}from"../../🧩️field/🟨️.mjs";
import{captureObservedSourceInputsV1,observationsHold,observedFieldV1}from"../../🟨️.mjs";

const artifactRoot=process.env.SEMIO_TEST_ARTIFACT_DIR;
test("field digests equal the Python oracle for every pointer of the corpus",()=>{
 const oracle=spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});
 expect(oracle.status).toBe(0);
 const expected=JSON.parse(oracle.stdout) as Record<string,string>;let compared=0;
 for(const document of corpus.documents)for(const pointer of document.pointers){
  expect(fieldFactV1("/x.json","json",pointer,()=>document.text).digest,document.id+"|"+pointer).toBe(expected[document.id+"|"+pointer]);compared++;
 }
 expect(compared).toBe(Object.keys(expected).length);
});

test("a field row holds while its extracted value is equal and fails the moment it differs",async()=>{
 if(!artifactRoot)throw Error("Original test artifact directory required");
 const root=mkdtempSync(join(artifactRoot,"fd-")),path=join(root,"policy.json");
 try{
  writeFileSync(path,JSON.stringify({presentationResources:["p"],other:1}));
  const captured=await captureObservedSourceInputsV1(async()=>observedFieldV1(path,"json","/presentationResources"),undefined,false);
  expect(captured.value).toEqual(["p"]);
  const row=captured.observations.find(candidate=>candidate.kind==="field")!;
  expect(row.path).toBe(path);expect(captured.observations.some(candidate=>candidate.kind==="file"&&candidate.path===path)).toBe(false);
  expect(await observationsHold([row])).toBe(true);
  writeFileSync(path,JSON.stringify({other:2,presentationResources:["p"]}));
  expect(await observationsHold([row])).toBe(true);
  writeFileSync(path,JSON.stringify({presentationResources:["p","q"],other:2}));
  expect(await observationsHold([row])).toBe(false);
  writeFileSync(path,"{broken");
  expect(await observationsHold([row])).toBe(false);
  rmSync(path);
  expect(await observationsHold([row])).toBe(false);
  await expect(captureObservedSourceInputsV1(async()=>observedFieldV1(path,"json","/presentationResources"),undefined,false)).rejects.toThrow();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a package store file is keyed by the identity of its package, not by its bytes",async()=>{
 if(!artifactRoot)throw Error("Original test artifact directory required");
 const root=mkdtempSync(join(artifactRoot,"fp-")),store=join(root,"node_modules",".bun","lib@1.2.3","node_modules","lib");
 try{
  const{mkdirSync}=await import("node:fs");mkdirSync(join(store,"dist"),{recursive:true});
  writeFileSync(join(store,"package.json"),JSON.stringify({name:"lib",version:"1.2.3"}));writeFileSync(join(store,"dist","main.js"),"a");
  const file=join(store,"dist","main.js"),fact=fieldFactV1(file,"package-identity","",()=>undefined,path=>require("node:fs").readFileSync(path,"utf8")),row={kind:"field",path:file,fact};
  expect(fact.digest.startsWith("!")).toBe(false);
  writeFileSync(file,"changed bytes");expect(await observationsHold([row])).toBe(true);
  writeFileSync(join(store,"package.json"),JSON.stringify({name:"lib",version:"1.2.4"}));expect(await observationsHold([row])).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});
