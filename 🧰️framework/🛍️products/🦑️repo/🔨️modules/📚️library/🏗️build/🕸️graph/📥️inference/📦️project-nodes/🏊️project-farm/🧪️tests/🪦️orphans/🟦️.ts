/** 🪦️ Contract: no worker process outlives its read. When the signal of the read is cancelled every worker of its pools ends within the shutdown grace and the grants are returned, and when the read process itself is killed without any chance to clean up (SIGKILL) every worker ends because its channel closes. Oracle: the process ids of real worker processes, probed by the operating system (`kill -0`) and listed by `pgrep`. */
import {expect,test} from "bun:test";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {GRAPH_FARM_POLICY_V1} from "../../🏊️pool/🟨️.mjs";

const parentFile=join(dirname(fileURLToPath(import.meta.url)),"..","🧫️fixtures","🪦️hub-parent","🟨️.mjs");
const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch(error){return (error as {code?:string}).code==="EPERM";}};
const gone=async(pids:number[],milliseconds:number)=>{const until=Date.now()+milliseconds;while(Date.now()<until&&pids.some(alive))await Bun.sleep(50);return pids.filter(alive);};
const start=async()=>{
 const environment:Record<string,string|undefined>={...process.env};delete environment.SEMIO_SCRIPT_PROCESS_INVOCATION;
 const child=Bun.spawn([process.execPath,parentFile],{stdin:"pipe",stdout:"pipe",stderr:"inherit",env:environment}),reader=child.stdout.getReader(),decoder=new TextDecoder();let buffer="";
 const next=async():Promise<Record<string,unknown>>=>{while(!buffer.includes("\n")){const{value,done}=await reader.read();if(done)throw Error("the read process ended: "+buffer);buffer+=decoder.decode(value);}const at=buffer.indexOf("\n"),line=buffer.slice(0,at);buffer=buffer.slice(at+1);return JSON.parse(line);};
 const ready=await next();
 return{child,ready,next,send:(line:string)=>{child.stdin.write(line+"\n");child.stdin.flush();}};
};

test.skipIf(process.platform==="win32")("cancelling the signal of the read ends every worker within the shutdown grace",async()=>{
 const read=await start();
 try{
  const pids=read.ready.children as number[];
  expect(read.ready.loops).toBe(2*GRAPH_FARM_POLICY_V1.loopsPerWorker);expect(pids.length,"two real workers").toBe(2);expect(pids.every(alive)).toBe(true);
  read.send("abort");await read.next();
  expect(await gone(pids,GRAPH_FARM_POLICY_V1.shutdownGraceMilliseconds+5000),"workers left after the cancellation").toEqual([]);
 }finally{read.send("exit");await read.child.exited;}
},120000);

test.skipIf(process.platform==="win32")("a read process that is killed leaves no worker behind",async()=>{
 const read=await start(),pids=read.ready.children as number[];
 expect(pids.length).toBe(2);expect(pids.every(alive)).toBe(true);
 read.child.kill("SIGKILL");await read.child.exited;
 expect(await gone(pids,15000),"workers left after the kill").toEqual([]);
},120000);
