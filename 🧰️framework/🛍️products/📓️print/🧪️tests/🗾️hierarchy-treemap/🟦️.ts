// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import {
  stratify, treemap, treemapBinary, treemapDice, treemapResquarify, treemapSlice, treemapSliceDice,
  treemapSquarify, type HierarchyNode, type HierarchyRectangularNode,
} from "d3-hierarchy";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "hierarchy-treemap";
const FIXTURE = "shared://🗾️hierarchy-treemap/hierarchy-treemap.tex";
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

const TILES = {
  squarify: treemapSquarify, resquarify: treemapResquarify, slice: treemapSlice,
  dice: treemapDice, "slice-dice": treemapSliceDice, binary: treemapBinary,
} as const;

type Options = Readonly<{
  size: [number, number]; tile?: keyof typeof TILES; ratio?: number; round?: boolean;
  padding?: number; paddingInner?: number; paddingTop?: number; paddingRight?: number;
  paddingBottom?: number; paddingLeft?: number;
}>;

/** 🗺️ d3's treemap on one fixture. */
function tiled(rows: readonly Row[], options: Options): HierarchyNode<Row> {
  const node = root(rows);
  const layout = treemap<Row>().size(options.size);
  const tile = options.tile === undefined ? treemapSquarify : TILES[options.tile];
  layout.tile(options.ratio === undefined ? tile : treemapSquarify.ratio(options.ratio));
  if (options.padding !== undefined) layout.padding(options.padding);
  if (options.paddingInner !== undefined) layout.paddingInner(options.paddingInner);
  if (options.paddingTop !== undefined) layout.paddingTop(options.paddingTop);
  if (options.paddingRight !== undefined) layout.paddingRight(options.paddingRight);
  if (options.paddingBottom !== undefined) layout.paddingBottom(options.paddingBottom);
  if (options.paddingLeft !== undefined) layout.paddingLeft(options.paddingLeft);
  if (options.round === true) layout.round(true);
  layout(node);
  return node;
}

/** 📤️ The four corner records a tiled fixture emits under one key. */
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
    tiling: {
      /** 🔮️ d3's six tilings on the balanced fixture. */
      oracle: () => ({
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
      subject: async (ctx) => (await subject(ctx)),
    },
    padding: {
      /** 🔮️ The uniform padding and the header-row padding vector. */
      oracle: () => ({
        projection: {
          ...rects("padding", tiled(DEEP, { size: [100, 60], padding: 2 })),
          ...rects("nested", tiled(DEEP, {
            size: [100, 60], paddingInner: 1.5, paddingTop: 6, paddingRight: 1, paddingBottom: 1, paddingLeft: 1,
          })),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
    "ratio-and-round": {
      /** 🔮️ A squarify ratio of one, and the rounded variant of the default. */
      oracle: () => ({
        projection: {
          ...rects("ratio", tiled(DEEP, { size: [100, 60], ratio: 1 })),
          ...rects("round", tiled(DEEP, { size: [100, 60], round: true })),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
    unbalanced: {
      /** 🔮️ Two tilings on the fixture whose branches differ in depth. */
      oracle: () => ({
        projection: {
          id: preorder(root(UNBALANCED)).map((each) => each.id!),
          ...rects("squarify", tiled(UNBALANCED, { size: [80, 50], tile: "squarify" })),
          ...rects("binary", tiled(UNBALANCED, { size: [80, 50], tile: "binary" })),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
