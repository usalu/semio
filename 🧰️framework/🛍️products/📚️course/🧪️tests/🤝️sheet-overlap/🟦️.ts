/** 🤝️ Law: two runs of a pooled task share an item by construction, so a cooperative test can always get two overlapping sheets.
 *
 * The pooled task deals one item per slot from a deck that only the high half of the run seed shuffles; the low half picks the index.
 * Seeds with the same high half therefore deal the same decks, and every low half that picks the same index in one slot shares that slot's item.
 * @see ../../🔨️modules/🃏️sheet/🟦️.ts
 * @see ../../../../../🎓️teaching/🏛️architecture/📚️course/🧪️tests/👥️shared-presence/🟦️.ts — the specs that need a shared item
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { runSeed, sheetOf, type Quiz } from "../../📦️packages/🟦️typescript/🟦️.ts";

const QUIZ = JSON.parse(readFileSync(new URL("../../../../../🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/❓️quiz/🔣️.json", import.meta.url), "utf8")) as Quiz;
const TASK = QUIZ.tasks.find((task) => task.kind === "classification" && task.pool !== undefined)!;

const dealt = (seed: number): ReadonlySet<string> => new Set(sheetOf(QUIZ, seed, "medium").tasks.find((task) => task.id === TASK.id)!.items.map((item) => item.id));
const shared = (left: ReadonlySet<string>, right: ReadonlySet<string>): readonly string[] => [...left].filter((id) => right.has(id));

describe("two sheets of a pooled task overlap", () => {
  it("deals one item per slot, as many as the task draws", () => {
    expect(TASK.pool!.slots.length).toBe(TASK.draw);
    for (let index = 0; index < 20; index++) expect(dealt(runSeed(`run-${index}`)).size).toBe(TASK.draw);
  });

  it("shares the item of a slot whenever the seeds share their high half and pick the same index in it", () => {
    const length = TASK.pool!.slots[0]!.length;
    for (let index = 0; index < 200; index++) {
      const first = runSeed(`first-${index}`) >>> 0;
      const second = ((first & 0xffff0000) | ((first & 0xffff) % length)) >>> 0;
      expect(shared(dealt(first), dealt(second)).length, `seeds ${first} and ${second}`).toBeGreaterThanOrEqual(1);
    }
  });

  it("needs at most 40 redraws of unrelated run ids to meet any sheet (the bound of the cooperative specs)", () => {
    let worst = 0;
    for (let index = 0; index < 300; index++) {
      const first = dealt(runSeed(`anna-${index}`));
      let redraws = 0;
      while (shared(first, dealt(runSeed(`ben-${index}-${redraws}`))).length === 0) redraws++;
      worst = Math.max(worst, redraws);
    }
    expect(worst).toBeLessThanOrEqual(40);
  });
});
