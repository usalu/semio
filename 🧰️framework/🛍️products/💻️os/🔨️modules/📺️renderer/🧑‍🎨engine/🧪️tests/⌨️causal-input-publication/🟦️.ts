import { describe, expect, it } from "vitest";
import law from "../../../../../🧫️fixtures/⌨️input-publication-receipt.json" with { type: "json" };
import actorFixture from "../../../../🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧫️fixtures/🔣️.json" with { type: "json" };
import { BrowserActorActionMailboxV1 } from "../../../../🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📮️requests/🟦️.ts";
import { inputAppliedV1, inputCommitReceiptMatchesPublicationV1, inputCommitReceiptV1 } from "../../🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts";

describe("causal input publication receipt", () => {
  it("does not release C for a same-value foreign publication and releases it for the exact local revision", () => {
    const receipt = inputCommitReceiptV1(BigInt(law.localCompletion.operation), BigInt(law.localCompletion.revision));
    const outcome = inputAppliedV1(1, receipt);
    let dispatches = 1;
    if (inputCommitReceiptMatchesPublicationV1(outcome, law.foreignPublication.revision)) dispatches += 1;
    expect(dispatches).toBe(law.expected.dispatchesBeforeLocalPublication);
    if (inputCommitReceiptMatchesPublicationV1(outcome, law.localPublication.revision)) dispatches += 1;
    expect(dispatches).toBe(law.expected.dispatchesAfterLocalPublication);
    expect(outcome).toEqual({ kind: "applied", inputSeq: 1, commit: law.localCompletion });
  });

  it("rejects absent and noncanonical revision carriers", () => {
    const outcome = inputAppliedV1(1, inputCommitReceiptV1(7n, 9n));
    for (const revision of [undefined, null, "09", "9.0", "-9", ""] as const) expect(inputCommitReceiptMatchesPublicationV1(outcome, revision)).toBe(false);
  });

  it("keeps a browser actor input pending across a foreign publication and returns only its exact native commit receipt", async () => {
    const fixture = actorFixture as any;
    const mailbox = new BrowserActorActionMailboxV1(() => {});
    try {
      let settled = false;
      const pending = mailbox.dispatchIntent(fixture.request, fixture.uiIntent.surface, { ...fixture.uiIntent, seq: BigInt(fixture.uiIntent.seq) }).then((result) => { settled = true; return result; });
      await Promise.resolve();
      expect(law.foreignPublication.revision).not.toBe(fixture.acknowledged.commit.revision);
      expect(mailbox.settle({ ...fixture.hostileResults[0], actionSequence: 1 })).toBe(false);
      expect(settled).toBe(false);
      expect(mailbox.settle({ ...fixture.acknowledged, actionSequence: 1 })).toBe(true);
      const result = await pending;
      expect(result.commit).toEqual(fixture.acknowledged.commit);
      expect(result.commit?.revision).toBe(law.localCompletion.revision);

      const refused = mailbox.dispatchIntent(fixture.request, fixture.uiIntent.surface, { ...fixture.uiIntent, seq: 1n });
      expect(mailbox.settle({ ...fixture.rejected, actionSequence: 2 })).toBe(true);
      await expect(refused).rejects.toThrow("action-refused");
    } finally {
      mailbox.close("causal actor test retired");
    }
  });
});
