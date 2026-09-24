// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { partition, stratify, type HierarchyNode, type HierarchyRectangularNode } from "d3-hierarchy";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "hierarchy-partition";
const FIXTURE = "shared://🧱️hierarchy-partition/hierarchy-partition.tex";
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
function preorder(node: HierarchyNode<Row>): HierarchyRectangularNode<Row>[] {
  const nodes: HierarchyRectangularNode<Row>[] = [];
  node.eachBefore((each) => nodes.push(each as HierarchyRectangularNode<Row>));
  return nodes;
}

/** 🧊 d3's partition on one fixture. */
function banded(rows: readonly Row[], size: [number, number], padding?: number, round?: boolean): HierarchyNode<Row> {
  const node = root(rows);
  const layout = partition<Row>().size(size);
  if (padding !== undefined) layout.padding(padding);
  if (round === true) layout.round(true);
  layout(node);
  return node;
}

/** 📤️ The four corner records a banded fixture emits under one key. */
function rects(key: string, node: HierarchyNode<Row>): Record<string, (number | string)[]> {
  const nodes = preorder(node);
  return {
    [`${key}/x0`]: grid(nodes.map((each) => each.x0)),
    [`${key}/y0`]: grid(nodes.map((each) => each.y0)),
    [`${key}/x1`]: grid(nodes.map((each) => each.x1)),
    [`${key}/y1`]: grid(nodes.map((each) => each.y1)),
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
    icicle: {
      /** 🔮️ The plain, the padded and the rounded partition of the balanced fixture. */
      oracle: () => ({
        projection: {
          id: preorder(root(DEEP)).map((each) => each.id!),
          ...rects("plain", banded(DEEP, [100, 60])),
          ...rects("padded", banded(DEEP, [100, 60], 1.5)),
          ...rects("rounded", banded(DEEP, [100, 60], undefined, true)),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
    sunburst: {
      /** 🔮️ The same partition over a full turn: start and end angle, inner and outer radius. */
      oracle: () => {
        const nodes = preorder(banded(DEEP, [2 * Math.PI, 24]));
        return {
          projection: {
            id: nodes.map((each) => each.id!),
            "sunburst/startAngle": grid(nodes.map((each) => each.x0)),
            "sunburst/endAngle": grid(nodes.map((each) => each.x1)),
            "sunburst/innerRadius": grid(nodes.map((each) => each.y0)),
            "sunburst/outerRadius": grid(nodes.map((each) => each.y1)),
          },
        };
      },
      subject: async (ctx) => (await subject(ctx)),
    },
    unbalanced: {
      /** 🔮️ The row count of the unbalanced fixture comes from the root height. */
      oracle: () => ({
        projection: {
          id: preorder(root(UNBALANCED)).map((each) => each.id!),
          ...rects("plain", banded(UNBALANCED, [80, 50])),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
