import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import {parseSqliteDatabaseSchema,exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("GLTF handwritten semantic schema agrees with independent SQLite table and foreign-key interpretation",async()=>{
 const names=["📄️document","🧩️extras","🌳️node","🏔️mesh","📦️buffer","🖌️material","🖼️texture","🦴️skin","🎬️animation","🎥️camera"];
 const sql=(await Promise.all(names.map(name=>Bun.file(new URL(`../../../🪶️sqlite/${name}/🗄️.sql`,import.meta.url)).text()))).join("\n");
 const parsed=parseSqliteDatabaseSchema(sql);
 expect(parsed.tables).toHaveLength(57);
 const oracle=new Database(":memory:");
 try{
  oracle.exec("PRAGMA application_id=1397576526;PRAGMA user_version=1");
  oracle.exec(sql);
  expect(oracle.query("SELECT count(*) AS count FROM sqlite_schema WHERE type='table'").get()).toEqual({count:57});
  expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);
  const owned=Database.deserialize(await exportSqliteDatabase(parsed));
  try{expect(owned.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(owned.query("PRAGMA foreign_key_check").all()).toEqual([]);}finally{owned.close();}
  expect((await importSqliteDatabase(oracle.serialize())).tables.map(table=>table.name)).toEqual(parsed.tables.map(table=>table.name));
 }finally{oracle.close();}
});
