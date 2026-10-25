import{afterAll,expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,utimesSync,writeFileSync,statSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const api=await import(new URL("../../🟨️.mjs",import.meta.url).href) as typeof import("../../🟨️.mjs");
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

const revision=createHash("sha256").update("analysis-stamp-law").digest("hex");
let analyses=0;
const text=api.registerSourceAnalysisV1({id:"test:text:v1",revision,analyze:(_path:string,bytes:Uint8Array)=>({text:Buffer.from(bytes).toString("utf8")})});
const provider=api.registerSourceAnalysisV1({id:"test:line-count:v1",revision,analyze:(_path:string,bytes:Uint8Array)=>{analyses++;return{lines:Buffer.from(bytes).toString("utf8").split("\n").length};}});

/** 🧪️ One source file, analysed once inside a capture; returns the recorded analysis row. */
async function recorded(content:string,using=provider){
 const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(parent,{recursive:true});
 const root=mkdtempSync(join(realpathSync(parent),"analysis-stamp-"));roots.push(root);
 const path=join(root,"source.txt");writeFileSync(path,content);
 const captured=await api.captureObservedSourceInputsV1(async()=>api.observedSourceAnalysisV1(using,path),undefined,false);
 const row=captured.observations.find((row:{kind:string})=>row.kind==="analysis") as {kind:string;path:string;fact:unknown};
 return{path,row};
}

test("an analysis whose source keeps its size, inode and stamps holds without the file being read again",async()=>{
 const{path,row}=await recorded("one\ntwo\n");
 expect(await api.observationsHold([row],1,new Set(),new Set())).toBe(true);
 const before=analyses;
 expect(await api.observationsHold([row],1,new Set(),new Set())).toBe(true);
 expect(analyses).toBe(before);
 expect(statSync(path).size).toBe(8);
});

test("an edit that changes the result is stale, one that keeps the result replays to a hold",async()=>{
 const{path,row}=await recorded("one\ntwo\n");
 writeFileSync(path,"one\ntwo\nthree\n");
 expect(await api.observationsHold([row],1,new Set(),new Set())).toBe(false);
 const same=await recorded("one\ntwo\n");
 writeFileSync(same.path,"uno\ndos\n");
 expect(await api.observationsHold([same.row],1,new Set(),new Set())).toBe(true);
});

test("a rewrite of the same size with the old modification time still moves the change time, so the bytes are read and the edit is found",async()=>{
 const{path,row}=await recorded("aaaa\nbbbb\n",text);
 const stamp=statSync(path);writeFileSync(path,"cccc\ndddd\n");utimesSync(path,stamp.atime,stamp.mtime);
 expect(await api.observationsHold([row],1,new Set(),new Set())).toBe(false);
});
