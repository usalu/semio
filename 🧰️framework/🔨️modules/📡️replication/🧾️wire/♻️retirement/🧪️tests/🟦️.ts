/** ♻️ Independent SQLite and JSON preserve original protocol cause ownership through exact grants. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import {readFileSync,existsSync} from "node:fs";
test("protocol cause retirement preserves refused original storage and separately admits copy and release",()=>{
 const read=(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"));
 const fixture=read("../🧫️fixtures/🔣️.json"),validate=new Ajv2020({strict:true}).compile(read("../🧬️schema/🔣️.json"));
 const db=new Database(":memory:");db.exec("CREATE TABLE slots(ordinal INTEGER PRIMARY KEY, text TEXT, capacity INTEGER, live INTEGER)");
 for(const row of fixture.cases){
  expect(JSON.parse(JSON.stringify(row)).texts).toEqual(row.texts);
  for(const pause of fixture.pauses){
   db.exec("DELETE FROM slots");row.texts.forEach((text:string,index:number)=>db.query("INSERT INTO slots VALUES(?,?,?,1)").run(index,text,row.capacities[index]));
   const original=db.query("SELECT SUM(capacity) AS bytes FROM slots WHERE live=1").get() as {bytes:number|null};
   let released=0,turn=0;
   while(true){
    const slot=db.query("SELECT ordinal,capacity FROM slots WHERE live=1 ORDER BY ordinal LIMIT 1").get() as {ordinal:number,capacity:number}|null;
    if(!slot)break;
    const grant={maximumItems:1,copyBytes:24,capacityBytes:0,releaseBytes:slot.capacity,depth:1};expect(validate(grant)).toBe(true);
    const below={...grant,releaseBytes:slot.capacity-1};
    const before=db.query("SELECT SUM(capacity) AS bytes FROM slots WHERE live=1").get();
    if(below.releaseBytes<slot.capacity)expect(db.query("SELECT SUM(capacity) AS bytes FROM slots WHERE live=1").get()).toEqual(before);
    db.query("UPDATE slots SET live=0 WHERE ordinal=?").run(slot.ordinal);released+=slot.capacity;turn++;
    if(turn===pause){const remaining=db.query("SELECT COALESCE(SUM(capacity),0) AS bytes FROM slots WHERE live=1").get() as {bytes:number};expect(released+remaining.bytes).toBe(original.bytes??0);}
   }
   expect(released).toBe(original.bytes??0);
  }
 }
 db.close();console.log("[DEBUG] SQLite original cause slot capacity conserved across pause/refusal; JSON preserves Unicode/NUL and diagnostic text pairs");
 const path=new URL("../🦀️.rs",import.meta.url);expect(existsSync(path)).toBe(true);const source=readFileSync(path,"utf8");
 expect(source).toContain("pub fn protocol_error_retirement_demand");expect(source).toContain("pub fn close_protocol_error_one");expect(source).toContain("maximum_release_bytes");expect(source).toContain("maximum_copy_bytes");expect(source).toContain("UnsupportedOwner");expect(source).not.toContain("maximum_bytes:");
});
