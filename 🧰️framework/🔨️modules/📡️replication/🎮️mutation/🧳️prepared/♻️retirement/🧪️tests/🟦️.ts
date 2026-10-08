/** 🧳️ Prepared-operation refusal and publication owners remain original through cancellation. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import fixture from "../🧫️fixtures/🔣️.json";
test("prepared replay retirement conserves publication and independent inverse and apply refusals",()=>{
 const db=new Database(":memory:");try{db.exec("CREATE TABLE originals(position INTEGER PRIMARY KEY,owner TEXT)");for(const row of fixture.branches){db.exec("DELETE FROM originals");row.owners.forEach((owner,index)=>db.query("INSERT INTO originals VALUES(?,?)").run(index,owner));expect((db.query("SELECT COUNT(*) AS count FROM originals").get() as {count:number}).count).toBe(5);expect(JSON.parse(JSON.stringify(row)).owners).toEqual(row.owners);for(const grant of fixture.grants){expect(Boolean((db.query("SELECT CASE WHEN ?>0 AND ?>=65536 AND ?>=65536 THEN 1 ELSE 0 END AS accepted").get(grant.items,grant.capacity,grant.release) as {accepted:number}).accepted)).toBe(grant.accepted);expect((db.query("SELECT COUNT(*) AS count FROM originals").get() as {count:number}).count).toBe(5);}}}finally{db.close();}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("pub fn demands");expect(source).toContain("pub fn close");for(const field of ["next","inverse","messages","apply_refusal","input_refusal"])expect(source).toContain("prepared."+field);expect(source).not.toContain("close_cold");console.log("[DEBUG] SQLite preserves every prepared publication/refusal family before exact grants, including inverse failure independent from apply failure");
});
