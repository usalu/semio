import { test, expect } from "bun:test";
import { readFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";
import { Database } from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import actor from "../../../📤️receipt/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };

/** 💼️ All six owned job headers retain the actual actor receipt and bounded body metadata. */
test("original owned job headers preserve the six current reply shapes without dynamic storage", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, extent: 161 })).toBe(false);
  const db = new Database(":memory:");
  try {
    db.run("create table original_job (kind text primary key, prefix blob unique, bytes blob)");
    for (const row of fixture.cases) {
      const bytes = Buffer.alloc(fixture.extent), base = Buffer.from(actor.failureHex, "hex");
      bytes.set(Buffer.from(row.prefixHex, "hex"));
      if (row.refusal === "ownershipLimit") base[6] = 3;
      bytes.set(base, 8);
      row.body.forEach((value, index) => bytes.writeBigUInt64LE(BigInt(value), 144 + index * 8));
      db.query("insert into original_job values (?,?,?)").run(row.kind, bytes.subarray(0, 8), bytes);
      expect(db.query("select length(bytes) as extent, hex(prefix) as prefix from original_job where kind=?").get(row.kind)).toEqual({ extent: 184, prefix: row.prefixHex.toUpperCase() });
      expect(bytes.subarray(8, 12).toString()).toBe("SAC2");
      expect(row.body.map((_, index) => bytes.readBigUInt64LE(144 + index * 8).toString())).toEqual(row.body.map(String));
    }
    expect(db.query("select count(*) as count from original_job").get()).toEqual({ count: 6 });
    for(const row of fixture.loanHandbacks)expect(Boolean((db.query("select (?1='original' and ?2=184) as accepted").get(row.pointer,row.extent)as{accepted:number}).accepted)).toBe(row.accepted);
    const source = resolve(import.meta.dir, "../🦀️.rs");
    expect(existsSync(source)).toBe(true);
    const text = readFileSync(source, "utf8");
    expect(text).toContain("pub struct OriginalOwnedJobFixedReceipt");
    expect(text).toContain("[u8; 184]");
    expect(text).toContain("OriginalActorFixedReceipt::read");
    expect(text.includes("Vec<")).toBe(false);
    expect(text).toContain("OriginalOwnedJobReceiptLoan");expect(text).toContain("if self.borrowed");
  } finally { db.close(); }
  console.log("[DEBUG] Original owned job headers sixShapes=true fixed184=true originalActorReceipt=true independentBufferSQLiteAjv=true");
});
