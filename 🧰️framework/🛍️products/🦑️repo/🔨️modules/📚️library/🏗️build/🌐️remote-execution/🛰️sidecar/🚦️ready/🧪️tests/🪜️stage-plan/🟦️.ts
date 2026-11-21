import {describe,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {Database} from "bun:sqlite";
import dependencyInputs from "./🔣️dependency-inputs.json";
import {READY_STAGES_V1,createReadyPortsV1,readyDigestsV1,type ReadyStageV1} from "../../🟦️.ts";

/** 🪜️ The stage plan of the readiness loop is injectable: a lab runs the real runner and the real ports over its own list of stages, the default is the declared one. */
const lab:readonly ReadyStageV1[]=[{name:"first",kind:"repo",arguments:["--help"],timeoutMilliseconds:1000,inputs:[],bootstrapInput:false},{name:"second",kind:"repo",arguments:["--help"],timeoutMilliseconds:1000,inputs:[],bootstrapInput:false}];
describe("ready stage plan",()=>{
 test("digests follow the given stages and the declared ones by default",async()=>{
  const hash=()=>"x";
  expect(readyDigestsV1({platform:"linux",hash,stages:lab}).stages.map(stage=>stage.name)).toEqual(["first","second"]);
  expect(readyDigestsV1({platform:"linux",hash}).stages.map(stage=>stage.name)).toEqual(READY_STAGES_V1.map(stage=>stage.name));
 });
 test("ports carry the stage plan they were given",async()=>{
  const root=mkdtempSync(join(tmpdir(),"ready-plan-"));
  try{
   const ports=createReadyPortsV1({root,environment:{},origin:"o",group:undefined,stages:lab,say:()=>{}});
   expect(ports.stages).toBe(lab);
   expect((await ports.digests()).stages.map(stage=>stage.name)).toEqual(["first","second"]);
   expect(createReadyPortsV1({root,environment:{},origin:"o",group:undefined,say:()=>{}}).stages).toBeUndefined();
  }finally{rmSync(root,{recursive:true,force:true});}
 });
});

test("the dependencies receipt follows scoped lock and member changes",async()=>{
 const root=mkdtempSync(join(tmpdir(),"ready-dependency-plan-"));
 using model=new Database(":memory:");
 try{
  const write=(path:string,content:unknown)=>{const file=join(root,path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,JSON.stringify(content));};
  for(const [path,data] of Object.entries(dependencyInputs.files))write(path,data);
  for(const path of dependencyInputs.locks)write(path,"initial");
  const ports=createReadyPortsV1({root,environment:{},origin:"fixture",group:undefined,say:()=>{}});
  let previous=await ports.digests();
  for(const change of dependencyInputs.changes){
   write(change.path,change.content);
   const next=await ports.digests(),expected=(model.query("SELECT value FROM json_each(?) ORDER BY value").all(JSON.stringify(change.affects)) as {value:string}[]).map(row=>row.value);
   expect(next.stages.filter((stage,index)=>stage.digest!==previous.stages[index]?.digest).map(row=>row.name),change.path).toEqual(expected);
   previous=next;
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});
