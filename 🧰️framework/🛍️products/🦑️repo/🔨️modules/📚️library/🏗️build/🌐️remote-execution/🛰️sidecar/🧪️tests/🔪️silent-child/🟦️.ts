import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {testScratchRootV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";

/** 🔪️ Contract: a readiness child that is alive but whose `ready.json` stopped beating (a wedged supervisor under a live launcher) is ended by the launcher and started again; the launcher keeps living. */
const pids=(file:string):number[]=>{try{return readFileSync(file,"utf8").split("\n").filter(Boolean).map(Number);}catch{return[];}};

test("the launcher ends a wedged readiness child whose record stopped beating and starts a new one",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("rexec-silent-child"),"p-")),store=join(directory,"store"),origin="g1-test",log=join(directory,"pids.txt");
 mkdirSync(join(store,"v1","origins",origin),{recursive:true});
 writeFileSync(join(store,"v1","origins",origin,"ready.json"),JSON.stringify({version:1,origin,state:"running",ready:false,stage:"dependencies",updatedMs:Date.now()-3_600_000,beatMilliseconds:100}));
 const launcher=Bun.spawn([process.execPath,join(import.meta.dir,"🧬️launcher.ts"),join(import.meta.dir,"🧬️wedged.ts")],{stdout:"ignore",stderr:"pipe",cwd:directory,env:{...process.env,F59_PIDS:log,SEMIO_FLEET_STORE_ROOT:store,SEMIO_FLEET_ORIGIN:origin,SEMIO_SIDECAR_SILENCE_GRACE_MILLISECONDS:"300",SEMIO_SIDECAR_SILENCE_POLL_MILLISECONDS:"100"}});
 try{
  const deadline=Date.now()+60_000;
  while(Date.now()<deadline&&pids(log).length<2)await Bun.sleep(200);
  const seen=pids(log);
  expect(seen.length,"a second child was started").toBeGreaterThanOrEqual(2);
  expect(new Set(seen).size).toBe(seen.length);
  let alive=true;try{process.kill(seen[0]!,0);}catch{alive=false;}
  expect(alive,"the wedged child is gone").toBe(false);
  expect(launcher.exitCode,"the launcher lives").toBeNull();
 }finally{launcher.kill("SIGKILL");for(const pid of pids(log))try{process.kill(pid,"SIGKILL");}catch{}rmSync(directory,{recursive:true,force:true});}
},90_000);
