/** 🎭️ The cast exporter: figure-only frames, frame counts, mirror, thermal clips and timelines, one stable viewBox, the manifest schema (ajv) and the output re-read with jsdom.
 *
 * @see ../../🧫️fixtures/🎭️cast-export/🔣️.json — the language-agnostic vectors
 * @see ../../🔨️modules/📤️export/🎭️cast/🟦️.ts — the exporter
 */
import { spawn, spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import { JSDOM } from "jsdom";
import { afterAll, beforeAll, describe, expect, it } from "vitest";
import { castManifestIssues, type CastIndex, type CastManifest } from "../../🔨️modules/📤️export/🎭️cast/🧬️schema/🟦️.ts";
import { exportCast, isEntry, parseArguments, processAlive, watchParent } from "../../🔨️modules/📤️export/🎭️cast/🟦️.ts";
import { loadCatalogFromFiles, loadEnvironments } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { STAFFAGE_ROOT, fixture, withScratch } from "../🧰️support/🧰️.ts";

type Expected = { asset: string; view: string; frames: number; fps: number; durationS: number; loop: boolean; mmPerMetre: number };
type Vectors = { manifest: CastManifest; inks: Record<"dark" | "light", string>; expected: Record<string, Expected>; mirrorOf: Record<string, string>; viewpoints: Record<string, string>; viewpointsDiffer: [string, string][]; identicalOutput: Record<string, string>; refused: { name: string; cast: unknown; message: string }[]; invalid: { name: string; cast: unknown; issue: string }[] };

const vectors = fixture<Vectors>("🎭️cast-export");
const schema = JSON.parse(readFileSync(join(STAFFAGE_ROOT, "🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json"), "utf8"));
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
const ids = Object.keys(vectors.manifest.casts);
const read = (root: string, id: string): CastIndex => JSON.parse(readFileSync(join(root, id, "🔣️.json"), "utf8")) as CastIndex;
const svgOf = (root: string, id: string, name: string): string => readFileSync(join(root, id, name), "utf8");

describe("🎭️ cast manifest schema", () => {
  it("accepts the fixture manifest in ajv and in the twin", () => {
    expect(validate(vectors.manifest), JSON.stringify(validate.errors?.slice(0, 3))).toBe(true);
    expect(castManifestIssues(vectors.manifest)).toEqual([]);
  });

  for (const { name, cast, issue } of vectors.invalid)
    it(`rejects ${name} in ajv and in the twin`, () => {
      const manifest = { version: 1, casts: { bad: cast } };
      expect(validate(manifest)).toBe(false);
      expect(castManifestIssues(manifest).join("\n")).toContain(issue);
    });

  it("accepts the course cast manifest in ajv and in the twin, with every asset, pose, clip and preset in the catalog", async () => {
    const course = JSON.parse(readFileSync(join(STAFFAGE_ROOT, "../⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json"), "utf8")) as CastManifest;
    expect(validate(course), JSON.stringify(validate.errors?.slice(0, 3))).toBe(true);
    expect(castManifestIssues(course)).toEqual([]);
    const catalog = await loadCatalogFromFiles(join(STAFFAGE_ROOT, "🗂️catalog"));
    const presets = new Set(((await loadEnvironments(join(STAFFAGE_ROOT, "🗂️catalog")))?.presets ?? []).map((preset) => preset.id));
    for (const [id, cast] of Object.entries(course.casts)) {
      const asset = catalog.assets.find((candidate) => candidate.id === cast.asset);
      expect(asset, `${id}: asset ${cast.asset}`).toBeDefined();
      if (cast.pose !== undefined) expect(asset!.poses.map((pose) => pose.id), `${id}: pose`).toContain(cast.pose);
      if (cast.clip !== undefined) expect(asset!.clips.map((clip) => clip.id), `${id}: clip`).toContain(cast.clip);
      for (const slug of [cast.environment, ...(cast.timeline ?? []).map((key) => key.environment)]) if (slug !== undefined) expect(presets.has(slug), `${id}: preset ${slug}`).toBe(true);
    }
  });

  it("rejects a cast id that is not a slug and a wrong version", () => {
    const manifest = { version: 2, casts: { "Bad Id": vectors.manifest.casts["radiator-rated"] } };
    expect(validate(manifest)).toBe(false);
    expect(castManifestIssues(manifest).length).toBeGreaterThanOrEqual(2);
  });
});

describe("🎭️ cast export", { timeout: 60_000 }, () => {
  let root: string;
  let indexes: readonly CastIndex[];

  beforeAll(async () => {
    root = mkdtempSync(join(tmpdir(), "staffage-cast-"));
    indexes = await exportCast(vectors.manifest, ids, root);
  }, 120_000);

  afterAll(() => rmSync(root, { recursive: true, force: true }));

  it("writes the fixture index fields and one svg per frame", () => {
    for (const id of ids) {
      const expected = vectors.expected[id]!;
      const index = read(root, id);
      expect(index).toMatchObject({ version: 1, cast: id, asset: expected.asset, view: expected.view, viewpoint: vectors.manifest.casts[id]!.viewpoint, fps: expected.fps, loop: expected.loop, mmPerMetre: expected.mmPerMetre });
      expect(index.durationS).toBeCloseTo(expected.durationS, 9);
      expect(index.frames).toHaveLength(expected.frames);
      expect(index.frames[0]).toBe("0000.svg");
      expect(readdirSync(join(root, id)).sort()).toEqual(["🔣️.json", ...new Set(index.frames)].sort());
      expect(indexes.find((candidate) => candidate.cast === id)).toEqual(index);
      expect(index.heightM).toBeGreaterThan(0.3);
    }
  });

  it("gives a clip durationS × fps frames", () => {
    const walker = read(root, "walker-line");
    expect(walker.frames).toHaveLength(Math.round(walker.durationS * walker.fps));
  });

  it("thins a long transient to 600 evenly spaced samples and writes repeated frames once", async () => {
    const out = mkdtempSync(join(tmpdir(), "staffage-thin-"));
    try {
      const manifest = { version: 1, casts: { toast: { asset: "toaster", clip: "toast-cycle", view: "thermal", environment: "indoor-heated-winter", theme: "dark", viewpoint: "elevation" } } } as unknown as CastManifest;
      const [index] = await exportCast(manifest, ["toast"], out);
      expect(index!.frames).toHaveLength(600);
      expect(index!.fps).toBeCloseTo(599 / index!.durationS, 9);
      expect(readdirSync(join(out, "toast")).length).toBe(new Set(index!.frames).size + 1);
    } finally {
      rmSync(out, { recursive: true, force: true });
    }
  }, 300_000);

  it("thins a long pose timeline to 600 samples that end exactly on the last key time", async () => {
    const out = mkdtempSync(join(tmpdir(), "staffage-thin-timeline-"));
    try {
      const manifest = { version: 1, casts: { slow: { asset: "panel-radiator", pose: "comfort", view: "thermal", theme: "dark", viewpoint: "elevation", fps: 15, timeline: [{ t: 0, environment: "office-neutral" }, { t: 100, environment: "summer-hot-sun" }] } } } as unknown as CastManifest;
      const [index] = await exportCast(manifest, ["slow"], out);
      expect(index!.durationS).toBe(100);
      expect(index!.loop).toBe(false);
      expect(index!.frames).toHaveLength(600);
      expect(index!.fps).toBeCloseTo(599 / 100, 9);
      expect(((index!.frames.length - 1) / index!.fps)).toBeCloseTo(100, 9);
    } finally {
      rmSync(out, { recursive: true, force: true });
    }
  }, 300_000);

  it("writes the same frames for a timeline given out of order", () => {
    for (const [unsorted, sorted] of Object.entries(vectors.identicalOutput)) {
      const a = read(root, unsorted);
      const b = read(root, sorted);
      expect({ ...a, cast: "" }).toEqual({ ...b, cast: "" });
      for (const name of new Set(a.frames)) expect(svgOf(root, unsorted, name)).toBe(svgOf(root, sorted, name));
    }
  });

  it("draws the front and the plan viewpoint, not the elevation", () => {
    for (const [id, viewpoint] of Object.entries(vectors.viewpoints)) expect(vectors.manifest.casts[id]!.viewpoint, id).toBe(viewpoint);
    for (const [a, b] of vectors.viewpointsDiffer) expect(svgOf(root, a, "0000.svg"), `${a} against ${b}`).not.toBe(svgOf(root, b, "0000.svg"));
    const plan = read(root, "commuter-plan");
    const elevation = read(root, "walker-line");
    expect(plan.heightM, "heightM is the physical figure height whatever the viewpoint").toBeCloseTo(elevation.heightM, 5);
    const depth = plan.boundsM.maxY - plan.boundsM.minY;
    expect(depth).toBeLessThan(elevation.heightM / 2);
    expect(depth).toBeGreaterThan(0.4);
  });

  it("lets a plan clip move: the limbs stretch (followTip), so the first and the passing frame differ", () => {
    const frames = read(root, "commuter-plan").frames;
    expect(new Set(frames).size).toBeGreaterThan(1);
    expect(svgOf(root, "commuter-plan", frames[0]!)).not.toBe(svgOf(root, "commuter-plan", frames[Math.floor(frames.length / 2)]!));
  });

  it("keeps one viewBox and one origin alignment across all frames of every cast", () => {
    for (const id of ids) {
      const index = read(root, id);
      const boxes = new Set(index.frames.map((name) => new JSDOM(svgOf(root, id, name), { contentType: "image/svg+xml" }).window.document.documentElement.getAttribute("viewBox")));
      expect(boxes.size, id).toBe(1);
      const [, , width, height] = [...boxes][0]!.split(" ").map(Number);
      expect(index.originMm[0]).toBeGreaterThan(0);
      expect(index.originMm[0]).toBeLessThanOrEqual(width!);
      expect(index.originMm[1]).toBeLessThanOrEqual(height!);
      expect((index.boundsM.maxX - index.boundsM.minX) * index.mmPerMetre).toBeCloseTo(width!, 2);
      expect((index.boundsM.maxY - index.boundsM.minY) * index.mmPerMetre).toBeCloseTo(height!, 2);
    }
  });

  it("draws the figure only: no text, legend, title or annotation items and a transparent paper", () => {
    for (const id of ids) {
      const document = new JSDOM(svgOf(root, id, "0000.svg"), { contentType: "image/svg+xml" }).window.document;
      expect(document.querySelectorAll("text"), id).toHaveLength(0);
      expect(document.getElementById("staffage-legend")?.children.length ?? 0, id).toBe(0);
      expect(document.getElementById("staffage-annotations")?.children.length ?? 0, id).toBe(0);
      expect(document.getElementById("staffage-background"), id).toBeNull();
      expect(document.getElementById("staffage-cast"), id).not.toBeNull();
    }
  });

  it("mirrors about the world origin", () => {
    for (const [mirrored, plain] of Object.entries(vectors.mirrorOf)) {
      const a = read(root, plain);
      const b = read(root, mirrored);
      expect(b.boundsM.minX).toBeCloseTo(-a.boundsM.maxX, 5);
      expect(b.boundsM.maxX).toBeCloseTo(-a.boundsM.minX, 5);
      expect(b.boundsM.maxY).toBeCloseTo(a.boundsM.maxY, 6);
      expect(b.originMm[0] + a.originMm[0]).toBeCloseTo((a.boundsM.maxX - a.boundsM.minX) * a.mmPerMetre, 2);
      expect(svgOf(root, mirrored, "0000.svg")).toContain("matrix(-1 0 0 1");
      expect(svgOf(root, plain, "0000.svg")).not.toContain("matrix(-1 0 0 1");
    }
  });

  it("renders thermal clips and timelines with changing temperatures, not one steady picture", () => {
    for (const id of ["walker-thermal", "radiator-transient"]) {
      const index = read(root, id);
      expect(svgOf(root, id, index.frames[0]!)).not.toBe(svgOf(root, id, index.frames.at(-1)!));
    }
  });

  it("draws the analytic views in the ink of the cast theme", () => {
    const { inks } = vectors;
    const dark = svgOf(root, "radiator-thermal-dark", "0000.svg");
    const light = svgOf(root, "radiator-thermal-light", "0000.svg");
    expect(dark).toContain(`"${inks.dark}"`);
    expect(dark).not.toContain(`stroke="${inks.light}"`);
    expect(light).toContain(`stroke="${inks.light}"`);
    expect(light).not.toContain(`stroke="${inks.dark}"`);
  });

  it("draws breathing plumes figure-only: more parcels, no emitter markers, labels or text", () => {
    const count = (svg: string): number => new JSDOM(svg, { contentType: "image/svg+xml" }).window.document.querySelectorAll("path").length;
    const frames = read(root, "breather-plume").frames;
    const plain = frames.map((name) => count(svgOf(root, "breather-plain", name)));
    const plume = frames.map((name) => count(svgOf(root, "breather-plume", name)));
    expect(Math.max(...plume)).toBeGreaterThan(Math.max(...plain));
    for (const name of frames) {
      const svg = svgOf(root, "breather-plume", name);
      expect(svg).not.toContain('id="emitter');
      expect(svg).not.toContain("<text");
    }
  });

  it("develops the air before the first frame of a looping transport cast, bounds the figure alone and thickens the pathlines", async () => {
    const out = mkdtempSync(join(tmpdir(), "staffage-air-"));
    try {
      const base = { asset: "standing-adult", clip: "idle-breathing", view: "thermal", environment: "office-still", theme: "dark", viewpoint: "elevation", transport: ["pathlines"], fps: 2 };
      const manifest = { version: 1, casts: { cold: base, warm: { ...base, transportAgeS: 5, transportStrokeMm: 0.3 } } } as unknown as CastManifest;
      await exportCast(manifest, ["cold", "warm"], out);
      const lines = (id: string, frame: number): string[] => [...new JSDOM(svgOf(out, id, read(out, id).frames[frame]!), { contentType: "image/svg+xml" }).window.document.querySelectorAll("g[id^='pathline']")].map((g) => g.querySelector("path")?.getAttribute("stroke-width") ?? "");
      expect(lines("cold", 0)).toHaveLength(0);
      expect(lines("warm", 0).length).toBeGreaterThan(20);
      expect(new Set(lines("warm", 0))).toEqual(new Set(["0.3"]));
      const warm = read(out, "warm");
      expect(warm.figureBoundsM!.maxY).toBeLessThan(warm.boundsM.maxY - 0.3);
      expect(warm.figureBoundsM!.maxY).toBeCloseTo(warm.heightM, 1);
      expect(read(out, "cold").figureBoundsM).toBeDefined();
    } finally {
      rmSync(out, { recursive: true, force: true });
    }
  }, 120_000);

  it("is language-independent: no frame of any cast contains text", () => {
    for (const id of ids) for (const name of new Set(read(root, id).frames)) {
      const svg = svgOf(root, id, name);
      expect(svg, `${id}/${name}`).not.toContain("<text");
      expect(svg, `${id}/${name}`).not.toContain("<tspan");
    }
  });

  it("is deterministic", async () => {
    await withScratch(async (directory) => {
      await exportCast(vectors.manifest, ["radiator-rated"], directory, { });
      expect(svgOf(directory, "radiator-rated", "0000.svg")).toBe(svgOf(root, "radiator-rated", "0000.svg"));
    });
  });

  it("never writes a NaN into a frame of a thermal clip with a coupled wall", async () => {
    const manifest = { version: 1, casts: { kettle: { asset: "electric-kettle", clip: "boil", view: "thermal", environment: "office-neutral", theme: "dark", viewpoint: "elevation", thermalSpanK: 78 } } } as unknown as CastManifest;
    await withScratch(async (directory) => {
      const [index] = await exportCast(manifest, ["kettle"], directory);
      expect(index!.frames.length).toBeGreaterThan(1);
      for (const name of readdirSync(join(directory, "kettle")).filter((file) => file.endsWith(".svg"))) expect(svgOf(directory, "kettle", name)).not.toContain("NaN");
    });
  });
});

describe("🎭️ cast command", () => {
  it("parses the contract invocation", () => {
    const parsed = parseArguments(["--manifest", "m.json", "--ids", "a,b", "--out", "o"]);
    expect(parsed.ids).toEqual(["a", "b"]);
    expect(parsed).not.toHaveProperty("language");
    expect(() => parseArguments(["--manifest", "m.json", "--out", "o"])).not.toThrow();
    expect(() => parseArguments(["--manifest", "m.json"])).toThrow(/--out/u);
  });

  it("knows no language: --lang is refused", () => {
    expect(() => parseArguments(["--manifest", "m.json", "--out", "o", "--lang", "de"])).toThrow(/--lang/u);
  });

  it("stops on an aborted signal and leaves nothing behind", async () => {
    await withScratch(async (directory) => {
      const controller = new AbortController();
      controller.abort();
      await expect(exportCast(vectors.manifest, ["walker-line"], directory, { signal: controller.signal })).rejects.toThrow();
      expect(existsSync(join(directory, "walker-line"))).toBe(false);
    });
  });

  for (const { name, cast, message } of vectors.refused)
    it(`refuses ${name} before writing anything, even after valid casts`, async () => {
      await withScratch(async (directory) => {
        const manifest = { version: 1, casts: { fine: vectors.manifest.casts["radiator-rated"], bad: cast } } as unknown as CastManifest;
        await expect(exportCast(manifest, ["fine", "bad"], directory)).rejects.toThrow(message);
        expect(readdirSync(directory)).toEqual([]);
      });
    });

  it("refuses unknown ids before writing", async () => {
    await withScratch(async (directory) => {
      await expect(exportCast(vectors.manifest, ["nobody"], directory)).rejects.toThrow(/unknown cast ids/u);
      expect(readdirSync(directory)).toEqual([]);
    });
  });
});

type Cancellation = { manifest: CastManifest; completeIds: string[]; slowIds: string[]; slowFrames: number; cancelAfterFrames: number; cancelledExitCode: number; linkedEntry: { ids: string[]; mustReport: string; mustWrite: string }; progress: Record<"pid" | "cast" | "frames" | "cancelled", string>; expectedProgress: Record<string, string[]> };

const cancellation = fixture<Cancellation>("🎭️cast-cancellation");
const listing = (directory: string): string[] => readdirSync(directory, { recursive: true, encoding: "utf8" }).sort();
const cli = join(STAFFAGE_ROOT, "🔨️modules/📤️export/🎭️cast/🟦️.ts");
const bun = process.versions.bun ? process.execPath : "bun";
const hasBun = spawnSync(bun, ["--version"], { encoding: "utf8" }).status === 0;
const requireBun = (): void => {
  if (!hasBun) throw new Error(`Bun is provisioned by repo, but \`${bun} --version\` failed: fix the environment (repo tools setup) instead of skipping this case.`);
};

describe("🛑 cast export cancellation and progress", () => {
  it("stops between two frames, removes its temporary folder and keeps the finished casts untouched", async () => {
    await withScratch(async (directory) => {
      await exportCast(cancellation.manifest, cancellation.completeIds, directory);
      const before = listing(directory);
      const bytes = readFileSync(join(directory, "quick", "🔣️.json"), "utf8");
      const controller = new AbortController();
      const started: string[] = [];
      let frames = 0;
      await expect(
        exportCast(cancellation.manifest, cancellation.slowIds, directory, {
          signal: controller.signal,
          started: (id) => started.push(id),
          progress: () => {
            if (++frames === cancellation.cancelAfterFrames) controller.abort();
          },
        }),
      ).rejects.toThrow();
      expect(frames).toBe(cancellation.cancelAfterFrames);
      expect(started).toEqual([cancellation.slowIds[0]]);
      expect(listing(directory)).toEqual(before);
      expect(readFileSync(join(directory, "quick", "🔣️.json"), "utf8")).toBe(bytes);
    });
  });

  it("never replaces a published cast when the signal fires after rendering", async () => {
    await withScratch(async (directory) => {
      const controller = new AbortController();
      await expect(exportCast(cancellation.manifest, cancellation.completeIds, directory, { signal: controller.signal, progress: (done, total) => { if (done === total) controller.abort(); } })).rejects.toThrow();
      expect(listing(directory)).toEqual([]);
    });
  });

  it("reports the process, every cast and the frames on stderr and exits 0", { timeout: 180_000 }, async () => {
    requireBun();
    await withScratch(async (directory) => {
      const manifest = join(directory, "manifest.json");
      writeFileSync(manifest, JSON.stringify(cancellation.manifest));
      const run = spawnSync(bun, [cli, "--manifest", manifest, "--ids", cancellation.completeIds.join(","), "--out", join(directory, "out")], { encoding: "utf8" });
      expect(run.status, run.stderr).toBe(0);
      const lines = run.stderr.split(/\r?\n/u);
      expect(lines.some((line) => new RegExp(cancellation.progress.pid, "u").test(line))).toBe(true);
      for (const [id, expected] of Object.entries(cancellation.expectedProgress))
        for (const line of expected) expect(lines.some((candidate) => candidate.startsWith(line)), `${id}: ${line}`).toBe(true);
      expect(listing(join(directory, "out"))).toEqual(["quick", join("quick", "0000.svg"), join("quick", "🔣️.json")]);
    });
  });

  it.skipIf(process.platform === "win32")("exits 130 on SIGTERM after the first frames and leaves an empty output folder", { timeout: 180_000 }, async () => {
    requireBun();
    await withScratch(async (directory) => {
      const manifest = join(directory, "manifest.json");
      writeFileSync(manifest, JSON.stringify(cancellation.manifest));
      const child = spawn(bun, [cli, "--manifest", manifest, "--ids", cancellation.slowIds.join(","), "--out", join(directory, "out")]);
      let stderr = "";
      let sent = false;
      child.stderr.on("data", (chunk: Buffer) => {
        stderr += chunk.toString("utf8");
        if (!sent && new RegExp(cancellation.progress.cast, "mu").test(stderr)) {
          sent = true;
          setTimeout(() => child.kill("SIGTERM"), 300);
        }
      });
      const code = await new Promise<number | null>((done) => child.on("close", done));
      expect(code, stderr).toBe(cancellation.cancelledExitCode);
      expect(new RegExp(cancellation.progress.cancelled, "mu").test(stderr)).toBe(true);
      expect(listing(join(directory, "out"))).toEqual([]);
    });
  });

  it("runs as the program when it is started through a linked folder, and as nothing when it is imported", { timeout: 180_000 }, async () => {
    requireBun();
    expect(isEntry(pathToFileURL(cli).href, cli)).toBe(true);
    expect(isEntry(pathToFileURL(cli).href, undefined)).toBe(false);
    expect(isEntry(pathToFileURL(cli).href, join(STAFFAGE_ROOT, "not-the-exporter.ts"))).toBe(false);
    await withScratch(async (directory) => {
      const manifest = join(directory, "manifest.json");
      writeFileSync(manifest, JSON.stringify(cancellation.manifest));
      const link = join(directory, "link");
      symlinkSync(STAFFAGE_ROOT, link, "junction");
      const linked = join(link, "🔨️modules/📤️export/🎭️cast/🟦️.ts");
      expect(isEntry(pathToFileURL(cli).href, linked)).toBe(true);
      const run = spawnSync(bun, [linked, "--manifest", manifest, "--ids", cancellation.linkedEntry.ids.join(","), "--out", join(directory, "out")], { encoding: "utf8" });
      expect(run.stderr).toContain(cancellation.linkedEntry.mustReport);
      expect(run.status, run.stderr).toBe(0);
      expect(existsSync(join(directory, "out", cancellation.linkedEntry.mustWrite))).toBe(true);
    });
  });

  it("parses --parent and refuses a bad process id", () => {
    const base = ["--manifest", "m.json", "--out", "o"];
    expect(parseArguments(base).parent).toBeUndefined();
    expect(parseArguments([...base, "--parent", "4242"]).parent).toBe(4242);
    for (const bad of ["abc", "0", "-3", "1.5"]) expect(() => parseArguments([...base, "--parent", bad])).toThrow(/--parent/u);
  });

  it("watches a parent: calls back once when it vanishes, never while it lives, and stops on request", async () => {
    let alive = true;
    let calls = 0;
    const stop = watchParent(1, () => calls++, 5, () => alive);
    await new Promise((done) => setTimeout(done, 40));
    expect(calls).toBe(0);
    alive = false;
    await new Promise((done) => setTimeout(done, 60));
    expect(calls).toBe(1);
    stop();
    let stopped = 0;
    const cancelled = watchParent(1, () => stopped++, 5, () => false);
    cancelled();
    await new Promise((done) => setTimeout(done, 40));
    expect(stopped).toBe(0);
    expect(processAlive(process.pid)).toBe(true);
  });

  it("exits 130 within seconds when the parent given by --parent is hard-killed, on every OS", { timeout: 180_000 }, async () => {
    requireBun();
    await withScratch(async (directory) => {
      const manifest = join(directory, "manifest.json");
      writeFileSync(manifest, JSON.stringify(cancellation.manifest));
      const parent = spawn(bun, ["-e", "setTimeout(() => {}, 120000)"], { stdio: "ignore" });
      const parentPid = parent.pid!;
      const child = spawn(bun, [cli, "--manifest", manifest, "--ids", cancellation.slowIds.join(","), "--out", join(directory, "out"), "--parent", String(parentPid)]);
      let stderr = "";
      let killedAt = 0;
      child.stderr.on("data", (chunk: Buffer) => {
        stderr += chunk.toString("utf8");
        if (killedAt === 0 && new RegExp(cancellation.progress.cast, "mu").test(stderr)) {
          killedAt = Date.now();
          setTimeout(() => parent.kill("SIGKILL"), 300);
        }
      });
      const code = await new Promise<number | null>((done) => child.on("close", done));
      expect(killedAt, stderr).toBeGreaterThan(0);
      expect(Date.now() - killedAt).toBeLessThan(30_000);
      expect(code, stderr).toBe(cancellation.cancelledExitCode);
      expect(new RegExp(cancellation.progress.cancelled, "mu").test(stderr)).toBe(true);
      expect(listing(join(directory, "out"))).toEqual([]);
    });
  });
});
