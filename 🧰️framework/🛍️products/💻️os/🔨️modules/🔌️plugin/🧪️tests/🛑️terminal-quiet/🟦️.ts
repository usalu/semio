import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { isToolRunActionLegal, isToolRunTerminal, TOOL_RUN_ACTION_IDS, TOOL_RUN_DISMISS_ACTION_ID, TOOL_RUN_EVENT_KEYS, TOOL_RUN_START_ACTION_ID, TOOL_RUN_STATES, ToolRunMachine, type ToolRunEffect, type ToolRunEvent, type ToolRunEventKey, type ToolRunState } from "../../../../../../🔨️modules/⏯️tool-run/🟦️.ts";

/** 🛑️ Third-party twin of the TERMINAL QUIET LAW (`🧫️fixtures/⏯️tool-run/🔣️.json`, `terminalQuiet`) — the
 * Rust law drives a real ledger over 64 driver turns and counts the effects a terminal run hands the host;
 * this reads the same claim off the framework's OWN TypeScript reducer (`⏯️tool-run/🟦️.ts`'s
 * `ToolRunMachine`), a second implementation of the transition table, and off `isToolRunActionLegal`.
 *
 * 🪪️ Every wake a run hands the host is a REQUEST the host answers by dispatching an action back into the
 * guest, so a request a run that is OVER keeps re-making is an endless action storm on a shell nobody is
 * touching: generation3d's finalized read-only `previewEval` filled the console with `toolRunPace` on a
 * quiet, converged editor (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, 2026-09-14 23:25). A terminal run
 * admits no event that schedules work, and offers no action but the next `toolRunStart` and `toolRunDismiss`. */

const SCHEDULING_EFFECTS: readonly ToolRunEffect[] = ["schedule", "driveOneUnit", "holdResult", "reconfigure", "refold", "beginFinalize"];

function eventOf(key: ToolRunEventKey, run: bigint, generation: number): ToolRunEvent {
  if (key === "start") return { type: "start", run };
  if (key === "closed") return { type: "closed" };
  if (key === "abort") return { type: "abort", run, generation, publishing: false };
  if (key === "abortWhilePublishing") return { type: "abort", run, generation, publishing: true };
  if (key === "settingsChanged" || key === "baseChanged" || key === "dismiss") return { type: key, run };
  return { type: key, run, generation } as ToolRunEvent;
}

export function testToolRunTerminalQuiet(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../🧫️fixtures/⏯️tool-run/🔣️.json`, "utf8")) as {
    schema: string;
    terminalQuiet: { note: string; turns: number; rows: readonly { id: string; toolId: string; units: number; terminalBy: string; expected: { hasPendingWork: boolean; effects: number } }[] };
  };
  assert.equal(fixture.schema, "framework.plugin.tool-run.v1");
  const expected = fixture.terminalQuiet;
  assert.ok(expected.turns > 0, "the law drives real turns");
  for (const row of expected.rows) {
    assert.equal(row.expected.effects, 0, `${row.id}: a terminal run arms nothing`);
    assert.equal(row.expected.hasPendingWork, false, `${row.id}: a terminal run is no driver work`);
  }

  const terminal = TOOL_RUN_STATES.filter(isToolRunTerminal);
  assert.deepEqual([...terminal], ["finalized", "aborted", "faulted"], "the terminal states the law covers");
  const admitted: string[] = [];
  for (const state of terminal) {
    for (const key of TOOL_RUN_EVENT_KEYS) {
      const outcome = ToolRunMachine.apply({ run: 7n, generation: 3, state: state as ToolRunState }, eventOf(key, key === "start" ? 8n : 7n, 3));
      if (!outcome.ok) continue;
      admitted.push(`${state}/${key}->${outcome.transition.effect}`);
      assert.ok(!SCHEDULING_EFFECTS.includes(outcome.transition.effect), `${state}/${key}: a terminal run must never admit an event that schedules work, got ${outcome.transition.effect}`);
      if (key === "start") assert.equal(outcome.transition.slot?.state, "starting", "a start off a terminal run is a NEW run, never the old one waking up");
      else assert.equal(outcome.transition.slot, null, `${state}/${key}: the only other admitted events retire the run`);
    }
  }
  assert.deepEqual(
    admitted.map((entry) => entry.split("/")[1]?.split("->")[0]).filter((key, index, all) => all.indexOf(key) === index).sort(),
    ["closed", "dismiss", "start"],
    "a terminal run admits nothing but the next start, a dismiss and the instance closing",
  );

  for (const state of terminal) {
    for (const id of TOOL_RUN_ACTION_IDS) {
      const legal = isToolRunActionLegal(id, state as ToolRunState);
      assert.equal(legal, id === TOOL_RUN_START_ACTION_ID || id === TOOL_RUN_DISMISS_ACTION_ID, `${state}: ${id} is offered only if it starts the next run or clears this one`);
    }
  }

  console.log(`toolRun terminalQuiet turns=${expected.turns} rows=${expected.rows.length} terminalStates=${terminal.join(",")} admitted=${admitted.join(" ")}`);
}
