/** 🔁️ Attached document replacement law (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING, live fault F4): a document that
 * stays attached keeps a bound port whatever its replacement's load does. The law drives {@link replaceAttachedDocumentV1}
 * over the real {@link DocumentAttachmentLaneV1} and {@link LatestDocumentReplacementV1} with a counting port, and
 * fast-check runs arbitrary sequences of loads that resolve, reject or are superseded: once everything settled exactly one
 * port is bound, it is the newest one, and every rejected load reached its caller. */
import { expect, test } from "bun:test";
import fc from "fast-check";
import { DocumentAttachmentLaneV1, LatestDocumentReplacementV1, replaceAttachedDocumentV1 } from "../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";

/** 🔌️ The ports of one document as a shell keeps them: the bound port (its ordinal) or none, and every bind and retire. */
class CountingPorts {
  bound: number | null = null;
  binds = 0;
  retires = 0;
  readonly lane = new DocumentAttachmentLaneV1(async () => { this.retire(); });

  retire(): void {
    if (this.bound === null) return;
    this.bound = null;
    this.retires += 1;
  }

  async bind(): Promise<void> {
    this.binds += 1;
    this.bound = this.binds;
  }
}

/** 📥️ One read-back as the shell runs it: retire the bound port, then replace through the lane. */
function readBack(ports: CountingPorts, replacements: LatestDocumentReplacementV1<string>, value: string, load: (value: string) => Promise<void>): Promise<boolean> {
  ports.retire();
  return replacements.replace(value, (candidate, latest) => replaceAttachedDocumentV1(ports.lane, "owner", latest, { retired: undefined, load: () => load(candidate), bind: () => ports.bind() }));
}

test("a refused load leaves the document bound and attached, and its caller hears the refusal", async () => {
  const ports = new CountingPorts();
  await ports.lane.attach("owner", () => true, () => ports.bind());
  expect(ports.bound).toBe(1);
  const replacements = new LatestDocumentReplacementV1<string>();
  await expect(readBack(ports, replacements, "refused", async () => { throw new Error("the program refused the archive"); })).rejects.toThrow("the program refused the archive");
  expect([ports.bound, ports.binds]).toEqual([2, 2]);
  expect(await readBack(ports, replacements, "loaded", async () => {})).toBe(true);
  expect([ports.bound, ports.binds]).toEqual([3, 3]);
  await ports.lane.close("owner");
  expect(ports.bound).toBeNull();
});

test("a load superseded while its port retired loads and binds nothing; its successor binds once", async () => {
  const ports = new CountingPorts();
  await ports.lane.attach("owner", () => true, () => ports.bind());
  const replacements = new LatestDocumentReplacementV1<string>();
  const loaded: string[] = [];
  let release!: () => void;
  const held = new Promise<void>((resolve) => { release = resolve; });
  const first = readBack(ports, replacements, "first", async (value) => { await held; loaded.push(value); });
  await new Promise((resolve) => setTimeout(resolve, 0));
  const second = readBack(ports, replacements, "second", async (value) => { loaded.push(value); });
  const third = readBack(ports, replacements, "third", async (value) => { loaded.push(value); });
  release();
  expect(await Promise.all([first, second, third])).toEqual([false, false, true]);
  expect(loaded).toEqual(["first", "third"]);
  expect(ports.bound).toBe(ports.binds);
  expect(ports.binds).toBe(2);
});

test("after any settled sequence of loads exactly the newest port is bound and every refusal was heard", async () => {
  await fc.assert(
    fc.asyncProperty(fc.array(fc.record({ fails: fc.boolean(), overlap: fc.boolean() }), { minLength: 1, maxLength: 12 }), async (loads) => {
      const ports = new CountingPorts();
      await ports.lane.attach("owner", () => true, () => ports.bind());
      const replacements = new LatestDocumentReplacementV1<string>();
      const settled: Promise<"loaded" | "superseded" | "refused">[] = [];
      for (const [index, load] of loads.entries()) {
        const run = readBack(ports, replacements, `archive-${index}`, async () => { await new Promise((resolve) => setTimeout(resolve, 0)); if (load.fails) throw new Error("refused"); });
        settled.push(run.then((current) => (current ? "loaded" : "superseded"), () => "refused"));
        if (!load.overlap) await settled.at(-1);
      }
      const outcomes = await Promise.all(settled);
      await ports.lane.drain();
      expect(ports.bound).not.toBeNull();
      expect(ports.bound).toBe(ports.binds);
      expect(outcomes.at(-1)).toBe(loads.at(-1)!.fails ? "refused" : "loaded");
      outcomes.forEach((outcome, index) => { if (outcome === "loaded") expect(loads[index]!.fails).toBe(false); });
    }),
    { numRuns: 100 },
  );
});
