import{test,expect}from"bun:test";
import Ajv from"ajv";
import stableStringify from"fast-json-stable-stringify";

type Corpus={ownerCounts:number[],copyGrants:number[],cases:{text:string,reservedCapacity:number}[],cancelCuts:number[]};
const fixture:Corpus=await Bun.file(new URL("../🧫️fixtures/🔣️.json",import.meta.url)).json();
const schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();
const grants=await Bun.file(new URL("../../../🗂️ordered/♻️retirement/🧬️schema/🔣️.json",import.meta.url)).json();
test("typed retirement queue independently separates reservations, original frames, work and physical releases",()=>{
    const ajv=new Ajv({strict:true});ajv.addSchema(grants).addSchema(schema);
    const grant=ajv.compile({$ref:schema.$id+"#/$defs/Grant"});const admission=ajv.compile({$ref:schema.$id+"#/$defs/Admission"});const demand=ajv.compile({$ref:schema.$id+"#/$defs/Demand"});
    expect(admission({frameCapacityBytes:0,reservedSlot:false})).toBe(true);expect(admission({frameCapacityBytes:-1,reservedSlot:true})).toBe(false);expect(demand({copyBytes:1,capacityBytes:0,releaseBytes:257,depth:1})).toBe(true);expect(grant({maximumItems:1,maximumBytes:257})).toBe(false);
    for(const row of fixture.cases)for(const count of fixture.ownerCounts)for(const copy of fixture.copyGrants){
        expect(Buffer.byteLength(row.text)).toBe(new TextEncoder().encode(row.text).length);expect(row.reservedCapacity).toBeGreaterThan(Buffer.byteLength(row.text));
        const queue=Array.from({length:count},()=>row.text);expect(JSON.parse(stableStringify(queue))).toEqual(queue);
        expect(grant({maximumItems:1,maximumCopyBytes:copy,maximumCapacityBytes:0,maximumReleaseBytes:row.reservedCapacity,maximumDepth:count+1})).toBe(true);
        let bytes=0;for(const source of queue){let remaining=Buffer.byteLength(source);while(remaining>0){const processed=Math.min(copy,remaining);remaining-=processed;bytes+=processed;}}
        expect(bytes).toBe(new TextEncoder().encode(row.text).length*count);
    }
});
