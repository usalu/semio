/** 🔎️ Strict language-neutral scalar operation-wire fixtures with independent LEB128 and IEEE754 oracles. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { encodeScalarRecordFixture, type ScalarRecordFixtureCase as Case, type ScalarRecordFixtureField as Field } from "@semio-tech/framework-pack-record-ts/scalar-witness/testing";
import { valueScalarWitnessFixtureUrl } from "@semio-tech/framework-pack-ts/testing/fixtures";

//#region 🔣️Fixture
type Fixture = {version:number;grants:number[];cancelAfterSteps:number[];terminalEmpty:boolean;capture:{ordinal:number;value:number;laterOrdinal:number;laterValue:number;projections:number;wire:number[]};cases:Case[]};
//#endregion 🔣️Fixture

//#region 🧪️Oracle
export function testScalarRecordWireFixture():void {
  const fixture:Fixture=JSON.parse(readFileSync(valueScalarWitnessFixtureUrl,"utf8"));
  
  assert.equal(new Set(fixture.cases.map(test=>test.id)).size,fixture.cases.length);
  assert.equal(fixture.version,1);assert.deepEqual(fixture.grants,[1,64,4096]);assert.deepEqual(fixture.cancelAfterSteps,[0,1,17,4097]);
  assert.equal(fixture.cases.length,9);assert.deepEqual(fixture.capture,{ordinal:3,value:7,laterOrdinal:99,laterValue:123,projections:1,wire:[1,3,0,1,0,4,7]});
  const captured=fixture.capture;
  const before:Case={id:"capture",ordinal:captured.ordinal,fields:[{type:"u64",value:String(captured.value)},null,null],wireBytes:captured.wire.length,symbols:0};
  assert.deepEqual([...encodeScalarRecordFixture(before,true).bytes],captured.wire);
  const later:Case={...before,ordinal:captured.laterOrdinal,fields:[{type:"u64",value:String(captured.laterValue)},null,null]};
  assert.notDeepEqual(encodeScalarRecordFixture(before,true).bytes,encodeScalarRecordFixture(later,true).bytes);
  for(const test of fixture.cases){assert.ok(test.fields.filter(field=>field?.type==="text").length<=2);for(const field of test.fields)if(field?.type==="u64")assert.ok(BigInt(field.value)<=18446744073709551615n);
    const actual=encodeScalarRecordFixture(test,false),oracle=encodeScalarRecordFixture(test,true);assert.deepEqual(actual,oracle);assert.equal(actual.bytes.length,test.wireBytes,test.id);assert.equal(actual.symbols,test.symbols,test.id);
    for(const grant of fixture.grants){const partitioned=Buffer.concat(Array.from({length:Math.ceil(actual.bytes.length/grant)},(_,i)=>actual.bytes.subarray(i*grant,(i+1)*grant)));assert.deepEqual(partitioned,oracle.bytes);}
    for(const cancel of fixture.cancelAfterSteps){const prefix=actual.bytes.subarray(0,Math.min(cancel,actual.bytes.length));assert.deepEqual(prefix,oracle.bytes.subarray(0,prefix.length));}
    assert.equal(createHash("sha256").update(actual.bytes).digest("hex"),createHash("sha256").update(oracle.bytes).digest("hex"));
  }
}
//#endregion 🧪️Oracle
