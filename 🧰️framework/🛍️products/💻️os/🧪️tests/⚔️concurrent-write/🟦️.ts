/** ⚔️ LAW: the language-agnostic concurrent-write fixture the database law walks
 * (`🔨️modules/🛢️db/🗿️artifact/🧪️tests/⚔️concurrent-write/🦀️.rs`) uses shared examples, and an independent
 * reference of the grading rule derives every commit's outcome: a write is concurrent only with the writes another actor committed after the
 * operation it observed; it conflicts when both touch a shared path (readable path-map diffs) or their declared targets share a segment (an
 * empty target is the whole artifact); a supersession may name any author's operation the log holds (else `history.unknown-target`,
 * whatever the policy) and grades and conflicts like a write by its declared target; every other history transition never grades or
 * conflicts; `vigilant` refuses a conflict, `normal` reports it (ticket 26/09/23 LD item 2, ticket 26/09/30
 * NON-DESTRUCTIVE-HISTORY-EDITING `📋️design.md` §2, §9.1). */
export async function registerConcurrentWriteTests(vitest: NonNullable<ImportMeta["vitest"]>): Promise<void> {
  const { describe, it, expect } = vitest;
  const { default: fixture } = await import("../../🔨️modules/🛢️db/🗿️artifact/🧫️fixtures/⚔️concurrent-write/🔣️.json");
  type Supersede = { readonly inputs: readonly { readonly target: string; readonly replacement: string }[]; readonly target: readonly string[] };
  type Writes = { readonly target?: readonly string[]; readonly paths?: readonly string[]; readonly transition?: boolean; readonly supersede?: Supersede };
  type Write = { readonly id: string; readonly actor: string; readonly observed: string | null; readonly writes: Writes };

  const pathsOverlap = (a: readonly string[], b: readonly string[]): boolean =>
    a.some((left) => b.some((right) => left === right || left.startsWith(`${right}/`) || right.startsWith(`${left}/`)));
  const declaredTarget = (write: Write): readonly string[] => write.writes.supersede?.target ?? write.writes.target ?? [];
  const conflicts = (written: Write, unseen: Write): boolean => {
    if (written.writes.paths !== undefined && unseen.writes.paths !== undefined) return pathsOverlap(written.writes.paths, unseen.writes.paths);
    const left = declaredTarget(written);
    const right = declaredTarget(unseen);
    return left.length === 0 || right.length === 0 || left.some((segment) => right.includes(segment));
  };

  describe("ConcurrentWrite", () => {

    it("derives every commit from the grading rule", () => {
      let commits = 0;
      for (const vector of fixture.vectors) {
        const committed: Write[] = [];
        for (const [index, commit] of vector.commits.entries()) {
          const write = commit.write as Write;
          const codes: string[] = [];
          const supersede = write.writes.supersede;
          if (supersede !== undefined && supersede.inputs.some((input) => !committed.some((entry) => entry.id === input.target))) {
            codes.push("history.unknown-target");
            expect({ outcome: "refused", codes }, `${vector.id} commit ${index}`).toEqual({ outcome: commit.outcome, codes: commit.codes });
            commits += 1;
            continue;
          }
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
      expect(commits).toBeGreaterThanOrEqual(57);
    });
  });
}
