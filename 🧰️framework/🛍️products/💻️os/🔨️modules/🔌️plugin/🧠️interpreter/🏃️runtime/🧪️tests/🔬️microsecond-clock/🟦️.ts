import {test} from "bun:test";
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import {readModuleTreeSource} from "@semio-tech/framework-compiler-ts/syntax/module-tree";

/** 🎟️ One authored deadline corpus: duration units, absolute boundaries and snapshot resumes. */
type DeadlineFixture={duration:{milliseconds:number;microseconds:number}[];absolute:{now:string;deadline:string;expired:boolean}[];snapshot:{now:string;deadline:string;fuelUsed:number;expired:boolean}[]};

/** 🕰️ SQLite independently qualifies exact duration units and an unchanged absolute deadline. */
test("original interpreter deadline preserves exact microseconds and caller clock boundary",()=>{
 const fixture=JSON.parse(readFileSync(new URL("🎟️deadline.json",import.meta.url),"utf8")),schema=JSON.parse(readFileSync(new URL("🎟️deadline.schema.json",import.meta.url),"utf8"));assert(new Ajv2020({strict:true}).validate<DeadlineFixture>(schema,fixture));const db=new Database(":memory:");
 try{for(const row of fixture.duration)assert.equal(Number((db.query("select cast(? as integer)*1000 as us").get(row.milliseconds) as {us:number}).us),row.microseconds);for(const row of fixture.absolute){const actual=db.query("select cast(? as integer)>=cast(? as integer) as expired").get(BigInt(row.now),BigInt(row.deadline)) as {expired:number};assert.equal(Boolean(actual.expired),row.expired);}assert.equal(fixture.duration[0]!.microseconds,1000);}finally{db.close();}
});

/** 🧳️ Progress cannot renew a supplied snapshot deadline. */
test("snapshot resumes preserve the supplied absolute original deadline while fuel advances",()=>{
 const fixture=JSON.parse(readFileSync(new URL("🎟️deadline.json",import.meta.url),"utf8")),schema=JSON.parse(readFileSync(new URL("🎟️deadline.schema.json",import.meta.url),"utf8"));assert(new Ajv2020({strict:true}).validate<DeadlineFixture>(schema,fixture));const db=new Database(":memory:");
 try{for(const row of fixture.snapshot){const actual=db.query("select cast(? as integer)>=cast(? as integer) as expired").get(BigInt(row.now),BigInt(row.deadline)) as {expired:number};assert.equal(Boolean(actual.expired),row.expired)}assert(fixture.snapshot.at(-1)!.fuelUsed>fixture.snapshot[0]!.fuelUsed);assert.equal(new Set(fixture.snapshot.map((row:{deadline:string})=>row.deadline)).size,1)}finally{db.close()}
 const source=readModuleTreeSource(new URL("../../🦀️.rs",import.meta.url)),resume=source.slice(source.indexOf("pub fn resume_snapshot("),source.indexOf("/// 🪶️ The six owned"));assert.match(resume,/deadline_us: u64/u);assert.match(resume,/fuel, deadline_us, OwnedDeadline::Original/u);assert.doesNotMatch(resume,/NoFuelProgress|deadline_ms|\*1000/u);
 const host=readFileSync(new URL("../../../../🖥️host/🚪️io/🪶️snapshot/🦀️.rs",import.meta.url),"utf8");assert.doesNotMatch(host,/budget\.deadline_ms|epoch\.arm\(/u);assert.match(host,/arm_original\(store,deadline,budget\.deadline_us\)/u);
});
