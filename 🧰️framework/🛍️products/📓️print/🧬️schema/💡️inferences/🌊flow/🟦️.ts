/** 🌊 Flows: the TypeScript twin of `semio-viz-flow`. The Sankey layout with the four alignments
 * and d3's relaxation schedule, its horizontal link ribbons, and the alluvial diagram — a Sankey
 * over ordered categorical stages.
 * @see ../../../🖋️latex/semio-viz-flow.sty
 */
import { curveBumpX, vizPathRecorder, type VizPathCommand, type VizPathContext } from "../✒️mark/🟦️.ts";

//#region 🔖️Model
/** 🌊 One Sankey node after layout. */
export type VizSankeyNode = {
  readonly name: string;
  index: number;
  depth: number;
  height: number;
  layer: number;
  value: number;
  fixedValue?: number;
  x0: number;
  x1: number;
  y0: number;
  y1: number;
  sourceLinks: VizSankeyLink[];
  targetLinks: VizSankeyLink[];
};

/** 🌊 One Sankey link after layout: its endpoints, its value and the ribbon it occupies. */
export type VizSankeyLink = { source: VizSankeyNode; target: VizSankeyNode; value: number; index: number; width: number; y0: number; y1: number };

/** 🌊 The alignments `\SemioVizLayout{sankey}` accepts. */
export const VIZ_SANKEY_ALIGNMENTS = ["left", "right", "center", "justify"] as const;

export type VizSankeyAlignment = (typeof VIZ_SANKEY_ALIGNMENTS)[number];

/** 🌊 Options of `\SemioVizLayout{sankey}`; the defaults are d3-sankey's. */
export type VizSankeyOptions = {
  readonly extent?: readonly [readonly [number, number], readonly [number, number]];
  readonly nodeWidth?: number;
  readonly nodePadding?: number;
  readonly align?: VizSankeyAlignment;
  readonly iterations?: number;
};

/** 🌊 The Sankey result: the same node and link objects, positioned. */
export type VizSankeyLayout = { readonly nodes: VizSankeyNode[]; readonly links: VizSankeyLink[] };
//#endregion 🔖️Model

//#region 🔖️Sankey
function ascendingBreadth(a: VizSankeyNode, b: VizSankeyNode): number {
  return a.y0 - b.y0;
}

function ascendingSourceBreadth(a: VizSankeyLink, b: VizSankeyLink): number {
  return ascendingBreadth(a.source, b.source) || a.index - b.index;
}

function ascendingTargetBreadth(a: VizSankeyLink, b: VizSankeyLink): number {
  return ascendingBreadth(a.target, b.target) || a.index - b.index;
}

function alignmentOf(align: VizSankeyAlignment): (node: VizSankeyNode, n: number) => number {
  switch (align) {
    case "left":
      return (node) => node.depth;
    case "right":
      return (node, n) => n - 1 - node.height;
    case "center":
      return (node) => (node.targetLinks.length > 0 ? node.depth : node.sourceLinks.length > 0 ? Math.min(...node.sourceLinks.map((link) => link.target.depth)) - 1 : 0);
    default:
      return (node, n) => (node.sourceLinks.length > 0 ? node.depth : n - 1);
  }
}

/** 🌊 `\SemioVizLayout{sankey}`: positions nodes in layers and links as ribbons between them. */
export function vizSankey(input: { nodes: readonly { name: string; fixedValue?: number }[]; links: readonly { source: string | number; target: string | number; value: number }[] }, options: VizSankeyOptions = {}): VizSankeyLayout {
  const [[x0, y0], [x1, y1]] = options.extent ?? [
    [0, 0],
    [1, 1],
  ];
  const dx = options.nodeWidth ?? 24;
  const dy = options.nodePadding ?? 8;
  const iterations = options.iterations ?? 6;
  const align = alignmentOf(options.align ?? "justify");
  const nodes: VizSankeyNode[] = input.nodes.map((entry, i) => ({ name: entry.name, index: i, depth: 0, height: 0, layer: 0, value: 0, fixedValue: entry.fixedValue, x0: 0, x1: 0, y0: 0, y1: 0, sourceLinks: [], targetLinks: [] }));
  const byName = new Map(nodes.map((node) => [node.name, node] as const));
  const links: VizSankeyLink[] = input.links.map((entry, i) => {
    const source = typeof entry.source === "number" ? nodes[entry.source]! : byName.get(entry.source)!;
    const target = typeof entry.target === "number" ? nodes[entry.target]! : byName.get(entry.target)!;
    const link: VizSankeyLink = { source, target, value: entry.value, index: i, width: 0, y0: 0, y1: 0 };
    source.sourceLinks.push(link);
    target.targetLinks.push(link);
    return link;
  });
  for (const node of nodes) {
    const outgoing = node.sourceLinks.reduce((total, link) => total + link.value, 0);
    const incoming = node.targetLinks.reduce((total, link) => total + link.value, 0);
    node.value = node.fixedValue === undefined ? Math.max(outgoing, incoming) : node.fixedValue;
  }
  const n = nodes.length;
  let current = new Set(nodes);
  let depth = 0;
  while (current.size > 0) {
    const next = new Set<VizSankeyNode>();
    for (const node of current) {
      node.depth = depth;
      for (const link of node.sourceLinks) next.add(link.target);
    }
    depth += 1;
    if (depth > n) throw new Error("circular link");
    current = next;
  }
  current = new Set(nodes);
  let height = 0;
  while (current.size > 0) {
    const next = new Set<VizSankeyNode>();
    for (const node of current) {
      node.height = height;
      for (const link of node.targetLinks) next.add(link.source);
    }
    height += 1;
    if (height > n) throw new Error("circular link");
    current = next;
  }
  const layerCount = Math.max(...nodes.map((node) => node.depth)) + 1;
  const kx = (x1 - x0 - dx) / (layerCount - 1);
  const columns: VizSankeyNode[][] = Array.from({ length: layerCount }, () => []);
  for (const node of nodes) {
    const i = Math.max(0, Math.min(layerCount - 1, Math.floor(align(node, layerCount))));
    node.layer = i;
    node.x0 = x0 + i * kx;
    node.x1 = node.x0 + dx;
    columns[i]!.push(node);
  }
  const py = Math.min(dy, (y1 - y0) / (Math.max(...columns.map((column) => column.length)) - 1));
  const reorderLinks = (column: readonly VizSankeyNode[]): void => {
    for (const node of column) {
      node.sourceLinks.sort(ascendingTargetBreadth);
      node.targetLinks.sort(ascendingSourceBreadth);
    }
  };
  const reorderNodeLinks = (node: VizSankeyNode): void => {
    for (const link of node.targetLinks) link.source.sourceLinks.sort(ascendingTargetBreadth);
    for (const link of node.sourceLinks) link.target.targetLinks.sort(ascendingSourceBreadth);
  };
  const ky = Math.min(...columns.map((column) => (y1 - y0 - (column.length - 1) * py) / column.reduce((total, node) => total + node.value, 0)));
  for (const column of columns) {
    let y = y0;
    for (const node of column) {
      node.y0 = y;
      node.y1 = y + node.value * ky;
      y = node.y1 + py;
      for (const link of node.sourceLinks) link.width = link.value * ky;
    }
    const shift = (y1 - y + py) / (column.length + 1);
    column.forEach((node, i) => {
      node.y0 += shift * (i + 1);
      node.y1 += shift * (i + 1);
    });
    reorderLinks(column);
  }
  const targetTop = (source: VizSankeyNode, target: VizSankeyNode): number => {
    let y = source.y0 - ((source.sourceLinks.length - 1) * py) / 2;
    for (const link of source.sourceLinks) {
      if (link.target === target) break;
      y += link.width + py;
    }
    for (const link of target.targetLinks) {
      if (link.source === source) break;
      y -= link.width;
    }
    return y;
  };
  const sourceTop = (source: VizSankeyNode, target: VizSankeyNode): number => {
    let y = target.y0 - ((target.targetLinks.length - 1) * py) / 2;
    for (const link of target.targetLinks) {
      if (link.source === source) break;
      y += link.width + py;
    }
    for (const link of source.sourceLinks) {
      if (link.target === target) break;
      y -= link.width;
    }
    return y;
  };
  const resolveCollisionsTopToBottom = (column: VizSankeyNode[], yIn: number, from: number, alpha: number): void => {
    let y = yIn;
    for (let i = from; i < column.length; i += 1) {
      const node = column[i]!;
      const shift = (y - node.y0) * alpha;
      if (shift > 1e-6) {
        node.y0 += shift;
        node.y1 += shift;
      }
      y = node.y1 + py;
    }
  };
  const resolveCollisionsBottomToTop = (column: VizSankeyNode[], yIn: number, from: number, alpha: number): void => {
    let y = yIn;
    for (let i = from; i >= 0; i -= 1) {
      const node = column[i]!;
      const shift = (node.y1 - y) * alpha;
      if (shift > 1e-6) {
        node.y0 -= shift;
        node.y1 -= shift;
      }
      y = node.y0 - py;
    }
  };
  const resolveCollisions = (column: VizSankeyNode[], alpha: number): void => {
    const i = column.length >> 1;
    const subject = column[i]!;
    resolveCollisionsBottomToTop(column, subject.y0 - py, i - 1, alpha);
    resolveCollisionsTopToBottom(column, subject.y1 + py, i + 1, alpha);
    resolveCollisionsBottomToTop(column, y1, column.length - 1, alpha);
    resolveCollisionsTopToBottom(column, y0, 0, alpha);
  };
  for (let i = 0; i < iterations; i += 1) {
    const alpha = 0.99 ** i;
    const beta = Math.max(1 - alpha, (i + 1) / iterations);
    for (let c = columns.length - 2; c >= 0; c -= 1) {
      const column = columns[c]!;
      for (const source of column) {
        let y = 0;
        let w = 0;
        for (const link of source.sourceLinks) {
          const v = link.value * (link.target.layer - source.layer);
          y += sourceTop(source, link.target) * v;
          w += v;
        }
        if (!(w > 0)) continue;
        const shift = (y / w - source.y0) * alpha;
        source.y0 += shift;
        source.y1 += shift;
        reorderNodeLinks(source);
      }
      column.sort(ascendingBreadth);
      resolveCollisions(column, beta);
    }
    for (let c = 1; c < columns.length; c += 1) {
      const column = columns[c]!;
      for (const target of column) {
        let y = 0;
        let w = 0;
        for (const link of target.targetLinks) {
          const v = link.value * (target.layer - link.source.layer);
          y += targetTop(link.source, target) * v;
          w += v;
        }
        if (!(w > 0)) continue;
        const shift = (y / w - target.y0) * alpha;
        target.y0 += shift;
        target.y1 += shift;
        reorderNodeLinks(target);
      }
      column.sort(ascendingBreadth);
      resolveCollisions(column, beta);
    }
  }
  for (const node of nodes) {
    let sy = node.y0;
    let ty = node.y0;
    for (const link of node.sourceLinks) {
      link.y0 = sy + link.width / 2;
      sy += link.width;
    }
    for (const link of node.targetLinks) {
      link.y1 = ty + link.width / 2;
      ty += link.width;
    }
  }
  return { nodes, links };
}

/** 🌊 The horizontal ribbon centreline of one Sankey link, ready to be stroked at `link.width`. */
export function vizSankeyLinkHorizontal(link: VizSankeyLink, context?: VizPathContext): VizPathCommand[] {
  const recorder = context === undefined ? vizPathRecorder() : undefined;
  const sink = context ?? recorder!;
  const curve = curveBumpX(sink);
  curve.lineStart();
  curve.point(link.source.x1, link.y0);
  curve.point(link.target.x0, link.y1);
  curve.lineEnd();
  return recorder?.commands ?? [];
}
//#endregion 🔖️Sankey

//#region 🔖️Alluvial
/** 🏞️ One observation moving through the ordered stages of an alluvial diagram. */
export type VizAlluvialRow = { readonly stages: readonly string[]; readonly value: number };

/** 🏞️ `\SemioVizLayout{alluvial}`: a Sankey over ordered categorical stages, where a category may
 * recur in several stages and is therefore keyed by `stage/category`. */
export function vizAlluvial(rows: readonly VizAlluvialRow[], options: VizSankeyOptions & { stageNames?: readonly string[] } = {}): VizSankeyLayout & { readonly stages: readonly (readonly string[])[] } {
  const depth = Math.max(0, ...rows.map((row) => row.stages.length));
  const stages: string[][] = Array.from({ length: depth }, () => []);
  const nodeNames: string[] = [];
  const key = (stage: number, category: string): string => `${options.stageNames?.[stage] ?? stage}/${category}`;
  for (const row of rows) {
    row.stages.forEach((category, stage) => {
      const name = key(stage, category);
      if (!stages[stage]!.includes(name)) {
        stages[stage]!.push(name);
        nodeNames.push(name);
      }
    });
  }
  const flows = new Map<string, number>();
  for (const row of rows) {
    for (let stage = 0; stage + 1 < row.stages.length; stage += 1) {
      const edge = `${key(stage, row.stages[stage]!)} ${key(stage + 1, row.stages[stage + 1]!)}`;
      flows.set(edge, (flows.get(edge) ?? 0) + row.value);
    }
  }
  const layout = vizSankey(
    {
      nodes: nodeNames.map((name) => ({ name })),
      links: [...flows.entries()].map(([edge, value]) => {
        const [source, target] = edge.split(" ");
        return { source: source!, target: target!, value };
      }),
    },
    options,
  );
  return { ...layout, stages };
}
//#endregion 🔖️Alluvial
