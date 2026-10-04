import {test,expect} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import {blake3} from "@noble/hashes/blake3.js";
import {Database} from "bun:sqlite";
const owner=resolve(import.meta.dir,"../../.."),read=(path:string)=>JSON.parse(readFileSync(resolve(owner,path),"utf8"));
function canonical(tags:Record<string,number>,graph:any[]):Buffer{
 const bytes:number[]=[];
 const integer=(input:number)=>{let value=BigInt(input);do{const byte=Number(value&127n);value>>=7n;bytes.push(byte|(value?128:0));}while(value);};
 const text=(value:string)=>{const raw=Buffer.from(value,"utf8");integer(raw.length);bytes.push(...raw);};
 const shape=(value:any)=>{bytes.push(tags[value.kind]!);switch(value.kind){
 case "enum":integer(value.variants.length);for(const[id,label]of value.variants){integer(id);text(label);}break;
 case "tuple":shape(value.item);bytes.push(value.len===null?0:1);if(value.len!==null)integer(value.len);break;
 case "list":case "block":case "map":shape(value.item);break;
 case "record":case "table":integer(value.record);break;
 case "statements":integer(value.variants.length);for(const[label,id]of value.variants){text(label);integer(id);}break;
 case "quantity":case "angle":text(value.unit);break;case "ref":text(value.entity);break;
 case "coord":case "dim":bytes.push(value.dims);break;
 case "embed":text(value.lang);break;case "embedFrom":text(value.key);break;
 }};
 integer(graph.length);for(const fields of graph){integer(fields.length);for(const field of fields){integer(field.id);text(field.key);bytes.push(Number(field.optional)|(Number(field.flatten)<<1));shape(field.shape);}}
 return Buffer.from(bytes);
}
test("independent BLAKE3 reproduces every authored canonical schema graph",()=>{
 const fixture=read("🧫️fixtures/🔑️schema-hash/🔣️.json");
 expect(Object.keys(fixture.shapeTags).length).toBe(27);
 for(const sample of fixture.cases){const bytes=canonical(fixture.shapeTags,sample.graph);expect(bytes.toString("hex")).toBe(sample.canonicalHex);expect(Buffer.from(blake3(bytes)).toString("hex")).toBe(sample.schemaHash);}
 expect(fixture.cases.map((sample:any)=>sample.name)).toEqual(["flat","nested","nested-inner-field-changed","recursive","every-shape"]);
});
test("failed schema storage accounts for its actual returned diagnostic owner",()=>{
 const fixture=read("🧫️fixtures/🔑️schema-hash/💰️storage/🔣️.json");
 expect(fixture.failureChannel,"closed diagnostic ownership facet").toBeDefined();
 const failure=fixture.failureChannel;
 expect(failure.scratch).toBe("callerDataAllowanceMonotonic");
 expect(failure.diagnostic).toBe("actualReturnedMessageCapacity");
 expect(failure.requests).toBe("completeSystemRequestsIncludingDiagnostic");
 expect(failure.release).toBe("dropReturnedDiagnosticInsideObservation");
 expect(failure.admission).toBe("mayPrecedeCanceledMaterialization");
 expect(failure.vectors.map((vector:any)=>vector.scope)).toEqual(["zero","short","cumulative","start","admittedInterior","materializedInterior","unicodeDiagnostic"]);
 const db=new Database(":memory:");
 db.exec("CREATE TABLE failed_scope(requested INTEGER NOT NULL,admitted INTEGER NOT NULL,diagnostic INTEGER NOT NULL,released INTEGER NOT NULL,CHECK(requested<=admitted+diagnostic),CHECK(requested=released));");
 const insert=db.query("INSERT INTO failed_scope VALUES(?,?,?,?)");
 for(const vector of failure.vectors){
  const bytes=Buffer.byteLength(vector.message,"utf8");
  expect(new TextEncoder().encode(vector.message).length).toBe(bytes);
  expect(bytes).toBe(vector.diagnostic);
  insert.run(vector.requested,vector.admitted,vector.diagnostic,vector.released);
  if(vector.scope==="zero"){expect(vector.admitted).toBe(0);expect(vector.requested).toBe(vector.diagnostic);}
 }
 expect(db.query("SELECT count(*) AS count FROM failed_scope").get()).toEqual({count:failure.vectors.length});
 expect(()=>insert.run(47,0,46,47)).toThrow();
 expect(()=>insert.run(46,0,46,45)).toThrow();
 db.close();
});
test("controlled schema storage has a closed exact caller ownership contract",()=>{
 const path="🧫️fixtures/🔑️schema-hash/💰️storage/🔣️.json";
 expect(existsSync(resolve(owner,path)),"actual neutral controlled schema storage contract").toBe(true);
 const fixture=read(path),schema=read("🧬️schema/🔑️schema-hash/💰️storage/🔣️.json");
 const expected={schema:"pack.schema.controlled-storage/v1",nativeLaw:"schema_hash_controlled_full_allocator_requests_and_same_caller_are_admitted",directions:["encode","decode"],cases:["flat","nested","nested-inner-field-changed","recursive","every-shape","empty","mutualRecursive","broadRepeatedEdges"],semantic:"independentCanonicalBlake3",requests:"fullSystemAllocatorRequests",allowance:"observedExactAndOneByteShort",zero:"ownershipLimitBeforeScratchRequest",cumulative:"sameCallerNoRefund",cancellation:"canceledBeforePublication",released:"completeTemporaryBacking",abiEstimate:false,inlineHasherHeapCharge:false,failureChannel:{scratch:"callerDataAllowanceMonotonic",diagnostic:"actualReturnedMessageCapacity",requests:"completeSystemRequestsIncludingDiagnostic",release:"dropReturnedDiagnosticInsideObservation",admission:"mayPrecedeCanceledMaterialization",vectors:[
  {scope:"zero",message:"native encoding ownership exceeds caller limit",requested:46,admitted:0,diagnostic:46,released:46},
  {scope:"short",message:"native encoding ownership exceeds caller limit",requested:174,admitted:160,diagnostic:46,released:174},
  {scope:"cumulative",message:"native encoding ownership exceeds caller limit",requested:366,admitted:320,diagnostic:46,released:366},
  {scope:"start",message:"native encoding canceled",requested:24,admitted:0,diagnostic:24,released:24},
  {scope:"admittedInterior",message:"native encoding canceled",requested:24,admitted:64,diagnostic:24,released:24},
  {scope:"materializedInterior",message:"native encoding canceled",requested:88,admitted:64,diagnostic:24,released:88},
  {scope:"unicodeDiagnostic",message:"任意\u0000🙂",requested:11,admitted:0,diagnostic:11,released:11}
 ]}};
 expect(fixture).toEqual(expected);
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
 for(const change of [(value:any)=>value.requests="incomingBytes",(value:any)=>value.abiEstimate=true,(value:any)=>value.inlineHasherHeapCharge=true,(value:any)=>value.cumulative="newControlPerCall",(value:any)=>value.failureChannel.diagnostic="unobservedGuess",(value:any)=>value.failureChannel.foreign=true,(value:any)=>value.failureChannel.vectors[0].foreign=true]){const wrong=structuredClone(fixture);change(wrong);expect(validate(wrong)).toBe(false);}
});
