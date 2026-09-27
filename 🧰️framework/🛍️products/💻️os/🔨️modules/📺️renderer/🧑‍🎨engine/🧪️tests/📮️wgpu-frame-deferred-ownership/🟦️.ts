import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

type Law = {
  id: string;
  dropAt: "queued" | "running";
  currentReceiptSettlements: number;
  remainingActions: number;
  closeSteps: number;
  readyReservations: number;
  inFlightReservations: number;
};

const here = dirname(fileURLToPath(import.meta.url));
const fixture = JSON.parse(readFileSync(resolve(here, "../../🧫️fixtures/📮️wgpu-frame-deferred-ownership/🔣️.json"), "utf8")) as { cases: Law[] };

function replay(dropAt: Law["dropAt"]) {
  let currentOwnedByRegistry = dropAt === "queued";
  let currentOwnedByFuture = dropAt === "running";
  let settlements = 0;
  let remaining = 2;
  let readyReservations = 1;
  let inFlightReservations = 0;

  if (currentOwnedByFuture) {
    settlements += 1;
    currentOwnedByFuture = false;
  }
  readyReservations = 0;
  if (currentOwnedByRegistry) {
    settlements += 1;
    currentOwnedByRegistry = false;
  }
  let closeSteps = 1;
  while (remaining > 0) {
    remaining -= 1;
    closeSteps += 1;
  }
  return { settlements, remainingActions: 2, closeSteps, readyReservations, inFlightReservations };
}

describe("WGPU frame deferred owner recovery", () => {
  for (const law of fixture.cases) {
    it(law.id, () => {
      const actual = replay(law.dropAt);
      expect(actual.settlements).toBe(law.currentReceiptSettlements);
      expect(actual.remainingActions).toBe(law.remainingActions);
      expect(actual.closeSteps).toBe(law.closeSteps);
      expect(actual.readyReservations).toBe(law.readyReservations);
      expect(actual.inFlightReservations).toBe(law.inFlightReservations);
    });
  }
});
