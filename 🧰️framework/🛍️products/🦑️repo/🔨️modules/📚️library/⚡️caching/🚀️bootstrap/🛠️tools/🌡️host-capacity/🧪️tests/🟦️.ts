import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {nxHostResourceSettingsV1} from "../🟦️.ts";

const plan={version:1 as const,id:"aaaaaaaa1",jobs:3,cpu:6,memoryMegabytes:6144,fixedMegabytes:1536,floorMegabytes:6553};

test("The plan renders into Nx as its task parallelism and says which fields Nx can only approximate",()=>{
 const settings=nxHostResourceSettingsV1(plan);
 expect(settings.environment).toEqual({NX_PARALLEL:"3"});
 expect(settings.mapping.map(row=>[row.field,row.support])).toEqual([["jobs","exact"],["cpu","approximate"],["memoryMegabytes","approximate"],["fixedMegabytes","unsupported"],["floorMegabytes","unsupported"]]);
});

const root=join(import.meta.dir,"../../../../../../../../../..").replace(/[\\/]+$/u,""),live=process.env.SEMIO_NX_INTEGRATION==="1";
test.skipIf(!live)("The repository's Nx environment takes its parallelism from the grant and real Nx runs exactly that many tasks at once",async()=>{
 const {nxBuildEnvironmentV1}=await import("../../../../../📦️packages/🟦️typescript/💾️storage/🚀️bootstrap/📜️script/🟦️.ts");
 expect(nxBuildEnvironmentV1(root,{SEMIO_HOST_CAPACITY_PLAN:JSON.stringify(plan),SEMIO_BUILD_PARALLEL:"9"}).NX_PARALLEL).toBe("3");
 const nx=createRequire(join(root,".nx/installation/package.json")).resolve("nx/bin/nx"),workspace=mkdtempSync(join(tmpdir(),"hb-nx-"));
 try{
  writeFileSync(join(workspace,"nx.json"),JSON.stringify({}));
  writeFileSync(join(workspace,"package.json"),JSON.stringify({name:"hc",private:true}));
  for(let index=0;index<8;index++){
   const project=join(workspace,`p${index}`);mkdirSync(project);
   writeFileSync(join(project,"project.json"),JSON.stringify({name:`p${index}`,targets:{t:{executor:"nx:run-commands",options:{command:`perl -MTime::HiRes=time -e 'printf q(%.3f ), time' > ${join(workspace,`s${index}.txt`)}; sleep 1.5; perl -MTime::HiRes=time -e 'printf q(%.3f), time' >> ${join(workspace,`s${index}.txt`)}`},cache:false}}}));
  }
  const run=spawnSync("node",[nx,"run-many","-t","t","--skip-nx-cache"],{cwd:workspace,encoding:"utf8",env:{...process.env,NX_DAEMON:"false",NX_TUI:"false",NX_WORKSPACE_ROOT_PATH:workspace,...nxHostResourceSettingsV1(plan).environment}});
  expect(run.status,run.stderr.slice(-2000)+run.stdout.slice(-2000)).toBe(0);
  const spans=Array.from({length:8},(_,index)=>readFileSync(join(workspace,`s${index}.txt`),"utf8").trim().split(/\s+/u).map(Number) as [number,number]);
  expect(Math.max(...spans.map(([start])=>spans.filter(([from,to])=>from<=start&&start<to).length))).toBe(plan.jobs);
 }finally{rmSync(workspace,{recursive:true,force:true});}
},300_000);
