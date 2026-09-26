/** 📤️ LAW (every announced operation also names the newest foreign operation its author had applied): the language-agnostic outbound-announcement fixture the kernel store law walks
 * (`🔨️modules/🏪️store/🧪️tests/📤️outbound-announcement/🦀️.rs`) is admitted by its schema through Ajv (third-party), and an
 * independent reference of the coalescing rule — a keyed gesture amends the tail applied edit of the same key, every other gesture
 * starts a new edit, an undo announces one transition and leaves its edit in the ledger — derives every step's expectation: each
 * gesture announces exactly the operations it appended (ticket 26/09/23 C10 09:4x: a coalesced keystroke re-announced the
 * previous, already accepted one and the hub refused the writer's typing). */
export async function registerOutboundAnnouncementTests(vitest: NonNullable<ImportMeta["vitest"]>): Promise<void> {
  const { describe, it, expect } = vitest;
  const { default: fixture } = await import("../../🔨️modules/🏪️store/🧫️fixtures/📤️outbound-announcement/🔣️.json");
  const { default: schema } = await import("../../🔨️modules/🏪️store/🧬️schema/📤️outbound-announcement/🔣️.json");
  type Expect = { announcedOperations: number; announcedTransitions: number; edits: number; tailOperations: number; observed: string | null };
  type Edit = { key: string | null; operations: number };

  describe("OutboundAnnouncement", () => {
    it("owns a fixture its schema admits", async () => {
      const { default: Ajv } = await import("ajv");
      const validate = new Ajv({ strict: false, allErrors: true }).compile(schema);
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    });

    it("derives every step from the coalescing rule: each gesture announces exactly its own operations", () => {
      let steps = 0;
      for (const vector of fixture.vectors) {
        const ledger: Edit[] = [];
        const applied: number[] = [];
        let foreign: string | null = null;
        for (const [index, step] of vector.steps.entries()) {
          const observed: Expect = { announcedOperations: 0, announcedTransitions: 0, edits: 0, tailOperations: 0, observed: null };
          if ("gesture" in step && step.gesture !== undefined) {
            const tail = applied.length === 0 ? undefined : ledger[applied[applied.length - 1]!];
            if (step.gesture.coalesceKey !== null && tail !== undefined && tail.key === step.gesture.coalesceKey) tail.operations += step.gesture.items;
            else applied.push(ledger.push({ key: step.gesture.coalesceKey, operations: step.gesture.items }) - 1);
            observed.announcedOperations = step.gesture.items;
            observed.observed = foreign;
          } else if ("undo" in step && step.undo === true) {
            applied.pop();
            observed.announcedTransitions = 1;
          } else if ("remote" in step && step.remote !== undefined) {
            applied.push(ledger.push({ key: null, operations: 1 }) - 1);
            foreign = step.remote.id;
          } else throw new Error(`${vector.id} step ${index} names neither one gesture, an undo nor one remote operation`);
          observed.edits = ledger.length;
          observed.tailOperations = applied.length === 0 ? 0 : ledger[applied[applied.length - 1]!]!.operations;
          expect(observed, `${vector.id} step ${index}`).toEqual(step.expect);
          steps += 1;
        }
      }
      expect(steps).toBeGreaterThanOrEqual(22);
    });
  });
}
