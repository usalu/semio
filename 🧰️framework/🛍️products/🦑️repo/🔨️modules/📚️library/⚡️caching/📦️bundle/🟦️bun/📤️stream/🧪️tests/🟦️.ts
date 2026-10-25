/** 📨️ Contract: the Bun bundle data stream emits canonical frames byte-identical to an independent Python encoder, decodes any cutting of its wire into the same files as a Python decoder, and refuses every hostile stream described in `./📋️frames.json` and in the unit's own `./🔣️.json` with its own message. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {bunBundleStreamFramesV1,createBunBundleStreamV1,type BunBundleStreamEventV1} from "../🟦️.ts";
import policySchema from "../🧬️schema/🔣️.json";
import own from "./🔣️.json";
import corpus from "./📋️frames.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py"),policy=policySchema.definitions.Protocol.const,encoder=new TextEncoder();
const ask=(request:object):any=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify(request),encoding:"utf8",maxBuffer:256*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const bytesOf=(content:any):Uint8Array=>encoder.encode("repeat" in content?content.repeat.repeat(content.times):content.text);
const filesOf=(set:any)=>set.files.map((file:any)=>({name:file.name,bytes:bytesOf(file.content)}));
const wireOf=(frames:readonly string[]):Uint8Array=>encoder.encode(frames.join(""));
const cut=(bytes:Uint8Array,sizes:readonly number[]):Uint8Array[]=>{
 if(!sizes.length)return[bytes];const pieces:Uint8Array[]=[];let offset=0,at=0;
 while(offset<bytes.length){const size=sizes[at++%sizes.length]!;pieces.push(bytes.subarray(offset,offset+size));offset+=size;}
 return pieces;
};
const refusal=(action:()=>unknown):Error|undefined=>{try{action();return undefined;}catch(error){return error as Error;}};
const sha=(bytes:Uint8Array):string=>createHash("sha256").update(bytes).digest("hex");

function edit(frames:string[],ops:readonly any[]):{frames:string[];tail:Uint8Array[]}{
 const rows=[...frames],tail:Uint8Array[]=[];
 for(const op of ops){
  if(op.op==="drop")rows.splice(op.at,1);
  else if(op.op==="insert")rows.splice(op.at,0,JSON.stringify(op.frame)+"\n");
  else if(op.op==="replace")rows[op.at]=JSON.stringify(op.frame)+"\n";
  else if(op.op==="replace-text")rows[op.at]=op.text;
  else tail.push(new Uint8Array(op.bytes).fill(97));
 }
 return{frames:rows,tail};
}

test("the corpus is closed by its schema and the policy, schema and own fixture agree with it",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,hostile:[{...corpus.hostile[0],ops:[{op:"explode"}]}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,sets:[{...corpus.sets[0],files:corpus.sets[0]!.files.slice(0,1)}]})).toBe(false);
 expect(policy).toEqual({version:1,chunkBytes:32768,maximumFrameBytes:65536,maximumOutputBytes:536870912,maximumFiles:3,maximumMetadataBytes:16777216,maximumWireBytes:738197504});
 expect(own.hostile.sort()).toEqual(["after-complete","bad-hash","chunk-before-file","duplicate-file","metadata-too-large","missing-complete","overlong-partial","wrong-request"]);
 expect(new Set(corpus.hostile.map(row=>row.name))).toEqual(new Set([...own.hostile,...corpus.hostile.map(row=>row.name)]));
});

test("frames equal the independent encoder byte for byte, the authored frames and the chunk count of the corpus",()=>{
 const encoded=ask({op:"encode",rows:corpus.sets.map(set=>({files:set.files,request:corpus.requestSha256}))});
 for(const[index,set]of corpus.sets.entries()){
  const frames=[...bunBundleStreamFramesV1(corpus.requestSha256,filesOf(set))];
  expect(frames,set.name).toEqual(encoded[index]);if(set.frames)expect(frames,set.name).toEqual(set.frames);
  expect(frames.filter(frame=>frame.startsWith('{"type":"chunk"')).length,set.name).toBe(set.chunks);
  expect(frames.at(-1),set.name).toBe(JSON.stringify({type:"complete",requestSha256:corpus.requestSha256})+"\n");
  for(const frame of frames)expect(Buffer.byteLength(frame),set.name).toBeLessThanOrEqual(policy.maximumFrameBytes);
 }
 for(const row of own.files)expect([...bunBundleStreamFramesV1(corpus.requestSha256,own.files.map(file=>({name:file.name,bytes:encoder.encode(file.text)})))].length).toBeGreaterThan(4);
});

test("any cutting of the wire decodes to the same files as the Python decoder and emits events in order",()=>{
 for(const set of corpus.sets){
  const frames=[...bunBundleStreamFramesV1(corpus.requestSha256,filesOf(set))],wire=wireOf(frames),python=ask({op:"decode",wires:[new TextDecoder().decode(wire)]})[0];
  for(const piece of corpus.wire){
   const events:BunBundleStreamEventV1[]=[],stream=createBunBundleStreamV1(corpus.requestSha256,event=>{events.push(event);});
   for(const part of cut(wire,piece.sizes))stream.push(part);
   const done=stream.finish(),label=set.name+" / "+piece.name;
   expect(done.map(file=>({name:file.name,byteLength:file.byteLength,sha256:file.sha256})),label).toEqual(python.files.map((file:any)=>({name:file.name,byteLength:file.byteLength,sha256:file.sha256})));
   expect<unknown>(events.filter(event=>event.type==="file").map(event=>event.name),label).toEqual(set.files.map(file=>file.name));
   expect<unknown>(events.filter(event=>event.type==="end").map(event=>event.name),label).toEqual(set.files.map(file=>file.name));
   for(const[at,file]of filesOf(set).entries()){
    const joined=Buffer.concat(events.filter((event):event is Extract<BunBundleStreamEventV1,{type:"chunk"}>=>event.type==="chunk"&&event.name===file.name).map(event=>event.bytes));
    expect(sha(joined),label).toBe(sha(file.bytes));expect(joined.toString("base64"),label).toBe(python.files[at].base64);
   }
  }
  expect(python.complete).toBe(corpus.requestSha256);
 }
});

test("the generator refuses incomplete, misordered, unknown, repeated and oversized output and a bad request digest",()=>{
 for(const row of corpus.generator){
  const error=refusal(()=>[...bunBundleStreamFramesV1((row as any).requestSha256??corpus.requestSha256,row.files.map(name=>({name,bytes:encoder.encode("x")})))]);
  expect(error?.message,row.name).toBe(row.error);
 }
 const big=new Uint8Array(policy.maximumMetadataBytes+1);
 expect(refusal(()=>[...bunBundleStreamFramesV1(corpus.requestSha256,[{name:"source.js",bytes:encoder.encode("x")},{name:"metadata.json",bytes:big}])])?.message).toBe("Bun output file capacity refused");
});

test("every hostile stream is refused with its own message and a refused stream stays refused",()=>{
 const tiny=corpus.sets.find(set=>set.name==="tiny")!,base=[...bunBundleStreamFramesV1(corpus.requestSha256,filesOf(tiny))];
 for(const row of corpus.hostile){
  const {frames,tail}=edit(base,row.ops),seen:string[]=[];
  const stream=createBunBundleStreamV1(corpus.requestSha256,event=>{seen.push(event.type);return (row as any).sink==="async"?Promise.reject(Error("late")):undefined;});
  const error=refusal(()=>{for(const piece of [wireOf(frames),...tail])stream.push(piece);if((row as any).finish)stream.finish();});
  expect(error,row.name).toBeDefined();
  if(row.error==="SyntaxError")expect(error!.name,row.name).toBe("SyntaxError");else expect(error!.message,row.name).toBe(row.error);
  if((row as any).again)expect(refusal(()=>stream.push(encoder.encode("{}\n")))?.message,row.name).toBe((row as any).again);
  else if(!(row as any).finish)expect(refusal(()=>stream.finish())?.message,row.name).toBe("Complete Bun output stream required");
 }
 const wireLimit=createBunBundleStreamV1(corpus.requestSha256,()=>undefined);
 expect(refusal(()=>wireLimit.push(new Uint8Array(policy.maximumFrameBytes+1).fill(97)))?.message).toBe("Bun partial frame capacity exceeded");
 expect(()=>createBunBundleStreamV1("nope",()=>undefined)).toThrow("Actual Bun data consumer required");
 expect(()=>createBunBundleStreamV1(corpus.requestSha256,undefined as never)).toThrow("Actual Bun data consumer required");
});
