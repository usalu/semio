import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {planInactiveCargoRetirementV1,type CargoRetirementPolicyV1} from "../🟦️.ts";
import type {CacheUnit} from "../../🟦️.ts";

test("neutral inactive budget retires recent unused data while preserving actual live and bounded work",()=>{
 const validate=new Ajv({strict:false}).compile(schema);expect(validate(fixture.policy)).toBe(true);
 const db=new Database(":memory:");db.exec("CREATE TABLE units(path TEXT,bytes INTEGER,recency INTEGER,held INTEGER)");
 try{for(const unit of fixture.units)db.query("INSERT INTO units VALUES(?,?,?,?)").run(unit.path,unit.bytes,unit.recencyMs,Number(unit.lockHeld));
  for(const limit of [fixture.policy.maximumDeletionBytes,fixture.maximumDeletionBytes]){
   const policy={...fixture.policy,maximumDeletionBytes:limit} as CargoRetirementPolicyV1,plan=planInactiveCargoRetirementV1(fixture.units as CacheUnit[],policy,fixture.availableBytes),required=Math.max(180-policy.retainedBytes,policy.targetAvailableBytes-fixture.availableBytes,0),selected:string[]=[];let bytes=0;
   for(const unit of db.query("SELECT path,bytes FROM units WHERE held=0 ORDER BY recency,path").all() as {path:string;bytes:number}[]){if(bytes>=required)break;if(bytes+unit.bytes>limit)continue;selected.push(unit.path);bytes+=unit.bytes;}
   expect(plan.units.map(unit=>unit.path)).toEqual(selected);expect(selected).toEqual(limit===fixture.policy.maximumDeletionBytes?fixture.selected:fixture.boundedSelected);expect(plan.bytes).toBe(bytes);expect(plan.unmetBytes).toBe(Math.max(0,required-bytes));expect(plan.remainingBytes).toBe(180-bytes);
  }
  expect(()=>planInactiveCargoRetirementV1(fixture.units as CacheUnit[],{...fixture.policy,maximumDeletionBytes:0} as CargoRetirementPolicyV1,40)).toThrow("policy");
 }finally{db.close();}
});
