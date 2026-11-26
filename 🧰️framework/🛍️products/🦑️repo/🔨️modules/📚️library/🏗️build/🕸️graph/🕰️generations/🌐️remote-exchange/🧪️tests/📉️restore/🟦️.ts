import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {mkdtemp,rm} from "node:fs/promises";
import {tmpdir} from "node:os";
import {basename,join} from "node:path";
import Ajv from "ajv";
import {REAPI_V1,decodeProtobufV1,encodeProtobufV1} from "../../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import type {RemoteExecutionContentV1} from "../../../../../🌐️remote-execution/🗄️content/🟦️.ts";
import {createNodeFilePortV1} from "../../../🟨️.mjs";
import {loadGenerationGraphV1} from "../../../🔌️load/🟨️.mjs";
import {createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";
import {generationExchangeKeyV1,publishGenerationV1,remoteGenerationExchangeV1} from "../../🟦️.ts";
import cases from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

test("remote restore stages small batches and publishes only after all required bytes qualify",async()=>{
 expect(new Ajv({strict:true}).validate(schema,cases)).toBe(true);
 using oracle=new Database(":memory:");
 const rule=oracle.query<{status:string;sealed:number},Record<string,string>>(`SELECT CASE $defect WHEN 'none' THEN 'restored' WHEN 'manifest' THEN 'incomplete'
  WHEN 'abort' THEN 'aborted' ELSE 'failed' END AS status,CAST($defect='none' AS INTEGER) AS sealed`);
 const scratch=await mkdtemp(join(tmpdir(),"graph-restore-batches-"));
 let receiver:ReturnType<typeof Bun.serve>|undefined;
 try{
  const source=join(scratch,"source"),workspace=join(scratch,"target"),fs=createNodeFilePortV1(),platform="darwin-arm64",instance="semio";
  await seed(source,ringFiles(18));
  const producer=createMockHost(source),identity=await producer.host.identity(),key=generationExchangeKeyV1(identity,platform,"local");
  await loadGenerationGraphV1(producer.host);
  let raw:Uint8Array|undefined;
  const blobs=new Map<string,Uint8Array>(),events:string[]=[];
  const content:RemoteExecutionContentV1={
   has:async(_instance,hash)=>blobs.has(hash),missing:async(_instance,rows)=>rows.filter(row=>!blobs.has(row.hash)),
   read:async(_instance,{hash})=>blobs.get(hash),stream:async()=>undefined,
   async write(_instance,{hash},bytes){if(!(bytes instanceof Uint8Array))throw Error("expected bytes");blobs.set(hash,bytes);},
   readActionResult:async()=>raw,async writeActionResult(_instance,_hash,bytes){raw=bytes;},
  };
  const common={content,instance,key,workspace,platform,files:fs,identity};
  expect((await publishGenerationV1({...common,root:join(source,".gen"),workspace:source,stale:0})).status).toBe("uploaded");
  const head=decodeProtobufV1(REAPI_V1.actionResult,raw!),note=JSON.parse(Buffer.from(head.stdout_raw as Uint8Array).toString("utf8"));
  receiver=Bun.serve({hostname:"127.0.0.1",port:0,fetch(request){
   const [kind,hash]=new URL(request.url).pathname.split("/").slice(-2);
   if(kind==="cas")events.push("payload");
   const bytes=kind==="ac"?raw:blobs.get(hash!);
   return bytes?new Response(Uint8Array.from(bytes)):new Response(null,{status:404});
  }});
  const exchange=remoteGenerationExchangeV1({environment:{SEMIO_REMOTE_CACHE_ENDPOINT:receiver.url.origin},workspace,identity:async()=>identity,platform})!;
  for(const [index,row] of cases.entries()){
   const root=join(workspace,String(index)),pointer=join(root,"current.json"),controller=new AbortController();
   const listed=(head.output_files as {path:string;digest:{hash:string;size_bytes:number|string}}[]).map(entry=>({...entry,digest:{...entry.digest}}));
   if(row.defect==="size"){
    const entry=listed.find(entry=>entry.path.startsWith("o/"))!;
    entry.digest.size_bytes=Number(entry.digest.size_bytes)-1;
   }
   raw=encodeProtobufV1(REAPI_V1.actionResult,{...head,output_files:listed,stdout_raw:Buffer.from(JSON.stringify({...note,owners:note.owners+(row.defect==="manifest"?1:0)}))});
   events.length=0;
   const files={...fs,async write(path:string,bytes:Uint8Array,options?:Parameters<typeof fs.write>[2]){
    if(path!==pointer){
     events.push("stage");
     const hash=Buffer.from(await crypto.subtle.digest("SHA-256",Uint8Array.from(bytes))).toString("hex");
     expect(basename(path).split(".")[0]).toBe(hash);
    }
    const wrote=await fs.write(path,bytes,options);
    if(row.defect==="abort"&&path!==pointer)controller.abort();
    return wrote;
   }};
   const options={root,fs,implementation:note.implementation,requireComplete:true as const};
   const outcome=await exchange.restore({...options,fs:files,signal:controller.signal});
   const status=controller.signal.aborted?"aborted":outcome.status;
   expect({status,sealed:Number((await fs.read(pointer))!==undefined)},row.name).toEqual(rule.get({$defect:row.defect})!);
   if(row.defect==="none")expect(events.indexOf("stage")).toBeLessThan(events.lastIndexOf("payload"));
   if(row.defect==="manifest")expect(events).toEqual(["payload"]);
   if(row.defect!=="none"){
    raw=encodeProtobufV1(REAPI_V1.actionResult,head);
    expect((await exchange.restore(options)).status,row.name).toBe("restored");
   }
  }
 }finally{await receiver?.stop(true);await rm(scratch,{recursive:true,force:true});}
});
