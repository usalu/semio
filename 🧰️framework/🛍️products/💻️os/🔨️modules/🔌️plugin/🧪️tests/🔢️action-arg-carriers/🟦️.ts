import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

/** 🔢️ Third-party twin of the ACTION ARGUMENT CARRIER LAW (`🧫️fixtures/⏯️tool-run/🔣️.json`,
 * `actionArgCarriers`) — an independent TypeScript model of how a §2.5 tool-run action reads the
 * identity arguments (`runId`, `generation`) out of whatever wire value carried them, driven from the
 * SAME fixture the Rust laws drive.
 *
 * Rust reads each row through the real reader and then dispatches a real `toolRunFinalize` in the
 * declared carrier; this file rebuilds the reader from the fixture's own prose, so the two
 * implementations can only agree by agreeing on the law rather than on one another's code.
 *
 * 🪪️ A JSON number that is an exact non-negative integer IS the identity it names. `generation` crosses
 * as a float — the shell's own action arguments build it with a number value, and a guest that mints it
 * as an unsigned integer arrives the same way — so a reader that accepted only an exact integer carrier
 * or a numeric string answered "no identity", and no identity is `toolRun.stale`. EVERY finalize of
 * EVERY app was refused: runs reached `complete` and stayed there, and because a start against a live
 * run is `toolRun.busy`, no tool could run a second time in a session. Measured on 6018 as a 3d preview
 * that never re-evaluated after an inspector edit
 * (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */

type Carrier = "uint" | "float" | "string";

interface CarrierRow {
  id: string;
  carrier: Carrier;
  value: number | string;
  reads: number | null;
}

/** 🔢️ The independent reader: every carrier of an exact non-negative integer answers that integer, and
 * nothing else answers at all. */
function readIdentityArg(value: unknown): number | null {
  if (typeof value === "number") {
    return Number.isFinite(value) && value >= 0 && Number.isInteger(value) ? value : null;
  }
  if (typeof value === "string") {
    if (!/^\d+$/.test(value)) return null;
    const parsed = Number(value);
    return Number.isSafeInteger(parsed) ? parsed : null;
  }
  return null;
}

export function testToolRunActionArgCarriers(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../🧫️fixtures/⏯️tool-run/🔣️.json`, "utf8")) as {
    schema: string;
    actionArgCarriers: { note: string; rows: CarrierRow[]; finalizeCarrier: Carrier };
  };
  assert.equal(fixture.schema, "framework.plugin.tool-run.v1");
  const { rows, finalizeCarrier } = fixture.actionArgCarriers;
  assert.ok(rows.length > 0, "the carrier law must declare rows");

  for (const row of rows) {
    const read = readIdentityArg(row.value);
    assert.equal(read, row.reads, `${row.id}: ${row.carrier} carrier`);
    console.log(`toolRun actionArgCarriers ${row.id} carrier=${row.carrier} read=${read ?? "-"}`);
  }

  // 🪪️ The carrier the finalize really travels in must be one the law admits — otherwise the end-to-end
  // reading proves nothing about the shape the shell actually sends.
  const finalizeRow = rows.find((row) => row.carrier === finalizeCarrier && row.reads !== null);
  assert.ok(finalizeRow, `the declared finalize carrier ${finalizeCarrier} must be a carrier that names an identity`);

  // 🔢️ Every carrier of the same exact integer is the same identity — the property the whole law exists
  // for, stated without reference to any row's own expectation.
  assert.equal(readIdentityArg(3), readIdentityArg("3"), "an integer and its decimal text name the same identity");
  assert.equal(readIdentityArg(3.0), readIdentityArg(3), "a float that is an exact integer names that integer");
  assert.equal(readIdentityArg(2.5), null, "a fractional number names no identity");
  assert.equal(readIdentityArg(-1), null, "a negative number names no identity");
  assert.equal(readIdentityArg(undefined), null, "an absent argument names no identity");
}
