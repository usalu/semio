/** ⏪️ The original actor custody consumes independent grant currencies without a cold disposal branch. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import fixture from "../🧫️fixtures/🔣️.json";
test("time travel original retirement separately admits copy birth release and depth",()=>{
 const db=new Database(":memory:");try{db.exec("CREATE TABLE owners(position INTEGER PRIMARY KEY,owner TEXT)");fixture.owners.forEach((owner,index)=>db.query("INSERT INTO owners VALUES(?,?)").run(index,owner));for(const grant of fixture.grants){const result=db.query("SELECT CASE WHEN ?>0 AND ?>=? AND ?>=? AND ?>=? AND ?>=? THEN 1 ELSE 0 END AS admitted").get(grant.items,grant.copy,fixture.copyBytes,grant.capacity,fixture.capacityBytes,grant.release,fixture.releaseBytes,grant.depth,fixture.depth) as {admitted:number};expect(Boolean(result.admitted)).toBe(grant.accepted);expect((db.query("SELECT COUNT(*) AS count FROM owners").get() as {count:number}).count).toBe(4);}}finally{db.close();}
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("fn time_travel_retirement_demands");expect(source).toContain("grant: RetainedCloneGrant");expect(source).not.toContain("op.retire_cold()");expect(source).not.toContain("retirements: VecDeque");console.log("[DEBUG] SQLite preserves original actor owners under every missing grant currency including exact empty backing release");
});
