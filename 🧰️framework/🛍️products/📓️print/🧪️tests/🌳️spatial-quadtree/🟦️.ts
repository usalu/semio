/** 🌳️ Adapter for `spatial-quadtree`: the spatial index the collision layouts stand on.
 *
 * Subject: `semio-viz-spatial`'s quadtree, probed through `semio-viz-probe`.
 * Oracle: `d3-quadtree`, on the two things a quadtree makes observable — the square it covers and
 * the nearest point a query finds — plus the node count, which compares the trees' shape.
 */
import { quadtree } from "d3-quadtree";
import { type AdapterContext, defineTestAdapter } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { type ProbeProjection, type ProbeRecord, compileVizProbe, roundProbeNumbers } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";

//#region 🔖️Vectors
const CASE = "spatial-quadtree";
const FIXTURE = "local://spatial-quadtree.tex";
const DECIMALS = 6;
type Point = readonly [number, number];

/** 🥒️ The vector table of the running scenario, as one record per row. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, (row[index] ?? "").trim()])));
}

/** ✂️ A `;`-separated cell as a list of numbers. */
function list(cell: string): number[] {
  return cell.split(";").map((item) => Number(item.trim()));
}

/** 🌳️ The point set the fixture's `qt-scatter` table holds, in its own row order. */
const SCATTER: readonly Point[] = [
  [1, 1],
  [5, 2],
  [2, 7],
  [9, 9],
  [4, 4],
  [7.5, 3.25],
  [0.5, 8.5],
];

/** 🌳️ One d3 quadtree over the given points, built the way `d3.quadtree(data)` builds it. */
function tree(points: readonly Point[]) {
  return quadtree<Point>()
    .x((point) => point[0])
    .y((point) => point[1])
    .addAll([...points]);
}

/** 🔢️ How many nodes `visit` walks, which is the tree's shape reduced to one number. */
function nodeCount(built: ReturnType<typeof tree>): number {
  let nodes = 0;
  built.visit(() => {
    nodes += 1;
    return false;
  });
  return nodes;
}

/** 🔎️ The 1-based row index of the point a query finds, 0 when it finds none. */
function found(built: ReturnType<typeof tree>, points: readonly Point[], x: number, y: number, radius?: number): number {
  const hit = radius === undefined ? built.find(x, y) : built.find(x, y, radius);
  if (hit === undefined) return 0;
  return points.findIndex((point) => point[0] === hit[0] && point[1] === hit[1]) + 1;
}

/** 🎯️ Compiles the committed fixture and returns the records of one scenario. */
async function records(ctx: AdapterContext): Promise<ProbeRecord[]> {
  return roundProbeNumbers(await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id }), DECIMALS);
}

/** 🧮️ The probe's records under their own keys, geometry records keyed by their kind. */
function projectionOf(probed: readonly ProbeRecord[]): ProbeProjection {
  const projection: Record<string, (number | string)[]> = {};
  for (const record of probed) projection[record.key] = [...(projection[record.key] ?? []), ...record.values];
  return projection;
}
//#endregion 🔖️Vectors

//#region 🔖️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    cover: {
      /** 🔮️ The extent, the point count and the node count d3-quadtree reports for each table. */
      oracle: (ctx: AdapterContext) => {
        const extents: number[] = [];
        const sizes: number[] = [];
        for (const row of rows(ctx)) {
          const xs = list(row.x!);
          const ys = list(row.y!);
          const points: Point[] = xs.map((value, index) => [value, ys[index]!] as Point);
          const built = tree(points);
          const [[x0, y0], [x1, y1]] = built.extent()!;
          extents.push(x0, y0, x1, y1);
          sizes.push(built.size(), nodeCount(built));
        }
        return { projection: { "geometry/quadtree/extent": extents, "geometry/quadtree/size": sizes } };
      },
      /** 🎯️ The extent and the counts `\SemioVizQuadtree` emitted while building the same tables. */
      subject: async (ctx: AdapterContext) => ({ projection: projectionOf(await records(ctx)) }),
    },
    find: {
      /** 🔮️ The point d3-quadtree's `find` returns for each unbounded query. */
      oracle: (ctx: AdapterContext) => {
        const built = tree(SCATTER);
        const projection: Record<string, readonly number[]> = {};
        rows(ctx).forEach((row, index) => {
          projection[`find/${index}`] = [found(built, SCATTER, Number(row.x), Number(row.y))];
        });
        return { projection };
      },
      /** 🎯️ The row index `\SemioVizQuadtreeFind` answers for the same queries. */
      subject: async (ctx: AdapterContext) => ({ projection: projectionOf(await records(ctx)) }),
    },
    "find-radius": {
      /** 🔮️ The point d3-quadtree's `find` returns inside each search radius, or none. */
      oracle: (ctx: AdapterContext) => {
        const built = tree(SCATTER);
        const projection: Record<string, readonly number[]> = {};
        rows(ctx).forEach((row, index) => {
          projection[`radius/${index}`] = [found(built, SCATTER, Number(row.x), Number(row.y), Number(row.radius))];
        });
        return { projection };
      },
      /** 🎯️ The row index `\SemioVizQuadtreeFind` answers inside the same radii. */
      subject: async (ctx: AdapterContext) => ({ projection: projectionOf(await records(ctx)) }),
    },
  },
});
//#endregion 🔖️Adapter
