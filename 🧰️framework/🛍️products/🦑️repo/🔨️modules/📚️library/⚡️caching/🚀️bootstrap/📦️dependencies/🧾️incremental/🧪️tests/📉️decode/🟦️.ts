import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {decodeInstalledLedgerV1,decodeOwnershipIndexV1} from "../../🟦️.ts";
import fixtures from "./🔣️.json" with {type:"json"};

const root="/workspace",serialize=(rows:readonly unknown[])=>Buffer.from(rows.map(row=>JSON.stringify(row)+"\n").join(""));
const evidence=(bytes:Uint8Array,rows:number)=>{const sha256=createHash("sha256").update(bytes).digest("hex");return {sha256,name:sha256,bytes:bytes.length,rows};};
const query={
 installed:`SELECT json_object('path', value->>1,
  'stamp', json_array(value->>2,value->>3,value->>4,value->>5,value->>6,value->>7,value->>8),
  'settled', json(CASE value->>9 WHEN 1 THEN 'true' ELSE 'false' END),
  'children', value->>10, 'directories', value->>11, 'files', value->>12, 'links', value->>13) AS row
  FROM json_each(?) ORDER BY key`,
 ownership:`SELECT json_object('path', CASE value->>1 WHEN '.' THEN '/workspace' ELSE '/workspace/'||(value->>1) END,
  'stamp', printf('%s:%s:%s:%s:%s',value->>2,value->>3,value->>4,value->>5,value->>6),
  'count', value->>7, 'entries', (SELECT json_group_array(json_object('name',item.value->>0,'kind',item.value->>1)) FROM json_each(ledger.value->8) AS item)) AS row
  FROM json_each(?) AS ledger ORDER BY key`
};
for(const fixture of fixtures)test("incremental "+fixture.kind+" evidence projects one raw row at a time",()=>{
 const kind=fixture.kind as keyof typeof query,db=new Database(":memory:");
 try{
  const expected=db.query<{row:string},[string]>(query[kind]).all(JSON.stringify(fixture.rows)).map(row=>JSON.parse(row.row));
  const bytes=serialize(fixture.rows),sealed=evidence(bytes,fixture.rows.length),parse=JSON.parse;
  let pending=0,peak=0,parsed=0;
  JSON.parse=(text:string)=>{
   const row=parse(text);
   if(!Array.isArray(row))return row;
   pending++;parsed++;peak=Math.max(peak,pending);
   let projected=false;
   return new Proxy(row,{get(target,key,receiver){
    if(key==="0"&&!projected){projected=true;pending--;}
    return Reflect.get(target,key,receiver);
   }});
  };
  let actual;
  try{actual=kind==="installed"?decodeInstalledLedgerV1(bytes,sealed):decodeOwnershipIndexV1(root,bytes,sealed);}
  finally{JSON.parse=parse;}
  expect(actual).toEqual(expected);
  expect(parsed).toBe(fixture.rows.length);
  expect(peak).toBe(fixture.maximumPending);
  expect(pending).toBe(0);
 }finally{db.close();}
});

test("incremental evidence refuses invalid seals, row counts, UTF-8 and disconnected paths",()=>{
 const installed=fixtures[0]!.rows,ownership=fixtures[1]!.rows;
 for(const [rows,decode] of [[installed,decodeInstalledLedgerV1],[ownership,(bytes:Buffer,ref:ReturnType<typeof evidence>)=>decodeOwnershipIndexV1(root,bytes,ref)]] as const){
  const bytes=serialize(rows),sealed=evidence(bytes,rows.length),invalid=Buffer.concat([bytes,Buffer.from([0xff,0x0a])]);
  expect(()=>decode(bytes,{...sealed,sha256:"0".repeat(64)})).toThrow("Sealed");
  expect(()=>decode(bytes,{...sealed,bytes:bytes.length-1})).toThrow("Sealed");
  for(const count of [rows.length-1,rows.length+1])expect(()=>decode(bytes,{...sealed,rows:count})).toThrow("Complete");
  expect(()=>decode(invalid,evidence(invalid,rows.length+1))).toThrow();
 }
 const [rootRow,first,last]=installed,missing=serialize([rootRow,first,["g","a-missing/child",...last!.slice(2)]]);
 expect(()=>decodeInstalledLedgerV1(missing,evidence(missing,installed.length))).toThrow("Connected");
});
