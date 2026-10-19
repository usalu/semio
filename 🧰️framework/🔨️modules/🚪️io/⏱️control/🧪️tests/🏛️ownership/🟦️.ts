import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import toml from "@iarna/toml";
import schema from "../../🧬️schema/🏛️ownership.json";
import fixture from "../🏛️ownership.json";

test("native IO control owns actual generic bodies and SQLite receiving remains in its concrete producer",()=>{expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);const root=resolve(import.meta.dir,".."),manifest=toml.parse(readFileSync(resolve(root,"📦️packages/🦀️rust/Cargo.toml"),"utf8")) as any,control=readFileSync(resolve(root,"🦀️.rs"),"utf8"),provider=readFileSync(resolve(root,"../🪶️sqlite-snapshot/🫴️receiving/🦀️.rs"),"utf8"),database=new Database(":memory:");try{expect(manifest.package.name).toBe(fixture.owner);database.exec("CREATE TABLE dependency(name TEXT PRIMARY KEY)");for(const name of Object.keys(manifest.dependencies))database.query("INSERT INTO dependency VALUES(?)").run(name);expect(database.query("SELECT name FROM dependency ORDER BY name").all()).toEqual(fixture.dependencies.map(name=>({name})));expect(control.includes("crate::sqlite_snapshot")).toBe(false);expect(control.includes("pub struct NativeSnapshotDecodeOwner")).toBe(true);expect(provider.includes("ReceivingSettlement::new")).toBe(true);expect(provider.includes("self.scoped_native")).toBe(true);expect(provider.includes("self.native().checkpoint()")).toBe(true);}finally{database.close();}});
