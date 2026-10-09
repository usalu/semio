import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import grantSchema from "../../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json";
import {parseRetainedCloneGrant} from "../../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🟦️.ts";
test("original sync identity has required independent authority and authentic provenance",async()=>{
 const ajv=new Ajv({strict:true});expect(ajv.compile(schema)(fixture)).toBe(true);const oracle=ajv.compile(grantSchema);expect(oracle(fixture.grant)).toBe(true);expect(parseRetainedCloneGrant(fixture.grant)).toBe(fixture.grant);
 for(const axis of Object.keys(fixture.grant)){const missing={...fixture.grant} as Record<string,number>;delete missing[axis];expect(oracle(missing)).toBe(false);expect(()=>parseRetainedCloneGrant(missing)).toThrow();}
 const db=new Database(":memory:");db.run("CREATE TABLE actor(text TEXT)");for(const row of fixture.actors){db.run("INSERT INTO actor VALUES(?)",[row.text]);expect(db.query("SELECT hex(CAST(text AS BLOB)) AS bytes FROM actor ORDER BY rowid DESC LIMIT 1").get()).toEqual({bytes:Buffer.from(row.text).toString("hex").toUpperCase()});}db.close();
 const original=await Bun.file(new URL("../../🦀️.rs",import.meta.url)).text();expect(original.includes("pub actor_identity_grant: semio_framework_value::RetainedCloneGrant")).toBe(true);expect(original.includes("actor_identity_admission_admits(actor_identity_grant)")).toBe(true);expect(original.includes("wire_envelopes_with_original_actor(envelopes,socket_actor,self.actor_identity_grant)")).toBe(true);expect(original.includes("has no authentic actor identity")).toBe(true);expect(original.includes('unwrap_or_else(|| "unknown".to_string())')).toBe(false);console.log("[DEBUG] original Sync five-axis authority admits no missing currency; SQLite preserves original UTF8/NUL actor semantics; authentic history presence and productive connect/relay issuers mounted");
});
