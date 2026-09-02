/** 🔎️ Strict language-neutral scalar operation-wire fixtures with independent LEB128 and IEEE754 oracles. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import Ajv from "ajv";
import lebModule from "@webassemblyjs/leb128/lib/leb.js";
import { encodeF64, decodeF64 } from "@webassemblyjs/ieee754";

//#region 🔣️Fixture
type Field = {type:"text";unit:string;repeat:number;suffix:string}|{type:"u64"|"f64";value:string}|null;
type Case = {id:string;ordinal:number;fields:Field[];wireBytes:number;symbols:number};
type Fixture = {version:number;grants:number[];cancelAfterSteps:number[];terminalEmpty:boolean;capture:{ordinal:number;value:number;laterOrdinal:number;laterValue:number;projections:number;wire:number[]};cases:Case[]};
const leb = (lebModule as unknown as {default?:typeof lebModule}).default ?? lebModule;
function unsigned(value:bigint):number[] { const bytes:number[]=[]; do { let next=Number(value&127n); value>>=7n; if(value)next|=128; bytes.push(next); }while(value); return bytes; }
function oracleUnsigned(value:bigint):number[] { const bytes=Buffer.alloc(8);bytes.writeBigUInt64LE(value);return [...leb.encodeUIntBuffer(bytes)]; }
function float(value:number):number[] { const bytes=Buffer.alloc(8); if(Number.isNaN(value))bytes.writeBigUInt64LE(0x7ff8000000000000n);else bytes.writeDoubleLE(value);return [...bytes]; }
export function encodeScalarRecordFixture(test:Case,oracle:boolean):{bytes:Buffer;symbols:number} {
  const texts=test.fields.flatMap(field=>field?.type==="text"?[field.unit.repeat(field.repeat)+field.suffix]:[]);
  const symbols=[...new Set(texts)].filter(text=>Buffer.byteLength(text)<=128||texts.filter(other=>other===text).length>=2).sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b)));
  const integer=oracle?oracleUnsigned:unsigned;const output=[1,...integer(BigInt(test.ordinal)),...integer(BigInt(symbols.length))];
  for(const symbol of symbols)output.push(...integer(BigInt(Buffer.byteLength(symbol))),...Buffer.from(symbol));
  output.push(...integer(BigInt(test.fields.filter(Boolean).length)));
  test.fields.forEach((field,index)=>{if(!field)return;output.push(...integer(BigInt(index)));
    if(field.type==="text"){const text=field.unit.repeat(field.repeat)+field.suffix;const symbol=symbols.indexOf(text);output.push(symbol<0?7:6,...integer(BigInt(symbol<0?Buffer.byteLength(text):symbol)));if(symbol<0)output.push(...Buffer.from(text));}
    else if(field.type==="u64")output.push(4,...integer(BigInt(field.value)));
    else {const number=Number(field.value);let bytes=oracle?[...encodeF64(number)]:float(number);if(Number.isNaN(number)&&oracle){assert.ok(Number.isNaN(decodeF64(bytes)));bytes=[0,0,0,0,0,0,248,127];assert.ok(Number.isNaN(decodeF64(bytes)));}output.push(5,...bytes);}
  });
  return {bytes:Buffer.from(output),symbols:symbols.length};
}
//#endregion 🔣️Fixture

//#region 🧪️Oracle
export function testScalarRecordWireFixture():void {
  const fixture:Fixture=JSON.parse(readFileSync(new URL("./🧪️fixture/🔣️.json",import.meta.url),"utf8"));
  const validate=new Ajv({strict:true,allErrors:true}).compile(JSON.parse(readFileSync(new URL("./🧪️fixture.schema.json",import.meta.url),"utf8")));
  assert.ok(validate(fixture),JSON.stringify(validate.errors));assert.equal(new Set(fixture.cases.map(test=>test.id)).size,fixture.cases.length);
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
  const hostile=[{...fixture,unexpected:true},{...fixture,grants:[0,4096]},{...fixture,terminalEmpty:false},{...fixture,cases:fixture.cases.map((test,index)=>index?test:{...test,fields:[null,null,null,null]})}];
  for(const invalid of hostile)assert.equal(validate(invalid),false);
  console.log(`[DEBUG] scalar wire source oracle: ${fixture.cases.length} exact binary cases, ${hostile.length} strict hostile fixtures; native cursor lifecycle is a separate gate`);
}
//#endregion 🧪️Oracle
