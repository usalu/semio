// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { vizStratify, vizTreemap, type VizHierarchyNode } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🗾️hierarchy-treemap/🟦️.ts";
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

type Options = Readonly<{
  size: [number, number]; tile?: string; round?: boolean; padding?: number; paddingInner?: number;
  paddingTop?: number; paddingRight?: number; paddingBottom?: number; paddingLeft?: number;
}>;

/** 🗺️ The twin's treemap on one fixture. */
function tiled(rows: readonly Row[], options: Options): VizHierarchyNode<Row> {
  return vizTreemap(root(rows), {
    size: options.size,
    tile: options.tile ?? "squarify",
    round: options.round,
    paddingInner: options.paddingInner ?? options.padding,
    paddingOuter: options.padding,
    paddingTop: options.paddingTop,
    paddingRight: options.paddingRight,
    paddingBottom: options.paddingBottom,
    paddingLeft: options.paddingLeft,
  });
}

/** 📤️ The four corner records a tiled fixture emits under one key. */
function rects(key: string, node: VizHierarchyNode<Row>): Record<string, (number | string)[]> {
  const nodes = preorder(node);
  return {
    [`${key}/x0`]: grid(nodes.map((each) => each.x0)),
    [`${key}/y0`]: grid(nodes.map((each) => each.y0)),
    [`${key}/x1`]: grid(nodes.map((each) => each.x1)),
    [`${key}/y1`]: grid(nodes.map((each) => each.y1)),
  };
}

/** 🔮️ The oracle of `hierarchy-treemap`, reused so both subjects meet the same numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`hierarchy-treemap declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    tiling: {
      oracle: oracle("tiling"),
      /** 🎯️ The twin's six tilings on the balanced fixture. */
      subject: () => ({
        projection: {
          id: preorder(root(DEEP)).map((each) => each.id!),
          ...rects("squarify", tiled(DEEP, { size: [100, 60], tile: "squarify" })),
          ...rects("resquarify", tiled(DEEP, { size: [100, 60], tile: "resquarify" })),
          ...rects("slice", tiled(DEEP, { size: [100, 60], tile: "slice" })),
          ...rects("dice", tiled(DEEP, { size: [100, 60], tile: "dice" })),
          ...rects("slice-dice", tiled(DEEP, { size: [100, 60], tile: "slice-dice" })),
          ...rects("binary", tiled(DEEP, { size: [100, 60], tile: "binary" })),
        },
      }),
    },
    padding: {
      oracle: oracle("padding"),
      /** 🎯️ The uniform padding and the header-row padding vector. */
      subject: () => ({
        projection: {
          ...rects("padding", tiled(DEEP, { size: [100, 60], padding: 2 })),
          ...rects("nested", tiled(DEEP, {
            size: [100, 60], paddingInner: 1.5, paddingTop: 6, paddingRight: 1, paddingBottom: 1, paddingLeft: 1,
          })),
        },
      }),
    },
    unbalanced: {
      oracle: oracle("unbalanced"),
      /** 🎯️ Two tilings on the fixture whose branches differ in depth. */
      subject: () => ({
        projection: {
          id: preorder(root(UNBALANCED)).map((each) => each.id!),
          ...rects("squarify", tiled(UNBALANCED, { size: [80, 50], tile: "squarify" })),
          ...rects("binary", tiled(UNBALANCED, { size: [80, 50], tile: "binary" })),
        },
      }),
    },
  },
});
// #endregion 🧭️Adapter
