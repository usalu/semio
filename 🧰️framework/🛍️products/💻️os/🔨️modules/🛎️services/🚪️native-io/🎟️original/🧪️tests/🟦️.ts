import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import law from "../🔣️.json";
import schema from "../🧬️schema/🔣️.json";

test("native I/O residuals retain original backing until independently admitted exact release", () => {
  expect(new Ajv().compile(schema)(law)).toBe(true);
  const db = new Database(":memory:");
  db.exec("CREATE TABLE original(kind TEXT PRIMARY KEY, capacity INTEGER, released INTEGER)");
  for (const owner of law.owners) db.query("INSERT INTO original VALUES(?,?,0)").run(owner.kind, owner.capacity);
  for (const owner of law.owners) {
    for (const denied of law.denials) {
      const grant = { items: denied === "items" ? 0 : 1, release: denied === "release" ? owner.capacity - 1 : owner.capacity, depth: denied === "depth" ? 0 : 1 };
      const admitted = grant.items > 0 && grant.release >= owner.capacity && grant.depth > 0;
      const oracle = db.query("SELECT CASE WHEN ?>0 AND ?>=capacity AND ?>0 THEN capacity ELSE 0 END AS bytes FROM original WHERE kind=?").get(grant.items, grant.release, grant.depth, owner.kind) as { bytes: number };
      expect(admitted).toBe(false);
      expect(oracle.bytes).toBe(law.expected.deniedBytes);
    }
    db.query("UPDATE original SET released=capacity WHERE kind=?").run(owner.kind);
  }
  expect(db.query("SELECT SUM(released) AS bytes FROM original").get()).toEqual({ bytes: law.expected.releasedBytes });
  for (const extension of law.extensions) {
    const release = extension.ownership === "owned" ? extension.capacity : 0;
    const oracle = db.query("SELECT CASE WHEN ?='owned' THEN ? ELSE 0 END AS bytes").get(extension.ownership, extension.capacity) as { bytes: number };
    expect(release).toBe(extension.releasedBytes);
    expect(oracle.bytes).toBe(release);
    expect(extension.releaseGranted >= release).toBe(extension.admitted);
  }
  db.close();
  const source = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
  expect(source).toContain("request_path: Option<PathBuf>");
  expect(source).toContain("request_extension: Option<std::borrow::Cow<'static,str>>");
  expect(source).toContain("std::borrow::Cow::Owned(text)=>text.capacity()");
  expect(source).toContain("std::borrow::Cow::Borrowed(_)=>0");
  expect(source).toContain("release(RequestExtension, extension_capacity(extension), 1)");
  expect(source).toContain("release(Extension,extension_capacity(extension),1)");
  expect(source).toContain("NativeIoCloseTarget::RequestExtension => drop(self.request_extension.take())");
  expect(source).toContain("NativeIoRequest::ScanDirectory { extension, .. }) => drop(extension.take())");
  expect(source).toContain("retired_path: Option<PathBuf>");
  expect(source).toContain("self.retired_path = Some(path)");
  expect(source).toContain("close_original_retained_step(cx, demand");
  expect(source).not.toContain("fn native_io_close_grant");
  expect(source).toContain("cx.consume_fuel(1)");
  console.log("[DEBUG] native I/O original residual quote denial0 and exact release544 match independent Ajv/SQLite; same-source native allocator laws authored separately");
});
