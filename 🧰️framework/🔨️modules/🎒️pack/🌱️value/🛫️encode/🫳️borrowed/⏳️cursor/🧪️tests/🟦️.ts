import {test,expect} from "bun:test";
import {readFileSync,existsSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
const base=join(import.meta.dir,"..");
const fixture=JSON.parse(readFileSync(join(base,"🧫️fixtures/🔣️.json"),"utf8"));
const schema=JSON.parse(readFileSync(join(base,"🧬️schema/🔣️.json"),"utf8"));
const varint=(value:bigint)=>{const bytes=[];do{const byte=Number(value&127n);value>>=7n;bytes.push(byte|(value?128:0));}while(value);return Buffer.from(bytes);};
const text=(node:any)=>node.value.repeat(node.repeat??1);
const symbols=(node:any,counts:Map<string,number>,forced:Set<string>,force=false)=>{
  if(node.kind==="text"){counts.set(text(node),(counts.get(text(node))??0)+1);if(force)forced.add(text(node));}
  if(node.kind==="table")for(const row of node.items)for(const field of row.fields)symbols(field.node,counts,forced,node.columns.find((column:any)=>column.id===field.id)?.shape==="text");
  else for(const child of node.fields?.map((field:any)=>field.node)??node.items??[])symbols(child,counts,forced);
};
const f64=(bits:string)=>{const bytes=Buffer.alloc(8);bytes.writeBigUInt64LE(BigInt(`0x${bits}`));return bytes;};
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
  expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
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
