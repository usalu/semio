/** ⏭️ Contract: progress of the dependency binding publication survives attempts. A binding whose endpoint is already the canonical link to its issued target and exactly the observed row is resumed by the endpoint fence alone, without a descriptor session, so a restarted attempt costs milliseconds per finished binding and pays only for the bindings that still differ; a link that changed since the observation is never resumed. */
import{expect,test}from"bun:test";
import{lstatSync,mkdirSync,mkdtempSync,readlinkSync,rmSync,symlinkSync,unlinkSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{javascriptDependencyProfileTopV1,javascriptDependencyStampV1,javascriptDependencyStrictAncestorsV1,publishJavascriptDependencyPreservingSourceDescriptorV1,resumeJavascriptDependencyPublishedBindingV1}from"../../🟦️.ts";

type Row={sourceDirectory:string;kind:"link"|"missing";stamp:string|null;target:string|null};
const stat=(path:string)=>lstatSync(path,{bigint:true});
const anchor=(path:string)=>{const row=stat(path);return JSON.stringify([row.dev,row.ino,row.mode].map(String));};
const fixture=(count:number,linked:(index:number)=>boolean)=>{
 const root=mkdtempSync(join(tmpdir(),"semio-resume-")),parent=join(root,"ws","node_modules"),store=join(root,"store");mkdirSync(parent,{recursive:true});mkdirSync(store);
 const bindings=Array.from({length:count},(_,index)=>({sourceDirectory:join(parent,"p"+index),targetDirectory:join(store,"t"+index)}));
 for(const [index,binding] of bindings.entries()){mkdirSync(binding.targetDirectory);if(linked(index))symlinkSync(binding.targetDirectory,binding.sourceDirectory,"dir");}
 return{root,parent,bindings};
};
const observe=(parent:string,bindings:readonly{sourceDirectory:string;targetDirectory:string}[])=>{
 const expected=new Map<string,Row>(),parents=new Map<string,string>([[parent,javascriptDependencyStampV1(stat(parent))]]),ancestors=new Map<string,string>();
 for(const binding of bindings){const row=lstatSync(binding.sourceDirectory,{bigint:true,throwIfNoEntry:false});expected.set(binding.sourceDirectory,row?{sourceDirectory:binding.sourceDirectory,kind:"link",stamp:javascriptDependencyStampV1(row),target:readlinkSync(binding.sourceDirectory)}:{sourceDirectory:binding.sourceDirectory,kind:"missing",stamp:null,target:null});}
 for(const path of javascriptDependencyStrictAncestorsV1(parent))ancestors.set(path,anchor(path));ancestors.set(parent,anchor(parent));
 return{expected,parents,ancestors};
};
const pass=async(parent:string,bindings:readonly{sourceDirectory:string;targetDirectory:string}[],stopAt:number=Infinity)=>{
 const {expected,parents,ancestors}=observe(parent,bindings),counts={resumed:0,published:0,changed:0};let index=0;
 for(const binding of bindings){
  if(index++===stopAt)throw Error("interrupted");
  const previous=expected.get(binding.sourceDirectory)!,resumed=resumeJavascriptDependencyPublishedBindingV1(binding,previous,parents.get(parent)!,expected,parents,ancestors);
  if(resumed){counts.resumed++;continue;}
  const source={sha256:"s",rows:[...expected.values()],parents:[{path:parent,identity:parents.get(parent)!}],ancestors:[...ancestors].map(([path,identity])=>({path,identity}))} as never;
  const result=await publishJavascriptDependencyPreservingSourceDescriptorV1(binding,previous as never,source,{sourceSha256:"s",generation:"g",readySha256:"r",inputSha256:"i",recipeSha256:"c",fundingSha256:"f"},()=>{});
  counts.published++;if(result.changed)counts.changed++;
  expected.set(binding.sourceDirectory,{sourceDirectory:binding.sourceDirectory,kind:"link",stamp:javascriptDependencyStampV1(result.binding.identity!),target:result.binding.target!});
 }
 return counts;
};

test("a finished binding is resumed by the fence, an unfinished or changed one is not",async()=>{
 const {root,parent,bindings}=fixture(6,index=>index<3);
 try{
  const {expected,parents,ancestors}=observe(parent,bindings);
  const verdicts=bindings.map(binding=>resumeJavascriptDependencyPublishedBindingV1(binding,expected.get(binding.sourceDirectory)!,parents.get(parent)!,expected,parents,ancestors)!==undefined);
  expect(verdicts).toEqual([true,true,true,false,false,false]);
  const first=bindings[0]!;unlinkSync(first.sourceDirectory);symlinkSync(bindings[1]!.targetDirectory,first.sourceDirectory,"dir");
  expect(resumeJavascriptDependencyPublishedBindingV1(first,expected.get(first.sourceDirectory)!,parents.get(parent)!,expected,parents,ancestors)).toBeUndefined();
  expect(resumeJavascriptDependencyPublishedBindingV1(bindings[1]!,expected.get(bindings[1]!.sourceDirectory)!,parents.get(parent)!,expected,parents,ancestors)).toBeDefined();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("an interrupted run resumes where it stopped and a changed link invalidates only itself",async()=>{
 const {root,parent,bindings}=fixture(48,()=>false);
 try{
  await expect(pass(parent,bindings,20)).rejects.toThrow("interrupted");
  expect(readlinkSync(bindings[19]!.sourceDirectory)).toBe(bindings[19]!.targetDirectory);expect(lstatSync(bindings[20]!.sourceDirectory,{throwIfNoEntry:false})).toBeUndefined();
  expect(await pass(parent,bindings)).toEqual({resumed:20,published:28,changed:28});
  unlinkSync(bindings[7]!.sourceDirectory);symlinkSync(bindings[8]!.targetDirectory,bindings[7]!.sourceDirectory,"dir");
  expect(await pass(parent,bindings)).toEqual({resumed:47,published:1,changed:1});
  expect(await pass(parent,bindings)).toEqual({resumed:48,published:0,changed:0});
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("resuming a finished binding is at least ten times cheaper than publishing it again",async()=>{
 const {root,parent,bindings}=fixture(120,()=>true);
 try{
  const {expected,parents,ancestors}=observe(parent,bindings);
  const start=performance.now();for(const binding of bindings)expect(resumeJavascriptDependencyPublishedBindingV1(binding,expected.get(binding.sourceDirectory)!,parents.get(parent)!,expected,parents,ancestors)).toBeDefined();
  const resume=performance.now()-start,{expected:again,parents:more,ancestors:chain}=observe(parent,bindings),before=performance.now();
  for(const binding of bindings){const source={sha256:"s",rows:[...again.values()],parents:[{path:parent,identity:more.get(parent)!}],ancestors:[...chain].map(([path,identity])=>({path,identity}))} as never;
   expect((await publishJavascriptDependencyPreservingSourceDescriptorV1(binding,again.get(binding.sourceDirectory) as never,source,{sourceSha256:"s",generation:"g",readySha256:"r",inputSha256:"i",recipeSha256:"c",fundingSha256:"f"},()=>{})).changed).toBe(false);}
  const publish=performance.now()-before;
  expect(resume*10).toBeLessThan(publish);
  expect(dirname(bindings[0]!.sourceDirectory)).toBe(parent);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the profile keeps the costliest native symbols so the bounded log line never cuts them off",()=>{
 const rows={a:[5,0.1],b:[1,90],c:[9,7],d:[1,7],e:[2,3],f:[2,2],g:[1,1],h:[1,0.5]} as Record<string,readonly[number,number]>;
 expect(Object.keys(javascriptDependencyProfileTopV1(rows))).toEqual(["b","c","d","e","f","g"]);
 expect(Object.keys(javascriptDependencyProfileTopV1(rows,2))).toEqual(["b","c"]);
});
