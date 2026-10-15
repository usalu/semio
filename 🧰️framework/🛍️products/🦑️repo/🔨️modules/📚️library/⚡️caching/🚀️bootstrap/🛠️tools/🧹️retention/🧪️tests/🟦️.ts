import "../../📦️sdk/📥️source/🔗️derivative/🧪️tests/🟦️.ts";
import "../📥️physical/🧪️tests/🟦️.ts";
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
/** 🧮️ Independent SQLite orders the same immutable retention cohort as the first-party planner. */
test("bounded toolkit retention keeps current and newest images without an age override",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const database=new Database(":memory:");try{database.exec("CREATE TABLE images(identity TEXT PRIMARY KEY,createdAt REAL NOT NULL)");for(const row of fixture.images)database.query("INSERT INTO images VALUES (?1,?2)").run(row.identity,row.createdAt);const independent=database.query("SELECT identity FROM images WHERE identity<>?1 AND identity NOT IN (SELECT identity FROM images ORDER BY createdAt DESC,identity ASC LIMIT ?2) ORDER BY identity ASC").all(fixture.current,fixture.keepNewest).map(row=>(row as {identity:string}).identity);expect(independent).toEqual(fixture.eligible);const provider=await import("../🟦️.ts");expect(typeof provider.planNxToolingRetentionV1).toBe("function");expect(provider.planNxToolingRetentionV1(fixture.images,fixture.current,fixture.keepNewest)).toEqual(independent);expect(provider.planNxToolingRetentionV1([...fixture.images].reverse(),fixture.current,fixture.keepNewest)).toEqual(independent);expect(()=>provider.planNxToolingRetentionV1([...fixture.images,fixture.images[0]!],fixture.current,fixture.keepNewest)).toThrow();expect(()=>provider.planNxToolingRetentionV1(fixture.images,"a".repeat(64),fixture.keepNewest)).toThrow();}finally{database.close()}
});
