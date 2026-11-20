import {expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {setTimeout as pause} from "node:timers/promises";
import {captureOwnedCommandV1} from "../../../../../🎛️owned-execution/🟦️.ts";
import {sweepWorkspaceSandboxImagesV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import cases from "./🔣️.json" with {type:"json"};

const policy=schema.definitions.Protocol.const;

test("workspace sandbox image collector cases are closed data",()=>{
 const verify=new Ajv({strict:true}).compile(schema.definitions.WorkspaceCollectorCases);
 expect(verify(cases)).toBe(true);
 expect(verify([...cases,{mode:"hung-inventory",outcome:"deadline",detach:true}])).toBe(false);
});

/** 🔎️ Treats a reparented zombie as settled while the OS finishes reaping it. */
function canRun(pid:number):boolean{
 try{process.kill(pid,0);}catch(error){if((error as NodeJS.ErrnoException).code==="ESRCH")return false;throw error;}
 if(process.platform==="linux"){try{if(readFileSync("/proc/"+pid+"/stat","utf8").split(") ").at(-1)?.startsWith("Z "))return false;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return false;throw error;}}
 return true;
}

for(const row of cases)test.skipIf(process.platform==="win32"&&(row.mode==="hung-inventory"||row.mode==="caller-stop"))("scoped image sweep "+row.mode,async()=>{
 const root=resolve(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"sw-"))),marker=join(root,"pid"),caller=new AbortController(),reason=Object.freeze({stopped:"private-image-sweep"});
 const descendant='require("node:fs").writeFileSync('+JSON.stringify(marker)+',JSON.stringify([process.pid,process.ppid]));setInterval(()=>{},1000);';
 const hung='require("node:child_process").spawnSync(process.execPath,["-e",'+JSON.stringify(descendant)+'],{stdio:"inherit"});';
 const waitForMarker=async()=>{for(let end=performance.now()+4000;!existsSync(marker)&&performance.now()<end;)await pause(10);if(!existsSync(marker))throw Error("Substitute inventory did not start");};
 let launches=0,outcome="completed",failure:unknown;
 try{
  if(row.mode==="already-stopped")caller.abort(reason);
  await sweepWorkspaceSandboxImagesV1(root,caller.signal,async(command,args,cwd,options)=>{
   launches++;
   expect(command).toBe(process.execPath);expect(cwd).toBe(root);expect(args.slice(1)).toEqual(["--root="+root,policy.workspaceOnlyFlag]);expect(options?.output).toEqual(policy.workspaceCollectorOutput);
   const run=captureOwnedCommandV1(command,["-e",row.mode==="exit"?'console.log("swept");':hung],cwd,options);
   if(row.mode==="caller-stop")void waitForMarker().then(()=>caller.abort(reason),error=>caller.abort(error));
   return run;
  }).catch(error=>{failure=error;outcome=error instanceof DOMException&&error.name==="TimeoutError"?"deadline":"cancelled";});
  expect(outcome).toBe(row.outcome);expect(launches).toBe(row.mode==="already-stopped"?0:1);
  if(row.outcome==="cancelled")expect(failure).toBe(reason);
  if(row.mode==="hung-inventory"||row.mode==="caller-stop"){
   const pids=JSON.parse(readFileSync(marker,"utf8"))as number[];expect(pids).toHaveLength(2);
   for(let end=performance.now()+2000;pids.some(canRun)&&performance.now()<end;)await pause(10);
   expect(pids.some(canRun)).toBe(false);
  }
 }finally{rmSync(root,{recursive:true,force:true});}
},policy.workspaceCollectorBudgetMilliseconds+8000);
