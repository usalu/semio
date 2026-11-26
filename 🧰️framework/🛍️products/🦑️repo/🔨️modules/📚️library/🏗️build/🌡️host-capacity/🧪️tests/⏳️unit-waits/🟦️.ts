/** ⏳️ Contract: the waiting compile units of a broker are published atomically below the gate directory, read back only while their process lives and they are fresh, and shown by `repo capacity status` in the chosen language.
 * @see ../../⏳️unit-waits/🟦️.ts */
import {existsSync,mkdtempSync,rmSync,writeFileSync,mkdirSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {expect,test} from "bun:test";
import type {UnitWaitV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts";
import {createUnitWaitPublisherV1,readUnitWaitsV1,unitWaitLinesV1,writeUnitWaitsV1} from "../../⏳️unit-waits/🟦️.ts";

const row:UnitWaitV1={unit:"syn",reason:"queue",predictedMegabytes:612,seconds:40,place:1,count:1,running:2,heldMegabytes:500,budgetMegabytes:700,grantMegabytes:614};
const scratch=()=>mkdtempSync(join(tmpdir(),"unit-waits-"));

test("rows are written atomically, read while fresh and alive, and removed when empty",()=>{
 const directory=scratch();
 try{
  writeUnitWaitsV1(directory,[row],1000);
  expect(readUnitWaitsV1(directory,2000).map(record=>({pid:record.pid,rows:record.rows}))).toEqual([{pid:process.pid,rows:[row]}]);
  expect(readUnitWaitsV1(directory,1000+3*10000+1),"stale after three publish periods").toEqual([]);
  expect(existsSync(join(directory,"unit-waits",`${process.pid}.json`)),"stale file removed").toBe(false);
  writeUnitWaitsV1(directory,[row],5000);
  writeUnitWaitsV1(directory,[],5001);
  expect(readUnitWaitsV1(directory,5002)).toEqual([]);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a record of a dead process and a torn file are dropped, never shown",()=>{
 const directory=scratch();
 try{
  mkdirSync(join(directory,"unit-waits"),{recursive:true});
  writeFileSync(join(directory,"unit-waits","999999.json"),JSON.stringify({v:1,pid:999999,atMs:1000,rows:[row]}));
  writeFileSync(join(directory,"unit-waits","torn.json"),"{\"v\":1,");
  expect(readUnitWaitsV1(directory,1500,()=>false)).toEqual([]);
  expect(existsSync(join(directory,"unit-waits","999999.json"))).toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the status lines name the unit, its place and how long it waits, in English and German",()=>{
 const directory=scratch();
 try{
  writeUnitWaitsV1(directory,[row,{...row,unit:"quote",reason:"growth",place:0,count:0}],10000);
  const records=readUnitWaitsV1(directory,10000);
  const en=unitWaitLinesV1("en",records,12000),de=unitWaitLinesV1("de",records,12000);
  expect(en[0]).toBe(`[capacity]   unit wait, pid ${process.pid}: unit syn waits in the broker queue (place 1 of 1): it needs about 612 MB, the 2 running unit(s) hold 500 MB of the 700 MB budget; waited 42 s`);
  expect(en[1]).toContain("unit quote waits for a wider memory grant");
  expect(de[0]).toContain("Einheit syn wartet in der Warteschlange des Brokers (Platz 1 von 1)");
  expect(de[0]).toContain("42 s gewartet");
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the publisher writes into the gate directory of its environment and never throws",()=>{
 const directory=scratch();
 try{
  const publish=createUnitWaitPublisherV1({SEMIO_HOST_CAPACITY_DIRECTORY:directory});
  publish([row]);
  expect(readUnitWaitsV1(directory,Date.now())).toHaveLength(1);
  publish([]);
  expect(readUnitWaitsV1(directory,Date.now())).toEqual([]);
  expect(()=>createUnitWaitPublisherV1({SEMIO_HOST_CAPACITY_DIRECTORY:join(directory,"\0bad")})([row])).not.toThrow();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
