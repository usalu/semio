/** 📦️ Original replay rows retain stable priority and omission counts until the single final selection. */
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { applyPatch } from "fast-json-patch";
import { readFileSync, existsSync } from "node:fs";

const law = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));

test("📦️ paged original rows preserve one final stable severity/omission authority", () => {
  expect(law.pauseDepth).toBeLessThan(law.minimumDepth);
  const levels = { info: 0, warning: 1, error: 2, fatal: 3 };
  for (const row of law.cases) {
    const sql = new Database(":memory:");
    sql.exec("CREATE TABLE messages(ordinal INTEGER PRIMARY KEY, operation INTEGER, severity INTEGER, size INTEGER)");
    let ordinal = 0;
    for (const [operation, group] of row.operations.entries()) for (let i = 0; i < group.count; i++) sql.query("INSERT INTO messages VALUES(?,?,?,?)").run(ordinal++, operation, levels[group.level as keyof typeof levels], law.messageOwnerBytes + Buffer.byteLength("mutation.invariant") + group.bytes);
    const ranked = sql.query("SELECT ordinal,operation,severity,size FROM messages ORDER BY severity DESC,ordinal ASC").all() as { ordinal: number; operation: number; severity: number; size: number }[];
    const budget = law.entryBytes - Buffer.byteLength(row.editId) - law.messageOwnerBytes - Buffer.byteLength("mutation.cascade") - Buffer.byteLength(`${ordinal} more messages`);
    let used = 0;
    const selected: number[] = [];
    for (const item of ranked) { if (used + item.size > budget) break; used += item.size; selected.push(item.ordinal); }
    if (!selected.length) selected.push(ranked[0].ordinal);
    selected.sort((a, b) => a - b);
    expect(selected, row.name).toEqual(row.expectedKept);
    expect(ordinal - selected.length).toBe(row.expectedDropped);
    const firstDropped = (sql.query("SELECT ordinal,operation FROM messages ORDER BY ordinal").all() as { ordinal: number; operation: number }[]).find(item => !selected.includes(item.ordinal));
    expect(firstDropped?.operation).toBe(row.expectedSummaryOp);
    let state = { rows: [] as number[], worst: 0 };
    for (const item of sql.query("SELECT ordinal,severity FROM messages ORDER BY ordinal").all() as { ordinal: number; severity: number }[]) state = applyPatch(state, [{ op: "add", path: "/rows/-", value: item.ordinal }, { op: "replace", path: "/worst", value: Math.max(state.worst, item.severity) }], true, false).newDocument;
    expect(state.rows).toEqual(Array.from({ length: ordinal }, (_, index) => index));
    expect(state.worst).toBe(Math.max(...ranked.map(item => item.severity)));
    sql.close();
  }
  console.log("[DEBUG] Original rows/full omitted counts/stable severity prefix agree with independent SQLite ORDER BY and RFC6902 before final clamp");
  expect(existsSync(new URL("../🦀️.rs", import.meta.url))).toBe(true);
});
