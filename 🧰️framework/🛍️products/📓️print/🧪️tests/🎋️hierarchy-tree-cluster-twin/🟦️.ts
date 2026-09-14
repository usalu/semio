// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizCluster, vizStratify, vizTree, type VizHierarchyNode } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🌴️hierarchy-tree-cluster/🟦️.ts";
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

type Extent = Readonly<{ size?: [number, number]; nodeSize?: [number, number]; separation?: [number, number] }>;

/** ⚙️ Turns one extent row into the options object the twin layouts take. */
function options(extent: Extent): { size?: [number, number]; nodeSize?: [number, number]; separation?: (a: VizHierarchyNode<Row>, b: VizHierarchyNode<Row>) => number } {
  const separation = extent.separation === undefined
    ? undefined
    : (a: VizHierarchyNode<Row>, b: VizHierarchyNode<Row>) => (a.parent === b.parent ? extent.separation![0] : extent.separation![1]);
  return extent.nodeSize === undefined ? { size: extent.size, separation } : { nodeSize: extent.nodeSize, separation };
}

/** 🌳️ The twin's tidy tree on one fixture. */
function treeOf(rows: readonly Row[], extent: Extent): VizHierarchyNode<Row> {
  return vizTree(root(rows), options(extent));
}

/** 🍂️ The twin's cluster on one fixture. */
function clusterOf(rows: readonly Row[], extent: Extent): VizHierarchyNode<Row> {
  return vizCluster(root(rows), options(extent));
}

/** 📤️ The three records a laid-out fixture emits under one suffix. */
function placed(suffix: string, node: VizHierarchyNode<Row>, withIds: boolean): Record<string, (number | string)[]> {
  const nodes = preorder(node);
  const projection: Record<string, (number | string)[]> = {
    [`x/${suffix}`]: grid(nodes.map((each) => each.x)),
    [`y/${suffix}`]: grid(nodes.map((each) => each.y)),
  };
  if (withIds) projection[`id/${suffix}`] = nodes.map((each) => each.id!);
  return projection;
}

/** 🔮️ The oracle of `hierarchy-tree-cluster`, reused so both subjects meet the same numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`hierarchy-tree-cluster declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "tree-size": {
      oracle: oracle("tree-size"),
      /** 🎯️ The twin's tree normalised onto [100,60] on both fixtures. */
      subject: () => ({
        projection: {
          ...placed("deep", treeOf(DEEP, { size: [100, 60] }), true),
          ...placed("unbalanced", treeOf(UNBALANCED, { size: [100, 60] }), true),
        },
      }),
    },
    "tree-node-size": {
      oracle: oracle("tree-node-size"),
      /** 🎯️ The twin's `nodeSize` branch, which never normalises. */
      subject: () => ({
        projection: {
          ...placed("deep", treeOf(DEEP, { nodeSize: [12, 20] }), false),
          ...placed("unbalanced", treeOf(UNBALANCED, { nodeSize: [12, 20] }), false),
        },
      }),
    },
    "tree-separation": {
      oracle: oracle("tree-separation"),
      /** 🎯️ The twin's separation feeding both the first walk and the normalisation. */
      subject: () => ({
        projection: {
          "x/deep": grid(preorder(treeOf(DEEP, { size: [100, 60], separation: [1, 2.5] })).map((each) => each.x)),
          "x/unbalanced": grid(preorder(treeOf(UNBALANCED, { nodeSize: [10, 10], separation: [2, 3] })).map((each) => each.x)),
        },
      }),
    },
    "cluster-size": {
      oracle: oracle("cluster-size"),
      /** 🎯️ The twin's cluster normalised onto [100,60] on both fixtures. */
      subject: () => ({
        projection: {
          ...placed("deep", clusterOf(DEEP, { size: [100, 60] }), true),
          ...placed("unbalanced", clusterOf(UNBALANCED, { size: [100, 60] }), true),
        },
      }),
    },
    "cluster-node-size": {
      oracle: oracle("cluster-node-size"),
      /** 🎯️ The twin's cluster `nodeSize`, which anchors the whole tree on its root. */
      subject: () => ({
        projection: {
          ...placed("deep", clusterOf(DEEP, { nodeSize: [12, 20] }), false),
          ...placed("unbalanced", clusterOf(UNBALANCED, { nodeSize: [12, 20], separation: [1.5, 3] }), false),
        },
      }),
    },
  },
});
// #endregion 🧭️Adapter
