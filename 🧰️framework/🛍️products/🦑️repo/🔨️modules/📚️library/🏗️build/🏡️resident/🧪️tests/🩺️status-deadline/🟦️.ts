import {afterAll,expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {createServer,type Socket} from "node:net";
import {mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createHash} from "node:crypto";
import Ajv from "ajv";
import {HostUnavailableErrorV1,requestHostStatusV1,retireHostServiceV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {runHostCommandV1} from "../../⌨️command/🌐️host/🟨️.mjs";
import {encodeFrameV1} from "../../📨️protocol/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import fixture from "./🔣️.json";

const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","hst-"));
const environment={...process.env,XDG_RUNTIME_DIR:base,LOCALAPPDATA:base};
afterAll(()=>rmSync(base,{recursive:true,force:true}));

const schema={type:"array",items:{type:"object",additionalProperties:false,required:["mode","result","signals"],properties:{
 mode:{enum:["reply","absent","silent","cancel","retire-unknown","retire-silent","command-silent"]},
 result:{enum:["answered","absent","timeout","parent","retained","command-timeout"]},
 signals:{type:"integer",minimum:0,maximum:1}
}}};

test("local status scenarios agree with the independent SQL oracle",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 using oracle=new Database(":memory:");
 const rule=oracle.query(`SELECT CASE $mode WHEN 'reply' THEN 'answered' WHEN 'absent' THEN 'absent' WHEN 'silent' THEN 'timeout' WHEN 'cancel' THEN 'parent' WHEN 'command-silent' THEN 'command-timeout' ELSE 'timeout' END AS result, $mode='retire-silent' AS signals`);
 for(const row of fixture)expect(rule.get({$mode:row.mode})).toEqual({result:row.result,signals:row.signals});
});

for(const row of fixture)test("host status "+row.mode,async()=>{
 const implementation=createHash("sha256").update(base+row.mode).digest("hex");
 const runtime=hostEndpointsV1(implementation,environment),parent=new AbortController(),reason=Error("the original operation stopped");
 mkdirSync(runtime.directory,{recursive:true});
 const sockets=new Set<Socket>();
 let requests=0,signals=0,watchdog:ReturnType<typeof setTimeout>|undefined;
 const server=createServer(socket=>{
  sockets.add(socket);socket.on("close",()=>{sockets.delete(socket);});socket.on("error",()=>undefined);
  socket.once("data",()=>{
   requests++;
   if(row.mode==="cancel"){parent.abort(reason);return;}
   if(row.mode==="reply"||row.mode==="retire-silent"&&requests===1)socket.write(encodeFrameV1({t:"service-status",id:1,pid:process.pid,instance:"0123456789abcdef",implementation,startedAt:0,rssMb:0,indexBytes:0,indexes:0,workspaces:[],clients:1,queue:0,inFlight:0}));
  });
 });
 try{
  if(row.mode!=="absent")await new Promise<void>((accept,reject)=>{server.once("error",reject);server.listen(runtime.endpoint,accept);});
  const options={implementation,environment,signal:parent.signal,timeoutMilliseconds:100};
  const errors:string[]=[];
  const operation=(row.mode==="command-silent"?
   runHostCommandV1({...options,args:["status"],languages:["de"],command:[],runForeground:async()=>2,policy:{statusReplyMilliseconds:100},stdout:{write:text=>{throw Error(text);}},stderr:{write:text=>errors.push(text)}}).then(code=>code===1?"command-timeout":"command-returned"):
   row.mode.startsWith("retire-")?
   retireHostServiceV1({...options,only:process.pid,kill:(pid,signal)=>{expect([pid,signal]).toEqual([process.pid,"SIGTERM"]);signals++;}}).then(reply=>reply.retired?"retired":"retained"):
   requestHostStatusV1(options).then(reply=>reply?.pid===process.pid?"answered":"absent")
  ).catch(error=>error===reason?"parent":error instanceof HostUnavailableErrorV1&&error.reason==="timeout"?"timeout":Promise.reject(error));
  const outcome=await Promise.race([operation,new Promise<never>((_,reject)=>{watchdog=setTimeout(()=>reject(Error("The local status probe did not settle")),500);})]);
  expect({result:outcome,signals}).toEqual({result:row.result,signals:row.signals});
  if(row.mode!=="absent")expect(requests).toBeGreaterThan(0);
  if(row.mode==="command-silent")expect(errors).toEqual(["Zeitüberschreitung beim Warten auf den Status des lokalen Graphdienstes.\n"]);
 }finally{
  clearTimeout(watchdog);
  for(const socket of sockets)socket.destroy();
  if(server.listening)await new Promise<void>((accept,reject)=>server.close(error=>error?reject(error):accept()));
 }
});
