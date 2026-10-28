/** 🧳️ Contract: the closures of the shared command inputs, which a prepared project record asks about, are carried through the preparation of a refresh and dropped by its final release. A release between the preparation and the build keeps them, so the build answers the record from them without walking the command sources again; an ordinary release drops them and the record is answered by a new walk; an edit inside a carried closure is noticed, never answered from the carried entry. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{generationHostV1}from"../../🟦️.ts";

const inference=await import("../../📥️inference/🟨️.mjs") as any,{createScriptInputCache,libraryBootstrap}=inference;await libraryBootstrap;
const observation=await import("../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../📥️inference/🧭️script-inputs/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const sleep=(ms:number)=>new Promise<void>(resolve=>setTimeout(resolve,ms));
const write=(root:string,files:Record<string,string>)=>{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};

function world(){
 const root=realpathSync(mkdtempSync(join(tmpdir(),"carried-closures-")));roots.push(root);
 write(root,{"tools/a.mjs":'import "./b.mjs";\n',"tools/b.mjs":"export const b=1;\n","tools/c.mjs":"export const c=1;\n"});
 const walked:string[]=[],cachePath=join(root,"np","project.json"),entry=join(root,"tools/a.mjs");
 const fresh=()=>{const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),add=scripts.modules.add.bind(scripts.modules);scripts.modules.add=(path:string)=>{walked.push(path);return add(path);};return scriptInputs.closureApiV1(scripts,root);};
 const record=()=>{const api=fresh();return observation.observeInferenceSnapshotV1(()=>observation.observeProjectInferenceV1({cachePath,key:"project",owner:"project"},async()=>{
  const list=await api.closureOf([entry],[],"test-discovery"),answer=await api.some(list,["some",["c.mjs"]]);
  return{inputs:[...list],answer,...api.table([...list].map((item:any)=>item.closureRef))};
 })) as Promise<{answer:boolean}>;};
 const signal=new AbortController().signal,control={signal,remainingMilliseconds:()=>null,publish:async()=>{},yieldContinuation:async()=>{}},invocation={policy:{version:1,owner:"carried-closures-law",maximumElapsedMilliseconds:0},control,capabilities:{}};
 const host=generationHostV1(root,invocation as never,control as never,{catalogProviders:[],owners:[]} as never);
 return{root,walked,record,host:host as unknown as {release():Promise<void>;carryClosures(on:boolean):void}};
}

test("a release between the preparation and the build keeps the closures: the prepared record is answered without a new walk",async()=>{
 const w=world();
 expect((await w.record()).answer).toBe(false);expect(w.walked.length).toBeGreaterThan(0);
 w.host.carryClosures(true);await w.host.release();w.host.carryClosures(false);
 w.walked.length=0;
 expect((await w.record()).answer).toBe(false);
 expect(w.walked).toEqual([]);
});

test("an ordinary release drops the closures: the same record needs a new walk of the command sources",async()=>{
 const w=world();
 await w.record();
 await w.host.release();
 w.walked.length=0;
 expect((await w.record()).answer).toBe(false);
 expect(w.walked.length).toBeGreaterThan(0);
});

test("the final release after a carried one drops them, and an edit inside a carried closure is noticed",async()=>{
 const w=world();
 await w.record();
 w.host.carryClosures(true);await w.host.release();w.host.carryClosures(false);
 await sleep(30);write(w.root,{"tools/b.mjs":'import "./c.mjs";\nexport const b=2;\n'});
 w.walked.length=0;
 expect((await w.record()).answer).toBe(true);
 expect(w.walked.length).toBeGreaterThan(0);
 await w.host.release();w.walked.length=0;
 expect((await w.record()).answer).toBe(true);
 expect(w.walked.length).toBeGreaterThan(0);
});
