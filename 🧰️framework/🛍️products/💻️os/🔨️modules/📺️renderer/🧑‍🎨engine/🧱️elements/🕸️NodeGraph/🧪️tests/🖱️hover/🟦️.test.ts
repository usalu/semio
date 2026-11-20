import { expect, test } from "vitest";
import { createNodeGraphInteractionLedger } from "../../🫱️interaction-publication/🟦️.ts";
import hoverCases from "../🧫️fixtures/🔣️hover.json";

for (const { id, steps } of hoverCases) {
  test(id, () => {
    const ledger = createNodeGraphInteractionLedger();
    for (const step of steps) {
      const revision = ledger.sceneHoverRevision;
      if (step.local) ledger.beginLocalHover();
      if ("scene" in step) {
        expect(ledger.reconcileSceneHover(step.scene ?? undefined)).toEqual(step.write ?? undefined);
      }
      expect(ledger.sceneHoverRevision).toBe(revision + (step.write ? 1 : 0));
      if (step.read) {
        const readRevision = ledger.sceneHoverRevision - (step.stale ? 1 : 0);
        expect(ledger.publishHover(step.read, readRevision)).toBe(step.publish);
      }
    }
  });
}
