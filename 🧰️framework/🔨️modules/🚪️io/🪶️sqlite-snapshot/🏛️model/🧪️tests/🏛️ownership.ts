import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import toml from "@iarna/toml";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🏛️ownership.json";
import fixture from "./🏛️ownership.json";
test("relational SQLite model owns schema parsing and original control without physical producer dependency",()=>{expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);const root=resolve(import.meta.dir,".."),manifest=toml.parse(readFileSync(resolve(root,"📦️packages/🦀️rust/Cargo.toml"),"utf8")) as {package:{name:string};dependencies:Record<string,unknown>},source=readFileSync(resolve(root,"🦀️.rs"),"utf8"),database=new Database(":memory:");try{database.exec("CREATE TABLE dependency(name TEXT PRIMARY KEY)");for(const name of Object.keys(manifest.dependencies))database.query("INSERT INTO dependency VALUES(?)").run(name);expect(database.query("SELECT name FROM dependency ORDER BY name").all()).toEqual(fixture.dependencies.map(name=>({name})));expect(manifest.package.name).toBe(fixture.owner);for(const body of["pub struct SqliteSnapshotControl","pub struct SqliteDatabase","fn parse_table","fn lex","with_retirement_owner","with_retirement_child","admit_reconstruction_bytes"])expect(source.includes(body)).toBe(true);expect(source.includes("pub mod transfer")).toBe(false);expect(source.includes("pub mod artifact")).toBe(false);expect(Buffer.byteLength(source)).toBeLessThan(fixture.sourceMaximumBytes);}finally{database.close();}});
