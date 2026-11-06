import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import fixture from"./emit-original.json";
import schema from"./emit-original.schema.json";
test("original emission vectors satisfy their neutral schema",()=>expect(new Ajv().compile(schema)(fixture)).toBe(true));
for(const row of fixture.cases)test(row.id,()=>{
 const permitted=row.items&&row.copy&&row.depth&&row.fuel&&!row.cancel;
 const result=row.published?(row.source?"refused":"published"):!row.source||!permitted||row.busy?"pending":row.receiver?"refused":"published";
 const moved=!row.published&&result==="published";
 expect([result,moved?false:row.source,moved||row.receiver,moved||row.published]).toEqual([row.result,row.afterSource,row.afterReceiver,row.afterPublished]);
 const db=new Database(":memory:");
 try{
  const oracle=db.query("SELECT CASE WHEN ?1 THEN CASE WHEN ?2 THEN 'refused' ELSE 'published' END WHEN NOT ?2 OR NOT ?3 OR ?4 THEN 'pending' WHEN ?5 THEN 'refused' ELSE 'published' END AS outcome").get(+row.published,+row.source,+permitted,+row.busy,+row.receiver)as{outcome:string};
  expect(oracle.outcome).toBe(row.result);
 }finally{db.close();}
});
