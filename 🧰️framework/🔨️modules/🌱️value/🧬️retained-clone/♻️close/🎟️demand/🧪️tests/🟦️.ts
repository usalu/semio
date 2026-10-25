import "../../../../♻️retirement/🧪️tests/⏱️work/🟦️.ts";
import "../../../🧬️contract/🔤️string-cursor/🧪️tests/🟦️.ts";
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv2020 from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {nestedRetainedCloseDemand,type RetainedCloseDemandSource} from "../🟦️.ts";

test("one original complete close quote visits each child once",()=>{
 expect(new Ajv2020({strict:true}).compile(schema)(fixture)).toBe(true);const database=new Database(":memory:");
 try{for(const row of fixture.cases){
  const oracle=database.query<{visits:number;depth:number},[number]>("WITH RECURSIVE path(n) AS (SELECT 0 UNION ALL SELECT n+1 FROM path WHERE n<?) SELECT count(*) AS visits,max(n)+1 AS depth FROM path").get(row.depth)!;
  expect(oracle.visits).toBe(row.expectedVisits);expect(oracle.depth).toBe(row.expectedDepth);
  let visits=0;const seen:number[]=[];let owner:RetainedCloseDemandSource={closeStarted:true,closeDemand:body=>{visits++;seen.push(body);return{copyBytes:1,capacityBytes:17,releaseBytes:23,depth:1};}};
  for(let level=0;level<row.depth;level++){const child=owner;owner={closeStarted:true,closeDemand:body=>{visits++;seen.push(body);return nestedRetainedCloseDemand(child,body);}};}
  const demand=owner.closeDemand(row.body);expect(visits).toBe(row.expectedVisits);expect(demand).toEqual({copyBytes:1,capacityBytes:17,releaseBytes:23,depth:row.expectedDepth});expect(seen.every(body=>body===row.body)).toBe(true);
 }
 }finally{database.close();}
});

/** 🪄️ Requires actual derived ownership stages rather than post-effect numeric admission. */
test("derived clone preflights every physical axis and quotes each selected child once",()=>{
 const source=readFileSync(new URL("../../../../✨️derive/🧬️retained-clone/🦀️.rs",import.meta.url),"utf8");
 expect(source).toContain("fn close_started(&self) -> bool { self.closing }");
 expect(source).toContain("fn close_demands(&self,maximum_body_bytes:usize)");
 expect(source).toContain("nested_retained_clone_close_demands::<#ty>");
 expect(source).toContain("nested_retained_clone_demands::<#ty>");
 expect(source).toContain("let demand=#value_crate::retained_clone::RetainedCloneCursor::advance_demands(self,source,body)?;");
 expect(source.match(/retained_clone_grant_admits\(grant,demand\)/g)?.length).toBe(2);
 expect(source).toContain("let nested=#value_crate::retained_clone::nested_retained_clone_grant(grant)?;");
 expect(source).toContain("retained_capacity_bytes:demand.capacity_bytes");
 expect(source).toContain("released_bytes:demand.release_bytes");
 expect(source).not.toContain("#(#depth_children)*");
 const root=readFileSync(new URL("../../../../🧪️tests/🥣️companion/🌱️retained-clone/🦀️.rs",import.meta.url),"utf8");
 expect(root).toContain('mod close_demand_tests;');
});

/** 🎟️ Shared native denial axes agree with independent relational admission. */
test("derived clone neutral denial axes cannot spend another physical currency",()=>{
 const db=new Database(":memory:");
 try{
  const admitted=db.query<{admitted:number},{ $axis:string }>(`SELECT
   (CASE $axis WHEN 'items' THEN 0 ELSE 1 END)>0 AND
   (CASE $axis WHEN 'copy' THEN 0 ELSE 8 END)>=8 AND
   (CASE $axis WHEN 'capacity' THEN 0 ELSE 12 END)>=12 AND
   (CASE $axis WHEN 'release' THEN 0 ELSE 16 END)>=16 AND
   (CASE $axis WHEN 'depth' THEN 0 ELSE 3 END)>=3 AS admitted`);
  for(const axis of fixture.derive.denials)expect(admitted.get({$axis:axis})?.admitted).toBe(0);
  expect(admitted.get({$axis:"exact"})?.admitted).toBe(1);
  expect(fixture.derive.cuts).toContain(0);
 }finally{db.close();}
});
