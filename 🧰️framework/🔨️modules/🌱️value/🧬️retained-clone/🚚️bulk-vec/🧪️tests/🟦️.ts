import Ajv from "ajv";
import {expect,test} from "bun:test";
import law from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

const sample=(index:number)=>(index*law.sample.multiplier+law.sample.offset)%law.sample.modulo;

test("bulk bitwise vector clone chunks agree with an independent page-wise typed-array copy",()=>{
  const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,pageBytes:4096})).toBe(false);expect(validate({...law,overheadTurns:0})).toBe(false);
  for(const count of law.sampleCounts){
    const source=new Uint16Array(count);for(let index=0;index<count;index++)source[index]=sample(index);
    const bytes=new Uint8Array(source.buffer),target=new Uint8Array(bytes.byteLength);let chunks=0;
    for(let start=0;start<bytes.byteLength;start+=law.pageBytes){target.set(bytes.subarray(start,Math.min(start+law.pageBytes,bytes.byteLength)),start);chunks++;}
    expect(Array.from(new Uint16Array(target.buffer))).toEqual(Array.from(source));
    expect(chunks).toBe(Math.ceil(bytes.byteLength/law.pageBytes));
    const turns=law.overheadTurns+Math.max(chunks,law.emptyCompletionTurns);
    expect(turns).toBeLessThanOrEqual(law.overheadTurns+Math.max(Math.ceil(count*law.elementBytes/law.pageBytes),law.emptyCompletionTurns));
    if(count===2097152){expect(bytes.byteLength).toBe(4*1024*1024);expect(turns).toBe(law.maximumTurnsForFourMebibytes);expect(count*law.elementwiseTurnsPerElement/turns).toBeGreaterThan(400000);}
  }
  console.log("[DEBUG] Bulk vector clone chunk arithmetic and sample bytes agree with an independent typed-array page copy");
});
