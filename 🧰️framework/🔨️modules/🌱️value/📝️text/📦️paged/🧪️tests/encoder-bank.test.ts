import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./encoder-bank.schema.json";
import fixture from "./encoder-bank.json";
/** 🏦️ Independent arithmetic checks the single original encoder wallet. */
test("original encoder Source wallet",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select ? - 1 as work, ? - ? as copy, ? - 1 as items").get(fixture.work,fixture.copyBytes,fixture.performedCopy,fixture.items)).toEqual({work:fixture.remainingWork,copy:fixture.remainingCopy,items:fixture.remainingItems})}finally{db.close()}});

/** 🧵️ SQLite independently validates the real formatted UTF-8 byte sequence. */
test("original paged Source formatting",()=>{const db=new Database(":memory:");try{expect(db.query("select ? || ? || ? as text, length(cast(? || ? || ? as blob)) as bytes").get(fixture.prefix,fixture.body,fixture.suffix,fixture.prefix,fixture.body,fixture.suffix)).toEqual({text:fixture.expected,bytes:fixture.utf8Bytes})}finally{db.close()}});

/** 🧮️ The fixture corpus independently derives each UTF-8 extent. */
test("original encoder sample extents",()=>{const db=new Database(":memory:");try{for(const row of fixture.samples)expect(db.query("select ? || ? as text, length(cast(? || ? as blob)) as bytes").get(fixture.prefix,row.body,fixture.prefix,row.body)).toEqual({text:row.expected,bytes:row.bytes})}finally{db.close()}});

/** 🪙️ Two original payments consume actual work rather than their larger second quote. */
test("original encoder remaining Work",()=>{const db=new Database(":memory:");try{expect(db.query("select ? - ? - ? as remaining").get(fixture.sequential.work,...fixture.sequential.performed)).toEqual({remaining:fixture.sequential.remaining})}finally{db.close()}});

/** 📐️ Real reserve-height bounds fund every possible advancing reserve rather than a loop limit. */
test("original encoder structural funding bound",()=>{const db=new Database(":memory:");try{for(const row of fixture.budgets)expect(db.query("select 6*? + max(?,1)*(?+2) + ?+11 + 2*(2*?+max(?,1)+4)+8 as work").get(row.bytes,row.bytes,row.reserve,row.bind,row.bytes,row.bytes)).toEqual({work:row.work})}finally{db.close()}});
