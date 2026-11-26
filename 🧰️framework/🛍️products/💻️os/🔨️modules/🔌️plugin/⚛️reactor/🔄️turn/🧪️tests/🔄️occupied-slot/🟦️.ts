import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import cases from "./🔣️.json";

test("response and invocation polling inspect occupancy before taking an owner", () => {
  const db = new Database(":memory:");
  try {
    const query = db.query("SELECT key AS slot FROM json_each(?1) WHERE value = 1 ORDER BY key < ?2, key LIMIT 1");
    for (const law of cases) {
      const result = query.get(JSON.stringify(law.slots), law.start) as {slot:number} | null;
      expect(result?.slot ?? null).toBe(law.selected);
    }
    for (const path of ["📥️receiving", "🔌️extension-invocation-and-command-ingress"]) {
      const source = readFileSync(new URL("../../" + path + "/🦀️.rs", import.meta.url), "utf8");
      expect(source).toContain("take_next_occupied_slot(");
      expect(source).not.toMatch(/find_map\(\|index\|\w+\[index\]\.take\(\)/);
    }
  } finally { db.close(); }
});
