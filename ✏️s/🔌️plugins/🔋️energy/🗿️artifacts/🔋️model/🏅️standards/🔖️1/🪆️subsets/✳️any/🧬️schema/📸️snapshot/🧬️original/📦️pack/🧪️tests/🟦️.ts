import {expect,test} from "bun:test";
import {blake3} from "@noble/hashes/blake3.js";
import law from "./🔣️.json";
import genesis from "../../🌱️genesis/🧪️tests/🔣️.json";
import {Database} from "bun:sqlite";
test("native intrinsic Energy Pack schema digest agrees with the independent BLAKE3 implementation",()=>{const bytes=Uint8Array.from([1,law.fields.length,law.fields[0].id,new TextEncoder().encode(law.fields[0].key).length,...new TextEncoder().encode(law.fields[0].key),0,14]);expect([...bytes]).toEqual(law.canonicalSchemaBytes);expect(Buffer.from(blake3(bytes)).toString("hex")).toBe(law.schemaHash);});
test("authored empty Energy genesis calendar and thermal fields agree with SQLite JSON",()=>{const db=new Database(":memory:");const wire=JSON.stringify(genesis);expect((db.query("SELECT json_extract(?, '$.run_period.year') AS year").get(wire)as{year:number}).year).toBe(2026);expect((db.query("SELECT sum(value) AS total FROM json_each(?, '$.ground_temperature.building_surface_c')").get(wire)as{total:number}).total).toBe(216);expect((db.query("SELECT json_extract(?, '$.schema') AS schema").get(wire)as{schema:string}).schema).toBe("energy.model");db.close();});
