/** 🧱️ Shared contract witnesses stay with the framework that owns their semantics. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { hierarchy } from "d3-hierarchy";

const root = resolve(import.meta.dir, "../../..");
const fixture = <T = Record<string, unknown>>(path: string): T => JSON.parse(readFileSync(resolve(root, path), "utf8"));
type Layer = { id: string; locked: boolean; children?: Layer[] };
type Protection = { locked: boolean; inherited: boolean; descendant: boolean; editable: boolean; structural: boolean; canChangeLock: boolean };

test("shared pixel lock vectors match the independent hierarchy oracle", () => {
  const vectors = fixture<{ layers: Layer[]; cases: { id: string; expected: Protection }[] }>("🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧫️fixtures/🔒️protection/🔣️.json");
  const tree = hierarchy<Layer>({ id: "root", locked: false, children: vectors.layers }, node => node.children);
  for (const row of vectors.cases) {
    const node = tree.descendants().find(node => node.data.id === row.id)!;
    const locked = node.data.locked;
    const inherited = node.ancestors().slice(1).some(node => node.data.locked);
    const descendant = node.descendants().slice(1).some(node => node.data.locked);
    const editable = !locked && !inherited;
    expect({ locked, inherited, descendant, editable, structural: editable && !descendant, canChangeLock: !inherited }).toEqual(row.expected);
  }
});
