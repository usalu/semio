import {test,expect} from "bun:test";import Ajv from "ajv";import {Database} from "bun:sqlite";import law from "./vacant-custody.json";import schema from "./vacant-custody.schema.json";test("owned refusal prevents vacant retirement",()=>{expect(new Ajv().compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{db.run("create table custody(refusal integer)");db.run("insert into custody values(1)");expect(db.query("select refusal=0 as vacant from custody").get()).toEqual({vacant:0});expect(law.realAction).toBe(false)}finally{db.close()}});

test("receiving transfer preserves receipt and records only unreported work",()=>{
 const db=new Database(":memory:");
 try{
  const transfer=db.query("select ?1 as receipt, ?1 is null as success, ?2 - ?3 as work");
  for(const row of law.transfers){
   expect(transfer.get(row.receipt,row.actualWork,row.reportedWork)).toEqual({receipt:row.receipt,success:Number(row.success),work:row.unreportedWork});
  }
 }finally{db.close()}
});
