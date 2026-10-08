/** ♻️ SQLite independently accounts for one full preparation-close grant. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
test("one-item preparation closure forwards every admitted axis exactly once",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE admitted(axis TEXT PRIMARY KEY, bytes INTEGER)");
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 for(const [axis,bytes] of Object.entries(law.grant) as [string,number][])db.query("INSERT INTO admitted VALUES(?,?)").run(axis,bytes);
 expect(db.query("SELECT SUM(bytes) AS total FROM admitted").get()).toEqual({total:law.total});
 const root=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");
 const start=root.indexOf("fn spend_item_close_grant(");const end=root.indexOf("\n    }",start);const body=root.slice(start,end);
 expect(body.includes("RetainedCloneStep")).toBe(true);
 expect(body.includes("current.close_step(child_grant)")).toBe(true);
 expect(body.includes("current.next_close_depth_demand()?.checked_add(1)")).toBe(true);
 expect(body.includes("child_grant.retained_grant()")).toBe(true);
 expect(body.includes("std::mem::size_of_val(current.as_ref())")).toBe(true);
 expect(body.includes("bytes > grant.maximum_release_bytes")).toBe(true);
 expect(body.includes("released_bytes: bytes")).toBe(true);
 expect(body.includes("maximum_bytes")).toBe(false);
 expect(body.includes("spend_close_byte_grant")).toBe(false);
 expect(root.includes("matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_))")).toBe(true);
 db.close();console.log("[DEBUG] SQLite four-axis grant conservation retains independent copy/capacity/release/depth; preparation close takes one genuine caller turn");
});
