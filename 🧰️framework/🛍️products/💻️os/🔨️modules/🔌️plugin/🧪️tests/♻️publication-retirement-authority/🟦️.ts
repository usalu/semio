import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

/** ♻️ Third-party twin of the publication-retirement fixture: every lane declares the same retirement law, and only window-transient forgives a rejected authority. */
export function testPublicationRetirementAuthorityOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/♻️publication-retirement-authority/🔣️.json", import.meta.url), "utf8"));
  const laneSchema = {
    type: "array",
    minItems: 7,
    maxItems: 7,
    items: {
      type: "object",
      additionalProperties: false,
      required: ["id", "lane", "storeLabel", "supersededFault", "rejectionIsFatal", "terminalOutcome", "rejectedFault", "falseTerminalFault"],
      properties: {
        id: { type: "string", minLength: 1 },
        lane: { type: "string", pattern: "^[A-Z][A-Za-z]+$" },
        storeLabel: { type: "string", minLength: 1 },
        supersededFault: { type: "string", minLength: 1 },
        rejectionIsFatal: { type: "boolean" },
        terminalOutcome: { enum: ["rejected", "retired"] },
        rejectedFault: { type: ["string", "null"] },
        falseTerminalFault: { type: "string", minLength: 1 },
      },
    },
  };
  const validate = new Ajv({ strict: true }).compile(laneSchema);
  assert(validate(fixture.lanes), JSON.stringify(validate.errors));
  assert.equal(fixture.schema, "framework.plugin.publication-retirement-authority.v1");
  assert.equal(fixture.law.incompleteRetirementTurnIsOk, true);
  assert.equal(fixture.law.rejectionSurfacesOnceAtTheTerminalTurn, true);
  assert.equal(fixture.law.terminalTurnRequiresTerminalEmptiness, true);
  assert.ok(fixture.law.minimumIncompleteTurnsWhileRejected >= 1);

  type Lane = { id: string; lane: string; storeLabel: string; rejectionIsFatal: boolean; terminalOutcome: string; rejectedFault: string | null; falseTerminalFault: string };
  const lanes = fixture.lanes as Lane[];
  assert.equal(new Set(lanes.map((row) => row.id)).size, lanes.length, "every lane id is distinct");
  assert.equal(new Set(lanes.map((row) => row.storeLabel)).size, lanes.length, "every store label is distinct");
  assert.deepEqual(
    lanes.map((row) => row.lane),
    ["Artifact", "Config", "Draft", "Presence", "Transient", "WindowConfig", "WindowTransient"],
  );

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
  console.log(`[DEBUG] publication retirement authority: ${lanes.length} lanes, ${lanes.length - lenient.length} fatal on rejection, refreshed re-begin accepted=${reBegin.refreshedAuthorityBeginAccepted}`);
}

if (import.meta.main) testPublicationRetirementAuthorityOracle();
