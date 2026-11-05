import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json";
import schema from "./🧬️schema.json";
test("original preparation admission separates reserved backing and original handoff",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");
 try{for(const row of fixture.rows){
  const i=row.input,grow=i.length===i.capacity,copy=grow?2*i.length*i.cellBytes:2*i.cellBytes,born=grow?(i.length+1)*i.cellBytes:0,release=grow?i.capacity*i.cellBytes:0;
  const admitted=i.original&&i.items>0&&i.depth>0&&i.copyBytes>=copy&&i.capacityBytes>=born&&i.releaseBytes>=release;
  const value={length:i.length+Number(admitted&&!grow),capacity:admitted&&grow?i.length+1:i.capacity,original:i.original&&!(admitted&&!grow),work:Number(admitted),copiedBytes:admitted?copy:0,bornBytes:admitted?born:0,releasedBytes:admitted?release:0,complete:!i.original||admitted&&!grow};
  const sql=db.query("select case when ?1 then ?2+case when ?3 then 0 else 1 end else ?2 end length,case when ?1 and ?3 then ?2+1 else ?4 end capacity,?1 work").get(Number(admitted),i.length,Number(grow),i.capacity);
  expect(sql,row.id).toEqual({length:value.length,capacity:value.capacity,work:value.work});expect(value,row.id).toEqual(row.expected);
 }}finally{db.close()}
});
import preview from "./🪞️preview.json";
import previewSchema from "./🪞️schema.json";
test("plain preview preserves original ordered values through its header transfer",()=>{
 expect(new Ajv({strict:true}).compile(previewSchema)(preview)).toBe(true);
 const original=preview.values,iterator=original.values(),db=new Database(":memory:");
 try{
  db.exec("create table source(ordinal integer primary key,value integer)");
  const insert=db.prepare("insert into source values(?,?)");original.forEach((value,ordinal)=>insert.run(ordinal,value));
  expect([...iterator]).toEqual(db.query("select value from source order by ordinal").all().map(row=>(row as {value:number}).value));
  expect(original).toEqual(preview.expected.values);expect(original).toBe(preview.values);
  for(const axis of preview.deniedAxes){expect(["items","copy","depth"]).toContain(axis);expect(original).toEqual(preview.expected.values)}
  expect(preview.expected).toMatchObject({sameAllocation:true,headerWork:1,bornBytes:0,releasedBytes:0});
 }finally{db.close()}
});
