import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync, existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";

test("native canonical paged traversal retains original byte prefixes and closes every admitted alias", () => {
  const root = new URL("../", import.meta.url);
  const fixture = JSON.parse(readFileSync(new URL("🧪️tests/🧫️fixtures/🔣️.json", root), "utf8"));
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
});

test("original CQT header transfer agrees with strict neutral authority and SQLite alias identity",()=>{
 const root=new URL("../",import.meta.url);const law=JSON.parse(readFileSync(new URL("🧪️tests/🧫️fixtures/🎟️header.json",root),"utf8"));const schema=JSON.parse(readFileSync(new URL("🧬️schema/🎟️header.json",root),"utf8"));const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,headerAdmission:{...law.headerAdmission,copyBytes:1}})).toBe(false);
 const db=new Database(":memory:");db.run("CREATE TABLE aliases (id INTEGER PRIMARY KEY, owner TEXT NOT NULL, source INTEGER NOT NULL)");db.run("INSERT INTO aliases VALUES (1,'source',17),(2,'projection',17)");
 for(const axis of law.deniedAxes){expect(["items","depth"]).toContain(axis);expect(db.query("SELECT source FROM aliases WHERE id=2").get()).toEqual({source:17});}
 db.run("UPDATE aliases SET owner='cursor' WHERE id=2");expect(db.query("SELECT id,source FROM aliases WHERE owner='cursor'").get()).toEqual({id:2,source:17});db.run("DELETE FROM aliases WHERE id=2");expect(db.query("SELECT COUNT(*) AS count FROM aliases").get()).toEqual({count:1});db.run("DELETE FROM aliases WHERE id=1");expect(db.query("SELECT COUNT(*) AS count FROM aliases").get()).toEqual({count:0});db.close();
 expect(law.headerAdmission).toEqual({items:1,copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1});expect(law.pendingHeaderCloseCopyBytes).toBe(0);
});

test("original scalar traversal uses real initialized bytes and independent positive grant policy",()=>{
 const root=new URL("../",import.meta.url);const rows=JSON.parse(readFileSync(new URL("🧪️tests/🧫️fixtures/🔣️.json",root),"utf8"));
 for(const row of rows.scalarCases){expect(JSON.stringify(row.value)).toBe(row.expected);}
 expect(rows.retainedPolicy).toEqual({maximumItems:1,maximumCopyBytes:4096,maximumCapacityBytes:4096,maximumReleaseBytes:4096,maximumDepth:64});
 for(const value of [true,false,null,42,-42,0.125]){const bytes=Buffer.from(JSON.stringify(value));expect([...bytes]).toEqual([...new TextEncoder().encode(JSON.stringify(value))]);expect(bytes.length).toBeLessThanOrEqual(64);}
 const source=readFileSync(new URL("🦀️.rs",root),"utf8");expect(source).toContain("frame.scalar.write_node(scalar)");expect(source).not.toContain("frame.scalar = ScalarBytes::from_node");
 const native=readFileSync(new URL("🧪️tests/🦀️.rs",root),"utf8");expect(native).toContain("rows[\"retainedPolicy\"]");expect(native).not.toContain("maximum_copy_bytes:demand.copy_bytes");expect(existsSync(new URL("🧬️schema/🔣️.json",root))).toBe(false);
});

test("original nested CQT header events preserve independent JSON body and SQLite aliases",()=>{
 const root=new URL("../",import.meta.url);const law=JSON.parse(readFileSync(new URL("🧪️tests/🧫️fixtures/🎟️header.json",root),"utf8"));expect(law.frameTransferCopyBytes).toBe(0);expect(law.childAliasCopyBytes).toBe(0);expect(law.terminalFrameCopyBytes).toBe(0);expect(law.scalarInitializationCopyBytes).toBe(64);expect(law.firstObjectKeyHeaderCopyBytes).toBe(0);
 for(const depth of law.traversalDepths){let value:unknown={a:"Ä\0\n🧩\"\\",z:[true,42]};for(let n=0;n<depth;n++)value=[value];const oracle=JSON.stringify(value);expect(JSON.parse(oracle)).toEqual(value);const db=new Database(":memory:");db.run("CREATE TABLE aliases (ordinal INTEGER PRIMARY KEY, source INTEGER NOT NULL)");for(let n=0;n<=depth;n++)db.run("INSERT INTO aliases VALUES (?,17)",[n]);expect(db.query("SELECT DISTINCT source FROM aliases").all()).toEqual([{source:17}]);for(let n=depth;n>=0;n--)db.run("DELETE FROM aliases WHERE ordinal=?",[n]);expect(db.query("SELECT COUNT(*) AS count FROM aliases").get()).toEqual({count:0});db.close();}
});

/** 🗺️ First-party map ranks retain canonical JSON ordering without a dictionary projection. */
test("retained map canonical keys agree with independent SQLite binary ordering",()=>{const root=new URL("../",import.meta.url),law=JSON.parse(readFileSync(new URL("🧪️tests/🧫️fixtures/🗺️retained-map.json",root),"utf8")),schema=JSON.parse(readFileSync(new URL("🧬️schema/🗺️retained-map.json",root),"utf8"));expect(validateJsonSchemaSubset(schema,law)).toEqual([]);expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE entries(key TEXT PRIMARY KEY COLLATE BINARY,value INTEGER NOT NULL)");for(const row of law.cases){db.exec("DELETE FROM entries");for(const{key,value}of row.entries)db.query("INSERT INTO entries VALUES(?,?) ON CONFLICT(key) DO UPDATE SET value=excluded.value").run(key,value);const actual=Object.fromEntries([...new Map<string,number>(row.entries.map(({key,value}:{key:string,value:number}):[string,number]=>[key,value]))].sort(([a],[b])=>{const aa=new TextEncoder().encode(a),bb=new TextEncoder().encode(b);for(let i=0;i<Math.min(aa.length,bb.length);i++){if(aa[i]!==bb[i])return aa[i]!-bb[i]!;}return aa.length-bb.length;})),other=Object.fromEntries((db.query("SELECT key,value FROM entries ORDER BY key COLLATE BINARY").all()as {key:string,value:number}[]).map(({key,value})=>[key,value]));expect(JSON.stringify(actual)).toBe(row.expected);expect(JSON.stringify(other)).toBe(row.expected);}}finally{db.close()}const source=readFileSync(new URL("🦀️.rs",root),"utf8");expect(source).toContain("ArtifactCanonicalJsonTree for semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap<K,V>");console.log("[DEBUG] Retained dictionary canonical bytes agree with SQLite key ranks, Unicode/NUL and duplicate replacement; authored native traversal is separate");});
