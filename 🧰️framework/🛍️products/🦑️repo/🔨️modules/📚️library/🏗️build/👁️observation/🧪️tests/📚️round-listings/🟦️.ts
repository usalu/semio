import{afterAll,expect,test}from"bun:test";
import fs,{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{createHash}from"node:crypto";
import{directoryFactOfV1,withObservationRoundV1,withObservedWorldV1}from"../../🟨️.mjs";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const write=(base:string,path:string)=>{mkdirSync(dirname(join(base,path)),{recursive:true});writeFileSync(join(base,path),"x\n");};
const sha=(names:readonly string[])=>createHash("sha256").update(JSON.stringify([...names].sort())).digest("hex");

/** 🔁️ A tree whose top level lists three directories: one with a file, two that hold nothing a repository can hold, so every listing of the top level walks below them. */
function tree():string{
 const base=mkdtempSync(join(tmpdir(),"round-listings-"));scratch.push(base);
 write(base,"live/a.ts");mkdirSync(join(base,"ghost/inner/deeper"),{recursive:true});mkdirSync(join(base,"husk/one"),{recursive:true});
 return base;
}
function counted(){const reads=new Map<string,number>();const port=new Proxy(fs,{get:(target,key)=>key==="readdirSync"?(path:string,...rest:unknown[])=>{reads.set(path,(reads.get(path)??0)+1);return(target.readdirSync as Function)(path,...rest);}:(target as never)[key as never]});return{reads,port};}

test("inside one round a directory is listed once however many facts walk below it, and the fact is the independent digest of the tracked names",()=>{
 const base=tree(),{reads,port}=counted();
 const facts=withObservedWorldV1(port as never,()=>withObservationRoundV1(()=>[directoryFactOfV1("readdir",base),directoryFactOfV1("readdir",join(base,"ghost")),directoryFactOfV1("readdir",join(base,"ghost","inner")),directoryFactOfV1("readdir",base)]));
 expect(directoryFactOfV1("readdirNames",base)).toBe(sha(["live"]));
 expect(facts[0]).toBe(facts[3]);
 expect(Math.max(...reads.values())).toBe(1);
});

test("outside a round the listing is read afresh every time, and a new round sees what changed since the previous one",()=>{
 const base=tree(),{reads,port}=counted();
 withObservedWorldV1(port as never,()=>{directoryFactOfV1("readdir",base);directoryFactOfV1("readdir",base);});
 expect(reads.get(join(base,"ghost"))).toBe(2);
 const round=()=>withObservedWorldV1(port as never,()=>withObservationRoundV1(()=>directoryFactOfV1("readdir",base)));
 const before=round();write(base,"ghost/inner/deeper/file.ts");
 expect(round()).not.toBe(before);
});

test("a listing that fails inside a round fails the same way for every later reader of the round",()=>{
 const base=tree(),{reads,port}=counted();
 const codes=withObservedWorldV1(port as never,()=>withObservationRoundV1(()=>[directoryFactOfV1("readdir",join(base,"absent")),directoryFactOfV1("readdir",join(base,"absent"))]));
 expect(codes).toEqual(["!ENOENT","!ENOENT"]);
 expect(reads.get(join(base,"absent"))).toBe(1);
});
