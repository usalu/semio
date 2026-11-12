/** 🚀 Export performance without changing the pictures: the linear exact distance transform against scipy and a brute-force oracle, the feature width against pinned values (regenerated from the corrected measure), the analytic widths and a shapely opening oracle,
 * the thermal session (shared history, shared integrations) against solving every frame alone, the pinned digests of exported frames, and cheap CPU and call-count guards that catch a return to the quadratic work.
 *
 * @see ../../🧫️fixtures/🚀️export-performance/🔣️.json — the language-agnostic vectors
 * @see ./🐍️.py — the scipy oracle of the distance vectors
 * @see ../../🔨️modules/🖨️representation/📐️measure/🟦️.ts — squaredDistanceTransform, featureWidthMm
 * @see ../../🔨️modules/🌡️thermal/🕰️timeline/🟦️.ts — solveThermalAt with a session
 */
import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset, Environment } from "../../🧬️schema/🟦️.ts";
import { exportCast } from "../../🔨️modules/📤️export/🎭️cast/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { figureState } from "../../🧫️fixtures/🧱️synthetic-figure/🟦️.ts";
import type { CastManifest } from "../../🔨️modules/📤️export/🎭️cast/🧬️schema/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles, loadEnvironments } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { type DistanceWindow, featureWidthMm, makeScratch, squaredDistanceTransform } from "../../🔨️modules/🖨️representation/📐️measure/🟦️.ts";
import { solveThermalAt, thermalSession } from "../../🔨️modules/🌡️thermal/🟦️.ts";
import type { EvaluationQuery } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";
requireTool("uv");

type Distance = { name: string; image: string[]; squared: number[][] | null };
type Shape = { name: string; rings: { x: number; y: number }[][]; minFeatureMm: number; analyticMm: number | null; referenceMm: number };
type Digests = Record<string, { frames: string[]; sha256: Record<string, string> }>;
type Vectors = { distance: Distance[]; featureWidth: Shape[]; castDigests: Digests };

const vectors = fixture<Vectors>("🚀️export-performance");
const castVectors = fixture<{ manifest: CastManifest }>("🎭️cast-export");
const INF = 1e20;
const engineIndependent = (svg: string): string => svg.replace(/"revision": "sha256:[0-9a-f]{64}"/gu, '"revision": "sha256:0"').replace(/data-value="([^"]*)"/gu, (_, v: string) => `data-value="${Number(Number(v).toPrecision(4))}"`);

const sourcesOf = (image: readonly string[]): { source: Uint8Array; nx: number; ny: number } => {
  const nx = image[0]!.length;
  const ny = image.length;
  const source = new Uint8Array(nx * ny);
  image.forEach((row, y) => [...row].forEach((cell, x) => (source[y * nx + x] = cell === "#" ? 1 : 0)));
  return { source, nx, ny };
};

function bruteForce(source: Uint8Array, nx: number, ny: number): number[] {
  const cells: [number, number][] = [];
  for (let i = 0; i < source.length; i++) if (source[i] === 1) cells.push([i % nx, Math.floor(i / nx)]);
  return Array.from({ length: nx * ny }, (_, i) => cells.reduce((best, [x, y]) => Math.min(best, (x - (i % nx)) ** 2 + (y - Math.floor(i / nx)) ** 2), INF));
}

function lcg(seed: number): () => number {
  let state = seed;
  return () => (state = (state * 1103515245 + 12345) % 2147483648) / 2147483648;
}

describe("🚀 exact distance transform", () => {
  for (const { name, image, squared } of vectors.distance)
    it(`equals scipy's Euclidean distance transform on ${name}`, () => {
      const { source, nx, ny } = sourcesOf(image);
      const grid = squaredDistanceTransform(source, nx, ny);
      if (squared === null) expect([...grid].every((value) => value === INF)).toBe(true);
      else expect([...grid]).toEqual(squared.flat());
    });

  it("equals brute force on seeded images of many shapes and densities, with a reused scratch", () => {
    const next = lcg(42);
    const scratch = makeScratch(40, 40);
    for (let round = 0; round < 60; round++) {
      const nx = 2 + Math.floor(next() * 38);
      const ny = 2 + Math.floor(next() * 38);
      const density = [0.002, 0.02, 0.2, 0.7][round % 4]!;
      const source = new Uint8Array(nx * ny);
      for (let i = 0; i < source.length; i++) source[i] = next() < density ? 1 : 0;
      const roomy = makeScratch(nx, ny);
      expect([...squaredDistanceTransform(source, nx, ny, roomy)], `round ${round}`).toEqual(bruteForce(source, nx, ny));
      if (nx * ny <= scratch.grid.length) expect([...squaredDistanceTransform(source, nx, ny, scratch).subarray(0, nx * ny)]).toEqual(bruteForce(source, nx, ny));
    }
  });

  it("is exact inside a window wherever the distance is within the cap, for the sparse and the dense sweep", () => {
    const next = lcg(7);
    for (let round = 0; round < 40; round++) {
      const nx = 12 + Math.floor(next() * 30);
      const ny = 12 + Math.floor(next() * 30);
      const mask = new Uint8Array(nx * ny);
      const cx = next() * nx;
      const cy = next() * ny;
      const radius = 3 + next() * 8;
      for (let i = 0; i < mask.length; i++) mask[i] = Math.hypot((i % nx) - cx, Math.floor(i / nx) - cy) < radius ? 1 : 0;
      const from = new Int32Array(ny).fill(nx);
      const to = new Int32Array(ny).fill(-1);
      for (let i = 0; i < mask.length; i++)
        if (mask[i] === 1) {
          const y = Math.floor(i / nx);
          const x = i % nx;
          from[y] = Math.min(from[y]!, x);
          to[y] = Math.max(to[y]!, x);
        }
      const source = new Uint8Array(nx * ny);
      for (let i = 0; i < mask.length; i++) source[i] = mask[i] === 1 && next() < 0.15 ? 1 : 0;
      if (!source.some((cell) => cell === 1)) continue;
      const cap = 2 + Math.floor(next() * 6);
      const truth = bruteForce(source, nx, ny);
      for (const sparse of [true, false]) {
        const window: DistanceWindow = { from, to, margin: cap, cap, sparse };
        const grid = squaredDistanceTransform(source, nx, ny, makeScratch(nx, ny), window);
        for (let i = 0; i < mask.length; i++) if (mask[i] === 1 && truth[i]! <= cap * cap) expect(grid[i], `round ${round} sparse ${sparse} cell ${i}`).toBe(truth[i]);
        for (let i = 0; i < mask.length; i++) if (mask[i] === 1 && truth[i]! > cap * cap) expect(grid[i]!, `round ${round} sparse ${sparse} cell ${i}`).toBeGreaterThan(cap * cap);
      }
    }
  });
});

describe("🚀 feature width", () => {
  it("stops at a proven lower bound that is at least the asked width and at most the exact width, never memoises it, and is exact below it", () => {
    for (const shape of vectors.featureWidth)
      for (const factor of [0.01, 0.3, 0.7, 1, 1.5]) {
        const atLeast = shape.referenceMm * factor;
        const bound = featureWidthMm(shape.rings, shape.minFeatureMm, atLeast);
        expect(bound).toBeLessThanOrEqual(shape.referenceMm);
        if (shape.referenceMm >= atLeast) expect(bound).toBeGreaterThanOrEqual(atLeast);
        else expect(bound).toBe(shape.referenceMm);
        expect(featureWidthMm(shape.rings, shape.minFeatureMm)).toBe(shape.referenceMm);
      }
  });

  for (const shape of vectors.featureWidth)
    it(`equals the pinned width on ${shape.name}${shape.analyticMm === null ? "" : ` and the analytic width ${shape.analyticMm} mm within a tenth`}`, () => {
      const width = featureWidthMm(shape.rings, shape.minFeatureMm);
      expect(width).toBe(shape.referenceMm);
      expect(featureWidthMm(shape.rings, shape.minFeatureMm)).toBe(shape.referenceMm);
      if (shape.analyticMm !== null) expect(Math.abs(width - shape.analyticMm)).toBeLessThan(0.1);
    });

  it("equals the shapely opening width of every shape within 3 % or 0.05 mm, the third-party oracle of the pinned values", { timeout: 300_000 }, () => {
    const directory = mkdtempSync(join(tmpdir(), "staffage-widths-"));
    try {
      const input = join(directory, "shapes.json");
      writeFileSync(input, JSON.stringify(vectors.featureWidth));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🚀️export-performance/🐍️.py"), "geometry", ["--widths", input])) as Record<string, number>;
      for (const shape of vectors.featureWidth) expect(Math.abs(shape.referenceMm - reference[shape.name]!), `${shape.name}: ${shape.referenceMm} against ${reference[shape.name]}`).toBeLessThanOrEqual(Math.max(0.03 * reference[shape.name]!, 0.05));
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });

  it("answers a repeated shape from the memo and never mixes shapes that differ by one coordinate", () => {
    const base = vectors.featureWidth[0]!;
    const moved = base.rings.map((ring) => ring.map((p) => ({ x: p.x, y: p.y * 1.5 })));
    expect(featureWidthMm(moved, base.minFeatureMm)).not.toBe(featureWidthMm(base.rings, base.minFeatureMm));
    expect(featureWidthMm(base.rings, base.minFeatureMm)).toBe(base.referenceMm);
  });
});

describe("🚀 thermal session", { timeout: 300_000 }, () => {
  const root = DEFAULT_CATALOG_ROOT;
  const setup = async (): Promise<{ readonly asset: (id: string) => Asset; readonly preset: (id: string) => Environment }> => {
    const catalog = await loadCatalogFromFiles(root);
    const presets = new Map(((await loadEnvironments(root))?.presets ?? []).map((p) => [p.id, p.environment]));
    return { asset: (id) => catalog.assets.find((a) => a.id === id)!, preset: (id) => presets.get(id)! };
  };
  const strip = (field: unknown): unknown => JSON.parse(JSON.stringify(field));

  it("returns bit-identical fields for frames solved in any order with a shared session, a looping clip with and without a window start", async () => {
    const { asset, preset } = await setup();
    const walker = asset("walking-commuter");
    const environment = preset("winter-overcast");
    const times = [0, 0.4, 1.1, 2.9, 7.3, 0.4, 400, 401.5, 399, 1200, 5.5];
    const shared = thermalSession();
    for (const timeS of times) {
      const query: EvaluationQuery = { kind: "clip", clip: "walk-cycle", timeS };
      expect(strip(solveThermalAt(walker, query, environment, { session: shared })), `walker t=${timeS}`).toEqual(strip(solveThermalAt(walker, query, environment)));
    }
  });

  it("returns bit-identical fields for a pose under an environment timeline", async () => {
    const { asset, preset } = await setup();
    const radiator = asset("panel-radiator");
    const timeline = { keys: [{ t: 0, environment: preset("office-neutral"), ease: "linear" as const }, { t: 100, environment: preset("summer-hot-sun"), ease: "linear" as const }] };
    const shared = thermalSession();
    for (const timeS of [0, 3, 3.5, 20, 60, 100, 41]) {
      const query = { kind: "pose", pose: "comfort", timeS } as unknown as EvaluationQuery;
      expect(strip(solveThermalAt(radiator, query, timeline, { session: shared })), `radiator t=${timeS}`).toEqual(strip(solveThermalAt(radiator, query, timeline)));
    }
  });

  it("evaluates the asset less often over a run of frames than solving every frame alone", async () => {
    const { asset, preset } = await setup();
    const walker = asset("walking-commuter");
    const environment = preset("winter-overcast");
    const frames = Array.from({ length: 12 }, (_, i) => i * 0.5);
    const counted = (session: ReturnType<typeof thermalSession> | undefined): number => {
      let calls = 0;
      const options = { evaluate: (a: Asset, q: EvaluationQuery) => (calls++, evaluate(a, q)), ...(session ? { session } : {}) };
      for (const timeS of frames) solveThermalAt(walker, { kind: "clip", clip: "walk-cycle", timeS }, environment, options);
      return calls;
    };
    const alone = counted(undefined);
    const shared = counted(thermalSession());
    expect(shared).toBeLessThan(alone * 0.8);
  });
});

describe("🚀 exported frames", { timeout: 300_000 }, () => {
  it("equal the pinned digests of the reference export (measured values compared to 4 significant digits, since V8 and JavaScriptCore differ in the last bit of exp and pow), with no helper files left behind", async () => {
    const out = mkdtempSync(join(tmpdir(), "staffage-perf-"));
    try {
      const ids = Object.keys(vectors.castDigests);
      const indexes = await exportCast(castVectors.manifest, ids, out);
      expect(indexes.map((index) => index.cast)).toEqual(ids);
      for (const index of indexes) {
        const pinned = vectors.castDigests[index.cast]!;
        expect(index.frames, index.cast).toEqual(pinned.frames);
        const files = readdirSync(join(out, index.cast)).sort();
        expect(files, index.cast).toEqual(Object.keys(pinned.sha256).sort());
        const boxes = new Set(files.filter((name) => name.endsWith(".svg")).map((name) => /<svg [^>]*width="([0-9.]+)mm" height="([0-9.]+)mm" viewBox="0 0 ([0-9.]+) ([0-9.]+)"/u.exec(readFileSync(join(out, index.cast, name), "utf8"))!.slice(1).join(" ")));
        expect(boxes.size, `${index.cast}: one shared size in millimetres, the viewBox equal to it`).toBe(1);
        const [width, height, viewWidth, viewHeight] = [...boxes][0]!.split(" ");
        expect([viewWidth, viewHeight], index.cast).toEqual([width, height]);
        for (const name of files) expect(createHash("sha256").update(engineIndependent(readFileSync(join(out, index.cast, name), "utf8"))).digest("hex"), `${index.cast}/${name}`).toBe(pinned.sha256[name]);
      }
      expect(readdirSync(out).sort()).toEqual([...ids].sort());
    } finally {
      rmSync(out, { recursive: true, force: true });
    }
  });

  it("round the measured values of the shapes to 6 significant digits", async () => {
    const out = mkdtempSync(join(tmpdir(), "staffage-perf-digits-"));
    try {
      await exportCast(castVectors.manifest, ["walker-thermal"], out);
      const svg = readFileSync(join(out, "walker-thermal", "0000.svg"), "utf8");
      const values = [...svg.matchAll(/data-value="([^"]*)"/gu)].map((match) => match[1]!);
      expect(values.length).toBeGreaterThan(5);
      for (const value of values) expect(value.replace(/^-?0?\.?0*/u, "").replace(/e.*$/u, "").replace(".", "").length, value).toBeLessThanOrEqual(6);
    } finally {
      rmSync(out, { recursive: true, force: true });
    }
  });
});

describe("🚀 cost guards", { timeout: 120_000 }, () => {
  const cpuSeconds = (body: () => void): number => {
    const before = process.cpuUsage();
    body();
    const used = process.cpuUsage(before);
    return (used.user + used.system) / 1e6;
  };

  it("transforms a 2.5 million cell raster in well under a second of CPU (budget 5 s; the linear sweep takes about 0.05 s)", () => {
    const nx = 1600;
    const ny = 1560;
    const source = new Uint8Array(nx * ny);
    for (let y = 600; y < 900; y++) for (let x = 500; x < 1100; x++) source[y * nx + x] = 1;
    const scratch = makeScratch(nx, ny);
    squaredDistanceTransform(source, nx, ny, scratch);
    expect(cpuSeconds(() => void squaredDistanceTransform(source, nx, ny, scratch))).toBeLessThan(5);
  });

  it("measures a shape that fills the raster cap in a few seconds of CPU (budget 30 s; the cap shape takes about 1 s)", () => {
    const shape = vectors.featureWidth.find((candidate) => candidate.name === "large-capped-raster")!;
    expect(cpuSeconds(() => void featureWidthMm(shape.rings.map((ring) => ring.map((p) => ({ x: p.x + 0.0001, y: p.y }))), shape.minFeatureMm))).toBeLessThan(30);
  });
});

describe("🚀 compact frame form", () => {
  const numbers = (svg: string): number[] => [...svg.matchAll(/ d="([^"]*)"/gu)].flatMap((match) => match[1]!.match(/-?\d*\.?\d+/gu)!.map(Number));
  const scene = depict(figureState(50), { ...defaultDepictionOptions("light-line", "en", 50), annotations: { dimensions: false, scaleBar: false, legend: false, labels: false, ground: false } });

  it("draws the same numbers with the presentation attributes on the layer groups and no ids on bare paths", () => {
    const plain = writeSvg(scene);
    const compact = writeSvg(scene, { compact: true });
    expect(numbers(compact)).toEqual(numbers(plain));
    expect(compact).toMatch(/<g id="staffage-[^"]*" inkscape:groupmode="layer"[^>]* fill-rule="evenodd" stroke-linecap="round" stroke-linejoin="round">/u);
    expect(/<path[^>]* stroke-line(cap|join)=/u.test(compact)).toBe(false);
    expect(/<path id=/u.test(compact)).toBe(false);
  });
});
