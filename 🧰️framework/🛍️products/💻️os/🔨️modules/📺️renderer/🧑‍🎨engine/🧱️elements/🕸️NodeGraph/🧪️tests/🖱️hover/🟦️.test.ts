import { expect, test } from "vitest";
import { createNodeGraphInteractionLedger } from "../../🫱️interaction-publication/🟦️.ts";
import hoverCases from "../🧫️fixtures/🔣️hover.json";

for (const { id, steps, window } of hoverCases) {
  test(id, () => {
    const ledger = createNodeGraphInteractionLedger(window);
    for (const step of steps) {
      const revision = ledger.sceneHoverRevision;
      if (step.local) ledger.beginLocalHover();
      if ("scene" in step) {
        expect(ledger.reconcileSceneHover(step.scene ?? undefined)).toEqual(step.write ?? undefined);
      }
      expect(ledger.sceneHoverRevision).toBe(revision + (step.write ? 1 : 0));
      if (step.read) {
        const readRevision = ledger.sceneHoverRevision - (step.stale ? 1 : 0);
        const direction = "portDirection" in step.read ? step.read.portDirection : undefined;
        const portDirection = direction === "in" || direction === "out" ? direction : undefined;
        expect(ledger.publishHover({ ...step.read, portDirection }, readRevision)).toBe(step.publish);
      }
    }
  });
}
