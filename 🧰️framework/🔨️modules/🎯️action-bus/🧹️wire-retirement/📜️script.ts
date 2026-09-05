/** 🧹️ Language-neutral raw page retirement model checked against Node Buffer byte ownership. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import Ajv from "ajv";

//#region 🧪️WireRetirement
export function testWireRetirementFixture():void {
  const fixture=JSON.parse(readFileSync(new URL("./🧪️fixture/🔣️.json",import.meta.url),"utf8"));
  const validate=new Ajv({strict:true,allErrors:true}).compile(JSON.parse(readFileSync(new URL("./🧪️fixture/🧬️.schema.json",import.meta.url),"utf8")));
  assert.ok(validate(fixture),JSON.stringify(validate.errors));assert.equal(new Set(fixture.cases.map((row:any)=>row.id)).size,5);
  const wire=Buffer.alloc(8);wire.writeBigUInt64LE(42n);assert.equal(wire.toString("hex"),fixture.shortClose.wireHex);
  let remaining=wire;let shortReleased=0;
  for(const row of fixture.shortClose.steps){
    const released=row.items===0?0:Math.min(row.bytes,remaining.length);
    const blocked=row.items===0||(remaining.length>0&&row.bytes===0);
    remaining=remaining.subarray(0,remaining.length-released);shortReleased+=released;
    assert.equal(blocked,row.blocked);assert.equal(released,row.releasedBytes);assert.equal(remaining.length,row.remaining);
    assert.equal(Number(!blocked&&remaining.length===0),row.releasedItems);assert.equal(shortReleased+remaining.length,fixture.shortClose.logicalBytes);
  }
  assert.equal(shortReleased,wire.length);assert.equal(validate({...fixture,shortClose:{...fixture.shortClose,backingReleaseLogicalBytes:4096}}),false);
  for(const row of fixture.cases){assert.ok(row.admitted<=row.declared);if(row.sealed)assert.equal(row.admitted,row.declared);
    const original=Buffer.from(Array.from({length:row.admitted},(_,index)=>index%251));
    for(const grant of fixture.grants){const pages=Array.from({length:Math.ceil(row.admitted/fixture.pageBytes)},(_,index)=>original.subarray(index*fixture.pageBytes,(index+1)*fixture.pageBytes));
      let capacity=Math.ceil(row.declared/fixture.pageBytes);const initialCapacity=capacity;const initialPages=pages.length;let items=0;let owned=row.admitted;let released=0;const disposal:number[]=[];
      if(grant===0&&owned>0){assert.equal(pages.reduce((sum,page)=>sum+page.length,0),owned);continue;}
      while(pages.length){const page=pages[pages.length-1];const count=Math.min(grant,page.length);assert.ok(count>0);assert.ok(count<=grant);
        for(let index=page.length-1;index>=page.length-count;index--)disposal.push(page[index]);
        pages[pages.length-1]=page.subarray(0,page.length-count);owned-=count;released+=count;
        if(!pages[pages.length-1].length){pages.pop();items++;}assert.equal(owned,pages.reduce((sum,page)=>sum+page.length,0));
      }
      if(capacity>0)items++;capacity=0;assert.equal(items,initialPages+Number(initialCapacity>0));assert.equal(owned,0);assert.equal(released,row.admitted);assert.equal(capacity,fixture.terminalBackingBytes);
      const expected=Buffer.from(original).reverse();assert.deepEqual(Buffer.from(disposal),expected);
      assert.equal(createHash("sha256").update(Buffer.from(disposal)).digest("hex"),createHash("sha256").update(expected).digest("hex"));
    }
  }
  const invalid=[{...fixture,pageBytes:4097},{...fixture,grants:[1,64]},{...fixture,terminalBackingBytes:4096},{...fixture,extra:true}];
  for(const value of invalid)assert.equal(validate(value),false);
  console.log(`[DEBUG] raw wire retirement source: ${fixture.cases.length} ownership cases, ${invalid.length+1} hostile fixtures, ${fixture.shortClose.steps.length} short-close frontiers; native grant/terminal behavior is separate`);
}
//#endregion 🧪️WireRetirement
