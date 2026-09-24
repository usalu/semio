// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cluster, stratify, tree, type HierarchyNode } from "d3-hierarchy";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "hierarchy-tree-cluster";
const FIXTURE = "shared://🌴️hierarchy-tree-cluster/hierarchy-tree-cluster.tex";
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
function preorder(node: HierarchyNode<Row>): HierarchyNode<Row>[] {
  const nodes: HierarchyNode<Row>[] = [];
  node.eachBefore((each) => nodes.push(each));
  return nodes;
}

type Extent = Readonly<{ size?: [number, number]; nodeSize?: [number, number]; separation?: [number, number] }>;

/** 🌳️ d3's tidy tree on one fixture. */
function treeOf(rows: readonly Row[], extent: Extent): HierarchyNode<Row> {
  const node = root(rows);
  const layout = tree<Row>();
  if (extent.nodeSize !== undefined) layout.nodeSize(extent.nodeSize); else layout.size(extent.size!);
  if (extent.separation !== undefined) {
    const [siblings, cousins] = extent.separation;
    layout.separation((a, b) => (a.parent === b.parent ? siblings : cousins));
  }
  layout(node);
  return node;
}

/** 🍂️ d3's cluster on one fixture. */
function clusterOf(rows: readonly Row[], extent: Extent): HierarchyNode<Row> {
  const node = root(rows);
  const layout = cluster<Row>();
  if (extent.nodeSize !== undefined) layout.nodeSize(extent.nodeSize); else layout.size(extent.size!);
  if (extent.separation !== undefined) {
    const [siblings, cousins] = extent.separation;
    layout.separation((a, b) => (a.parent === b.parent ? siblings : cousins));
  }
  layout(node);
  return node;
}

/** 📤️ The three records a laid-out fixture emits under one suffix. */
function placed(suffix: string, node: HierarchyNode<Row>, withIds: boolean): Record<string, (number | string)[]> {
  const nodes = preorder(node);
  const projection: Record<string, (number | string)[]> = {
    [`x/${suffix}`]: grid(nodes.map((each) => each.x!)),
    [`y/${suffix}`]: grid(nodes.map((each) => each.y!)),
  };
  if (withIds) projection[`id/${suffix}`] = nodes.map((each) => each.id!);
  return projection;
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
    "tree-size": {
      /** 🔮️ d3 tree().size([100,60]) on the balanced and the unbalanced fixture. */
      oracle: () => ({
        projection: {
          ...placed("deep", treeOf(DEEP, { size: [100, 60] }), true),
          ...placed("unbalanced", treeOf(UNBALANCED, { size: [100, 60] }), true),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
    "tree-node-size": {
      /** 🔮️ d3 tree().nodeSize([12,20]) — the branch that never normalises. */
      oracle: () => ({
        projection: {
          ...placed("deep", treeOf(DEEP, { nodeSize: [12, 20] }), false),
          ...placed("unbalanced", treeOf(UNBALANCED, { nodeSize: [12, 20] }), false),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
    "tree-separation": {
      /** 🔮️ d3 tree().separation() feeding both the first walk and the normalisation. */
      oracle: () => ({
        projection: {
          "x/deep": grid(preorder(treeOf(DEEP, { size: [100, 60], separation: [1, 2.5] })).map((each) => each.x!)),
          "x/unbalanced": grid(preorder(treeOf(UNBALANCED, { nodeSize: [10, 10], separation: [2, 3] })).map((each) => each.x!)),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
    "cluster-size": {
      /** 🔮️ d3 cluster().size([100,60]) on both fixtures. */
      oracle: () => ({
        projection: {
          ...placed("deep", clusterOf(DEEP, { size: [100, 60] }), true),
          ...placed("unbalanced", clusterOf(UNBALANCED, { size: [100, 60] }), true),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
    "cluster-node-size": {
      /** 🔮️ d3 cluster().nodeSize(), which anchors the whole tree on its root. */
      oracle: () => ({
        projection: {
          ...placed("deep", clusterOf(DEEP, { nodeSize: [12, 20] }), false),
          ...placed("unbalanced", clusterOf(UNBALANCED, { nodeSize: [12, 20], separation: [1.5, 3] }), false),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
