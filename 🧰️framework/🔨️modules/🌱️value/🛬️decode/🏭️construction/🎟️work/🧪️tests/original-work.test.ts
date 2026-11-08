import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import fixture from "../📐️.json";
import schema from "../🔣️.json";
/** ⛽️ Independent SQL distinguishes issuance, allocator attempts, native placement and publication. */
test("original shared construction retains exact work for denied, null, born and published turns",()=>{
 expect(semioSchemaAjvV1({strict:true}).validate(schema,fixture)).toBe(true);
 const db=new Database(":memory:");try{
  db.run("create table performed(action text primary key,work integer not null check(work>0),null_attempt integer,born integer,publication integer)");
  for(const row of [["issuance",1,1,1,0],["allocator attempt",1,1,1,0],["empty header placement",2,0,1,0],["unique backing",1,0,0,1],["payload placement",1,0,0,1],["alias publication",1,0,0,1],["output publication",1,0,0,1]])db.run("insert into performed values(?,?,?,?,?)",...row);
  expect(db.query("select sum(work*null_attempt) nullWork,sum(work*born) allocationWork,sum(work*publication) publicationWork from performed").get()).toEqual({nullWork:fixture.nullWork,allocationWork:fixture.allocationWork,publicationWork:fixture.publicationWork});
  for(const width of[4,8])expect(db.query("select ?*? copiedBytes").get(fixture.nullProgress.copyWords,width)).toEqual({copiedBytes:2*width});expect(fixture.nullProgress).toEqual({items:1,copyWords:2,capacityBytes:0,releaseBytes:0});expect(db.query("select 0 work, 0 additionalPosts, 0 additionalFuel").get()).toEqual(fixture.receiptRetry);expect(fixture.deniedWork).toBe(0);expect(fixture.clearBirthWork).toBe(1);expect(fixture.copiedBytes).toBe("quotedNativeHeader");expect(fixture.nullAdmission).toBe("requestedExtent");expect(fixture.nullPhysicalBytes).toBe(0);
 }finally{db.close()}
});