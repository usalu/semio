import{expect,test}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join,resolve}from"node:path";
import{crashScenariosV1,type CrashScenarioV1}from"./🎬️scenarios.ts";

const preload=resolve(import.meta.dir,"🔌️preload.ts"),child=resolve(import.meta.dir,"🏃️child.ts");
const scratch=()=>{const base=process.env.SEMIO_TEST_ARTIFACT_DIR?resolve(process.env.SEMIO_TEST_ARTIFACT_DIR):tmpdir();mkdirSync(base,{recursive:true});return mkdtempSync(join(base,"cr-"));};

/** 💥️ Kills the ceremony before each state-changing call in turn (optionally tearing the call's own write), recovers and checks the invariant. */
async function enumerate(scenario:CrashScenarioV1,tear:boolean):Promise<Readonly<{crashes:number;operations:readonly string[]}>>{
 let operations:string[]=[],crashes=0;
 for(let point=1;point<=256;point++){
  const root=scratch(),directory=join(root,"w"),journal=join(root,"ops.log");
  try{
   mkdirSync(directory);scenario.setup(directory);
   const run=Bun.spawnSync([process.execPath,"--preload",preload,child,scenario.id,directory],{env:{...process.env,SEMIO_CRASH_AT:String(point),SEMIO_CRASH_TEAR:tear?"1":"0",SEMIO_CRASH_JOURNAL:journal},stdout:"pipe",stderr:"pipe"});
   const rows=(existsSync(journal)?readFileSync(journal,"utf8"):"").split("\n").filter(Boolean).map(line=>{const[n,kind,...detail]=line.split("\t");return{n:Number(n),kind:kind!,detail:detail.join("\t")};});
   const killed=run.signalCode==="SIGKILL"||run.exitCode===137;
   if(!killed&&run.exitCode!==0)throw Error(`${scenario.id} point ${point} failed: ${run.stderr.toString().slice(0,600)}`);
   if(killed){crashes++;await scenario.recover(directory);}
   await scenario.invariant(directory,{point,completed:!killed,operations:rows});
   operations=rows.map(row=>(row.kind+" "+row.detail.split("/").slice(-2).join("/")).replace(/[a-f0-9]{64}/gu,"<hash>"));
   if(!killed)return{crashes,operations};
  }finally{rmSync(root,{recursive:true,force:true});}
 }
 throw Error(scenario.id+" never completed within 256 state-changing calls");
}

for(const scenario of crashScenariosV1){
 test.skipIf(process.platform==="win32")(`${scenario.id} survives a kill before each state-changing call and every torn write`,async()=>{
  const clean=await enumerate(scenario,false),torn=await enumerate(scenario,true);
  expect(clean.crashes,"the enumeration is not vacuous").toBeGreaterThanOrEqual(4);
  expect(torn.crashes).toBe(clean.crashes);expect(torn.operations).toEqual(clean.operations);
 },240000);
}
