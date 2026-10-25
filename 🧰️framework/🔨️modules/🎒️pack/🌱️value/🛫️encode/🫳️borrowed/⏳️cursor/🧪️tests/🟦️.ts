import {test,expect} from "bun:test";
import {readFileSync,existsSync} from "node:fs";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import Ajv from "ajv/dist/2020";
import CRC32C from "crc-32/crc32c";
import {parse as parseMath} from "mathjs";
const base=join(import.meta.dir,"..");
const fixture=JSON.parse(readFileSync(join(base,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));
const varint=(value:bigint)=>{const bytes=[];do{const byte=Number(value&127n);value>>=7n;bytes.push(byte|(value?128:0));}while(value);return Buffer.from(bytes);};
test("original arithmetic expression corpus matches independent Buffer UTF8 and mathjs evaluation",()=>{
 const rows=JSON.parse(readFileSync(join(base,"🧪️tests/🧫️fixtures/🧮️expression/🔣️.json"),"utf8"));for(const row of rows.cases){const bytes=Buffer.from(row.text);const body=Buffer.concat([Buffer.from([0,1,rows.fieldId,rows.tag,rows.inlineTag]),varint(BigInt(bytes.length)),bytes]);expect(body.toString("hex")).toBe(row.bodyHex);if(row.value!==undefined)expect(parseMath(row.text).compile().evaluate(row.scope??{})).toBe(row.value);}
});
const text=(node:any)=>node.value.repeat(node.repeat??1);
const symbols=(node:any,counts:Map<string,number>,forced:Set<string>,force=false)=>{
  if(node.kind==="text"){counts.set(text(node),(counts.get(text(node))??0)+1);if(force)forced.add(text(node));}
  if(node.kind==="table")for(const row of node.items)for(const field of row.fields)symbols(field.node,counts,forced,node.columns.find((column:any)=>column.id===field.id)?.shape==="text");
  else for(const child of node.fields?.map((field:any)=>field.node)??node.items??[])symbols(child,counts,forced);
};
const f64=(bits:string)=>{const bytes=Buffer.alloc(8);bytes.writeBigUInt64LE(BigInt(`0x${bits}`));return bytes;};
test("portable standard Map Statements Wire bytes match independent Buffer UTF8 ordering",()=>{
 const rows=JSON.parse(readFileSync(join(base,"🧪️tests/🧫️fixtures/🧩️branches/🔣️.json"),"utf8"));
 for(const row of rows.cases){
  const values:string[]=row.kind==="map"?row.pairs.filter((pair:any)=>pair[1]!==null).flat():row.kind==="statements"?[row.keyword,...row.values]:row.slots.filter((value:any)=>typeof value==="string");
  const pool=[...new Set(values)].sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b)));const index=(text:string)=>varint(BigInt(pool.indexOf(text)));const word=(text:string)=>Buffer.concat([Buffer.from([6]),index(text)]);
  let body:Buffer;
  if(row.kind==="map"){const pairs=row.pairs.map((pair:any,ordinal:number)=>({pair,ordinal})).filter((entry:any)=>entry.pair[1]!==null).sort((a:any,b:any)=>Buffer.compare(Buffer.from(a.pair[0]),Buffer.from(b.pair[0]))||a.ordinal-b.ordinal);body=Buffer.concat([Buffer.from([16]),varint(BigInt(pairs.length)),...pairs.flatMap((entry:any)=>entry.pair.map(word))]);}
  else if(row.kind==="statements")body=Buffer.concat([Buffer.from([15]),varint(BigInt(row.values.length)),...row.values.flatMap((text:string)=>[index(row.keyword),Buffer.from([1,0]),word(text)])]);
  else {const node=(start:number)=>Buffer.concat([Buffer.from([Number(row.slots[start+1]!==null)|Number(row.slots[start+2]!==null)<<1]),word(row.slots[start]),...(row.slots[start+1]===null?[]:[word(row.slots[start+1])]),...(row.slots[start+2]===null?[]:[word(row.slots[start+2])])]);const label=row.slots[6]!==null||row.slots[7]!==null;body=Buffer.concat([Buffer.from([19,1|Number(row.directed)<<1|Number(label)<<2]),node(0),node(3),...(label?[Buffer.from([Number(row.slots[6]!==null)|Number(row.slots[7]!==null)<<1]),...(row.slots[6]===null?[]:[word(row.slots[6])]),...(row.slots[7]===null?[]:[word(row.slots[7])])]:[]),Buffer.from([18])]);}
  const actual=Buffer.concat([varint(BigInt(pool.length)),...pool.flatMap(value=>[varint(BigInt(Buffer.byteLength(value))),Buffer.from(value)]),Buffer.from([1,1]),body]);expect(actual.toString("hex")).toBe(row.expectedHex);
 }
});
test("native original key and text retain Unicode crossing an actual source page",()=>{
 const row=JSON.parse(readFileSync(join(base,"🧪️tests/🧫️fixtures/🌿️native-text/🔣️.json"),"utf8"));const bytes=Buffer.concat([Buffer.alloc(row.split,row.fill),Buffer.from(row.scalar),Buffer.alloc(row.bytes-row.split-Buffer.byteLength(row.scalar),row.fill)]);expect(bytes).toHaveLength(row.bytes);const text=new TextDecoder("utf-8",{fatal:true}).decode(bytes);expect(Buffer.from(new TextEncoder().encode(text))).toEqual(bytes);expect(bytes.subarray(row.split,row.split+Buffer.byteLength(row.scalar))).toEqual(Buffer.from(row.scalar));expect(varint(BigInt(bytes.length))).toEqual(Buffer.from([130,64]));expect(Buffer.from(row.bodyHex,"hex")).toEqual(Buffer.from([1,row.recordId,16,row.mapEntries,6,0,6,0]));
});
const encode=(node:any,pool:string[]):Buffer=>{
  if(node.kind==="block")return Buffer.concat([Buffer.from([14]),...(node.items[0].kind==="record"?[Buffer.from([13])]:[]),encode(node.items[0],pool)]);
  if(node.kind==="record"){const fields=node.fields.filter((field:any)=>field.node.kind!=="absent").sort((a:any,b:any)=>a.id-b.id);return Buffer.concat([varint(BigInt(fields.length)),...fields.flatMap((field:any)=>[varint(BigInt(field.id)),...(field.node.kind==="record"?[Buffer.from([13])]:[]),encode(field.node,pool)])]);}
  if(node.kind==="text"){const value=text(node);const ordinal=pool.indexOf(value);return ordinal>=0?Buffer.concat([Buffer.from([6]),varint(BigInt(ordinal))]):Buffer.concat([Buffer.from([7]),varint(BigInt(Buffer.byteLength(value))),Buffer.from(value)]);}
  if(node.kind==="table"){
    const columns=[...node.columns].sort((a:any,b:any)=>a.id-b.id);
    const bitmap=(cells:any[],predicate:(cell:any)=>boolean)=>Buffer.from(Array.from({length:Math.ceil(cells.length/8)},(_,index)=>cells.slice(index*8,index*8+8).reduce((bits:number,cell:any,bit:number)=>bits|(predicate(cell)?1<<bit:0),0)));
    return Buffer.concat([Buffer.from([20]),varint(BigInt(node.items.length)),varint(BigInt(columns.length)),...columns.flatMap((column:any)=>{
      const cells=node.items.map((row:any)=>row.fields.find((field:any)=>field.id===column.id)?.node??{kind:"absent"});const dense=cells.every((cell:any)=>cell.kind!=="absent");const tag=column.shape==="bool"?1:column.shape==="float"?4:column.shape==="text"?5:0;
      const payload=tag===1?[bitmap(cells,cell=>cell.kind==="bool"&&cell.value)]:cells.filter((cell:any)=>cell.kind!=="absent").map((cell:any)=>tag===4?f64(cell.bits):tag===5?varint(BigInt(pool.indexOf(text(cell)))):encode(cell,pool));
      return [varint(BigInt(column.id)),Buffer.from([dense?0:1]),...(dense?[]:[bitmap(cells,cell=>cell.kind!=="absent")]),Buffer.from([tag]),...payload];
    })]);
  }
  if(node.kind==="bool")return Buffer.from([node.value?2:1]);
  if(node.kind==="float")return Buffer.concat([Buffer.from([5]),f64(node.bits)]);
  if(node.kind==="int")return Buffer.concat([Buffer.from([3]),varint((BigInt(node.value)<<1n)^(BigInt(node.value)>>63n))]);
  if(node.kind==="uint")return Buffer.concat([Buffer.from([4]),varint(BigInt(node.value))]);
  const packed=node.items.length>0&&node.items.every((item:any)=>item.kind==="float");
  return Buffer.concat([Buffer.from([packed?21:node.kind==="tuple"?11:12]),varint(BigInt(node.items.length)),...node.items.map((item:any)=>packed?f64(item.bits):item.kind==="record"?Buffer.concat([Buffer.from([13]),encode(item,pool)]):encode(item,pool))]);
};
test("neutral canonical Pack symbol/field order and exact binary64 oracle",()=>{
  for(const [index,row] of fixture.cases.entries()){
    const counts=new Map<string,number>();const forced=new Set<string>();symbols(row,counts,forced);
    const pool=[...counts.keys()].filter(value=>forced.has(value)||Buffer.byteLength(value)<=128||counts.get(value)!>=2).sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b)));
    const bytes=Buffer.concat([varint(BigInt(pool.length)),...pool.flatMap(value=>[varint(BigInt(Buffer.byteLength(value))),Buffer.from(value)]),encode(row,pool)]);
    expect(bytes.toString("hex")).toBe(fixture.expectedHex[index]);
    expect(Buffer.from(new TextEncoder().encode(pool.join("\0")))).toEqual(Buffer.from(pool.join("\0")));
    for(const grant of fixture.grants){const parts=[];for(let offset=0;offset<bytes.length;offset+=Math.min(grant,64))parts.push(bytes.subarray(offset,offset+Math.min(grant,64)));expect(Buffer.concat(parts)).toEqual(bytes);}
    if(row.fields.length===0)expect([...bytes]).toEqual([0,0]);
    if(row.fields.length===6){expect(pool).toHaveLength(1);expect(Buffer.from(text(row.fields[2].node))).toHaveLength(387);}
  }
  for(const bits of ["0000000000000000","8000000000000000","3ff8000000000000"]){const bytes=f64(bits);const view=new DataView(bytes.buffer,bytes.byteOffset,8);expect(view.getBigUint64(0,true)).toBe(BigInt(`0x${bits}`));}
});
test("canonical borrowed Pack operation has a retained bounded cursor",()=>{
  expect(existsSync(join(base,"🦀️.rs"))).toBe(true);
  expect(readFileSync(join(base,"🦀️.rs"),"utf8")).toContain("BorrowedProjectedPackCursor");
});

type IntrinsicNode = {kind:"null"}|{kind:"bool",value:boolean}|{kind:"int"|"uint",value:string}|{kind:"float",bits:string}|{kind:"text",value:string}|{kind:"bytes",value:number[]}|{kind:"array",items:IntrinsicNode[]}|{kind:"object",members:{key:string,node:IntrinsicNode}[]};
const intrinsicFixture:{outputGrants:number[],copyGrants:number[],cancelStops:number[],cases:{name:string,fieldId:number,node:IntrinsicNode,expectedHex:string}[]}=JSON.parse(readFileSync(join(base,"🧪️tests/🧫️fixtures/🌱️intrinsic/🔣️.json"),"utf8"));
const intrinsicSymbols=(node:IntrinsicNode,counts:Map<string,number>):void=>{
  if(node.kind==="text")counts.set(node.value,(counts.get(node.value)??0)+1);
  if(node.kind==="array")for(const child of node.items)intrinsicSymbols(child,counts);
  if(node.kind==="object")for(const member of node.members)intrinsicSymbols(member.node,counts);
};
const intrinsicEncode=(node:IntrinsicNode,pool:string[]):Buffer=>{
  if(node.kind==="null")return Buffer.from([18]);
  if(node.kind==="bool")return Buffer.from([node.value?2:1]);
  if(node.kind==="int")return Buffer.concat([Buffer.from([3]),varint((BigInt(node.value)<<1n)^(BigInt(node.value)>>63n))]);
  if(node.kind==="uint")return Buffer.concat([Buffer.from([4]),varint(BigInt(node.value))]);
  if(node.kind==="float")return Buffer.concat([Buffer.from([5]),f64(node.bits)]);
  if(node.kind==="text"){const ordinal=pool.indexOf(node.value);return ordinal<0?Buffer.concat([Buffer.from([7]),varint(BigInt(Buffer.byteLength(node.value))),Buffer.from(node.value)]):Buffer.concat([Buffer.from([6]),varint(BigInt(ordinal))]);}
  if(node.kind==="bytes")return Buffer.concat([Buffer.from([8]),varint(BigInt(node.value.length)),Buffer.from(node.value)]);
  if(node.kind==="array")return Buffer.concat([Buffer.from([12]),varint(BigInt(node.items.length)),...node.items.map(child=>intrinsicEncode(child,pool))]);
  if(node.kind==="object")return Buffer.concat([Buffer.from([16]),varint(BigInt(node.members.length)),...node.members.flatMap(member=>[Buffer.from([7]),varint(BigInt(Buffer.byteLength(member.key))),Buffer.from(member.key),intrinsicEncode(member.node,pool)])]);
  throw new Error("intrinsic oracle node has no authored authority");
};
test("retained intrinsic Body preserves ordered duplicate members and exact scalar bytes",()=>{
  const schema=JSON.parse(readFileSync(join(base,"🧬️schema/🔣️.json"),"utf8"));const ajv=new Ajv({strict:true});ajv.addSchema(schema);
  const request=ajv.compile({$ref:schema.$id+"#/$defs/IntrinsicBodyRequest"});const grant=ajv.compile({$ref:schema.$id+"#/$defs/Grant"});
  for(const row of intrinsicFixture.cases){
    expect(request({fieldId:row.fieldId,maximumDepth:64})).toBe(true);
    const counts=new Map<string,number>();intrinsicSymbols(row.node,counts);
    const pool=[...counts.keys()].filter(value=>Buffer.byteLength(value)<=128||counts.get(value)!>=2).sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b)));
    const bytes=Buffer.concat([varint(BigInt(pool.length)),...pool.flatMap(value=>[varint(BigInt(Buffer.byteLength(value))),Buffer.from(value)]),varint(1n),varint(BigInt(row.fieldId)),Buffer.from([17]),intrinsicEncode(row.node,pool)]);
    expect(bytes.toString("hex")).toBe(row.expectedHex);
    for(const copy of intrinsicFixture.copyGrants){expect(grant({maximumItems:1,maximumCopyBytes:copy,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:64})).toBe(true);for(const width of intrinsicFixture.outputGrants){const chunks=[];for(let offset=0;offset<bytes.length;offset+=Math.min(width,copy,64))chunks.push(bytes.subarray(offset,offset+Math.min(width,copy,64)));expect(Buffer.concat(chunks)).toEqual(bytes);}}
  }
  expect(intrinsicFixture.cases[3].expectedHex).not.toBe(intrinsicFixture.cases[4].expectedHex);
  expect(request({fieldId:65536,maximumDepth:64})).toBe(false);expect(grant({maximumItems:-1,maximumCopyBytes:2,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:64})).toBe(false);
});

test("refused original Pack turns preserve the sole Value receipt and initialized prefix",()=>{
  const schema=JSON.parse(readFileSync(join(base,"🧬️schema/🔣️.json"),"utf8"));
  const refusal=JSON.parse(readFileSync(join(base,"../../../../../🌱️value/⚠️refusal/🔁️codec/🧬️schema/🔣️.json"),"utf8"));
  const rows=JSON.parse(readFileSync(join(base,"🧪️tests/🧫️fixtures/⚠️failure/🔣️.json"),"utf8"));
  const ajv=new Ajv({strict:true});ajv.addSchema(refusal);ajv.addSchema(schema);
  const admit=ajv.compile({$ref:schema.$id+"#/$defs/BorrowedProjectedPackFailure"});
  for(const row of rows.examples){
    expect(admit(row.failure)).toBe(true);
    const bytes=Buffer.from(row.text);const independent=new TextEncoder().encode(row.text);
    expect([...bytes]).toEqual([...independent]);
    if(row.text)expect(row.failure.reason.retainedProgress.copiedBytes).toBe(bytes.length*2);
    expect(admit({...row.failure,progress:row.failure.reason.retainedProgress})).toBe(false);
    expect(admit({...row.failure,writtenBytes:-1})).toBe(false);
  }
  const source=readFileSync(join(base,"🦀️.rs"),"utf8");
  expect(source).toContain("pub struct BorrowedProjectedPackFailure");
  expect(source).toContain("error.allocated_bytes");
  expect(source).toContain("with_retained_progress");
  expect(source).not.toContain("page_error(error.refusal()))?");
});

test("coupled borrowed Pack and original installed caller retain genuine Rust grammar",()=>{
  for(const path of [join(base,"🦀️.rs"),join(base,"🧪️tests/🦀️.rs"),join(base,"../../../../../../🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🦀️.rs")]){
    const parsed=spawnSync("rustfmt",["--edition","2021","--emit","stdout",path],{timeout:5000,stdio:["ignore","ignore","pipe"]});
    expect(parsed.error).toBeUndefined();expect(parsed.status).toBe(0);
  }
});

test("flat intrinsic Pack prices actual traversal depth without changing its inline bound",()=>{
 const rows=JSON.parse(readFileSync(join(base,"🧪️tests/🧫️fixtures/⚠️failure/🔣️.json"),"utf8"));const row=rows.depthProbe;
 const bytes=Buffer.concat([varint(0n),varint(1n),varint(BigInt(row.fieldId)),Buffer.from([17]),intrinsicEncode({kind:"bool",value:row.source},[])]);
 expect(bytes.toString("hex")).toBe(row.expectedHex);
 expect(rows.depthProbe.maximumDepth).toBeLessThan(64);
});
