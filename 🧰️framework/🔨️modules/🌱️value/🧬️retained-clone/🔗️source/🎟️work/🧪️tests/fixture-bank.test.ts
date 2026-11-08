import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

test("fixture birth work is the sum of its actual original source stages",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");
 try{expect(db.query("select sum(value) as work from json_each(?)").get(JSON.stringify(fixture.birthStages))).toEqual({work:fixture.expected.birthCpu}); expect(db.query("select json_extract(?, '$.work') as work, json_extract(?, '$.inputPreserved') as preserved").get(...Array(2).fill(JSON.stringify(fixture.ownedAdmissionDenied)))).toEqual({work:3,preserved:1});
  expect(db.query("select json_extract(?, '$.work') as work, json_extract(?, '$.bound') as bound").get(...Array(2).fill(JSON.stringify(fixture.sharedBankCheck)))).toEqual({work:1,bound:0});
  expect(db.query("select json_extract(?, '$.allocatedBeforeRefusal') as allocated, json_extract(?, '$.inputPreserved') as preserved, json_extract(?, '$.deniedRecoveryWork') as denied").get(...Array(3).fill(JSON.stringify(fixture.partialAdmission)))).toEqual({allocated:1,preserved:1,denied:0});
  expect(db.query("select json_extract(?, '$.sourceIdentityPreserved') as source, json_extract(?, '$.bankWorkPreserved') as bank, json_extract(?, '$.closeConsumesWork') as close").get(...Array(3).fill(JSON.stringify(fixture.admittedSource)))).toEqual({source:1,bank:1,close:1});
  expect(db.query("select json_extract(?, '$.original') as original, json_extract(?, '$.projection') as projection, json_extract(?, '$.deniedWork') as denied").get(...Array(3).fill(JSON.stringify(fixture.dualProjection)))).toEqual({original:"fixture cancel request",projection:"fixture projection refusal",denied:0});
  expect(db.query("select json_extract(?, '$.firstWork') as first, json_extract(?, '$.closeBlocked') as blocked, json_extract(?, '$.recoveryWork') as recovered, json_extract(?, '$.sourcePreserved') as source").get(...Array(4).fill(JSON.stringify(fixture.pendingBank)))).toEqual({first:0,blocked:1,recovered:1,source:1});
  expect(db.query("select json_extract(?, '$.work') as work, json_extract(?, '$.sourcePreserved') as source").get(...Array(2).fill(JSON.stringify(fixture.closeDenial)))).toEqual({work:0,source:1});
  expect(db.query("select json_extract(?, '$.message') as message, json_extract(?, '$.deniedWork') as denied, json_extract(?, '$.publicationWork') as published").get(...Array(3).fill(JSON.stringify(fixture.cleanupRefusal)))).toEqual({message:"fixture cleanup refusal",denied:0,published:1});
 }finally{db.close();}
});
