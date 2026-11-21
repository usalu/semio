import{test,expect}from"bun:test";import Ajv from"ajv";import{Database}from"bun:sqlite";import corpus from"./🔣️.json";import schema from"./🧬️.json";test("return budget binding conserves its exact admitted stock",()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const db=new Database(":memory:");for(const row of corpus.cases){const q=db.query("select (?1<=?2 and ?3<=?4) as accepted, ?2-?1 as remaining, ?4-?3 as fuel").get(row.spent,row.grant,row.work,row.fuel)as{accepted:number,remaining:number,fuel:number};expect(Boolean(q.accepted)).toBe(row.accepted);if(row.accepted){expect(q.remaining).toBe(row.remaining);expect(q.fuel).toBe(row.remainingFuel);}}db.close();});
import observations from "./observation.json";
import observationSchema from "./observation-schema.json";

test("return budget observations distinguish initial and renewal fences",()=>{
 expect(new Ajv({strict:true}).validate(observationSchema,observations)).toBe(true);
 const db=new Database(":memory:");
 try{
  const select=db.query(`with input(phase,current,prior,spent,held,refused) as (
   values (?,json(?),json(?),json(?),json(?),?)
  ), visible as (
   select *,case when phase='fenced' then prior else current end as published from input
  ), stock as (
   select *,case when phase='fenced' and json_type(prior)='null' then current else published end as observed from visible
  ), balance as (
   select *,json_extract(observed,'$[0]')-json_extract(spent,'$[0]') as items,
    json_extract(observed,'$[1]')-json_extract(spent,'$[1]') as fuel
   from stock
  ), checked as (
   select *,case when refused then 'refused' when json_type(current)='null' then 'absent'
    when min(items,fuel)<0 then 'spent' end as error,
    items-json_extract(held,'$[0]') as net_items,fuel-json_extract(held,'$[1]') as net_fuel from balance
  )
  select coalesce(published,'null') as visible,case when json_type(observed)='array' then observed else '[0,0]' end as snapshot,
   coalesce(error,json_array(items,fuel)) as raw,case when error is not null then error
    when net_items<0 then 'held' when net_fuel<0 then 'heldFuel' else json_array(net_items,net_fuel) end as net
  from checked`);
  const decode=(value:string)=>value.startsWith("[")||value==="null"?JSON.parse(value):value;
  for(const row of observations){
   const actual=select.get(row.phase,JSON.stringify(row.current),JSON.stringify(row.prior),
    JSON.stringify(row.spent),JSON.stringify(row.held),Number(row.refused)) as Record<string,string>;
   expect(Object.fromEntries(Object.entries(actual).map(([key,value])=>[key,decode(value)])))
    .toEqual({visible:row.visible,snapshot:row.snapshot,raw:row.raw,net:row.net});
  }
 }finally{db.close();}
});
