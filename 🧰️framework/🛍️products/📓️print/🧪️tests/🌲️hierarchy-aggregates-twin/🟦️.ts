// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizStratify, type VizHierarchyNode } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🌳️hierarchy-aggregates/🟦️.ts";
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

/** 🌱️ Stratifies one fixture table with the twin, without aggregating it yet. */
function raw(rows: readonly Row[]): VizHierarchyNode<Row> {
  return vizStratify<Row>([...rows], { id: (row) => row[0], parentId: (row) => row[1] || null });
}

/** ➕️ The same, summed the way the twin's default value accessor sums. */
function root(rows: readonly Row[]): VizHierarchyNode<Row> {
  return raw(rows).sum((row) => Number(row[2]) || 0);
}

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🧭️ The nodes in the twin's preorder (`eachBefore`). */
function preorder(node: VizHierarchyNode<Row>): VizHierarchyNode<Row>[] {
  const nodes: VizHierarchyNode<Row>[] = [];
  node.eachBefore((each) => nodes.push(each));
  return nodes;
}

/** 📥️ The nodes in the twin's postorder (`eachAfter`). */
function postorder(node: VizHierarchyNode<Row>): VizHierarchyNode<Row>[] {
  const nodes: VizHierarchyNode<Row>[] = [];
  node.eachAfter((each) => nodes.push(each));
  return nodes;
}

/** 🔤️ One summed fixture with its children sorted by a comparator. */
function sorted(rows: readonly Row[], compare: (a: VizHierarchyNode<Row>, b: VizHierarchyNode<Row>) => number): VizHierarchyNode<Row> {
  const node = root(rows);
  node.sort(compare);
  return node;
}

/** 🔎️ The node with one identifier, in the preorder of a fixture. */
function find(node: VizHierarchyNode<Row>, id: string): VizHierarchyNode<Row> {
  const hit = preorder(node).find((each) => each.id === id);
  if (hit === undefined) throw new Error(`fixture has no node ${id}`);
  return hit;
}

/** 🔮️ The oracle of `hierarchy-aggregates`, reused so both subjects meet the same numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`hierarchy-aggregates declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "sum-and-count": {
      oracle: oracle("sum-and-count"),
      /** 🎯️ The twin's `sum()` and `count()` over the postorder of both fixtures. */
      subject: () => ({
        projection: {
          id: preorder(root(DEEP)).map((each) => each.id!),
          sum: grid(preorder(root(DEEP)).map((each) => each.value ?? 0)),
          count: grid(preorder(raw(DEEP).count()).map((each) => each.value ?? 0)),
          "id/unbalanced": preorder(root(UNBALANCED)).map((each) => each.id!),
          "sum/unbalanced": grid(preorder(root(UNBALANCED)).map((each) => each.value ?? 0)),
          "count/unbalanced": grid(preorder(raw(UNBALANCED).count()).map((each) => each.value ?? 0)),
        },
      }),
    },
    "depth-and-height": {
      oracle: oracle("depth-and-height"),
      /** 🎯️ The depth, the height and the postorder the twin derives from the structure. */
      subject: () => ({
        projection: {
          depth: preorder(root(DEEP)).map((each) => each.depth),
          height: preorder(root(DEEP)).map((each) => each.height),
          "depth/unbalanced": preorder(root(UNBALANCED)).map((each) => each.depth),
          "height/unbalanced": preorder(root(UNBALANCED)).map((each) => each.height),
          postorder: postorder(root(DEEP)).map((each) => each.id!),
          "postorder/unbalanced": postorder(root(UNBALANCED)).map((each) => each.id!),
        },
      }),
    },
    sort: {
      oracle: oracle("sort"),
      /** 🎯️ The five comparators, read back as the preorder the twin's `sort()` produces. */
      subject: () => ({
        projection: {
          "value-descending": preorder(sorted(DEEP, (a, b) => (b.value ?? 0) - (a.value ?? 0))).map((each) => each.id!),
          "value-ascending": preorder(sorted(DEEP, (a, b) => (a.value ?? 0) - (b.value ?? 0))).map((each) => each.id!),
          "name-descending": preorder(sorted(DEEP, (a, b) => (a.id! < b.id! ? 1 : a.id! > b.id! ? -1 : 0))).map((each) => each.id!),
          "height-descending": preorder(sorted(UNBALANCED, (a, b) => b.height - a.height)).map((each) => each.id!),
          "height-ascending": preorder(sorted(UNBALANCED, (a, b) => a.height - b.height)).map((each) => each.id!),
        },
      }),
    },
    traversal: {
      oracle: oracle("traversal"),
      /** 🎯️ The twin's `leaves()`, `ancestors()` and the breadth-first `links()`. */
      subject: () => ({
        projection: {
          leaves: root(DEEP).leaves().map((each) => each.id!),
          "leaves/unbalanced": root(UNBALANCED).leaves().map((each) => each.id!),
          ancestors: find(root(DEEP), "c3").ancestors().map((each) => each.id!),
          "ancestors/unbalanced": find(root(UNBALANCED), "saab").ancestors().map((each) => each.id!),
          links: root(DEEP).links().flatMap((link) => [link.source.id!, link.target.id!]),
        },
      }),
    },
  },
});
// #endregion 🧭️Adapter
