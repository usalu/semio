import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "../../📸️snapshot/🔣️.json" with {type:"json"};
test("original DXF typed ownership corpus keeps independent UTF8 and table/entity fields",()=>{
 const copy=JSON.parse(JSON.stringify(fixture));expect(copy).toEqual(fixture);expect(new Ajv({strict:false}).compile(schema)(fixture.snapshot)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE strings(value TEXT NOT NULL)");db.run("INSERT INTO strings VALUES(?)",fixture.snapshot.entities[0]!.text.value);const row=db.query("SELECT length(CAST(value AS BLOB)) AS bytes FROM strings").get();expect(row).toEqual({bytes:new TextEncoder().encode(fixture.snapshot.entities[0]!.text.value).length});expect(fixture.snapshot.tables.styles[0]!.fontName).toBe("Ä Font");expect(fixture.mutation.headerVar.value.value).toBe("Owned Grüße 😀");expect(fixture.copyGrant).toBe(7)}finally{db.close()}
});
