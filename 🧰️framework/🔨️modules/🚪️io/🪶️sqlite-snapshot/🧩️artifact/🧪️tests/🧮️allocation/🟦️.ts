import { expect, test } from "bun:test";

import { Database } from "bun:sqlite";
import fixture from "../../🧫️fixtures/🧮️allocation/🔣️.json";



function payloads(): { text: string; blob: Uint8Array } {
  return { text: fixture.textUnit.repeat(fixture.textRepeats), blob: Uint8Array.from({ length: fixture.blobBytes }, (_, index) => fixture.blobUnit[index % fixture.blobUnit.length]!) };
}


test("shared provider authored SQLite tables preserve IEEE bits and ordered relations independently", () => {
  const database = new Database(":memory:", { safeIntegers: true });
  const view = new DataView(new ArrayBuffer(8));
  try {
    database.exec("PRAGMA foreign_keys=ON");
    database.exec(fixture.schemaSql);
    for (const owner of fixture.owners) {
      const bits = BigInt("0x" + owner.binary64Bits);
      view.setBigUint64(0, bits);
      const scalar = view.getFloat64(0);
      expect(Number.isNaN(scalar) ? "nan" : "finite").toBe(owner.numericClass);
      database.run("INSERT INTO provider_owner VALUES(?,?,?,?,?,?)", [owner.id, owner.label, new Uint8Array(owner.blob), Number.isNaN(scalar) ? null : scalar, BigInt.asIntN(64, bits), owner.numericClass]);
    }
    for (const [index, ordinal] of fixture.ordinalInput.entries()) database.run("INSERT INTO provider_relation VALUES(?,?,?,?)", [index + 1, 1, ordinal, 7]);
    const ordered = database.query("SELECT id FROM provider_relation ORDER BY ordinal").all() as { id: bigint }[];
    expect(ordered.map(row => Number(row.id))).toEqual(fixture.ordinalExpectedIds);
    const rows = database.query("SELECT id,label,payload,scalar,scalar_ieee754_bits AS bits,scalar_numeric_class AS class,typeof(label) AS textStorage,typeof(payload) AS blobStorage,typeof(scalar_ieee754_bits) AS bitStorage FROM provider_owner ORDER BY id").all() as { id: bigint; label: string; payload: Uint8Array; scalar: number | null; bits: bigint; class: string; textStorage: string; blobStorage: string; bitStorage: string }[];
    for (const [index, row] of rows.entries()) {
      const expected = fixture.owners[index]!;
      expect(row.label).toBe(expected.label);
      expect([...row.payload]).toEqual(expected.blob);
      expect(row.textStorage).toBe("text");
      expect(row.blobStorage).toBe("blob");
      expect(row.bitStorage).toBe("integer");
      expect(BigInt.asUintN(64, row.bits).toString(16).padStart(16, "0")).toBe(expected.binary64Bits);
      view.setBigInt64(0, row.bits);
      const scalar = view.getFloat64(0);
      expect(Number.isNaN(scalar) ? "nan" : "finite").toBe(row.class);
      if (Number.isNaN(scalar)) expect(row.scalar).toBeNull();
      else { expect(Object.is(scalar, -0)).toBe(true); expect(row.scalar).toBe(0); }
    }
    expect(database.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(database.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    const bytes = database.serialize();
    const reopened = Database.deserialize(bytes, { safeIntegers: true });
    try {
      expect(reopened.query("SELECT r.id,o.label,r.score FROM provider_relation r JOIN provider_owner o ON o.id=r.owner_id ORDER BY r.ordinal").all()).toEqual(fixture.ordinalExpectedIds.map(id => ({ id: BigInt(id), label: "", score: 7n })));
      expect(reopened.query("SELECT scalar_ieee754_bits FROM provider_owner ORDER BY id").all()).toEqual(fixture.owners.map(owner => ({ scalar_ieee754_bits: BigInt.asIntN(64, BigInt("0x" + owner.binary64Bits)) })));
    } finally { reopened.close(); }
    database.run("DELETE FROM provider_relation");
    for (const [index, ordinal] of fixture.duplicateOrdinals.entries()) database.run("INSERT INTO provider_relation VALUES(?,?,?,?)", [index + 1, 1, ordinal, 7]);
    expect(database.query("SELECT ordinal,count(*) AS count FROM provider_relation GROUP BY ordinal HAVING count(*)>1").all()).toEqual([{ ordinal: 0n, count: 2n }]);
  } finally { database.close(); }
  console.log("[DEBUG] Independent Bun SQLite and DataView: two tables, two IEEE owners, three ordered relations, duplicate ordinal, reopen");
});

test("shared provider semantic bytes and late copy frontiers have independent SQLite authority", () => {
  const { text, blob } = payloads();
  expect(Buffer.byteLength(text, "utf8")).toBe(fixture.textBytes);
  expect(blob.byteLength).toBe(fixture.blobBytes);
  const utf8 = Buffer.from(text, "utf8");
  expect(Buffer.byteLength(utf8.subarray(0, fixture.copyFrontier).toString("utf8"))).toBe(fixture.copyFrontier);
  expect(fixture.copyFrontier).toBeLessThan(text.length * 2);
  expect(fixture.copyFrontier).toBeLessThan(blob.byteLength);
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE copy_authority(id INTEGER PRIMARY KEY,text_value TEXT NOT NULL,blob_value BLOB NOT NULL); CREATE TABLE retired_admission(id INTEGER PRIMARY KEY,requested INTEGER NOT NULL)");
    database.run("INSERT INTO copy_authority VALUES(?,?,?)", [1, text, blob]);
    expect(database.query("SELECT length(CAST(text_value AS BLOB)) AS textBytes,length(blob_value) AS blobBytes,typeof(text_value) AS textStorage,typeof(blob_value) AS blobStorage FROM copy_authority").get()).toEqual({ textBytes: fixture.textBytes, blobBytes: fixture.blobBytes, textStorage: "text", blobStorage: "blob" });
    const bytes = Buffer.byteLength(fixture.repeatedText);
    for (let index = 0; index < 2; index++) database.run("INSERT INTO retired_admission VALUES(?,?)", [index, bytes]);
    expect(database.query("SELECT sum(requested) AS bytes FROM retired_admission").get()).toEqual({ bytes: fixture.repeatedAllocationBytes });
    expect(database.query("SELECT CASE WHEN sum(requested)+? > ? THEN 'ownershipLimit' ELSE 'admitted' END AS kind FROM retired_admission").get(bytes, fixture.repeatedAllocationBytes)).toEqual({ kind: fixture.expectedOwnershipKind });
  } finally { database.close(); }
  console.log("[DEBUG] Independent UTF-8/octet authority: 100000 bytes each, 65536 interior frontier, repeated retired admission 8 bytes");
});

function cancellationDiagnosticLedger(corpus:typeof fixture.cancellationDiagnostics){const backing=(kind:string,text:string)=>kind==="borrowed"?0:new TextEncoder().encode(text).length;return corpus.cases.map(row=>({id:row.id,parent:row.parent,child:row.child,requestedBytes:row.parent*backing(corpus.parentBacking,corpus.parentMessage)+row.child*backing(corpus.childBacking,corpus.childMessage)}));}
test("shared cancellation diagnostic ledger has one exact borrowed owner",()=>{const corpus=fixture.cancellationDiagnostics,own=cancellationDiagnosticLedger(corpus),database=new Database(":memory:");try{const oracle=corpus.cases.map(row=>({id:row.id,parent:row.parent,child:row.child,...database.query("SELECT ?*(CASE WHEN ?='borrowed' THEN 0 ELSE length(CAST(? AS BLOB)) END)+?*(CASE WHEN ?='borrowed' THEN 0 ELSE length(CAST(? AS BLOB)) END) AS requestedBytes").get(row.parent,corpus.parentBacking,corpus.parentMessage,row.child,corpus.childBacking,corpus.childMessage) as {requestedBytes:number}}));expect(own).toEqual(corpus.cases);expect(oracle).toEqual(corpus.cases);expect(database.query("SELECT length(CAST(? AS BLOB)) AS parent,length(CAST(? AS BLOB)) AS child").get(corpus.parentMessage,corpus.childMessage)).toEqual({parent:new TextEncoder().encode(corpus.parentMessage).length,child:new TextEncoder().encode(corpus.childMessage).length});console.log("[DEBUG] Five cancellation diagnostic ledgers have borrowed backing0 and unchanged UTF8 messages; independent SQLite/UTF8 agree; native heap observation remains required");}finally{database.close();}});

import fileBoundFixture from "../../🧫️fixtures/📏️file-bound/🔣️.json";

test("native file forecasts retain a separate closed semantic cell budget",()=>{
 
 const db=new Database(":memory:");try{db.exec("CREATE TABLE physical_extent(bytes INTEGER NOT NULL)");for(const bytes of fileBoundFixture.chunks)db.run("INSERT INTO physical_extent VALUES(?)",[bytes]);expect((db.query("SELECT sum(bytes) AS bytes FROM physical_extent").get() as {bytes:number}).bytes).toBe(fileBoundFixture.expectedBytes);expect(fileBoundFixture.expectedBytes).toBe(fileBoundFixture.maxFileBytes);expect(fileBoundFixture.maxSemanticBytes).toBe(0);}finally{db.close();}
});

import extentFixture from "../../🧫️fixtures/🔮️semantic-extent/🔣️.json";

import {independentSqliteExtent} from "../../../🧪️tests/🔮️semantic-extent/🟦️.ts";
test("independent SQLite semantic census preserves the authored zero-byte NULL payload",()=>{
 
 const db=new Database(":memory:");try{db.exec(extentFixture.sql);db.run("INSERT INTO cell_payload VALUES(?,?,?,?,?,?,?)",[1,null,"","雪",7,0.5,new Uint8Array([0,255])]);const value=independentSqliteExtent(db.serialize());expect(value.rows).toBe(extentFixture.rows);expect(value.valueBytes).toBe(extentFixture.valueBytes);expect(value.tableWidths).toEqual({cell_payload:extentFixture.width});expect(db.query("SELECT typeof(missing) AS storage,length(CAST(label AS BLOB)) AS labelBytes,length(octets) AS octetBytes FROM cell_payload").get()).toEqual({storage:"null",labelBytes:3,octetBytes:2});}finally{db.close();}
});

test("current cancellation examples have no whole-trial schema authority", async()=>{const {existsSync}=await import("node:fs");expect(existsSync(new URL("../../🧬️schema/⚠️cancellation/🔣️.json",import.meta.url))).toBe(false);});
