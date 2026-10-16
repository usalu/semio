/** 🔎️ Strict language-neutral scalar operation-wire encoder fixture: the symbol-table record body written once by hand and once through independent LEB128 and IEEE754 libraries, so any owner can compare its own wire against both.
 * @see https://github.com/xtuc/webassemblyjs/tree/master/packages/leb128
 * @see https://github.com/xtuc/webassemblyjs/tree/master/packages/ieee754 */
import assert from "node:assert/strict";
import { createRequire } from "node:module";

export type ScalarRecordFixtureField = {type:"text";unit:string;repeat:number;suffix:string}|{type:"u64"|"f64";value:string}|null;
export type ScalarRecordFixtureCase = {id:string;ordinal:number;fields:ScalarRecordFixtureField[];wireBytes:number;symbols:number};
const requireOracle = createRequire(import.meta.url);
type LebOracle = {encodeUIntBuffer(bytes:Buffer):Uint8Array};
type Ieee754Oracle = {encodeF64(value:number):Uint8Array;decodeF64(bytes:ArrayLike<number>):number};
let lebOracle:LebOracle|null=null,ieee754Oracle:Ieee754Oracle|null=null;
const leb = ():LebOracle => lebOracle ??= ((module:{default?:LebOracle}&LebOracle)=>module.default ?? module)(requireOracle("@webassemblyjs/leb128/lib/leb.js"));
const ieee754 = ():Ieee754Oracle => ieee754Oracle ??= requireOracle("@webassemblyjs/ieee754") as Ieee754Oracle;
function unsigned(value:bigint):number[] { const bytes:number[]=[]; do { let next=Number(value&127n); value>>=7n; if(value)next|=128; bytes.push(next); }while(value); return bytes; }
function oracleUnsigned(value:bigint):number[] { const bytes=Buffer.alloc(8);bytes.writeBigUInt64LE(value);return [...leb().encodeUIntBuffer(bytes)]; }
function float(value:number):number[] { const bytes=Buffer.alloc(8);bytes.writeDoubleLE(value);return [...bytes]; }
/** 📦️ Encodes one scalar record with the hand-written encoder (`oracle` false) or the independent library encoders (`oracle` true). */
export function encodeScalarRecordFixture(test:ScalarRecordFixtureCase,oracle:boolean):{bytes:Buffer;symbols:number} {
  const texts=test.fields.flatMap(field=>field?.type==="text"?[field.unit.repeat(field.repeat)+field.suffix]:[]);
  const symbols=[...new Set(texts)].filter(text=>Buffer.byteLength(text)<=128||texts.filter(other=>other===text).length>=2).sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b)));
  const integer=oracle?oracleUnsigned:unsigned;const output=[1,...integer(BigInt(test.ordinal)),...integer(BigInt(symbols.length))];
  for(const symbol of symbols)output.push(...integer(BigInt(Buffer.byteLength(symbol))),...Buffer.from(symbol));
  output.push(...integer(BigInt(test.fields.filter(Boolean).length)));
  test.fields.forEach((field,index)=>{if(!field)return;output.push(...integer(BigInt(index)));
    if(field.type==="text"){const text=field.unit.repeat(field.repeat)+field.suffix;const symbol=symbols.indexOf(text);output.push(symbol<0?7:6,...integer(BigInt(symbol<0?Buffer.byteLength(text):symbol)));if(symbol<0)output.push(...Buffer.from(text));}
    else if(field.type==="u64")output.push(4,...integer(BigInt(field.value)));
    else {const number=Number(field.value);let bytes=oracle?[...ieee754().encodeF64(number)]:float(number);if(Number.isNaN(number)&&oracle){const raw=new ArrayBuffer(8);new DataView(raw).setFloat64(0,number,true);bytes=[...new Uint8Array(raw)];assert.ok(Number.isNaN(ieee754().decodeF64(bytes)));}output.push(5,...bytes);}
  });
  return {bytes:Buffer.from(output),symbols:symbols.length};
}
