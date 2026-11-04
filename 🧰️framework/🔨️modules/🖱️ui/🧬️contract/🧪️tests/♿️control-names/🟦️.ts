/** ♿️ The TypeScript twin of the unnamed-control law, answering the shared `🧫️fixtures/♿️control-names/🔣️.json` rows the Rust `control_is_unnamed` answers. */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { controlIsUnnamed, type NamedControlRecord } from "../../🛡️limits/🟦️.ts";

type Row = { case: string; component: NamedControlRecord["component"]; label?: string; parent: { role: string; label?: string } | null; unnamed: boolean };
const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/♿️control-names/🔣️.json", import.meta.url), "utf8")) as { cases: Row[] };

describe("unnamed control law", () => {
  it("answers every shared fixture row like the Rust gate", () => {
    for (const row of fixture.cases) {
      const node: NamedControlRecord = { component: row.component, accessibility: row.label === undefined ? {} : { label: row.label } };
      const parent: NamedControlRecord | null = row.parent === null ? null : { component: { type: "container", role: row.parent.role, ...(row.parent.label === undefined ? {} : { label: row.parent.label }) }, accessibility: {} };
      expect(controlIsUnnamed(node, parent), row.case).toBe(row.unnamed);
    }
  });
});
