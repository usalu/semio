import {expect,test} from "bun:test";
import fixture from "./🔣️.json";
import {runOwnedCommand} from "../../🟦️.ts";
import Ajv from "ajv";
import streamSchema from "../../🧬️schema/📤️stream.json";

test("Owned line observation preserves actual stdout and stderr provenance with an independent Node oracle",async()=>{
 const admit=new Ajv({strict:true}).compile(streamSchema);for(const row of fixture.expected)expect(admit(row.stream)).toBe(true);for(const stream of ["inherit","ignore","warning",undefined])expect(admit(stream)).toBe(false);
 const controller=new AbortController(),observed:{stream:string;line:string}[]=[],program="process.stdout.write("+JSON.stringify(fixture.stdout)+");process.stderr.write("+JSON.stringify(fixture.stderr)+");",node=Bun.spawn(["node","--eval",'const cp=require("node:child_process"),row=JSON.parse(process.argv[1]),result=cp.spawnSync(process.execPath,["--eval",row.program],{encoding:"utf8",timeout:row.maximumElapsedMilliseconds,maxBuffer:row.transport.maximumBytes});if(result.status!==0)throw Error(result.stderr);const events=[...result.stderr.slice(0,-1).split("\\n").map(line=>({stream:"stderr",line})),...result.stdout.slice(0,-1).split("\\n").map(line=>({stream:"stdout",line}))];console.log(JSON.stringify({events,bytes:Buffer.byteLength(result.stdout)+Buffer.byteLength(result.stderr)}));',JSON.stringify({...fixture,program})],{stdout:"pipe",stderr:"pipe"});
 try{
  await runOwnedCommand("node",["--eval",program],process.cwd(),"original-stream-provenance",fixture.maximumElapsedMilliseconds,{signal:controller.signal,output:fixture.transport,onLine:(line,stream)=>{observed.push({stream,line});}});
  const [stdout,stderr,code]=await Promise.all([new Response(node.stdout).text(),new Response(node.stderr).text(),node.exited]);expect(code,stderr).toBe(0);const independent=JSON.parse(stdout);expect(independent.events).toEqual(fixture.expected);expect(independent.bytes).toBeLessThanOrEqual(fixture.transport.maximumBytes);const ordered=[...observed.filter(row=>row.stream==="stderr"),...observed.filter(row=>row.stream==="stdout")];expect(ordered).toEqual(independent.events);expect(observed.length).toBe(fixture.expected.length);console.log("[DEBUG] "+JSON.stringify({observed:ordered,bytes:independent.bytes,lines:observed.length,signal:controller.signal.aborted}));
 }finally{if(node.exitCode===null){node.kill();await node.exited;}}
},fixture.maximumElapsedMilliseconds);
