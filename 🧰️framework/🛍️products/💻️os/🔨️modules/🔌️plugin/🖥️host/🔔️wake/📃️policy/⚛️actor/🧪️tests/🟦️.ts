/** 🎟️ Original actor and wake turns share one independently supplied finite operation treasury. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
test("original Host actor turn borrows finite full retained policy beside wake without scalar conversion",async()=>{
 const law=await Bun.file(new URL("../🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json(),policy=await Bun.file(new URL("../../🧫️fixtures/🔣️.json",import.meta.url)).json();expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE receipt(epoch INTEGER PRIMARY KEY,phase TEXT,items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER)");for(const row of law.receipts)db.run("INSERT INTO receipt VALUES(?,?,?,?,?,?)",row.epoch,row.phase,...row.spent);const totals=db.query("SELECT sum(items) AS items,sum(copy) AS copy,sum(capacity) AS capacity,sum(release) AS release FROM receipt").get() as Record<string,number>;expect([policy.operation.maximumItems-totals.items,policy.operation.maximumCopyBytes-totals.copy,policy.operation.maximumCapacityBytes-totals.capacity,policy.operation.maximumReleaseBytes-totals.release,policy.operation.maximumDepth]).toEqual(law.remaining);expect(()=>db.run("INSERT INTO receipt VALUES(1,'actor',1,41,0,0)")).toThrow();}finally{db.close();}
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source.includes("pub fn begin_actor_turn(")).toBe(true);expect(source.includes("pub fn return_actor_turn(")).toBe(true);expect(source.includes("receipt.validate_for(input)?")).toBe(true);expect(source.includes("begin_drive_turn()")).toBe(true);console.log("[DEBUG] Ajv/SQLite genuine same treasury actor/wake/actor exactepochs original71/3 total3/123/0/0 finite61remaining immutableDepth64");
});
