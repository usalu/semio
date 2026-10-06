import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import { DWG_SQLITE_SCHEMA } from "../../🟦️.ts";
import { parseSqliteDatabaseSchema,exportSqliteDatabase,importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("DWG embeds all 277 exact authored domain tables and independent SQLite accepts them",async()=>{
  const domains=["📄️document","🔧️header","✏️drawing","🗃️tables","📐️entities","📦️objects","🔗️associativity","🧮️evaluation","🧩️blocks","🎬️actions","🖌️styles/👁️visual","🖌️styles/🧱️material","🖌️styles/🗃️table","📃️layout","🖌️styles/↗️mleader","📏️constraints"];
  const sql=(await Promise.all(domains.map(domain=>Bun.file(new URL("../.."+domain+"/🗄️.sql",import.meta.url)).text()))).join("\n");
  expect(DWG_SQLITE_SCHEMA).toBe(sql);
  const database=parseSqliteDatabaseSchema(sql);expect(database.tables).toHaveLength(277);
  const oracle=Database.deserialize(await exportSqliteDatabase(database));
  try{expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(oracle.query("SELECT count(*) AS n FROM sqlite_schema WHERE type='table'").get()).toEqual({n:277});expect((await importSqliteDatabase(oracle.serialize())).tables.map(table=>table.name)).toEqual(database.tables.map(table=>table.name));}finally{oracle.close();}
});
