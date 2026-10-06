/** 🧪️ Canonical Pack cause projections retain source authority independently of display prose. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {inflateRawSync} from "node:zlib";
import grammarFixture from "../../🧫️fixtures/🧭️cause/📡️codec/🔣️.json";

test("system zlib independently confirms the closed retained DEFLATE grammar corpus",()=>{
 for(const row of grammarFixture.cases){
  let kind:"invalidValue"|null=null,raw:number[]=[];
  try{raw=Array.from(inflateRawSync(Uint8Array.from(row.stored)));}catch(error){
   if(!(error instanceof Error)||!("code"in error)||error.code!=="Z_DATA_ERROR")throw error;
   kind="invalidValue";
  }
  expect(kind as unknown).toBe(row.expectedKind);
  expect(raw).toEqual(row.raw);
  if(kind!==null){const projected=new PackError({kind:"RetainedMalformed",refusalKind:kind,what:"deflate",offset:0n,detail:"same deliberately misleading cancellation/limit prose"});expect(projected.refusalKind).toBe(kind);}
 }
});

import Ajv from "ajv/dist/2020.js";
import causeKindSchema from "../../🧬️schema/🧭️cause/🪪️kind/🔣️.json";
import causeKindFixture from "../../🧫️fixtures/🧭️cause/🪪️kind/🔣️.json";
import fixture from "../../🧫️fixtures/🧭️cause/🔣️.json";
import {PackError,type PackErrorData} from "../../🟦️.ts";
import {ValueError,type ValueRefusalKind} from "../../../../🌱️value/⚠️refusal/🟦️.ts";
import {TextError} from "../../../../⚠️diagnostic/🚧️text-error/🟦️.ts";

const kinds=["invalidValue","canceled","ownershipLimit","allocationFailed","workLimit","depthLimit","unsupportedOwner","invariantViolated"] as const;
const typedKind=(kind:string|null):ValueRefusalKind=>{
 const value=kinds.find(value=>value===kind);
 if(!value)throw new Error("missing authored kind");
 return value;
};

function input(row:typeof fixture.cases[number]):PackErrorData {
 switch(row.variant){
  case "BadMagic":case "ContentHashMismatch":return{kind:row.variant};
  case "UnsupportedVersion":return{kind:row.variant,major:2,minor:3};
  case "UnknownRequiredFlags":return{kind:row.variant,flags:4};
  case "UnsupportedCodec":return{kind:row.variant,codec:7};
  case "Truncated":return{kind:row.variant,offset:BigInt(row.offset)};
  case "ChecksumMismatch":return{kind:row.variant,segment:row.what,offset:BigInt(row.offset)};
  case "NonCanonical":return{kind:row.variant,detail:row.message};
  case "LimitExceeded":return{kind:row.variant,refusalKind:typedKind(row.kind),limit:row.message};
  case "RetainedMalformed":case "Malformed":return{kind:row.variant,refusalKind:typedKind(row.kind),what:row.what,offset:BigInt(row.offset),detail:row.message};
  case "RetainedAllocation":if(row.allocatedBytes===null)throw new Error("missing allocation authority");return{kind:row.variant,refusalKind:typedKind(row.kind),allocatedBytes:row.allocatedBytes,what:row.what,offset:BigInt(row.offset),detail:row.message};
  case "ValueRefusal":return{kind:row.variant,error:new ValueError(typedKind(row.kind),row.message)};
  case "TextRefusal":return{kind:row.variant,error:new TextError(typedKind(row.kind),row.message,{line:2,column:3,length:1})};
  case "Io":if(row.retry!=="never"&&row.retry!=="transient")throw new Error("missing authored retry policy");return{kind:row.variant,error:new ValueError(typedKind(row.kind),row.message),retry:row.retry};
  default:throw new Error("unknown authored variant");
 }
}

test("the closed producer corpus refuses absent kinds and absent transport policy",()=>{
 const validateCause=new Ajv({strict:true}).compile(causeKindSchema);
 for(const cause of causeKindFixture)expect(validateCause(cause)).toBe(true);
 for(const cause of [{kind:"transport",category:"unknown"},{kind:"transport",category:"nativeIo",refusalKind:"invalidValue"},{kind:"refusal",refusalKind:"externalFailure"},{kind:"refusal"},{kind:"transport"}])expect(validateCause(cause)).toBe(false);
 expect(new Set(fixture.cases.map(row=>row.expectedKind))).toEqual(new Set(kinds));
 const replace=(id:string,patch:Record<string,unknown>)=>({...fixture,cases:fixture.cases.map(row=>row.id===id?{...row,...patch}:row)});
 
});

test("actual canonical Pack projection matches independent SQLite output without deriving kind from prose",()=>{
 const database=new Database(":memory:");
 try{
  for(const row of fixture.cases){
   const data=input(row),error=new PackError(data);
   expect(error.causeKind as unknown).toEqual({kind:"refusal",refusalKind:row.expectedKind});
   const reference=database.query(`SELECT json_object('kind',CASE WHEN ?1 IN ('BadMagic','Truncated','ChecksumMismatch','ContentHashMismatch','NonCanonical') THEN 'invalidValue' WHEN ?1 IN ('UnsupportedVersion','UnknownRequiredFlags','UnsupportedCodec') THEN 'unsupportedOwner' ELSE ?2 END,'display',CASE ?1 WHEN 'BadMagic' THEN 'bad magic' WHEN 'UnsupportedVersion' THEN 'unsupported version 2.3' WHEN 'UnknownRequiredFlags' THEN 'unknown required feature bits 0x4' WHEN 'Truncated' THEN 'truncated at offset ' || ?4 WHEN 'ChecksumMismatch' THEN 'checksum mismatch in ' || ?5 || ' at offset ' || ?4 WHEN 'ContentHashMismatch' THEN 'content hash mismatch' WHEN 'NonCanonical' THEN 'non-canonical encoding: ' || ?3 WHEN 'UnsupportedCodec' THEN 'unsupported codec 7' WHEN 'LimitExceeded' THEN 'limit exceeded: ' || ?3 WHEN 'ValueRefusal' THEN 'schema error: ' || ?3 WHEN 'TextRefusal' THEN 'schema error: ' || ?3 || ' at 2:3' WHEN 'Io' THEN 'io error: ' || ?3 ELSE 'malformed ' || ?5 || ' at offset ' || ?4 || ': ' || ?3 END,'retry',?6,'allocatedBytes',?7) AS result`).get(row.variant,row.kind,row.message,row.offset,row.what,row.retry,row.allocatedBytes) as {result:string};
   const expected:unknown=JSON.parse(reference.result);
   expect(expected).toEqual({kind:row.expectedKind,display:row.display,retry:row.retry,allocatedBytes:row.allocatedBytes});
   expect({kind:error.refusalKind,display:error.message,retry:error.retry,allocatedBytes:data.kind==="RetainedAllocation"?data.allocatedBytes:null} as unknown).toEqual(expected);
   if(data.kind==="ValueRefusal"||data.kind==="TextRefusal"||data.kind==="Io")expect(error.cause).toBe(data.error);
   if(data.kind==="RetainedAllocation")expect(data.allocatedBytes).toBe(row.allocatedBytes!);
  }
  expect(new Set(fixture.cases.filter(row=>row.variant==="RetainedMalformed"||row.variant==="RetainedAllocation").map(row=>row.expectedKind)).size).toBeGreaterThan(3);
 }finally{database.close();}
});
import pagedFixture from "../../🧫️fixtures/🧭️cause/📋️paged/🔣️.json";

test("borrowed factories preserve lower kinds and byte witnesses against independent SQLite",()=>{
 
 const database=new Database(":memory:");
 try{
  for(const row of pagedFixture.cases){
   if(row.kind!=="ownershipLimit"&&row.kind!=="allocationFailed"&&row.kind!=="invariantViolated")throw new Error("unknown lower kind");
   const source:{kind:"ownershipLimit"|"allocationFailed"|"invariantViolated";reason:string}={kind:row.kind,reason:row.reason};
   const error=row.factory==="refusal"?PackError.fromPagedRefusal(source,"paged",71n):PackError.fromPagedAllocation({...source,allocatedBytes:row.allocatedBytes!},"paged",71n);
   const reference=database.query("SELECT json_object('kind',?1,'allocatedBytes',CASE ?2 WHEN 'allocation' THEN ?3 ELSE NULL END,'detail',?4,'what','paged','offset','71') AS result").get(row.kind,row.factory,row.allocatedBytes,row.reason) as {result:string};
   const expected:unknown=JSON.parse(reference.result);
   if(error.data.kind!=="RetainedMalformed"&&error.data.kind!=="RetainedAllocation")throw new Error("borrowed metadata was erased");
   expect({kind:error.refusalKind,allocatedBytes:error.data.kind==="RetainedAllocation"?error.data.allocatedBytes:null,detail:error.data.detail,what:error.data.what,offset:error.data.offset.toString()} as unknown).toEqual(expected);
   expect(error.refusalKind as unknown).toBe(row.expectedKind);
   expect(error.cause).toBeUndefined();
  }
 }finally{database.close();}
});
