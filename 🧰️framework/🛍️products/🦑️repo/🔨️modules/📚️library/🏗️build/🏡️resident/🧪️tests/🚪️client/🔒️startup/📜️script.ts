import {closeSync,existsSync,openSync,readFileSync,writeFileSync} from "node:fs";
import {createServer,type Server} from "node:net";
import {setTimeout as pause} from "node:timers/promises";
import {connectHostV1,withRestartOwnerV1} from "../../../🚪️client/🛰️plan/🟨️.mjs";
import {hostEndpointsV1} from "../../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {preparePrivateDirectoryV1} from "../../../📨️protocol/📍️endpoint/🔐️directory/🟨️.mjs";
import {physicalLockBackendV1} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/📁️physical/🟦️.ts";

const [role,implementation,mode]=process.argv.slice(2),environment={...process.env,XDG_RUNTIME_DIR:".",LOCALAPPDATA:process.cwd()},policy={startTimeoutMilliseconds:6000,startCeilingMilliseconds:mode==="no-socket"?800:8000,startPollMilliseconds:15,startLivenessMilliseconds:15,lockStaleMilliseconds:5};
const runtime=hostEndpointsV1(implementation,environment),stop=new AbortController(),reason=new Error("fixture cancelled");
const record=(event:string,value:string="")=>writeFileSync(role+"."+event,value);
const permit=async(file:string)=>{for(let until=performance.now()+9000;!existsSync(file);){if(performance.now()>until)throw Error("Fixture waiting for "+file);await pause(10);}return readFileSync(file,"utf8");};
let server:Server|undefined;
const start=async()=>{
 record("start",runtime.lock);
 if(mode==="no-socket")return{alive:()=>true};
 if(await permit(role+".go")==="fail")throw Error("fixture released");
 await new Promise<void>((accept,reject)=>{server=createServer(client=>client.on("error",()=>{}));server.once("error",reject);server.listen(runtime.endpoint,accept);});
 return{alive:()=>true};
};
try{
 if(mode==="held-empty"){
  preparePrivateDirectoryV1(runtime.directory);closeSync(openSync(runtime.lock,"wx",0o600));
  const held=physicalLockBackendV1();try{held.retain(runtime.lock);if(!held.tryLock())throw Error("Fixture could not own its empty lock");await start();}finally{held.close();}
  record("outcome","started");
 }else if(mode==="restart"){
  record("enter");
  const outcome=await withRestartOwnerV1({implementation,environment,policy},async()=>{record("start",runtime.lock.replace(/start-([0-9a-f]+)\.lock$/u,"restart-$1.lock"));await permit(role+".go");return "started";},async()=>"joined");
  record("outcome",outcome);
 }else{
  if(mode==="cancel")void permit(role+".cancel").then(()=>stop.abort(reason),error=>stop.abort(error));
  const found=await connectHostV1({implementation,environment,policy,signal:stop.signal,spawnService:start,onProgress:value=>{if(value.total===0)record("waiting");}});
  found.socket.destroy();record("outcome",found.started?"started":"joined");
 }
}catch(error){record("outcome",error===reason?"cancelled":(error as{reason?:string})?.reason??(error instanceof Error?error.message:String(error)));}
try{await permit("done");}finally{server?.close();}
