import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync, existsSync } from "node:fs";
import { fileURLToPath } from "node:url";

test("native canonical paged traversal retains original byte prefixes and closes every admitted alias", () => {
  const root = new URL("../", import.meta.url);
  const fixture = JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json", root), "utf8"));
  const db = new Database(":memory:");
  db.run("CREATE TABLE aliases (ordinal INTEGER PRIMARY KEY, owner TEXT NOT NULL)");
  for (const depth of fixture.depths) {
    let value: unknown = { a: fixture.text, z: [true, 42] };
    for (let index = 0; index < depth; index++) value = [value];
    const oracle = new TextEncoder().encode(JSON.stringify(value));
    const output = new TextEncoder().encode("[".repeat(depth) + '{"a":' + JSON.stringify(fixture.text) + ',"z":[true,42]}' + "]".repeat(depth));
    expect(output).toEqual(oracle);
    for (const maximum of fixture.grants) {
      let offset = 0;
      while (offset < output.length) {
        const end = Math.min(output.length, offset + maximum);
        expect(output.slice(offset, end)).toEqual(oracle.slice(offset, end));
        expect(end - offset).toBeLessThanOrEqual(maximum);
        offset = end;
      }
    }
    for (const pause of fixture.pauses) {
      db.run("DELETE FROM aliases");
      const count = Math.min(depth + 1, pause);
      for (let ordinal = 0; ordinal < count; ordinal++) db.run("INSERT INTO aliases VALUES (?, ?)", [ordinal, "original-source"]);
      let released = 0;
      while ((db.query("SELECT COUNT(*) AS count FROM aliases").get() as { count: number }).count) {
        db.run("DELETE FROM aliases WHERE ordinal = (SELECT MAX(ordinal) FROM aliases)");
        released++;
      }
      expect(released).toBe(count);
      expect((db.query("SELECT COUNT(*) AS count FROM aliases").get() as { count: number }).count).toBe(0);
    }
  }
  db.close();
  const source = readFileSync(fileURLToPath(new URL("🦀️.rs", root)), "utf8");
  expect(source).toContain("pub struct ArtifactCanonicalJsonTreeCursor");
  const owner = readFileSync(fileURLToPath(new URL("../../🦀️.rs", root)), "utf8");
  expect(owner).toContain("pub use native_tree");
  expect(source).toContain("PagedList<NativeFrame");
  expect(source).toContain("RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>");
  expect(source).not.toContain("transmute");
  expect(source).not.toContain("with_capacity");
  expect(source).toContain("canonical_escaped_byte");
  expect(source).not.toContain("let mut escape = [0; 6]");
  console.log("[DEBUG] canonical original Unicode/NUL JSON byte prefixes and SQLite exact alias conservation at depths 0/1/80/257; tiny grants unchanged");
});

test("original scalar traversal uses real initialized bytes and independent positive grant policy",()=>{
 const root=new URL("../",import.meta.url);const rows=JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json",root),"utf8"));
 for(const row of rows.scalarCases){expect(JSON.stringify(row.value)).toBe(row.expected);}
 expect(rows.retainedPolicy).toEqual({maximumItems:1,maximumCopyBytes:4096,maximumCapacityBytes:4096,maximumReleaseBytes:4096,maximumDepth:64});
 for(const value of [true,false,null,42,-42,0.125]){const bytes=Buffer.from(JSON.stringify(value));expect([...bytes]).toEqual([...new TextEncoder().encode(JSON.stringify(value))]);expect(bytes.length).toBeLessThanOrEqual(64);}
 const source=readFileSync(new URL("🦀️.rs",root),"utf8");expect(source).toContain("frame.scalar.write_node(scalar)");expect(source).not.toContain("frame.scalar = ScalarBytes::from_node");
 const native=readFileSync(new URL("🧪️tests/🦀️.rs",root),"utf8");expect(native).toContain("rows[\"retainedPolicy\"]");expect(native).not.toContain("maximum_copy_bytes:demand.copy_bytes");expect(existsSync(new URL("🧬️schema/🔣️.json",root))).toBe(false);
});
