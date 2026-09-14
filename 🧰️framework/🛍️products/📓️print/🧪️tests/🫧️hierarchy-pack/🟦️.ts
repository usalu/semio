// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { pack, stratify, type HierarchyCircularNode, type HierarchyNode } from "d3-hierarchy";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "hierarchy-pack";
const FIXTURE = "local://hierarchy-pack.tex";
const DECIMALS = 6;

type Row = readonly [id: string, parent: string, value: number];

/** 🌳️ `demo-hierarchy-deep` of `semio-viz-hierarchy.sty`, region 🔖️DemoData. */
const DEEP: readonly Row[] = [
  ["root", "", 0], ["a", "root", 0], ["b", "root", 0], ["c", "root", 0],
  ["a1", "a", 6], ["a2", "a", 3], ["a3", "a", 2],
  ["b1", "b", 8], ["b2", "b", 4],
  ["c1", "c", 5], ["c2", "c", 1], ["c3", "c", 7], ["c4", "c", 2],
];

/** 🌿️ `demo-hierarchy-unbalanced`: a four-level left branch beside a single leaf. */
const UNBALANCED: readonly Row[] = [
  ["r", "", 0], ["s", "r", 0], ["t", "r", 9],
  ["sa", "s", 0], ["sb", "s", 4],
  ["saa", "sa", 0], ["sab", "sa", 3],
  ["saaa", "saa", 2], ["saab", "saa", 5], ["saac", "saa", 1],
];

/** 🌱️ Stratifies one fixture table and sums its values, as \SemioVizHierarchyBuild does. */
function root(rows: readonly Row[]): HierarchyNode<Row> {
  return stratify<Row>().id((row) => row[0]).parentId((row) => row[1] || null)([...rows])
    .sum((row) => Number(row[2]) || 0);
}

/** 🔢️ Rounds an oracle's numbers onto the emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🧭️ The nodes in the layout's own preorder, which is what the fixture emits. */
function preorder(node: HierarchyNode<Row>): HierarchyCircularNode<Row>[] {
  const nodes: HierarchyCircularNode<Row>[] = [];
  node.eachBefore((each) => nodes.push(each as HierarchyCircularNode<Row>));
  return nodes;
}

type Options = Readonly<{ size: [number, number]; padding?: number; radius?: "value" | number }>;

/** 🫧 d3's circle packing on one fixture. */
function packed(rows: readonly Row[], options: Options): HierarchyNode<Row> {
  const node = root(rows);
  const layout = pack<Row>().size(options.size);
  if (options.padding !== undefined) layout.padding(options.padding);
  if (options.radius === "value") layout.radius((each) => each.value ?? 0);
  else if (typeof options.radius === "number") layout.radius(() => options.radius as number);
  layout(node);
  return node;
}

/** 📤️ The three circle records a packed fixture emits under one key. */
function circles(key: string, node: HierarchyNode<Row>): Record<string, (number | string)[]> {
  const nodes = preorder(node);
  return {
    [`${key}/x`]: grid(nodes.map((each) => each.x)),
    [`${key}/y`]: grid(nodes.map((each) => each.y)),
    [`${key}/r`]: grid(nodes.map((each) => each.r)),
  };
}

/** 🎯️ Compiles the committed fixture and projects the records of one scenario. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "default-radius": {
      /** 🔮️ The two-pass sqrt(value) branch of d3 pack(), square and oblong. */
      oracle: () => ({
        projection: {
          id: preorder(root(DEEP)).map((each) => each.id!),
          ...circles("plain", packed(DEEP, { size: [100, 100] })),
          ...circles("oblong", packed(DEEP, { size: [120, 80] })),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
    padding: {
      /** 🔮️ Padding added to and taken off every child radius around the packing. */
      oracle: () => ({ projection: { ...circles("padded", packed(DEEP, { size: [100, 100], padding: 3 })) } }),
      subject: async (ctx) => (await subject(ctx)),
    },
    "explicit-radius": {
      /** 🔮️ The single-pass branch d3 takes as soon as a radius accessor is given. */
      oracle: () => ({
        projection: {
          ...circles("value", packed(DEEP, { size: [100, 100], radius: "value" })),
          ...circles("constant", packed(DEEP, { size: [100, 100], radius: 4 })),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
    unbalanced: {
      /** 🔮️ The unbalanced fixture, whose deeper branch draws more random numbers. */
      oracle: () => ({
        projection: {
          id: preorder(root(UNBALANCED)).map((each) => each.id!),
          ...circles("plain", packed(UNBALANCED, { size: [90, 90] })),
          ...circles("padded", packed(UNBALANCED, { size: [90, 90], padding: 2 })),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
