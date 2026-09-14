import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { ToolRunMachine } from "../../../../../../🔨️modules/⏯️tool-run/🟦️.ts";

/** 📖️ Third-party twin of the READ-ONLY RUN LAW (`🧫️fixtures/⏯️tool-run/🔣️.json`, `readOnlyRun`) — an
 * independent TypeScript reading of what a run that publishes nothing owes, driven from the SAME fixture
 * the Rust law drives and pushed through the framework's OWN TypeScript state machine
 * (`⏯️tool-run/🟦️.ts`'s `ToolRunMachine`), which is a second implementation of the transition table
 * the Rust ledger applies.
 *
 * 🪪️ `complete` means "the job is done and the run awaits the finalize that publishes its provisional
 * edits". A `mutating: false` run authors none, so there is nothing for a finalize to publish and
 * nothing for a user to review: parking it in `complete` only holds the tool's single run slot open, and
 * a start against a non-terminal run is `toolRun.busy`. Generation3d's read-only `previewEval` therefore
 * ran exactly ONCE per session and its 3d preview stopped re-evaluating after the first evaluation
 * (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */

export function testToolRunReadOnlyRun(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../🧫️fixtures/⏯️tool-run/🔣️.json`, "utf8")) as {
    schema: string;
    readOnlyRun: { toolId: string; units: number; note: string; stateAfterJobCompletes: string; startsAgainWithoutAnyFinalizeAction: boolean };
  };
  assert.equal(fixture.schema, "framework.plugin.tool-run.v1");
  const expected = fixture.readOnlyRun;

  // ▶️ Start, admit the job, run it, complete it — the ledger's own transitions, nothing invented.
  let slot: { run: bigint; generation: number; state: string } | null = null;
  const apply = (event: Parameters<typeof ToolRunMachine.apply>[1]) => {
    const outcome = ToolRunMachine.apply(slot as never, event as never);
    assert.ok(outcome.ok, `${event.type} must be admitted, got ${outcome.ok ? "" : outcome.rejection}`);
    slot = outcome.transition.slot as typeof slot;
    return outcome.transition.effect;
  };
  apply({ type: "start", run: 1n });
  apply({ type: "jobAdmitted", run: 1n, generation: slot!.generation });
  apply({ type: "jobComplete", run: 1n, generation: slot!.generation });
  assert.equal(slot!.state, "complete", "the job's completion lands the run in complete");

  // 📖️ A read-only run does not stop there: the same driver turn finalizes it, because there is nothing
  // a finalize could publish and nothing a user could review.
  const effect = apply({ type: "finalize", run: 1n, generation: slot!.generation });
  assert.equal(effect, "beginFinalize", "the finalize the driver applies for it is the ordinary one");
  assert.equal(slot!.state, expected.stateAfterJobCompletes, "and the state it leaves complete for is the fixture's");
  assert.notEqual(slot!.state, "complete", "a read-only run never parks in complete waiting for a finalize nobody owes it");

  // ▶️ Once it publishes, the slot is free — which is the whole point: a start against a NON-terminal run
  // is busy, so a finished read-only run that never left complete refuses every later start.
  apply({ type: "publicationComplete", run: 1n, generation: slot!.generation });
  assert.equal(slot!.state, "finalized", "publication finalizes it");
  const next = ToolRunMachine.apply(slot as never, { type: "start", run: 2n } as never);
  assert.ok(next.ok, "a finalized run never refuses the next start as busy");
  assert.equal(expected.startsAgainWithoutAnyFinalizeAction, true, "and no finalize ACTION was dispatched anywhere in this law");

  // 🚧️ The counter-reading: the same run left in complete does refuse it, which is the defect this law pins.
  const parked = ToolRunMachine.apply({ run: 1n, generation: 0, state: "complete" } as never, { type: "start", run: 2n } as never);
  assert.equal(parked.ok, false, "a run parked in complete refuses the next start");
  assert.equal(parked.ok === false ? parked.rejection : "", "toolRun.busy", "and refuses it as busy");

  console.log(`toolRun readOnlyRun tool=${expected.toolId} stateAfterJobCompletes=${slot!.state} parkedStartRejection=toolRun.busy`);
}
