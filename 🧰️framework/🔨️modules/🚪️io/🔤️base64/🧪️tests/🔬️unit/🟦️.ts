import {test,expect} from "bun:test";
import vectors from "../../🧫️fixtures/🔣️rfc4648-base64-vectors.json";
import urlVectors from "../../🧫️fixtures/🔣️rfc4648-base64url-vectors.json";
import controlled from "../../🧫️fixtures/🎛️controlled/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import Ajv from "ajv";
import {base64StandardDecode,base64StandardEncode,base64StandardDecodeControlled,base64StandardEncodeControlled,base64UrlDecode,base64UrlEncode,decodeBase64Quad,Base64DecodeError,Base64DecodeCursor,type Base64DecodeParts,type Base64Progress} from "../../🟦️.ts";
function firstRefusal(parts:Base64DecodeParts):unknown{
 expect(parts.outcome.kind).toBe("refused");
 if(parts.outcome.kind!=="refused")throw Error("retained refusal lost");
 return parts.outcome.refusal;
}
test("standard base64 corpus and shared quartets agree with the system codec",()=>{
 for(const row of vectors.cases){expect(base64StandardEncode(Buffer.from(row.input_utf8))).toBe(row.encoded);expect(Buffer.from(base64StandardDecode(row.encoded)).toString()).toBe(row.input_utf8);}
 for(let length=0;length<=128;length++){const bytes=Uint8Array.from({length},(_,i)=>(i*73+length*19)&255),text=Buffer.from(bytes).toString("base64"),out=new Uint8Array(length);let at=0;
  for(let i=0;i<text.length;i+=4)at+=decodeBase64Quad(Array.from(text.slice(i,i+4),c=>c.charCodeAt(0)),i,i+4===text.length,out,at);
  expect(at).toBe(length);expect(out).toEqual(bytes);expect(base64StandardDecode(text)).toEqual(bytes);expect(base64StandardEncode(bytes)).toBe(text);
 }
});
test("malformed quartets preserve caller storage",()=>{
 for(const text of ["=m9v","Zm=v","Zh==","Zm9=","Z g=","Zm_=","Zm9é"]){const out=new Uint8Array([42,43,44]);expect(()=>decodeBase64Quad(Array.from(text,c=>c.charCodeAt(0)),0,true,out)).toThrow(Base64DecodeError);expect(Array.from(out)).toEqual([42,43,44]);}
 for(const text of ["Zg==","Zm8="]){const out=new Uint8Array([42,43,44]);expect(()=>decodeBase64Quad(Array.from(text,c=>c.charCodeAt(0)),0,false,out)).toThrow();expect(Array.from(out)).toEqual([42,43,44]);}
 const out=new Uint8Array([42,43]);expect(()=>decodeBase64Quad([90,109,57,118],0,true,out)).toThrow();expect(Array.from(out)).toEqual([42,43]);
});
test("base64url corpus remains distinct from standard transport",()=>{
 for(const row of urlVectors.cases){const bytes=Buffer.from(row.input_hex,"hex");expect(base64UrlEncode(bytes)).toBe(row.encoded);expect(Buffer.from(base64UrlDecode(row.encoded))).toEqual(bytes);expect(row.encoded).toBe(bytes.toString("base64url"));}
 for(const row of urlVectors.rejected)expect(()=>base64UrlDecode(row.encoded)).toThrow();
});
test("controlled neutral lengths agree with independent bytes and the variable production allowance",()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
 for(const length of controlled.lengths){
  const generator=controlled.generator,bytes=Uint8Array.from({length},(_,index)=>(index*generator.multiplier+length*generator.lengthMultiplier)%generator.modulus),text=Buffer.from(bytes).toString("base64");
  expect(validate({maximumOutputBytes:length})).toBe(true);
  expect(base64StandardDecodeControlled(text,{maximumOutputBytes:length,progress:()=>true})).toEqual(bytes);
  expect(base64StandardEncodeControlled(bytes,{maximumOutputBytes:text.length,progress:()=>true})).toBe(text);
  if(length){expect(()=>base64StandardDecodeControlled(text,{maximumOutputBytes:length-1,progress:()=>true})).toThrow();expect(()=>base64StandardEncodeControlled(bytes,{maximumOutputBytes:text.length-1,progress:()=>true})).toThrow();}
 }
 for(const maximumOutputBytes of [-1,.5,"8",Number.MAX_SAFE_INTEGER+1])expect(validate({maximumOutputBytes})).toBe(false);
 expect(validate({maximumOutputBytes:0,unownedAllowance:0})).toBe(false);
});
test("controlled cancellation and strict neutral refusal retain their defining phase",()=>{
 const length=controlled.cancellationInputLength,generator=controlled.generator,bytes=Uint8Array.from({length},(_,index)=>(index*generator.multiplier+length*generator.lengthMultiplier)%generator.modulus),text=Buffer.from(bytes).toString("base64");
 for(const row of controlled.cancellations){
  const last:{event?:Base64Progress}={},progress=(event:Base64Progress)=>{last.event=event;return !(event.phase===row.phase&&event.completed===row.completed);};
  expect(()=>row.phase==="encode"?base64StandardEncodeControlled(bytes,{maximumOutputBytes:text.length,progress}):base64StandardDecodeControlled(text,{maximumOutputBytes:length,progress})).toThrow();
  expect(String(last.event?.phase)).toBe(row.phase);expect(last.event?.completed).toBe(row.completed);
 }
 for(const row of controlled.malformed){try{base64StandardDecodeControlled(row.encoded,{maximumOutputBytes:row.encoded.length,progress:()=>true});throw Error("neutral malformed value accepted");}catch(error){expect(error).toBeInstanceOf(Base64DecodeError);expect(String((error as Base64DecodeError).detail.kind)).toBe(row.kind);}}
});
test("retained work grants preserve the original source and terminal output",()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile({...schema,$ref:"#/$defs/StepGrant"});
 const capacity=new Ajv({strict:true,allErrors:true}).compile({...schema,$ref:"#/$defs/OutputCapacity"});
 for(const value of [-1,.5,"4",Number.MAX_SAFE_INTEGER+1])expect(capacity(value)).toBe(false);
 for(const maximumWorkUnits of controlled.stepGrants)expect(validate({maximumWorkUnits})).toBe(true);
 for(const maximumWorkUnits of [-1,.5,"4",Number.MAX_SAFE_INTEGER+1])expect(validate({maximumWorkUnits})).toBe(false);
 for(const length of controlled.lengths){
  const generator=controlled.generator,bytes=Uint8Array.from({length},(_,index)=>(index*generator.multiplier+length*generator.lengthMultiplier)%generator.modulus),text=Buffer.from(bytes).toString("base64"),cursor=new Base64DecodeCursor(text);let calls=0,turns=0;
  const control={maximumOutputBytes:length,progress:()=>{calls++;return true;}};
  expect(cursor.takeOutput()).toBeNull();
  expect(cursor.outputCapacity()).toBe(0);
  while(!cursor.isComplete()){
   const demand=cursor.nextWorkDemand(),before=cursor.progress();expect(demand).toBeGreaterThan(0);expect(demand).toBeLessThanOrEqual(8192);
   for(const grant of controlled.stepGrants.filter(grant=>grant<demand)){const prior=calls;expect(cursor.step(grant,control)).toBe("pending");expect(cursor.progress()).toEqual(before);expect(calls).toBe(prior);}
   cursor.step(demand,control);expect(capacity(cursor.outputCapacity())).toBe(true);expect([0,length]).toContain(cursor.outputCapacity());expect(++turns).toBeLessThan(100000);
   if(!cursor.isComplete())expect(cursor.takeOutput()).toBeNull();
  }
  expect(cursor.outputCapacity()).toBe(length);expect(cursor.takeOutput()).toEqual(bytes);expect(cursor.outputCapacity()).toBe(0);expect(cursor.takeOutput()).toBeNull();
  expect(cursor.cancel()).toBe(false);const parts=cursor.intoParts();expect(parts.input).toBe(text);expect(parts.output).toBeNull();expect(parts.outcome).toEqual({kind:"complete"});expect(()=>cursor.step(8192,control)).toThrow();
 }
});
test("retained refusal moves the original partial output and freezes its first cause",()=>{
 const length=controlled.cancellationInputLength,generator=controlled.generator,bytes=Uint8Array.from({length},(_,index)=>(index*generator.multiplier+length*generator.lengthMultiplier)%generator.modulus),text=Buffer.from(bytes).toString("base64"),cursor=new Base64DecodeCursor(text),control={maximumOutputBytes:length,progress:(event:Base64Progress)=>event.phase!=="decode"||event.completed!==4096};let cause:unknown;
 try{while(!cursor.isComplete())cursor.step(cursor.nextWorkDemand(),control);}catch(error){cause=error;}
 expect(cause).toBeInstanceOf(Error);expect(cursor.outputCapacity()).toBe(length);expect(cursor.takeOutput()).toBeNull();expect(cursor.outputCapacity()).toBe(length);control.maximumOutputBytes=0;
 try{cursor.step(8192,control);throw Error("retained refusal lost");}catch(error){expect(error).toBe(cause);}
 const parts=cursor.intoParts();expect(parts.input).toBe(text);expect(firstRefusal(parts)).toBe(cause);expect(parts.written).toBe(3072);expect(parts.output?.length).toBe(length);expect(parts.output?.subarray(0,parts.written)).toEqual(bytes.subarray(0,3072));expect(()=>cursor.intoParts()).toThrow();
 for(const row of controlled.malformed){
  const malformed=new Base64DecodeCursor(row.encoded);let cause:unknown;
  try{while(!malformed.isComplete())malformed.step(malformed.nextWorkDemand(),{maximumOutputBytes:row.encoded.length,progress:()=>true});}catch(error){cause=error;}
  expect(cause).toBeInstanceOf(Base64DecodeError);expect(String((cause as Base64DecodeError).detail.kind)).toBe(row.kind);expect(malformed.cancel()).toBe(false);
  const parts=malformed.intoParts();expect(parts.input).toBe(row.encoded);expect(parts.output).toBeNull();expect(firstRefusal(parts)).toBe(cause);
 }
 const competing=controlled.competingRefusal,input=Buffer.from(competing.block.repeat(competing.repeats));input[competing.invalidByteOffset]=32;input[competing.paddingOffset]=61;
 try{base64StandardDecodeControlled(input.toString(),{maximumOutputBytes:input.length,progress:()=>true});throw Error("competing refusal accepted");}catch(error){expect(error).toBeInstanceOf(Base64DecodeError);expect(String((error as Base64DecodeError).detail.kind)).toBe(competing.kind);}
 const thrown=new Base64DecodeCursor("Zm9v");let caught=false;
 try{thrown.step(1,{maximumOutputBytes:3,progress:()=>{throw undefined;}});}catch(error){caught=true;expect(error).toBeUndefined();}
 expect(caught).toBe(true);caught=false;
 try{thrown.step(8192,{maximumOutputBytes:0,progress:()=>true});}catch(error){caught=true;expect(error).toBeUndefined();}
 expect(caught).toBe(true);const outcome=thrown.intoParts().outcome;expect(outcome.kind).toBe("refused");expect(outcome.kind==="refused"?outcome.refusal:"lost refusal").toBeUndefined();
 const cancelled=new Base64DecodeCursor("TWFu");let cancelledCause:unknown;
 try{cancelled.step(1,{maximumOutputBytes:3,progress:()=>{cancelled.cancel();throw undefined;}});}catch(error){cancelledCause=error;}
 expect(cancelledCause).toBeInstanceOf(Error);expect(cancelled.cancel()).toBe(false);expect(firstRefusal(cancelled.intoParts())).toBe(cancelledCause);
 const invalidGrant=new Base64DecodeCursor("TWFu");let grantCause:unknown;
 try{invalidGrant.step(-1,{maximumOutputBytes:3,progress:()=>true});}catch(error){grantCause=error;}
 expect(grantCause).toBeInstanceOf(RangeError);
 try{invalidGrant.step(100,{maximumOutputBytes:3,progress:()=>true});throw Error("invalid grant refusal lost");}catch(error){expect(error).toBe(grantCause);}
 expect(firstRefusal(invalidGrant.intoParts())).toBe(grantCause);
});
test("retained input ranges preserve the original backing and span-relative progress",()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile({...schema,$ref:"#/$defs/InputRange"});
 for(const length of controlled.lengths){
  const generator=controlled.generator,bytes=Uint8Array.from({length},(_,index)=>(index*generator.multiplier+length*generator.lengthMultiplier)%generator.modulus),encoded=Buffer.from(bytes).toString("base64");
  for(const row of controlled.inputRanges){
   const text=row.prefix+encoded+row.suffix,range={start:row.utf16Start,end:row.utf16Start+encoded.length};expect(validate(range)).toBe(true);
   const cursor=new Base64DecodeCursor(text,range),control={maximumOutputBytes:length,progress:(event:Base64Progress)=>{expect(event.total).toBe(encoded.length);expect(event.completed).toBeLessThanOrEqual(event.total);return true;}};
   range.start=0;range.end=0;
   while(!cursor.isComplete())cursor.step(cursor.nextWorkDemand(),control);
   expect(cursor.takeOutput()).toEqual(bytes);expect(Buffer.from(bytes)).toEqual(Buffer.from(text.slice(row.utf16Start,row.utf16Start+encoded.length),"base64"));
   const parts=cursor.intoParts();expect(parts.input).toBe(text);expect(parts.range).toEqual({start:row.utf16Start,end:row.utf16Start+encoded.length});expect(parts.outcome.kind).toBe("complete");
  }
 }
 for(const row of controlled.invalidInputRanges){
  const cursor=new Base64DecodeCursor(row.input,row.utf16);let cause:unknown;
  try{cursor.step(100,{maximumOutputBytes:100,progress:()=>{throw Error("invalid range reached progress");}});}catch(error){cause=error;}
  expect(cause).toBeInstanceOf(RangeError);const parts=cursor.intoParts();expect(parts.input).toBe(row.input);expect(parts.range).toEqual(row.utf16);expect(parts.output).toBeNull();expect(firstRefusal(parts)).toBe(cause);
 }
 for(const range of [{start:-1,end:4},{start:0.5,end:4},{start:0,end:5.5}])expect(validate(range)).toBe(false);
 const length=controlled.cancellationInputLength,generator=controlled.generator,bytes=Uint8Array.from({length},(_,index)=>(index*generator.multiplier+length*generator.lengthMultiplier)%generator.modulus),text="pk:"+Buffer.from(bytes).toString("base64")+":end",range={start:3,end:text.length-4},cursor=new Base64DecodeCursor(text,range);let cause:unknown;
 try{while(!cursor.isComplete())cursor.step(cursor.nextWorkDemand(),{maximumOutputBytes:length,progress:event=>event.phase!=="decode"||event.completed!==4096});}catch(error){cause=error;}
 const parts=cursor.intoParts();expect(cause).toBeInstanceOf(Error);expect(firstRefusal(parts)).toBe(cause);expect(parts.input).toBe(text);expect(parts.range).toEqual(range);expect(parts.output?.subarray(0,parts.written)).toEqual(bytes.subarray(0,3072));
 console.log("[DEBUG] Base64 ranged source agrees with Buffer and retains complete original text");
});
test.each(controlled.callbackMutations)("retained progress %s cannot reenter or consume its active owner",mutation=>{
  const cursor=new Base64DecodeCursor("TWFu"),control={maximumOutputBytes:3,progress:()=>{
   if(mutation==="cancel"){cursor.cancel();return true;}
   if(mutation==="withdraw")cursor.intoParts();else cursor.step(100,{maximumOutputBytes:3,progress:()=>true});
   return true;
  }};let cause:unknown;
  try{cursor.step(100,control);}catch(error){cause=error;}
  expect(cause).toBeInstanceOf(Error);expect(cursor.isComplete()).toBe(false);expect(cursor.takeOutput()).toBeNull();
  const parts=cursor.intoParts();expect(parts.input).toBe("TWFu");expect(parts.output).toBeNull();expect(firstRefusal(parts)).toBe(cause);
});
