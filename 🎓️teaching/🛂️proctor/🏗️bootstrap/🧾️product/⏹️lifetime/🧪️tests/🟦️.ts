import {test,expect} from "bun:test";
import Ajv from "ajv";
import {mkdtemp,readdir,rm} from "node:fs/promises";
import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {tmpdir} from "node:os";
import {launchProctor,PROCTOR_COPY_DIRECTORY} from "../../../🟦️.ts";
import {waitForProctorV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import cases from "./🧫️fixtures/🔣️.json" with {type:"json"};
import manifest from "../../../../📦️packages/🦀️rust/📋️project.json" with {type:"json"};
import routeSchema from "./🧬️schema/📋️route.json" with {type:"json"};

test("launcher tests are routed by the proctor test aggregate",()=>{
 expect(new Ajv({strict:false}).compile(routeSchema)(manifest)).toBe(true);
 expect(manifest.targets["test-launcher"].options.command).toBe("repo ./📜️script.ts test-launcher");
 expect(manifest.targets.test.dependsOn).toContain("test-launcher");
 const router=readFileSync(resolve(import.meta.dir,"../../../../📦️packages/🦀️rust/📜️script.ts"),"utf8");
 expect(router).toContain('.register("test-launcher", TestLauncherScript)');
 expect(router).toContain('"proctor:test-launcher"');
});

test("neutral exit and cancellation vectors preserve the first reason and drain",async()=>{
 expect(new Ajv().compile(schema)(cases)).toBe(true);
 for(const row of cases){let finish!:(status:number)=>void,stops=0;const exited=new Promise<number>(resolve=>finish=resolve),controller=new AbortController(),reason=Error("cancel");
  const result=waitForProctorV1({exited,stop:async()=>{stops++;finish(row.status);return exited;}},controller.signal);
  if(row.event==="abort")controller.abort(reason);else finish(row.status);
  if(row.outcome==="abort")await expect(result).rejects.toBe(reason);else expect(await result).toBe(row.status);
  expect(stops).toBe(row.stops);
 }
});

test("pre-existing cancellation drains and cleanup failures retain the first reason",async()=>{
 const controller=new AbortController(),reason=Error("cancel"),cleanup=Error("cleanup");controller.abort(reason);let stops=0;
 const child={exited:new Promise<number>(()=>{}),stop:async()=>{stops++;throw cleanup;}};
 try{await waitForProctorV1(child,controller.signal);throw Error("expected failure");}catch(error){expect(error).toBeInstanceOf(AggregateError);expect((error as AggregateError).errors).toEqual([reason,cleanup]);}
 expect(stops).toBe(1);
});

test("native owned child is stopped and settled on cancellation",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),root=await mkdtemp(join(resolve(output),"p-"));
 let child:ReturnType<typeof launchProctor>|undefined;
 try{child=launchProctor(process.execPath,root,["-e","setInterval(()=>{},1000)"],process.env,join(root,"child.log"));expect(child.pid).toBeDefined();const controller=new AbortController(),reason=Error("cancel"),result=waitForProctorV1(child,controller.signal);controller.abort(reason);await expect(result).rejects.toBe(reason);await child.exited;expect(await readdir(join(root,...PROCTOR_COPY_DIRECTORY))).toEqual([]);}finally{if(child)await child.stop(0);await rm(root,{recursive:true,force:true});}
},120000);
