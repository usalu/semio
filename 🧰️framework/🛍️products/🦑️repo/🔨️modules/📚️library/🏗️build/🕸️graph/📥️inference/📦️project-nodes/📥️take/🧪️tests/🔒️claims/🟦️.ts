import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdtempSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";
import {createOwnerClaimsV1,takeClaimedV1} from "../../🟨️.mjs";

test("the claim states of two processes on one directory equal the independent model",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"🔣️.json")],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const directory=mkdtempSync(join(tmpdir(),"owner-claims-"));let clock=0;
 try{
  const table=new Map<string,ReturnType<typeof createOwnerClaimsV1>>(),alive=(pid:number)=>(fixture.pids as Record<string,boolean>)[String(pid)]===true,answers:string[]=[];
  for(const step of fixture.steps){
   clock=step.at;
   if(!table.has(step.who))table.set(step.who,createOwnerClaimsV1({directory,pid:step.pid,now:()=>clock,alive,staleMilliseconds:fixture.staleMilliseconds}));
   const claims=table.get(step.who)!;
   if(step.op==="release")claims.release(step.stem);else answers.push(claims.claim(step.stem));
  }
  expect(answers).toEqual(JSON.parse(run.stdout));
  expect(answers).toContain("running");expect(answers).toContain("granted");
  expect(readdirSync(directory).filter(name=>name.endsWith(".stale")).length).toBe(0);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a reader that finds the owner claimed waits for the record and never infers it, and infers once the claim is released or stale",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"owner-claims-"));let clock=0;
 try{
  const alive=()=>true,leader=createOwnerClaimsV1({directory,pid:11,now:()=>clock,alive,staleMilliseconds:1000}),follower=createOwnerClaimsV1({directory,pid:22,now:()=>clock,alive,staleMilliseconds:1000});
  const mine=await takeClaimedV1({name:"o",stem:"x",claims:leader,look:async()=>undefined,sleep:async()=>{}});expect(mine.release).toBeDefined();
  let looks=0;const taken=await takeClaimedV1({name:"o",stem:"x",claims:follower,look:async()=>{looks++;return looks===3?"fetched":undefined;},sleep:async()=>{clock+=10;}});
  expect(taken).toEqual({outcome:"fetched"});expect(looks).toBe(3);
  let waits=0;const later=await takeClaimedV1({name:"o",stem:"x",claims:follower,look:async()=>undefined,sleep:async()=>{waits++;clock+=400;}});
  expect(later.release).toBeDefined();expect(waits).toBeGreaterThanOrEqual(2);expect(waits).toBeLessThanOrEqual(4);
  later.release!();expect(existsSync(join(directory,"x.json"))).toBe(false);
  mine.release!();
  writeFileSync(join(directory,"bad.json"),"{");
  expect((await takeClaimedV1({name:"b",stem:"bad",claims:follower,look:async()=>undefined,sleep:async()=>{}})).release).toBeDefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
