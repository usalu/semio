import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import law from "../🔣️.json";

test("original tool ordinal qualification agrees with SQLite and bounded byte comparisons",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE tools (id TEXT PRIMARY KEY, role TEXT NOT NULL)");for(const id of law.frameworkTools)db.query("INSERT INTO tools VALUES (?,?)").run(id,"framework");const row=db.query("SELECT id,role FROM tools ORDER BY id LIMIT 1 OFFSET ?").get(law.targetOrdinal) as {id:string,role:string};expect(row.id).toBe(law.targetId);expect(row.role).toBe(law.roles[1]);
 const target=Buffer.from(law.targetId);for(const id of law.frameworkTools){const source=Buffer.from(id);let equal=source.length===target.length;for(let offset=0;equal&&offset<source.length;offset+=law.comparisonBytes)equal=source.subarray(offset,offset+law.comparisonBytes).equals(target.subarray(offset,offset+law.comparisonBytes));expect(equal).toBe(id===law.targetId)}db.close();
});
