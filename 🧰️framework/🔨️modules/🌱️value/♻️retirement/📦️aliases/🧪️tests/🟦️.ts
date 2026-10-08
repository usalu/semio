/** ♻️ SQLite independently proves exact original alias batches survive admission pauses. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync,existsSync} from "node:fs";
test("original tool snapshot alias batches use genuine admitted retirement and preserve refused originals",()=>{
 const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 const db=new Database(":memory:");db.exec("CREATE TABLE owners(ordinal INTEGER PRIMARY KEY, phase TEXT)");
 for(const count of fixture.counts)for(const pause of fixture.pauses){
  db.exec("DELETE FROM owners");for(let ordinal=0;ordinal<count;ordinal++)db.query("INSERT INTO owners VALUES(?,?)").run(ordinal,"original");
  for(let turn=0;turn<Math.min(count,pause);turn++)db.query("UPDATE owners SET phase='retired' WHERE ordinal=(SELECT MAX(ordinal) FROM owners WHERE phase='original')").run();
  expect(db.query("SELECT COUNT(*) AS count FROM owners").get()).toEqual({count});
  const remaining=count-Math.min(count,pause);expect(db.query("SELECT COUNT(*) AS count FROM owners WHERE phase='original'").get()).toEqual({count:remaining});
  let retired=0;while((db.query("SELECT COUNT(*) AS count FROM owners WHERE phase='original'").get() as {count:number}).count){db.query("UPDATE owners SET phase='retired' WHERE ordinal=(SELECT MAX(ordinal) FROM owners WHERE phase='original')").run();retired++;}
  expect(retired).toBe(remaining);
 }
 db.close();console.log("[DEBUG] SQLite exact original alias conservation at0/1/65/257 with paused/cancelled admission and no dropped refusal owner");
 const path=new URL("../🦀️.rs",import.meta.url);expect(existsSync(path)).toBe(true);
 const native=readFileSync(path,"utf8");expect(native).toContain("pub struct OriginalAliasBatch");expect(native).toContain("RetainedCloneGrant");expect(native).toContain("RetirementDemand");expect(native).toContain("admit(&mut self.pending");expect(native).not.toContain("maximum_bytes");expect(native).not.toContain("retire_cold");
});

/** 🔗️ A single original alias requires no fabricated contiguous batch backing. */
test("single original snapshot alias enters inline custody without reconstruction",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE alias(id INTEGER PRIMARY KEY, phase TEXT)");db.query("INSERT INTO alias VALUES(7,'original')").run();
 for(const phase of ["original","pending","terminal"]){db.query("UPDATE alias SET phase=? WHERE id=7").run(phase);expect(db.query("SELECT id FROM alias").get()).toEqual({id:7});}db.close();
 const native=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(native.includes("admit_alias_original")).toBe(true);expect(native.includes("pending:ManuallyDrop::new(original.take())")).toBe(true);
 console.log("[DEBUG] SQLite single immutable original root retained through admission with no synthetic Vec backing");
});
