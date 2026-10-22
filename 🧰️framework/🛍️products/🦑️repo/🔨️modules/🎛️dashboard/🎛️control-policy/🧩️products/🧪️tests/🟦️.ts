import { expect, test } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

const root = resolve(import.meta.dir, "../../../../../../../..");

test("the active owner catalog has canonical dashboard controls and no IDE extension product", () => {
  const validate = new Ajv().compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  const catalog = JSON.parse(readFileSync(resolve(root, "📋️build-owners.json"), "utf8"));
  const rows = Array.isArray(catalog) ? catalog : catalog.owners;
  const db = new Database(":memory:");
  try {
    db.run("CREATE TABLE owners (name TEXT PRIMARY KEY)");
    for (const row of rows) db.run("INSERT INTO owners VALUES (?)", [row.name]);
    for (const row of fixture.owners) {
      expect(rows.some((owner: { name: string }) => owner.name === row.name), row.name).toBe(row.active);
      expect(db.query("SELECT COUNT(*) AS n FROM owners WHERE name = ?").get(row.name)).toEqual({ n: Number(row.active) });
    }
    for (const path of fixture.removedPaths) expect(existsSync(resolve(root, path)), path).toBe(false);
    const workspace = JSON.parse(readFileSync(resolve(root, "package.json"), "utf8"));
    expect(workspace.workspaces.some((path: string) => path.includes("🧩️vscode"))).toBe(false);
  } finally { db.close(); }
});


test("the workspace package has no retired IDE authoring tool dependencies",()=>{
  const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);const workspace=JSON.parse(readFileSync(resolve(root,"package.json"),"utf8")),db=new Database(":memory:"),count=(value:unknown,name:string)=>(db.query("WITH dependencies AS (SELECT key FROM json_each(?1,'$.dependencies') UNION ALL SELECT key FROM json_each(?1,'$.devDependencies') UNION ALL SELECT key FROM json_each(?1,'$.optionalDependencies') UNION ALL SELECT key FROM json_each(?1,'$.peerDependencies')) SELECT count(*) n FROM dependencies WHERE key=?2").get(JSON.stringify(value),name)as{n:number}).n;
  try{for(const name of fixture.retiredDependencies){expect(fixture.dependencySections.some(section=>Object.hasOwn(workspace[section]??{},name)),name).toBe(false);expect(count(workspace,name),name).toBe(0);for(const section of fixture.dependencySections){const counterexample={...workspace,[section]:{...(workspace[section]??{}),[name]:"fixture"}};expect(fixture.dependencySections.some(key=>Object.hasOwn(counterexample[key]??{},name))).toBe(true);expect(count(counterexample,name)).toBe(1);}}}finally{db.close();}
});
