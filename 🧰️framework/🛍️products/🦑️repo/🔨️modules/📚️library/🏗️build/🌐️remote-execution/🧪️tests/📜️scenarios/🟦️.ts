/** 📜️ Law: the wire scenarios of the service are language-agnostic. A client that is not ours (curl over HTTP/2 prior knowledge) replays them against an empty service and a decoder that is not ours (the Python oracle, reading the generated schema) reads every request and response; the statuses and decoded responses equal the fixture's. */
import {afterEach,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startClusterV1,type ClusterV1} from "../🧰️cluster/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";

const python=testPythonV1(),oracle=join(import.meta.dir,"..","..","📡️wire","🧪️tests","🐍️oracle","🐍️.py"),schema=join(import.meta.dir,"..","..","📡️wire","📋️messages","🔣️.json"),clusters:ClusterV1[]=[];
afterEach(async()=>{for(const cluster of clusters.splice(0))await cluster.stop();});

async function decode(message:string,hex:string):Promise<any>{
 const child=Bun.spawn([python!,"-I",oracle,schema],{stdin:"pipe",stdout:"pipe",stderr:"pipe"});
 child.stdin.write(JSON.stringify({message,hex})+"\n");await child.stdin.end();
 const [text,error]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text()]);await child.exited;
 expect(error).toBe("");const answer=JSON.parse(text);expect(answer.ok,`${message} ${hex}`).toBe(true);return answer.value;
}

const contains=(actual:any,expected:any,path="$"):void=>{
 if(Array.isArray(expected)){expect(Array.isArray(actual),path).toBe(true);expect(actual.length,path).toBe(expected.length);expected.forEach((row,index)=>contains(actual[index],row,`${path}[${index}]`));}
 else if(expected&&typeof expected==="object"){for(const [key,value] of Object.entries(expected))contains(actual?.[key],value,`${path}.${key}`);}
 else expect(actual,path).toEqual(expected);
};

test.skipIf(!python||Bun.spawnSync(["curl","--version"],{stdout:"ignore",stderr:"ignore"}).exitCode!==0)("language-agnostic scenarios replayed by curl over HTTP/2 and decoded by the Python oracle end with the expected statuses and responses",async()=>{
 const cluster=await startClusterV1({workers:0});clusters.push(cluster);
 const base=cluster.server.grpcUrl.replace("grpc://","http://"),directory=mkdtempSync(join(tmpdir(),"rxi-scenarios-"));
 try{
  expect(fixture.scenarios.length).toBeGreaterThanOrEqual(9);
  for(const scenario of fixture.scenarios){
   contains(await decode(scenario.requestMessage,scenario.requestHex),scenario.request,scenario.name+" request");
   const frame=Buffer.concat([Buffer.from([0,0,0,0,0]),Buffer.from(scenario.requestHex,"hex")]);frame.writeUInt32BE(frame.length-5,1);
   writeFileSync(join(directory,"in.bin"),frame);
   const call=Bun.spawn(["curl","-s","--http2-prior-knowledge","-X","POST",`${base}/${scenario.method}`,"-H","content-type: application/grpc","-H","te: trailers","--data-binary",`@${join(directory,"in.bin")}`,"-D",join(directory,"headers.txt"),"-o",join(directory,"out.bin")],{stdout:"pipe",stderr:"pipe"});
   expect(await call.exited,scenario.name).toBe(0);
   const headers=readFileSync(join(directory,"headers.txt"),"latin1"),statuses=[...headers.matchAll(/grpc-status: (\d+)/gu)].map(match=>Number(match[1]));
   expect(statuses.at(-1),`${scenario.name} status`).toBe(scenario.status);
   const body=readFileSync(join(directory,"out.bin"));
   if(scenario.responseMessage===null||scenario.expect===null){expect(body.length,scenario.name).toBe(0);continue;}
   expect(body[0]).toBe(0);const size=body.readUInt32BE(1);
   contains(await decode(scenario.responseMessage,body.subarray(5,5+size).toString("hex")),scenario.expect,scenario.name+" response");
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
});
