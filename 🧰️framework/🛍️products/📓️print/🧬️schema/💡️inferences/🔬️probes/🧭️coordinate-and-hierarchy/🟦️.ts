/** 🧭️ Differential checks of the coordinate and hierarchy kernels against the d3 packages registered as their oracles.
 *
 * @see ../🧰️probe-kit/🟦️.ts — the check shape and the sampling densities
 * @see ../🟦️.ts — the differential check table that assembles the topics
 */
import * as coordinate from "../../🧭coordinate/🟦️.ts";
import * as hierarchy from "../../🌳hierarchy/🟦️.ts";
import { vizCluster, vizTree } from "../../🌳hierarchy/🌲️tidy-tree-and-cluster/🟦️.ts";
import { vizPartition, vizTreemap } from "../../🌳hierarchy/🧱️treemap-tilings-and-partition/🟦️.ts";
import { vizPack, vizPackEnclose } from "../../🌳hierarchy/🫧️circle-packing/🟦️.ts";
import type { Level, VizKernelCheck } from "../🧰️probe-kit/🟦️.ts";

const FRAME = { x0: 10, y0: 20, x1: 170, y1: 120 } as const;

/** 🧪️ Conformance and round-trip checks for `🧭coordinate`; d3 has no coordinate layer, so the
 * evidence is the system's own inverse plus the anchors the LaTeX kernel documents. */
export function coordinateChecks(): VizKernelCheck[] {
  const probes: [number, number][] = [
    [0, 0],
    [0.25, 0.5],
    [0.5, 0.5],
    [1, 1],
    [0.8, 0.2],
  ];
  return [
    {
      module: "coordinate",
      name: "cartesian-corners",
      subject: () => {
        const system = coordinate.coordinateCartesian(FRAME);
        return [system.project(0, 0), system.project(1, 0), system.project(0, 1), system.project(1, 1)];
      },
      oracle: () => [
        [10, 120],
        [170, 120],
        [10, 20],
        [170, 20],
      ],
    },
    {
      module: "coordinate",
      name: "cartesian-round-trip",
      subject: () => {
        const system = coordinate.coordinateCartesian(FRAME);
        return probes.map(([u, v]) => system.invert!(...system.project(u, v)));
      },
      oracle: () => probes,
    },
    {
      module: "coordinate",
      name: "polar-round-trip",
      subject: () => {
        const system = coordinate.coordinatePolar(FRAME, { innerRadius: 5, outerRadius: 40 });
        return probes.filter(([u]) => u > 0 && u < 1).map(([u, v]) => system.invert!(...system.project(u, v)));
      },
      oracle: () => probes.filter(([u]) => u > 0 && u < 1),
    },
    {
      module: "coordinate",
      name: "polar-anchors",
      subject: () => {
        const system = coordinate.coordinatePolar(FRAME, { innerRadius: 0, outerRadius: 40 });
        return [system.project(0, 1), system.project(0.25, 1), system.project(0.5, 1)];
      },
      oracle: () => [
        [90, 30],
        [130, 70],
        [90, 110],
      ],
    },
    {
      module: "coordinate",
      name: "logpolar-round-trip",
      subject: () => {
        const system = coordinate.coordinateLogPolar(FRAME, { outerRadius: 40, base: 10 });
        return probes.filter(([u]) => u > 0 && u < 1).map(([u, v]) => system.invert!(...system.project(u, v)));
      },
      oracle: () => probes.filter(([u]) => u > 0 && u < 1),
    },
    {
      module: "coordinate",
      name: "ternary-barycentre",
      subject: () => {
        const system = coordinate.coordinateTernary(FRAME);
        const corners = [system.projectTernary(1, 0, 0), system.projectTernary(0, 1, 0), system.projectTernary(0, 0, 1)];
        const centre = system.projectTernary(1, 1, 1);
        return { centre, corners, average: [corners.reduce((total, point) => total + point[0], 0) / 3, corners.reduce((total, point) => total + point[1], 0) / 3] };
      },
      oracle: () => {
        const system = coordinate.coordinateTernary(FRAME);
        const corners = [system.projectTernary(1, 0, 0), system.projectTernary(0, 1, 0), system.projectTernary(0, 0, 1)];
        return { centre: [corners.reduce((total, point) => total + point[0], 0) / 3, corners.reduce((total, point) => total + point[1], 0) / 3], corners, average: [corners.reduce((total, point) => total + point[0], 0) / 3, corners.reduce((total, point) => total + point[1], 0) / 3] };
      },
    },
    {
      module: "coordinate",
      name: "parallel-axes",
      subject: () => {
        const system = coordinate.coordinateParallel(FRAME, { axes: ["a", "b", "c", "d"] });
        return { axes: system.axes().map((axis) => axis.name), positions: [0, 1 / 3, 2 / 3, 1].map((u) => system.project(u, 0)[0]) };
      },
      oracle: () => ({ axes: ["a", "b", "c", "d"], positions: [10, 63.33333333333333, 116.66666666666667, 170] }),
    },
    {
      module: "coordinate",
      name: "barycentric-centre",
      subject: () => coordinate.coordinateBarycentric(FRAME, 5).projectWeights([1, 1, 1, 1, 1]),
      oracle: () => [90, 70],
    },
  ];
}

type DemoTree = { name: string; value?: number; children?: DemoTree[] };

function demoTree(level: Level): DemoTree {
  const breadth = level === "quick" ? 3 : level === "long" ? 5 : 7;
  const build = (prefix: string, depth: number): DemoTree => {
    if (depth === 0) return { name: prefix, value: 1 + ((prefix.length * 7 + prefix.charCodeAt(prefix.length - 1)) % 23) };
    return { name: prefix, children: Array.from({ length: breadth }, (_, i) => build(`${prefix}${String.fromCharCode(97 + i)}`, depth - 1)) };
  };
  return build("r", level === "quick" ? 2 : 3);
}

function demoFlatTree(): { id: string; parentId?: string; value: number }[] {
  return [
    { id: "root", value: 0 },
    { id: "a", parentId: "root", value: 7 },
    { id: "b", parentId: "root", value: 3 },
    { id: "c", parentId: "root", value: 11 },
    { id: "a1", parentId: "a", value: 4 },
    { id: "a2", parentId: "a", value: 9 },
    { id: "b1", parentId: "b", value: 2 },
    { id: "c1", parentId: "c", value: 6 },
    { id: "c2", parentId: "c", value: 5 },
    { id: "c3", parentId: "c", value: 1 },
  ];
}

/** 🌳 Differential checks for `🌳hierarchy` against `d3-hierarchy`. */
export async function hierarchyChecks(level: Level): Promise<VizKernelCheck[]> {
  const d3Hierarchy = await import("d3-hierarchy");
  const data = demoTree(level);
  const flat = demoFlatTree();
  const ownRoot = (): hierarchy.VizHierarchyNode<DemoTree> => hierarchy.vizHierarchy<DemoTree>(data).sum((node) => node.value ?? 0).sort((a, b) => (b.value ?? 0) - (a.value ?? 0));
  const d3Root = () => d3Hierarchy.hierarchy(data).sum((node) => (node as DemoTree).value ?? 0).sort((a, b) => (b.value ?? 0) - (a.value ?? 0));
  const ownNodes = (root: hierarchy.VizHierarchyNode<DemoTree>, fields: readonly string[]) => {
    const out: number[][] = [];
    root.eachBefore((node) => out.push(fields.map((field) => (node as unknown as Record<string, number>)[field]!)));
    return out;
  };
  const d3Nodes = (root: { eachBefore(callback: (node: unknown) => void): unknown }, fields: readonly string[]) => {
    const out: number[][] = [];
    root.eachBefore((node) => out.push(fields.map((field) => (node as Record<string, number>)[field]!)));
    return out;
  };
  const tilings = ["squarify", "slice", "dice", "slice-dice", "binary", "resquarify"] as const;
  const d3Tilings = { squarify: d3Hierarchy.treemapSquarify, slice: d3Hierarchy.treemapSlice, dice: d3Hierarchy.treemapDice, "slice-dice": d3Hierarchy.treemapSliceDice, binary: d3Hierarchy.treemapBinary, resquarify: d3Hierarchy.treemapResquarify };
  return [
    {
      module: "hierarchy",
      name: "sum-count-depth-height",
      subject: () => ownNodes(ownRoot().count(), ["depth", "height", "value"]),
      oracle: () => d3Nodes(d3Root().count() as never, ["depth", "height", "value"]),
    },
    {
      module: "hierarchy",
      name: "stratify",
      subject: () => {
        const root = hierarchy.vizStratify(flat).sum((row) => row.value);
        return { ids: root.descendants().map((node) => node.id), values: root.descendants().map((node) => node.value), leaves: root.leaves().length, links: root.links().length };
      },
      oracle: () => {
        const root = d3Hierarchy.stratify<{ id: string; parentId?: string; value: number }>()(flat).sum((row) => row.value);
        return { ids: root.descendants().map((node) => node.id), values: root.descendants().map((node) => node.value), leaves: root.leaves().length, links: root.links().length };
      },
    },
    {
      module: "hierarchy",
      name: "tree-normalised",
      subject: () => ownNodes(vizTree(ownRoot(), { size: [120, 80] }), ["x", "y"]),
      oracle: () => d3Nodes(d3Hierarchy.tree().size([120, 80])(d3Root() as never) as never, ["x", "y"]),
    },
    {
      module: "hierarchy",
      name: "tree-node-size",
      subject: () => ownNodes(vizTree(ownRoot(), { nodeSize: [12, 30] }), ["x", "y"]),
      oracle: () => d3Nodes(d3Hierarchy.tree().nodeSize([12, 30])(d3Root() as never) as never, ["x", "y"]),
    },
    {
      module: "hierarchy",
      name: "cluster",
      subject: () => ownNodes(vizCluster(ownRoot(), { size: [120, 80] }), ["x", "y"]),
      oracle: () => d3Nodes(d3Hierarchy.cluster().size([120, 80])(d3Root() as never) as never, ["x", "y"]),
    },
    ...tilings.map((tile) => ({
      module: "hierarchy",
      name: `treemap-${tile}`,
      subject: () => ownNodes(vizTreemap(ownRoot(), { size: [160, 100], tile, paddingInner: 1, paddingOuter: 2 }), ["x0", "y0", "x1", "y1"]),
      oracle: () =>
        d3Nodes(
          d3Hierarchy
            .treemap()
            .size([160, 100])
            .tile(d3Tilings[tile])
            .paddingInner(1)
            .paddingOuter(2)(d3Root() as never) as never,
          ["x0", "y0", "x1", "y1"],
        ),
    })),
    {
      module: "hierarchy",
      name: "treemap-rounded",
      subject: () => ownNodes(vizTreemap(ownRoot(), { size: [160, 100], round: true }), ["x0", "y0", "x1", "y1"]),
      oracle: () => d3Nodes(d3Hierarchy.treemap().size([160, 100]).round(true)(d3Root() as never) as never, ["x0", "y0", "x1", "y1"]),
    },
    {
      module: "hierarchy",
      name: "partition",
      subject: () => ownNodes(vizPartition(ownRoot(), { size: [160, 100], padding: 1 }), ["x0", "y0", "x1", "y1"]),
      oracle: () => d3Nodes(d3Hierarchy.partition().size([160, 100]).padding(1)(d3Root() as never) as never, ["x0", "y0", "x1", "y1"]),
    },
    {
      module: "hierarchy",
      name: "pack",
      subject: () => ownNodes(vizPack(ownRoot(), { size: [160, 160] }), ["x", "y", "r"]),
      oracle: () => d3Nodes(d3Hierarchy.pack().size([160, 160])(d3Root() as never) as never, ["x", "y", "r"]),
    },
    {
      module: "hierarchy",
      name: "pack-padded",
      subject: () => ownNodes(vizPack(ownRoot(), { size: [160, 160], padding: 3 }), ["x", "y", "r"]),
      oracle: () => d3Nodes(d3Hierarchy.pack().size([160, 160]).padding(3)(d3Root() as never) as never, ["x", "y", "r"]),
    },
    {
      module: "hierarchy",
      name: "pack-enclose",
      subject: () => {
        const circles = [
          { x: 0, y: 0, r: 5 },
          { x: 10, y: 3, r: 7 },
          { x: -4, y: 9, r: 2 },
          { x: 6, y: -8, r: 4 },
          { x: 1, y: 1, r: 9 },
        ];
        return vizPackEnclose(circles);
      },
      oracle: () =>
        d3Hierarchy.packEnclose([
          { x: 0, y: 0, r: 5 },
          { x: 10, y: 3, r: 7 },
          { x: -4, y: 9, r: 2 },
          { x: 6, y: -8, r: 4 },
          { x: 1, y: 1, r: 9 },
        ]),
    },
    {
      module: "hierarchy",
      name: "lcg-stream",
      subject: () => {
        const random = hierarchy.vizLcg();
        return Array.from({ length: 8 }, () => random());
      },
      oracle: () => {
        let s = 1;
        return Array.from({ length: 8 }, () => {
          s = (1664525 * s + 1013904223) % 4294967296;
          return s / 4294967296;
        });
      },
    },
  ];
}
