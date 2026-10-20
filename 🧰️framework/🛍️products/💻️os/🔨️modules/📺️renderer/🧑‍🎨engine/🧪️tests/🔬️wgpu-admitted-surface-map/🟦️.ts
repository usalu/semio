import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import fixture from "../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-admitted-surface-map/🧫️fixtures/🔣️.json";
import schema from "../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-admitted-surface-map/🧬️schema/🔣️.json";

test("Ordered mutable surface lookup matches independent SQLite ordinal semantics",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE surfaces (id TEXT PRIMARY KEY, value INTEGER NOT NULL, ordinal INTEGER UNIQUE NOT NULL)");
  const insert=database.query("INSERT INTO surfaces VALUES (?, ?, ?)");fixture.insert.forEach(([id,value],index)=>insert.run(id!,value!,index));
  database.query("DELETE FROM surfaces WHERE id = ?").run(fixture.remove);insert.run(fixture.reuse[0]!,fixture.reuse[1]!,fixture.insert.length);
  const ordinal=database.query("SELECT id FROM surfaces ORDER BY ordinal LIMIT 1 OFFSET ?");
  const selected=ordinal.get(fixture.mutate.index) as {id:string};database.query("UPDATE surfaces SET value = value + ? WHERE id = ?").run(fixture.mutate.delta,selected.id);
  expect(database.query("SELECT id, value FROM surfaces ORDER BY ordinal").values()).toEqual(fixture.expected);expect(ordinal.get(fixture.absentIndex)).toBeNull();
  const source=readFileSync(new URL("../../🎯️targets/🧊️wgpu/🤝️contracts/📥️lifted/🗃️scene-state/🦀️.rs",import.meta.url),"utf8"),lookup=source.slice(source.indexOf("pub fn get_mut_at("),source.indexOf("pub fn len(",source.indexOf("pub fn get_mut_at(")));
  expect(lookup).toContain("self.order.get(index).copied().flatten()");expect(lookup).toContain("self.slots.get_mut(slot)");expect(lookup).not.toMatch(/clone\(|to_owned\(|to_string\(|Box::|Vec::/u);
 }finally{database.close();}
});
