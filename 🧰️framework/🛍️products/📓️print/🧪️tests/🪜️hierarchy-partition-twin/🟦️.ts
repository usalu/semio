// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizPartition, vizStratify, type VizHierarchyNode } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🧱️hierarchy-partition/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
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

/** 🌱️ Stratifies one fixture table with the twin and sums its values. */
function root(rows: readonly Row[]): VizHierarchyNode<Row> {
  return vizStratify<Row>([...rows], { id: (row) => row[0], parentId: (row) => row[1] || null })
    .sum((row) => Number(row[2]) || 0);
}

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🧭️ The nodes in the layout's own preorder. */
function preorder(node: VizHierarchyNode<Row>): VizHierarchyNode<Row>[] {
  const nodes: VizHierarchyNode<Row>[] = [];
  node.eachBefore((each) => nodes.push(each));
  return nodes;
}

/** 🧊 The twin's partition on one fixture. */
function banded(rows: readonly Row[], size: [number, number], padding?: number, round?: boolean): VizHierarchyNode<Row> {
  return vizPartition(root(rows), { size, padding, round });
}

/** 📤️ The four corner records a banded fixture emits under one key. */
function rects(key: string, node: VizHierarchyNode<Row>): Record<string, (number | string)[]> {
  const nodes = preorder(node);
  return {
    [`${key}/x0`]: grid(nodes.map((each) => each.x0)),
    [`${key}/y0`]: grid(nodes.map((each) => each.y0)),
    [`${key}/x1`]: grid(nodes.map((each) => each.x1)),
    [`${key}/y1`]: grid(nodes.map((each) => each.y1)),
  };
}

/** 🔮️ The oracle of `hierarchy-partition`, reused so both subjects meet the same numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`hierarchy-partition declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    icicle: {
      oracle: oracle("icicle"),
      /** 🎯️ The twin's plain, padded and rounded partition of the balanced fixture. */
      subject: () => ({
        projection: {
          id: preorder(root(DEEP)).map((each) => each.id!),
          ...rects("plain", banded(DEEP, [100, 60])),
          ...rects("padded", banded(DEEP, [100, 60], 1.5)),
          ...rects("rounded", banded(DEEP, [100, 60], undefined, true)),
        },
      }),
    },
    sunburst: {
      oracle: oracle("sunburst"),
      /** 🎯️ The same partition over a full turn: angles and radii of the twin. */
      subject: () => {
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
    },
    unbalanced: {
      oracle: oracle("unbalanced"),
      /** 🎯️ The row count of the unbalanced fixture comes from the root height. */
      subject: () => ({
        projection: {
          id: preorder(root(UNBALANCED)).map((each) => each.id!),
          ...rects("plain", banded(UNBALANCED, [80, 50])),
        },
      }),
    },
  },
});
// #endregion 🧭️Adapter
