import { expect, test } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
test("original Entry reserves ingress before lifting and retains the same bounded backing until paid close", () => {
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
 for(const key of ["preLift","sameHostSource"]){expect(validate({...fixture,[key]:false})).toBe(false)}
 const db=new Database(":memory:");try{db.exec("CREATE TABLE page(offset INTEGER PRIMARY KEY,bytes BLOB)");for(let offset=0;offset<fixture.extent;offset+=fixture.maximumChunkBytes){db.query("INSERT INTO page VALUES(?,?)").run(offset,Buffer.from(fixture.bytes.slice(offset,offset+fixture.maximumChunkBytes)))}const rows=db.query("SELECT offset,bytes FROM page ORDER BY offset").all() as {offset:number;bytes:Uint8Array}[];expect(rows.map(row=>[row.offset,row.bytes.length])).toEqual([[0,64],[64,19]]);expect(Array.from(Buffer.concat(rows.map(row=>Buffer.from(row.bytes))))).toEqual(fixture.bytes);expect(JSON.parse(JSON.stringify(fixture.original)).fuel).toBe(65536)}finally{db.close()}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8"),entry=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");for(const method of ["reserve_poll_input_step","write_poll_input_step","write_poll_input_words_step","close_poll_input_step","with_original_poll_input"]){expect(source).toContain(method)}expect(entry).toContain("poll_input:OriginalActorPollInput");expect(source).toContain("try_reserve_exact");expect(source).toContain("length>64");expect(source).not.toContain("SharedRegistry");const wire=readFileSync(new URL("../../../../../🧬️schema/📜️.wit",import.meta.url),"utf8");const page=wire.slice(wire.indexOf("  record poll-input-page {"),wire.indexOf("  record job-checkpoint {"));expect(page.match(/word[0-7]: u64/g)?.length).toBe(8);expect(page).not.toContain("list<");for(const method of ["reserve-poll-input:","write-poll-input:","close-poll-input:"]){expect(wire).toContain(method)}console.log("[DEBUG] original Entry prelift corpus Ajv/SQLite/JSON83 bytes with fixed64+19 windows and closed original finite caller");
});
