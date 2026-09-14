// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizPack, vizStratify, type VizHierarchyNode } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🫧️hierarchy-pack/🟦️.ts";
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

type Options = Readonly<{ size: [number, number]; padding?: number; radius?: "value" | number }>;

/** 🫧 The twin's circle packing on one fixture. */
function packed(rows: readonly Row[], options: Options): VizHierarchyNode<Row> {
  const radius = options.radius === "value"
    ? (node: VizHierarchyNode<Row>) => node.value ?? 0
    : typeof options.radius === "number" ? () => options.radius as number : undefined;
  return vizPack(root(rows), { size: options.size, padding: options.padding, radius });
}

/** 📤️ The three circle records a packed fixture emits under one key. */
function circles(key: string, node: VizHierarchyNode<Row>): Record<string, (number | string)[]> {
  const nodes = preorder(node);
  return {
    [`${key}/x`]: grid(nodes.map((each) => each.x)),
    [`${key}/y`]: grid(nodes.map((each) => each.y)),
    [`${key}/r`]: grid(nodes.map((each) => each.r)),
  };
}

/** 🔮️ The oracle of `hierarchy-pack`, reused so both subjects meet the same numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`hierarchy-pack declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "default-radius": {
      oracle: oracle("default-radius"),
      /** 🎯️ The twin's two-pass sqrt(value) branch, square and oblong. */
      subject: () => ({
        projection: {
          id: preorder(root(DEEP)).map((each) => each.id!),
          ...circles("plain", packed(DEEP, { size: [100, 100] })),
          ...circles("oblong", packed(DEEP, { size: [120, 80] })),
        },
      }),
    },
    padding: {
      oracle: oracle("padding"),
      /** 🎯️ Padding added to and taken off every child radius around the packing. */
      subject: () => ({ projection: { ...circles("padded", packed(DEEP, { size: [100, 100], padding: 3 })) } }),
    },
    "explicit-radius": {
      oracle: oracle("explicit-radius"),
      /** 🎯️ The single-pass branch the twin takes as soon as a radius accessor is given. */
      subject: () => ({
        projection: {
          ...circles("value", packed(DEEP, { size: [100, 100], radius: "value" })),
          ...circles("constant", packed(DEEP, { size: [100, 100], radius: 4 })),
        },
      }),
    },
    unbalanced: {
      oracle: oracle("unbalanced"),
      /** 🎯️ The unbalanced fixture, whose deeper branch draws more random numbers. */
      subject: () => ({
        projection: {
          id: preorder(root(UNBALANCED)).map((each) => each.id!),
          ...circles("plain", packed(UNBALANCED, { size: [90, 90] })),
          ...circles("padded", packed(UNBALANCED, { size: [90, 90], padding: 2 })),
        },
      }),
    },
  },
});
// #endregion 🧭️Adapter
