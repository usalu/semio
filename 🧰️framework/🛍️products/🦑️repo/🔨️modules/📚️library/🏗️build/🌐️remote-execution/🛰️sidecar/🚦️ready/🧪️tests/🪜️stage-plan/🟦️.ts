import {describe,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
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
