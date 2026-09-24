// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { stratify, type HierarchyNode } from "d3-hierarchy";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "hierarchy-aggregates";
const FIXTURE = "shared://🌳️hierarchy-aggregates/hierarchy-aggregates.tex";
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

/** 🧭️ `demo-hierarchy-path`: the delimited-path table, value on the full path only. */
const PATHS: readonly (readonly [string, number])[] = [
  ["system/core/data", 5], ["system/core/render", 8], ["system/ui/input", 3],
  ["system/ui/theme", 4], ["system/io/read", 6], ["system/io/write", 2],
];

/** 🧱️ Expands the path table into the rows \semio_viz_hierarchy_build_path:n creates. */
function pathRows(): Row[] {
  const seen = new Map<string, { parent: string; value: number }>();
  const order: string[] = [];
  for (const [path, value] of PATHS) {
    let prefix = "";
    let parent = "";
    for (const segment of path.split("/")) {
      prefix = prefix === "" ? segment : `${prefix}/${segment}`;
      if (!seen.has(prefix)) { seen.set(prefix, { parent, value: 0 }); order.push(prefix); }
      parent = prefix;
    }
    seen.get(path)!.value = value;
  }
  return order.map((id) => [id, seen.get(id)!.parent, seen.get(id)!.value] as const);
}

/** 🌱️ Stratifies one fixture table without aggregating it yet. */
function raw(rows: readonly Row[]): HierarchyNode<Row> {
  return stratify<Row>().id((row) => row[0]).parentId((row) => row[1] || null)([...rows]);
}

/** ➕️ The same, summed the way \SemioVizHierarchyBuild does by default. */
function root(rows: readonly Row[]): HierarchyNode<Row> {
  return raw(rows).sum((row) => Number(row[2]) || 0);
}

/** 🔢️ Rounds an oracle's numbers onto the emission grid the probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🧭️ The nodes in d3's preorder (`eachBefore`). */
function preorder(node: HierarchyNode<Row>): HierarchyNode<Row>[] {
  const nodes: HierarchyNode<Row>[] = [];
  node.eachBefore((each) => nodes.push(each));
  return nodes;
}

/** 📥️ The nodes in d3's postorder (`eachAfter`). */
function postorder(node: HierarchyNode<Row>): HierarchyNode<Row>[] {
  const nodes: HierarchyNode<Row>[] = [];
  node.eachAfter((each) => nodes.push(each));
  return nodes;
}

/** 🔤️ One summed fixture with its children sorted by a comparator. */
function sorted(rows: readonly Row[], compare: (a: HierarchyNode<Row>, b: HierarchyNode<Row>) => number): HierarchyNode<Row> {
  const node = root(rows);
  node.sort(compare);
  return node;
}

/** 🔎️ The node with one identifier, in the preorder of a fixture. */
function find(node: HierarchyNode<Row>, id: string): HierarchyNode<Row> {
  const hit = preorder(node).find((each) => each.id === id);
  if (hit === undefined) throw new Error(`fixture has no node ${id}`);
  return hit;
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
    "sum-and-count": {
      /** 🔮️ d3 `node.sum()` and `node.count()` on both fixtures. */
      oracle: () => ({
        projection: {
          id: preorder(root(DEEP)).map((each) => each.id!),
          sum: grid(preorder(root(DEEP)).map((each) => each.value ?? 0)),
          count: grid(preorder(raw(DEEP).count()).map((each) => each.value ?? 0)),
          "id/unbalanced": preorder(root(UNBALANCED)).map((each) => each.id!),
          "sum/unbalanced": grid(preorder(root(UNBALANCED)).map((each) => each.value ?? 0)),
          "count/unbalanced": grid(preorder(raw(UNBALANCED).count()).map((each) => each.value ?? 0)),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
    "depth-and-height": {
      /** 🔮️ The depth, the height and the postorder d3 derives from the same structure. */
      oracle: () => ({
        projection: {
          depth: preorder(root(DEEP)).map((each) => each.depth),
          height: preorder(root(DEEP)).map((each) => each.height),
          "depth/unbalanced": preorder(root(UNBALANCED)).map((each) => each.depth),
          "height/unbalanced": preorder(root(UNBALANCED)).map((each) => each.height),
          postorder: postorder(root(DEEP)).map((each) => each.id!),
          "postorder/unbalanced": postorder(root(UNBALANCED)).map((each) => each.id!),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
    sort: {
      /** 🔮️ The five comparators, read back as the preorder they produce. */
      oracle: () => ({
        projection: {
          "value-descending": preorder(sorted(DEEP, (a, b) => (b.value ?? 0) - (a.value ?? 0))).map((each) => each.id!),
          "value-ascending": preorder(sorted(DEEP, (a, b) => (a.value ?? 0) - (b.value ?? 0))).map((each) => each.id!),
          "name-descending": preorder(sorted(DEEP, (a, b) => (a.id! < b.id! ? 1 : a.id! > b.id! ? -1 : 0))).map((each) => each.id!),
          "height-descending": preorder(sorted(UNBALANCED, (a, b) => b.height - a.height)).map((each) => each.id!),
          "height-ascending": preorder(sorted(UNBALANCED, (a, b) => a.height - b.height)).map((each) => each.id!),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
    traversal: {
      /** 🔮️ d3 `leaves()`, `ancestors()` and the breadth-first `links()`. */
      oracle: () => ({
        projection: {
          leaves: root(DEEP).leaves().map((each) => each.id!),
          "leaves/unbalanced": root(UNBALANCED).leaves().map((each) => each.id!),
          ancestors: find(root(DEEP), "c3").ancestors().map((each) => each.id!),
          "ancestors/unbalanced": find(root(UNBALANCED), "saab").ancestors().map((each) => each.id!),
          links: root(DEEP).links().flatMap((link) => [link.source.id!, link.target.id!]),
        },
      }),
      subject: async (ctx) => (await subject(ctx)),
    },
    "path-stratify": {
      /** 🔮️ The expanded path table, stratified and summed like any other hierarchy. */
      oracle: () => {
        const rows = pathRows();
        return {
          projection: {
            id: preorder(root(rows)).map((each) => each.id!),
            sum: grid(preorder(root(rows)).map((each) => each.value ?? 0)),
            depth: preorder(root(rows)).map((each) => each.depth),
          },
        };
      },
      subject: async (ctx) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
