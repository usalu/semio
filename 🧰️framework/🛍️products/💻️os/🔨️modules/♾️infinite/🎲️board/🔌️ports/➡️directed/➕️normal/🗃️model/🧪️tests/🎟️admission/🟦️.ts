import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
const owner=new URL("../../",import.meta.url),source=readFileSync(new URL("🪣️fill-job/🦀️.rs",owner),"utf8"),parent=readFileSync(new URL("🦀️.rs",owner),"utf8");
const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🎟️admission/🔣️.json",import.meta.url),"utf8"));
const schema=JSON.parse(readFileSync(new URL("🧬️schema/🎟️admission/🔣️.json",owner),"utf8"));
test("original board fill descriptor layouts and separate semantic serial agree with SQLite",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");try{
  db.run("CREATE TABLE descriptor(owner TEXT,pages INTEGER)");
  for(const pages of law.scratchDescriptorPages)db.query("INSERT INTO descriptor VALUES('job',?)").run(pages);
  for(const pages of law.snapshotDescriptorPages)db.query("INSERT INTO descriptor VALUES('snapshot',?)").run(pages);
  expect(db.query("SELECT owner,COUNT(*) count,SUM(pages) pages FROM descriptor GROUP BY owner ORDER BY owner").all()).toEqual([{owner:"job",count:4,pages:8762},{owner:"snapshot",count:4,pages:784}]);
  db.run("CREATE TABLE capture(position INTEGER,admitted_pages INTEGER)");
  for(const position of [0,law.pageItems-1,law.pageItems]) db.query("INSERT INTO capture VALUES(?,?)").run(position,Math.ceil((position+1)/law.pageItems)-Math.ceil(position/law.pageItems));
  expect(db.query("SELECT position,CAST(position % ? = 0 AS INTEGER) pages FROM capture ORDER BY position").all(law.pageItems)).toEqual(db.query("SELECT position,admitted_pages pages FROM capture ORDER BY position").all());
 }finally{db.close();}
 expect(law.physicalOperation).not.toBe(law.semanticSerial);
 expect(source.includes("next_serial: operation.operation.0")).toBe(true);
 expect(source.includes("original_source: Some(original_source)")).toBe(true);
 expect(source.includes("Self::from_original_state(original, max_count, semantic_operation, binding)")).toBe(true);
 expect(source.includes("pub fn with_operation(")).toBe(false);
 expect(source.includes("pub fn step(&mut self, host: &BoardHost, cx: &mut semio_framework_job::StepContext")).toBe(true);
 expect(source.includes("let demand = self.next_step_demands(host)?")).toBe(true);
 expect(source.includes("self.validate_original_source(context)?")).toBe(true);
 expect(source.includes("self.validate_original_source(cx)?")).toBe(true);
 expect(parent.includes("Layout::array::<Option<BoardFillPage<T>>>(CAPACITY)")).toBe(true);
 expect(source.includes("pub fn next_close_demands(&self, _body: usize)")).toBe(true);
 expect(law.originalReceiptCount).toBe(1);
 console.log("[DEBUG] Ajv/SQLite Board original4+4 descriptor layouts, semantic73 differs physical11, original scoped birth and snapshot paid-close custody");
});
