import{physicalSourceOriginV1}from"../../../../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import ts from"typescript";
import{readFile}from"node:fs/promises";
import{brotliCompressSync,brotliDecompressSync}from"node:zlib";
import{createHash}from"node:crypto";
import{basename}from"node:path";
import{GENERATION_POLICY_V1,canonicalJsonV1}from"../../../../../../../../../🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
import{decodeFactIndexV1,encodeFactIndexV1}from"../../../../../../../../../🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import generationSchema from"../../../../../../../../../🏗️build/🕸️graph/🕰️generations/🧬️schema/🔣️.json";
import component from"../../🧬️schema/🔣️.json";
import schema from"./🧬️schema/🔣️.json";
import fixture from"./🧫️cases.json";
const origin=physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL);
const syntax=(name:string,text:string)=>ts.createSourceFile(name,text,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
const declarations=(tree:ts.SourceFile,names:string[])=>{const nodes=tree.statements.filter((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&names.includes(node.name?.text??""));expect(nodes).toHaveLength(names.length);return ts.transpileModule(nodes.map(node=>node.getText(tree)).join("\n"),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;};

test("generation closure proof reads binary indices and streamed frontiers with complete references",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);expect(validateJsonSchemaSubset(schema,fixture,schema)).toEqual([]);
 const worker=await readFile(new URL("../../👷️closure-read-worker/🟨️.mjs",origin),"utf8"),tree=syntax("worker.mjs",worker),code=declarations(tree,["readGenerationClosureV1"]),read=Function("P","document","GENERATION_POLICY_V1","generationSchema",code+";return readGenerationClosureV1")(component.definitions.Protocol.const,component,GENERATION_POLICY_V1,generationSchema),db=new Database(":memory:");
 expect(code).not.toContain("JSON.parse");expect(GENERATION_POLICY_V1.maximumIndexBytes).toBe(536870912);expect(component.definitions.PayloadRefusal.properties.expandedBytes.maximum).toBe(GENERATION_POLICY_V1.maximumIndexBytes);expect(component.definitions.PayloadRefusal.properties.maximumOutputBytes.maximum).toBe(GENERATION_POLICY_V1.maximumIndexBytes);
 const digest=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex"),decode=Function("P","document","GENERATION_POLICY_V1","generationSchema","decodeFactIndexV1","bytesAt","hash","basename","brotliDecompressSync","closurePayloadRefusalContextV1",code+"\n"+declarations(tree,["references","chargeReadBytes","payload"])+";return payload")(component.definitions.Protocol.const,component,GENERATION_POLICY_V1,generationSchema,decodeFactIndexV1,(row:any,budget:{remaining:number})=>{budget.remaining-=row.bytes.length;return row.bytes;},digest,basename,brotliDecompressSync,(error:unknown)=>error);
 const chunks=function*(bytes:Uint8Array,size:number){for(let at=0;at<bytes.length;at+=size)yield bytes.subarray(at,at+size);},limit=GENERATION_POLICY_V1.maximumIndexBytes;
 const oracle=(wire:string)=>{const refs=new Set<string>();for(const node of db.query("select key,value,type from json_tree(?1)").all(wire)as{key:unknown;value:unknown;type:string}[])for(const text of[typeof node.key==="string"?node.key:null,node.type==="text"?node.value:null])if(typeof text==="string")for(const match of text.matchAll(/SEMIO_(SOURCE_CLOSURE|CARGO_LOCK)_([a-f0-9]{64})(?![a-zA-Z0-9_])/gu))refs.add((match[1]==="SOURCE_CLOSURE"?"source:":"lock:")+match[2]);return[...refs].sort();};
 const proof=(kind:string,packed:Uint8Array,extension:string)=>({path:digest(packed)+extension,kind,bytes:packed});
 try{for(const row of fixture.cases){
  if(row.kind==="generation-frontier"){
   const bytes=Buffer.from(row.wire);for(const size of[1,7,65536]){if(row.accepted){const actual=read(chunks(bytes,size),row.kind,bytes.length);expect(actual.references,row.id).toEqual(row.references);expect(actual.expandedBytes,row.id).toBe(bytes.length);expect(actual.tokens,row.id).toBeLessThanOrEqual(bytes.length);const value=JSON.parse(row.wire);expect(new Ajv({strict:true}).validate({$ref:"#/definitions/Frontier",definitions:generationSchema.definitions},value),row.id).toBe(true);expect(actual.references,row.id).toEqual(oracle(row.wire));const packed=brotliCompressSync(Buffer.from(canonicalJsonV1(value))),expanded=brotliDecompressSync(packed,{maxOutputLength:limit});expect(read(chunks(expanded,size),row.kind,expanded.length).references,row.id).toEqual(row.references);if(size===65536){const metadata=proof(row.kind,packed,".json.br"),budget={remaining:component.definitions.Protocol.const.maximumProofBytes},decoded=decode(metadata,"workspace",budget,"image");expect(decoded.references,row.id).toEqual(row.references);expect(decoded.expandedBytes,row.id).toBe(packed.length+expanded.length);expect(budget.remaining,row.id).toBe(component.definitions.Protocol.const.maximumProofBytes-packed.length-expanded.length);expect(()=>decode({...metadata,path:"0".repeat(64)+".json.br"},"workspace",{remaining:component.definitions.Protocol.const.maximumProofBytes},"image")).toThrow("Authenticated generation closure document required");}}else expect(()=>read(chunks(bytes,size),row.kind,bytes.length),row.id).toThrow();}
   continue;
  }
  let binary:Buffer|undefined;try{binary=encodeFactIndexV1(JSON.parse(row.wire),{maximumBytes:limit});}catch{binary=undefined;}
  if(row.accepted){
   expect(binary,row.id).toBeDefined();const value=JSON.parse(row.wire);expect(new Ajv({strict:true}).validate({$ref:"#/definitions/FactIndex",definitions:generationSchema.definitions},value),row.id).toBe(true);
   const packed=brotliCompressSync(binary!),metadata=proof(row.kind,packed,".facts.br"),budget={remaining:component.definitions.Protocol.const.maximumProofBytes},decoded=decode(metadata,"workspace",budget,"image");
   expect(decoded.references,row.id).toEqual(row.references);expect(decoded.references,row.id).toEqual(oracle(row.wire));expect(decoded.expandedBytes,row.id).toBe(packed.length+binary!.length);expect(budget.remaining,row.id).toBe(component.definitions.Protocol.const.maximumProofBytes-packed.length-binary!.length);
   expect(()=>decode({...metadata,path:"0".repeat(64)+".facts.br"},"workspace",{remaining:component.definitions.Protocol.const.maximumProofBytes},"image")).toThrow("Authenticated generation closure document required");
  }else expect(()=>{if(!binary)throw Error("Unencodable logical index: "+row.id);decode(proof(row.kind,brotliCompressSync(binary),".facts.br"),"workspace",{remaining:component.definitions.Protocol.const.maximumProofBytes},"image");},row.id).toThrow();
 }
 const frontier=Buffer.from(fixture.cases.find(row=>row.kind==="generation-frontier"&&row.accepted)!.wire);expect(()=>read(chunks(frontier,7),"generation-frontier",frontier.length-1)).toThrow();expect(()=>read([Buffer.from([255])],"generation-frontier",1)).toThrow();expect(()=>read(chunks(frontier,7),"generation-index",frontier.length)).toThrow();
 const head='{"capturedAt":1,"directories":[],"members":[["x",',tail=']],"version":1}',deep=Buffer.from(head+"[".repeat(66)+"0"+"]".repeat(66)+tail);expect(()=>read(chunks(deep,7),"generation-frontier",deep.length)).toThrow();
 const many=function*(){yield Buffer.from(head+"[");const block=Buffer.from("0,".repeat(32768));for(let at=0;at<66;at++)yield block;yield Buffer.from("0]"+tail);};const dense=read(many(),"generation-frontier",limit);expect(dense.tokens).toBeGreaterThan(component.definitions.Protocol.const.maximumNodes);expect(dense.references).toEqual([]);
 let peak=process.memoryUsage().heapUsed;const before=peak,large=function*(){yield Buffer.from(head+'"');const block=Buffer.alloc(65536,97);for(let at=0;at<=component.definitions.Protocol.const.maximumExpandedBytes/65536;at++){peak=Math.max(peak,process.memoryUsage().heapUsed);yield block;}yield Buffer.from('"'+tail);};const streamed=read(large(),"generation-frontier",limit);expect(streamed.expandedBytes).toBeGreaterThan(component.definitions.Protocol.const.maximumExpandedBytes);expect(streamed.references).toEqual([]);expect(peak-before).toBeLessThan(64*1024*1024);
 const bigIndex=encodeFactIndexV1({version:1,owners:["a"],rows:[{kind:"file",path:"x",fact:"SEMIO_SOURCE_CLOSURE_"+"e".repeat(64)}],classes:[{r:[0],o:[0]}]},{maximumBytes:limit}),packedBig=brotliCompressSync(bigIndex);expect(()=>decode(proof("generation-index",packedBig,".facts.br"),"workspace",{remaining:packedBig.length+bigIndex.length-1},"image")).toThrow();expect(()=>decode(proof("generation-index",brotliCompressSync(Buffer.concat([bigIndex,Buffer.from([0])])),".facts.br"),"workspace",{remaining:component.definitions.Protocol.const.maximumProofBytes},"image")).toThrow("Closed binary fact index required");
 }finally{db.close();}
 const payload=tree.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text==="payload")!.getText(tree);expect(payload).toContain("GENERATION_POLICY_V1.maximumIndexBytes");expect(payload).toContain("readGenerationClosureV1");expect(payload).toContain("decodeFactIndexV1");expect(payload).toContain("budget.remaining=chargeReadBytes(budget.remaining,expanded.length)");
},60000);
