/** 🕸️ Differential checks of the network and flow kernels against the d3 packages registered as their oracles.
 *
 * @see ../🧰️probe-kit/🟦️.ts — the check shape and the d3 path recorder
 * @see ../🟦️.ts — the differential check table that assembles the topics
 */
import * as mark from "../../../../✒️mark/🟦️.ts";
import * as network from "../../../../🕸️network/🟦️.ts";
import * as flow from "../../../../🌊flow/🟦️.ts";
import { type Level, type VizKernelCheck, recordD3Path } from "../🧰️probe-kit/🟦️.ts";

function demoGraph(level: Level): { nodes: string[]; edges: [string, string][] } {
  const n = level === "quick" ? 12 : level === "long" ? 40 : 120;
  const nodes = Array.from({ length: n }, (_, i) => `n${i}`);
  const edges: [string, string][] = [];
  for (let i = 1; i < n; i += 1) {
    edges.push([`n${(i - 1) >> 1}`, `n${i}`]);
    if (i % 5 === 0 && i > 5) edges.push([`n${i - 5}`, `n${i}`]);
  }
  return { nodes, edges };
}

const CHORD_MATRIX = [
  [11975, 5871, 8916, 2868],
  [1951, 10048, 2060, 6171],
  [8010, 16145, 8090, 8045],
  [1013, 990, 940, 6907],
];

const SANKEY_INPUT = {
  nodes: [{ name: "a" }, { name: "b" }, { name: "c" }, { name: "d" }, { name: "e" }, { name: "f" }],
  links: [
    { source: "a", target: "c", value: 12 },
    { source: "b", target: "c", value: 7 },
    { source: "b", target: "d", value: 3 },
    { source: "c", target: "e", value: 10 },
    { source: "c", target: "f", value: 9 },
    { source: "d", target: "f", value: 3 },
    { source: "a", target: "d", value: 4 },
  ],
};

/** 🕸️ Differential checks for `🕸️network` against `d3-force` and `d3-chord`; the graph layouts have
 * no d3 counterpart and are checked against their own invariants. */
export async function networkChecks(level: Level): Promise<VizKernelCheck[]> {
  const d3Force = await import("d3-force");
  const d3Chord = await import("d3-chord");
  const graph = demoGraph(level);
  const ticks = level === "quick" ? 30 : level === "long" ? 150 : 300;
  const makeOwnNodes = () => graph.nodes.map((id) => ({ id, x: Number.NaN, y: Number.NaN, vx: Number.NaN, vy: Number.NaN }));
  const makeD3Nodes = () => graph.nodes.map((id) => ({ id }));
  const positions = (nodes: readonly { x: number; y: number }[]) => nodes.map((node) => [node.x, node.y]);
  return [
    {
      module: "network",
      name: "quadtree-invariants",
      subject: () => {
        const points = graph.nodes.map((_, i) => ({ x: (i * 37) % 101, y: (i * 53) % 89 }));
        const tree = network.vizQuadtree(points, (point) => point.x, (point) => point.y);
        let leaves = 0;
        let contained = true;
        tree.visit((node, x0, y0, x1, y1) => {
          if (Array.isArray(node)) return;
          let leaf: network.VizQuadLeaf<typeof points[number]> | undefined = node;
          while (leaf !== undefined) {
            leaves += 1;
            if (leaf.data.x < x0 || leaf.data.x > x1 || leaf.data.y < y0 || leaf.data.y > y1) contained = false;
            leaf = leaf.next;
          }
        });
        const square = Math.abs(tree.x1 - tree.x0 - (tree.y1 - tree.y0)) < 1e-9;
        return { leaves, contained, square, covers: tree.x0 <= 0 && tree.y0 <= 0 };
      },
      oracle: () => ({ leaves: graph.nodes.length, contained: true, square: true, covers: true }),
    },
    {
      module: "network",
      name: "force-initial-positions",
      subject: () => {
        const nodes = makeOwnNodes();
        network.initializeVizForceNodes(nodes);
        return positions(nodes);
      },
      oracle: () => {
        const nodes = makeD3Nodes();
        d3Force.forceSimulation(nodes as never).stop();
        return positions(nodes as never);
      },
    },
    {
      module: "network",
      name: "force-link-manybody-center",
      subject: () => {
        const nodes = makeOwnNodes();
        const links = graph.edges.map(([source, target]) => ({ source: graph.nodes.indexOf(source), target: graph.nodes.indexOf(target) }));
        const simulation = new network.VizForceSimulation(nodes);
        simulation.force("link", network.forceVizLink(links as never));
        simulation.force("charge", network.forceVizManyBody());
        simulation.force("center", network.forceVizCenter(0, 0));
        simulation.tick(ticks);
        return positions(nodes);
      },
      oracle: () => {
        const nodes = makeD3Nodes();
        const links = graph.edges.map(([source, target]) => ({ source: graph.nodes.indexOf(source), target: graph.nodes.indexOf(target) }));
        const simulation = d3Force
          .forceSimulation(nodes as never)
          .stop()
          .force("link", d3Force.forceLink(links as never))
          .force("charge", d3Force.forceManyBody())
          .force("center", d3Force.forceCenter(0, 0));
        simulation.tick(ticks);
        return positions(nodes as never);
      },
      tolerance: 1e-6,
    },
    {
      module: "network",
      name: "force-collide-x-y-radial",
      subject: () => {
        const nodes = makeOwnNodes();
        const simulation = new network.VizForceSimulation(nodes);
        simulation.force("collide", network.forceVizCollide({ radius: 6 }));
        simulation.force("x", network.forceVizX({ x: 12 }));
        simulation.force("y", network.forceVizY({ y: -8 }));
        simulation.force("radial", network.forceVizRadial(40, 0, 0, 0.05));
        simulation.tick(ticks);
        return positions(nodes);
      },
      oracle: () => {
        const nodes = makeD3Nodes();
        const simulation = d3Force
          .forceSimulation(nodes as never)
          .stop()
          .force("collide", d3Force.forceCollide(6))
          .force("x", d3Force.forceX(12))
          .force("y", d3Force.forceY(-8))
          .force("radial", d3Force.forceRadial(40, 0, 0).strength(0.05));
        simulation.tick(ticks);
        return positions(nodes as never);
      },
      tolerance: 1e-6,
    },
    {
      module: "network",
      name: "chord",
      subject: () => {
        const layout = network.vizChord(CHORD_MATRIX);
        return { groups: layout.groups, chords: layout.chords.map((chord) => ({ source: chord.source, target: chord.target })) };
      },
      oracle: () => {
        const layout = d3Chord.chord()(CHORD_MATRIX);
        return { groups: layout.groups.map(({ index, startAngle, endAngle, value }) => ({ index, startAngle, endAngle, value })), chords: layout.map((chord) => ({ source: { index: chord.source.index, startAngle: chord.source.startAngle, endAngle: chord.source.endAngle, value: chord.source.value }, target: { index: chord.target.index, startAngle: chord.target.startAngle, endAngle: chord.target.endAngle, value: chord.target.value } })) };
      },
    },
    {
      module: "network",
      name: "chord-padded",
      subject: () => network.vizChord(CHORD_MATRIX, { padAngle: 0.05 }).groups,
      oracle: () => d3Chord.chord().padAngle(0.05)(CHORD_MATRIX).groups.map(({ index, startAngle, endAngle, value }) => ({ index, startAngle, endAngle, value })),
    },
    {
      module: "network",
      name: "circular-and-arc-layout",
      subject: () => {
        const circular = network.vizCircularLayout({ nodes: ["a", "b", "c", "d"], edges: [] }, { radius: 10 });
        const arc = network.vizArcLayout({ nodes: ["a", "b", "c", "d"], edges: [] }, { length: 30 });
        return { circular: circular.map((node) => [node.x, node.y]), arc: arc.map((node) => [node.x, node.y]) };
      },
      oracle: () => ({
        circular: [
          [0, -10],
          [10, 0],
          [0, 10],
          [-10, 0],
        ],
        arc: [
          [0, 0],
          [10, 0],
          [20, 0],
          [30, 0],
        ],
      }),
    },
    {
      module: "network",
      name: "layered-invariants",
      subject: () => {
        const layout = network.vizLayeredLayout(graph, { layerGap: 20, nodeGap: 12 });
        const layerOf = new Map(layout.nodes.map((node) => [node.id, node.layer!] as const));
        const monotone = graph.edges.every(([source, target]) => layerOf.get(source)! !== layerOf.get(target)!);
        const distinct = layout.layers.every((rows) => new Set(rows).size === rows.length);
        const covered = layout.nodes.length === graph.nodes.length;
        const spacing = layout.layers.every((rows, l) => layout.nodes.filter((node) => node.layer === l).every((node, i, all) => i === 0 || Math.abs(node.x - all[i - 1]!.x - 12) < 1e-9));
        return { monotone, distinct, covered, spacing, layerCount: layout.layers.length };
      },
      oracle: () => {
        const longest = new Map<string, number>(graph.nodes.map((id) => [id, 0]));
        let changed = true;
        while (changed) {
          changed = false;
          for (const [source, target] of graph.edges) {
            const candidate = longest.get(source)! + 1;
            if (candidate > longest.get(target)!) {
              longest.set(target, candidate);
              changed = true;
            }
          }
        }
        return { monotone: true, distinct: true, covered: true, spacing: true, layerCount: Math.max(...longest.values()) + 1 };
      },
    },
  ];
}

/** 🌊 Differential checks for `🌊flow` against `d3-sankey`. */
export async function flowChecks(_level: Level): Promise<VizKernelCheck[]> {
  const d3Sankey = await import("d3-sankey");
  const alignments = ["left", "right", "center", "justify"] as const;
  const d3Alignments = { left: d3Sankey.sankeyLeft, right: d3Sankey.sankeyRight, center: d3Sankey.sankeyCenter, justify: d3Sankey.sankeyJustify };
  return [
    ...alignments.map((align) => ({
      module: "flow",
      name: `sankey-${align}`,
      subject: () => {
        const layout = flow.vizSankey(SANKEY_INPUT, { extent: [[0, 0], [400, 200]], align });
        return { nodes: layout.nodes.map((node) => [node.x0, node.x1, node.y0, node.y1]), links: layout.links.map((link) => [link.y0, link.y1, link.width]) };
      },
      oracle: () => {
        const layout = d3Sankey
          .sankey<{ name: string }, {}>()
          .nodeId(node => node.name)
          .nodeAlign(d3Alignments[align])
          .extent([
            [0, 0],
            [400, 200],
          ])({ nodes: SANKEY_INPUT.nodes.map((node) => ({ ...node })), links: SANKEY_INPUT.links.map((link) => ({ ...link })) });
        return { nodes: layout.nodes.map(node => [node.x0, node.x1, node.y0, node.y1]), links: layout.links.map(link => [link.y0, link.y1, link.width]) };
      },
      tolerance: 1e-9,
    })),
    {
      module: "flow",
      name: "sankey-link-path",
      subject: () => {
        const layout = flow.vizSankey(SANKEY_INPUT, { extent: [[0, 0], [400, 200]] });
        return flow.vizSankeyLinkHorizontal(layout.links[0]!);
      },
      oracle: () => {
        const layout = d3Sankey
          .sankey<{ name: string }, {}>()
          .nodeId(node => node.name)
          .extent([
            [0, 0],
            [400, 200],
          ])({ nodes: SANKEY_INPUT.nodes.map((node) => ({ ...node })), links: SANKEY_INPUT.links.map((link) => ({ ...link })) });
        const recorder = mark.vizPathRecorder();
        recordD3Path(d3Sankey.sankeyLinkHorizontal<{ name: string }, {}>(), recorder)(layout.links[0]!);
  
      return recorder.commands;
      },
    },
    {
      module: "flow",
      name: "alluvial-conservation",
      subject: () => {
        const rows = [
          { stages: ["low", "mid", "high"], value: 5 },
          { stages: ["low", "mid", "mid"], value: 3 },
          { stages: ["high", "mid", "low"], value: 7 },
          { stages: ["mid", "high", "high"], value: 2 },
        ];
        const layout = flow.vizAlluvial(rows, { extent: [[0, 0], [300, 150]] });
        const firstStage = layout.nodes.filter((node) => node.layer === 0).reduce((total, node) => total + node.value, 0);
        return { stages: layout.stages.map((stage) => stage.length), firstStage, linkTotal: layout.links.reduce((total, link) => total + link.value, 0) };
      },
      oracle: () => ({ stages: [3, 2, 3], firstStage: 17, linkTotal: 34 }),
    },
  ];
}
