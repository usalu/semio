/** 🔁️ LAW: the TS twin of the kernel's echo suppression walks the SAME language-agnostic fixture as the Rust runner
 * (`🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️document-echo-suppression/🦀️.rs`), using operation identity: a late
 * joiner applies the whole catch-up tail even when the hub stamped it with the joiner's own actor, the same user's second device applies
 * the first device's edits, and a genuine echo is applied exactly once (ticket 26/09/23 session 12, run s12i). */
export async function registerDocumentEchoSuppressionTests(
  vitest: NonNullable<ImportMeta["vitest"]>,
  twin: {
    readonly admitRemoteEnvelopes: <T>(known: Set<string>, envelopes: readonly T[], idOf: (envelope: T) => string) => T[];
    readonly noteAuthoredEnvelopeIds: (known: Set<string>, ids: Iterable<string>) => void;
  },
): Promise<void> {
  const { describe, it, expect } = vitest;
  const { default: fixture } = await import("../../🔨️modules/🏪️store/🧫️fixtures/document-echo-suppression-v1/🔣️.json");
  type Envelope = { readonly mutationId: string; readonly actor: string };

  describe("DocumentEchoSuppression", () => {
    it("walks every vector: applied by operation identity, never discarded by frame origin", () => {
      let frames = 0;
      for (const vector of fixture.vectors) {
        const known = new Set<string>();
        for (const [index, step] of vector.steps.entries()) {
          if (step.authored !== undefined) {
            twin.noteAuthoredEnvelopeIds(known, step.authored);
            continue;
          }
          if (step.frame === undefined) throw new Error(`${vector.id} step ${index} names neither authored ids nor a frame`);
          const applied = twin.admitRemoteEnvelopes(known, step.frame.envelopes as readonly Envelope[], (envelope) => envelope.mutationId).map((envelope) => envelope.mutationId);
          expect(applied, `${vector.id} step ${index}`).toEqual(step.expectApplied);
          frames += 1;
        }
      }
      expect(frames).toBeGreaterThanOrEqual(7);
    });
  });
}
