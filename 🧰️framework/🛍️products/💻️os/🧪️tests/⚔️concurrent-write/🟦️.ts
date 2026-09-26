/** ⚔️ LAW: the language-agnostic concurrent-write fixture the database law walks
 * (`🔨️modules/🛢️db/🗿️artifact/🧪️tests/⚔️concurrent-write/🦀️.rs`) is admitted by its schema through Ajv (third-party), and an independent
 * reference of the grading rule derives every commit's outcome: a write is concurrent only with the writes another actor committed after the
 * operation it observed; it conflicts when both touch a shared path (readable path-map diffs) or their declared targets share a segment (an
 * empty target is the whole artifact); history transitions never grade or conflict; `vigilant` refuses a conflict, `normal` reports it
 * (ticket 26/09/23 LD item 2). */
export async function registerConcurrentWriteTests(vitest: NonNullable<ImportMeta["vitest"]>): Promise<void> {
  const { describe, it, expect } = vitest;
  const { default: fixture } = await import("../../🔨️modules/🛢️db/🗿️artifact/🧫️fixtures/⚔️concurrent-write/🔣️.json");
  const { default: schema } = await import("../../🔨️modules/🛢️db/🗿️artifact/🧬️schema/⚔️concurrent-write/🔣️.json");
  type Writes = { readonly target?: readonly string[]; readonly paths?: readonly string[]; readonly transition?: boolean };
  type Write = { readonly id: string; readonly actor: string; readonly observed: string | null; readonly writes: Writes };

  const pathsOverlap = (a: readonly string[], b: readonly string[]): boolean =>
    a.some((left) => b.some((right) => left === right || left.startsWith(`${right}/`) || right.startsWith(`${left}/`)));
  const conflicts = (written: Write, unseen: Write): boolean => {
    if (written.writes.paths !== undefined && unseen.writes.paths !== undefined) return pathsOverlap(written.writes.paths, unseen.writes.paths);
    const left = written.writes.target ?? [];
    const right = unseen.writes.target ?? [];
    return left.length === 0 || right.length === 0 || left.some((segment) => right.includes(segment));
  };

  describe("ConcurrentWrite", () => {
    it("owns a fixture its schema admits", async () => {
      const { default: Ajv } = await import("ajv");
      const validate = new Ajv({ strict: false, allErrors: true }).compile(schema);
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    });

    it("derives every commit from the grading rule", () => {
      let commits = 0;
      for (const vector of fixture.vectors) {
        const committed: Write[] = [];
        for (const [index, commit] of vector.commits.entries()) {
          const write = commit.write as Write;
          const codes: string[] = [];
          if (write.writes.transition !== true) {
            const seen = write.observed === null ? 0 : committed.map((entry) => entry.id).lastIndexOf(write.observed) + 1;
            for (const unseen of committed.slice(seen)) {
              if (unseen.actor !== write.actor && unseen.writes.transition !== true && conflicts(write, unseen)) codes.push("mutation.clamped");
            }
          }
          const outcome = codes.length > 0 && vector.policy === "vigilant" ? "refused" : "accepted";
          expect({ outcome, codes }, `${vector.id} commit ${index}`).toEqual({ outcome: commit.outcome, codes: commit.codes });
          if (outcome === "accepted") committed.push(write);
          commits += 1;
        }
      }
      expect(commits).toBeGreaterThanOrEqual(27);
    });
  });
}
