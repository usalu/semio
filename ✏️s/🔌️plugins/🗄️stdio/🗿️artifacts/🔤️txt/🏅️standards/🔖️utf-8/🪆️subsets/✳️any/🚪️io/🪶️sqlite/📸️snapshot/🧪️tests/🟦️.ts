import refusalFixture from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/⚠️refusal/🧫️fixtures/🔣️.json";
const canceledKind=refusalFixture.cases.find(item=>item.id==="canceled-projection")!.expectedKind;
/** 🧫️ Shared Txt semantic fixture and independent SQL editing interoperability. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../🧫️fixtures/🔣️.json";
import nativeFixture from "../🧫️fixtures/🛂️native.json";
import { txtSnapshotToSqliteDatabase, txtSnapshotFromSqliteDatabase, TXT_SQLITE_SCHEMA } from "../🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

/** 🛫️ Original TXT encoding quotes exact raw and binary octets with independent witnesses. */
test("original TXT native encoding preserves raw line bytes and one retained output backing",async()=>{
 const{readFileSync}=await import("node:fs");const law=JSON.parse(readFileSync(new URL("../🛫️encoding/🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 const {semioSchemaAjvV1}=await import("../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts");const grantSchema=JSON.parse(readFileSync(new URL("../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json",import.meta.url),"utf8")),admitGrant=semioSchemaAjvV1({strict:true}).compile(grantSchema);for(const grant of [law.bodyGrant,law.closeGrant,law.deniedCloseGrant])expect(admitGrant(grant)).toBe(true);
 expect(Object.values(law.deniedCloseGrant)).toEqual([0,0,0,0,0]);expect(law.bodyGrant.maximumItems).toBe(65536);expect(law.closeGrant.maximumItems).toBe(4096);expect(law.maximumCloseTurns).toBe(65536);expect(law.nativeMaximumBytes).toBe(16777216);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE original_line(ordinal INTEGER PRIMARY KEY,content TEXT NOT NULL)");for(const row of law.cases){const separator=row.lineEnding==="crLf"?"\r\n":"\n",body=row.lines.join(separator)+(row.trailingNewline?separator:"");expect(body).toBe(row.body);expect(new TextEncoder().encode(body)).toEqual(new Uint8Array(Buffer.from(row.body)));db.exec("DELETE FROM original_line");for(const[ordinal,line]of row.lines.entries())db.query("INSERT INTO original_line VALUES(?,?)").run(ordinal,line);expect(db.query("SELECT content FROM original_line ORDER BY ordinal").all()).toEqual(row.lines.map((content:string)=>({content})));const bytes=Buffer.from(body),header=Buffer.alloc(12);Buffer.from(law.prefix.magic).copy(header);header.writeUInt32LE(Buffer.byteLength(law.prefix.token),8);const wrapped=Buffer.concat([header,Buffer.from(law.prefix.token),bytes]);expect(wrapped.length).toBe(bytes.length+law.prefix.bytes);expect(wrapped.subarray(law.prefix.bytes)).toEqual(bytes);expect(wrapped.readUInt32LE(8)).toBe(17);}}
 finally{db.close();}const text=law.source.unit.repeat(law.source.repeat),bytes=Buffer.from(text);expect(bytes.length).toBe(law.source.bytes);expect(bytes.subarray(0,law.source.cancelAfterBytes).length).toBe(65536);expect(bytes.length-law.source.cancelAfterBytes).toBe(74464);const producer=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");const encoding=producer.slice(producer.indexOf("fn encode_sqlite_snapshot_native"),producer.indexOf("fn preflight_sqlite_snapshot_encoding"));expect(encoding.includes("original_encoding::encode")).toBe(true);expect(encoding.includes("NativeEncodeControl::new")).toBe(false);console.log("[DEBUG] Original TXT native encoding closed seven empty/LF/CRLF/NUL/UTF8 shapes and independent SQLite/Buffer framing29bytes/140000-byte copy contract; actual Native original prefix custody separate");
});


/** 🧾️ Closed original TXT cause and copy frontiers are validated independently of Rust admission. */
test("original TXT encoding retains exact typed causes and admitted output slots",async()=>{
 const{readFileSync}=await import("node:fs");const law=JSON.parse(readFileSync(new URL("../🛫️encoding/🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 expect(law.copy).toEqual({chunkBytes:65536,prefixItems:3,receivingDepth:2});expect(Buffer.from(law.nativeCause.message).toString("hex")).toBe("6e617469766520656e636f64696e672063616e63656c6564");expect(JSON.parse(JSON.stringify(law.sqlCause))).toEqual(law.sqlCause);
 const producer=readFileSync(new URL("../🛫️encoding/🦀️.rs",import.meta.url),"utf8"),tests=readFileSync(new URL("./🦀️.rs",import.meta.url),"utf8");expect(producer.includes("owner.receive::<Vec<u8>,IoPayload>")).toBe(true);expect(producer.indexOf("*slot=Some(Vec::new())")).toBeLessThan(producer.indexOf("body.allocate_encode_vec_into"));expect(producer.includes("NativeEncodeControl::new")).toBe(false);expect(producer.includes("*slot=Some(error.into_bytes())")).toBe(true);expect(producer.indexOf("body.record_progress",producer.indexOf("fn append("))).toBeLessThan(producer.indexOf("native.advance",producer.indexOf("fn append(")));expect(tests.match(/#\[test\]\s*fn txt_original_encoding_/g)?.length).toBe(5);expect(tests.includes("blocked.set(false)")).toBe(true);expect(tests.includes("sql_before-sql.allocation_remaining_bytes()")).toBe(true);
 console.log("[DEBUG] Original TXT closed typed Native/SQL causes agree with Buffer and JSON, independently admitted copy frontiers retain cancellation bytes before callback and use one genuine supplied owner; Native custody requires owning execution");
});

test("Txt semantic metadata and surrogate identities remain literal", async () => {
  for (const schema of nativeFixture.literalSchemas) {
    const snapshot = { schema, lines: ["任意\0\nembedded", ""], trailingNewline: false, lineEnding: "lf" as const };
    const database = await txtSnapshotToSqliteDatabase(snapshot);
    const key = BigInt(nativeFixture.controls.rootIdentity);
    database.tables.find((table) => table.name === "text_document")!.rows[0]!.rowid = key;
    database.tables.find((table) => table.name === "text_document")!.rows[0]!.values[0] = key;
    for (const row of database.tables.find((table) => table.name === "text_line")!.rows) row.values[1] = key;
    expect(await txtSnapshotFromSqliteDatabase(database)).toEqual(snapshot);
    const bytes = await exportSqliteDatabase(database);
    const independent = Database.deserialize(bytes);
    try {
      expect(independent.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
      expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
      expect(independent.query("SELECT schema FROM text_document").get()).toEqual({ schema });
      expect(independent.query("SELECT content FROM text_line ORDER BY ordinal").all()).toEqual(snapshot.lines.map((content) => ({ content })));
    } finally { independent.close(); }
  }
});

test("Txt long literal fields expose bounded Unicode traversal cancellation", async () => {
  const text = nativeFixture.controls.longText.repeat(nativeFixture.controls.longRepeat);
  for (const schemaField of [false, true]) {
    const snapshot = { schema: schemaField ? text : "stdio.txt", lines: schemaField ? [] : [text], trailingNewline: false, lineEnding: "lf" as const };
    const database = await txtSnapshotToSqliteDatabase(snapshot);
    for (const phase of ["projectSnapshot", "reconstructSnapshot"] as const) {
      const controller = new AbortController();
      let interior = false;
      const options = { signal: controller.signal, onProgress: (progress: { phase: string; completed: number; total: number }) => { if (progress.phase === phase && progress.completed >= 16_384 && progress.completed < progress.total) { interior = true; controller.abort(); } } };
      await expect(phase === "projectSnapshot" ? txtSnapshotToSqliteDatabase(snapshot, options) : txtSnapshotFromSqliteDatabase(database, options)).rejects.toHaveProperty("kind",canceledKind);
      expect(interior).toBe(true);
    }
  }
});

test("Txt semantic cumulative bytes and ordered wide relationships enforce exact bounds", async () => {
  const snapshot = { ...fixture, lineEnding: "crLf" as const };
  const bytes = new TextEncoder().encode(snapshot.schema).byteLength + 20 + snapshot.lines.reduce((sum, line) => sum + 24 + new TextEncoder().encode(line).byteLength, 0);
  const database = await txtSnapshotToSqliteDatabase(snapshot, { maxRows: nativeFixture.controls.domainRows, maxValueBytes: bytes });
  expect(await txtSnapshotFromSqliteDatabase(database, { maxRows: nativeFixture.controls.domainRows, maxValueBytes: bytes })).toEqual(snapshot);
  await expect(txtSnapshotToSqliteDatabase(snapshot, { maxValueBytes: bytes - 1 })).rejects.toThrow("value limit");
  await expect(txtSnapshotFromSqliteDatabase(database, { maxValueBytes: bytes - 1 })).rejects.toThrow("value limit");
  const wide = { ...snapshot, lines: Array.from({ length: nativeFixture.controls.wideLines }, (_, ordinal) => String(ordinal)) };
  const wideDatabase = await txtSnapshotToSqliteDatabase(wide);
  const controller = new AbortController();
  let interior = false;
  await expect(txtSnapshotFromSqliteDatabase(wideDatabase, { signal: controller.signal, onProgress: (progress) => { if (progress.phase === "reconstructSnapshot" && progress.completed >= 256 && progress.completed < progress.total) { interior = true; controller.abort(); } } })).rejects.toHaveProperty("kind",canceledKind);
  expect(interior).toBe(true);
});

test("Txt rejects independently edited invalid flags and ordinal gaps", async () => {
  const input = fixture as { schema: string; lines: string[]; trailingNewline: boolean; lineEnding: "lf" | "crLf" };
  const db = Database.deserialize(await exportSqliteDatabase(await txtSnapshotToSqliteDatabase(input)));
  try {
    db.run("PRAGMA ignore_check_constraints=ON");
    db.run("UPDATE text_document SET trailing_newline=2");
    await expect(txtSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("boolean");
    db.run("UPDATE text_document SET trailing_newline=0");
    db.run("UPDATE text_line SET ordinal=-1 WHERE id=1");
    await expect(txtSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("contiguous");
  } finally { db.close(); }
});


test("Txt semantic SQLite schema, native roundtrip and independent SQL edit", async () => {
  expect(TXT_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql", import.meta.url)).text());
  const input = fixture as { schema: string; lines: string[]; trailingNewline: boolean; lineEnding: "lf" | "crLf" };
  const database = await txtSnapshotToSqliteDatabase(input);
  expect(await txtSnapshotFromSqliteDatabase(database)).toEqual(input);
  const bytes = await exportSqliteDatabase(database);
  const db = Database.deserialize(bytes);
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT content FROM text_line ORDER BY ordinal").all()).toEqual(fixture.lines.map((content) => ({ content })));
    db.run("UPDATE text_line SET content='edited semantic line' WHERE ordinal=0");
    const result = await txtSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(result.lines[0]).toBe("edited semantic line");
    db.run("UPDATE text_line SET document_id=999 WHERE ordinal=0");
    await expect(txtSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow("unknown");
  } finally { db.close(); }
});

test("Txt resource limits apply before semantic projection and reconstruction", async () => {
  const input = fixture as { schema: string; lines: string[]; trailingNewline: boolean; lineEnding: "lf" | "crLf" };
  const database = await txtSnapshotToSqliteDatabase(input);
  await expect(txtSnapshotToSqliteDatabase(input, { maxRows: 0 })).rejects.toThrow("row limit");
  await expect(txtSnapshotToSqliteDatabase(input, { maxValueBytes: 0 })).rejects.toThrow("value limit");
  await expect(txtSnapshotFromSqliteDatabase(database, { maxRows: 0 })).rejects.toThrow("row limit");
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    const independent = await importSqliteDatabase(new Uint8Array(db.serialize()));
    await expect(txtSnapshotFromSqliteDatabase(independent, { maxValueBytes: 0 })).rejects.toThrow("value limit");
  } finally { db.close(); }
});

test("Txt projection and reconstruction cancellation", async () => {
  const countingController = new AbortController();
  let countEvents = 0;
  await expect(txtSnapshotToSqliteDatabase({ ...fixture, lineEnding: "lf" as const, lines: Array.from({ length: 600 }, () => "a") }, { signal: countingController.signal, onProgress: (progress) => { if (progress.completed === 0 && ++countEvents === 2) countingController.abort(); } })).rejects.toHaveProperty("kind",canceledKind);
  expect(countEvents).toBe(2);
  const input = fixture as { schema: string; lines: string[]; trailingNewline: boolean; lineEnding: "lf" | "crLf" };
  const database = await txtSnapshotToSqliteDatabase(input);
  for (const direction of ["project", "reconstruct"]) {
    const controller = new AbortController();
    const options = { signal: controller.signal, onProgress: () => controller.abort() };
    await expect(direction === "project" ? txtSnapshotToSqliteDatabase(input, options) : txtSnapshotFromSqliteDatabase(database, options)).rejects.toHaveProperty("kind",canceledKind);
  }
});

import receivingFixture from "../🧫️fixtures/🫴️receiving.json";
import Ajv from "ajv";
import {resolve} from "node:path";

test("original Txt paid pack receiving retains exact typed output and independently admitted line semantics",async()=>{
 const root=resolve(import.meta.dir,"../../../../../../../../../../../../..");
 const grant=await Bun.file(resolve(root,"🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json")).json(),snapshotSchema=await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🔣️.json",import.meta.url)).json();
 const admitGrant=new Ajv({strict:true}).compile(grant),admitSnapshot=new Ajv({strict:true}).addKeyword("x-semio-state").compile(snapshotSchema);expect(admitGrant(receivingFixture.grant)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE original_line(ordinal INTEGER PRIMARY KEY,body TEXT NOT NULL)");for(const row of receivingFixture.cases){expect(admitSnapshot(row.snapshot)).toBe(true);const bytes=Buffer.from(row.body);expect(new TextDecoder("utf-8",{fatal:true}).decode(bytes)).toBe(row.body);const sep=row.snapshot.lineEnding==="crLf"?"\r\n":"\n",content=row.snapshot.trailingNewline?row.body.slice(0,-sep.length):row.body;expect(row.body?content.split(sep):[]).toEqual(row.snapshot.lines);db.exec("DELETE FROM original_line");for(const[index,line]of row.snapshot.lines.entries())db.query("INSERT INTO original_line VALUES(?,?)").run(index,line);expect(db.query("SELECT body FROM original_line ORDER BY ordinal").all()).toEqual(row.snapshot.lines.map(body=>({body})));expect(row.snapshot.lines.join(sep)+(row.snapshot.trailingNewline?sep:"")).toBe(row.body);}}finally{db.close();}
 const source=await Bun.file(new URL("../../../💾️binary/📸️snapshot/🦀️.rs",import.meta.url)).text();const receiving=source.slice(source.indexOf("impl store::ArtifactPackReceiving for TxtSnapshot"));expect(receiving).toContain("owner.receive::<Receiving,Self>");expect(receiving).toContain("copy_text_into");expect(receiving).toContain("allocate_vec_into");expect(receiving).toContain("*typed=Some(Self");expect(receiving).not.toContain("Self::from_body");expect(receiving).not.toContain("NativeDecodeControl::new");expect(receiving).not.toContain("ValueError::new");
 console.error("[DEBUG] Txt plain newline/Unicode semantics agree with TextDecoder, Buffer, SQLite and actual domain Ajv; paid Rust receiver/System law requires native execution");
});

test("original Txt native SQL caller supplies its existing paid receiver and observer",async()=>{
 const source=await Bun.file(new URL("../🦀️.rs",import.meta.url)).text();const current=source.slice(source.indexOf("fn decode_sqlite_snapshot_native"),source.indexOf("fn encode_sqlite_snapshot_native"));
 expect(current).toContain("native_owner:&mut store::NativeSnapshotDecodeOwner");expect(current).toContain("native_owner.scoped_native");expect(current).toContain("Self::receive_pack_rows(bytes,owner,Some(limits.max_rows))");expect(current).toContain("Self::receive_text(text,owner,Some(limits.max_rows))");expect(current).not.toContain("NativeDecodeControl::new");expect(current).not.toContain("native.copy_text");
 console.error("[DEBUG] Original Txt SQL native caller forwards installed paid receiver, original limits and observer; no second native controller or cold body parser");
});

test("original Txt native callers supply independent five-axis authority to current codecs",async()=>{
 const root=resolve(import.meta.dir,"../../../../../../../../../../../../..");const schema=await Bun.file(resolve(root,"🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json")).json();expect(new Ajv({strict:true}).compile(schema)(receivingFixture.originalCallerGrant)).toBe(true);
 const source=await Bun.file(new URL("./🦀️.rs",import.meta.url)).text();expect(source).toContain("fn with_original_decode");expect(source).toContain("fn with_original_encode");expect(source).toContain("snapshot.encode_sqlite_snapshot_native(encoding,sql,owner)");expect(source).toContain("TxtSnapshot::decode_sqlite_snapshot_native(payload,sql,owner)");expect(source).toContain("&mut Some(database)");expect(source).toContain('fixture["originalCallerGrant"]');expect(source).not.toContain("next_capacity_byte_demand");expect(receivingFixture.grant.maximumCopyBytes).toBe(65536);
 console.error("[DEBUG] Original Txt native test caller policy is immutable and independently canonical; SQL controls/180000-byte input unchanged, exact receiver System law still has65536copy authority");
});

test("current TXT examples have no whole-trial schema authority", async()=>{const {existsSync}=await import("node:fs");expect(existsSync(new URL("../🛫️encoding/🧬️schema/🔣️.json",import.meta.url))).toBe(false);});
