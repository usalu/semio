import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

/** ♻️ Third-party twin of the publication-retirement fixture: every lane declares the same retirement law, and only window-transient forgives a rejected authority. */
export function testPublicationRetirementAuthorityOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/♻️publication-retirement-authority/🔣️.json", import.meta.url), "utf8"));
  assert.equal(fixture.schema, "framework.plugin.publication-retirement-authority.v1");
  assert.equal(fixture.law.incompleteRetirementTurnIsOk, true);
  assert.equal(fixture.law.rejectionSurfacesOnceAtTheTerminalTurn, true);
  assert.equal(fixture.law.terminalTurnRequiresTerminalEmptiness, true);
  assert.ok(fixture.law.minimumIncompleteTurnsWhileRejected >= 1);

  type Lane = { id: string; lane: string; storeLabel: string; rejectionIsFatal: boolean; terminalOutcome: string; rejectedFault: string | null; falseTerminalFault: string };
  const lanes = fixture.lanes as Lane[];
  assert.equal(new Set(lanes.map((row) => row.id)).size, lanes.length, "every lane id is distinct");
  assert.equal(new Set(lanes.map((row) => row.storeLabel)).size, lanes.length, "every store label is distinct");

  // ♻️ The two terminal messages are derived from the lane's own store label, so a lane cannot
  // drift into another lane's wording.
  for (const row of lanes) {
    assert.equal(row.falseTerminalFault, `${row.storeLabel} publication closed without terminal emptiness`, `${row.id} false-terminal wording`);
    assert.equal(row.rejectionIsFatal, row.terminalOutcome === "rejected", `${row.id} fatality matches its terminal outcome`);
    assert.equal(row.rejectedFault, row.rejectionIsFatal ? `${row.storeLabel} publication rejected stale or cancelled authority` : null, `${row.id} rejection wording`);
    assert.ok(!row.falseTerminalFault.includes("is retiring a rejected authority"), `${row.id} never re-raises the per-turn fault`);
  }
  const lenient = lanes.filter((row) => !row.rejectionIsFatal);
  assert.deepEqual(lenient.map((row) => row.lane), ["WindowTransient"], "window-transient is the one lenient lane");

  const reBegin = fixture.windowTransientReBegin;
  assert.equal(reBegin.staleAuthorityBeginAccepted, false, "the authority captured at admission is stale once the superseding write lands");
  assert.equal(reBegin.refreshedAuthorityBeginAccepted, true, "refreshing the authority re-admits the same window");
  assert.equal(reBegin.refreshedGenerationExceedsCaptured, true);
  assert.ok(reBegin.capturedRevision < reBegin.supersedingRevision && reBegin.supersedingRevision < reBegin.rejectedRevision && reBegin.rejectedRevision < reBegin.reBeginRevision, "the re-begin fixture revisions are ordered");
}

if (import.meta.main) testPublicationRetirementAuthorityOracle();
