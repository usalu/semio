import { expect, test } from "bun:test";
import some from "lodash/some";
import { readFileSync } from "node:fs";
import { join, resolve, sep } from "node:path";
import { claimDirectoriesV1 } from "../../../🟦️.ts";
import { completedCargoClosureRosterV1 } from "../../📦️retention/📋️inventory/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const root = resolve(process.cwd(), "linear-fixture");
const absolute = (segments: readonly string[]): string => join(root, ...segments);

test("the claimed-directory set answers exactly what a scan of every claim answers, checked against lodash", () => {
  for (const vector of fixture.beneath) {
    const claims: string[] = vector.claims.map(absolute), directories = claimDirectoriesV1(claims);
    const scanned = vector.asked.map((segments: string[]) => some(claims, (claim: string) => claim.startsWith(absolute(segments) + sep)));
    expect(vector.asked.map((segments: string[]) => directories.has(absolute(segments)))).toEqual(vector.expected);
    expect(scanned).toEqual(vector.expected);
  }
});

function best(run: () => void, repeats = 3): number {
  let fastest = Infinity;
  for (let attempt = 0; attempt < repeats; attempt++) {
    const started = performance.now();
    run();
    fastest = Math.min(fastest, performance.now() - started);
  }
  return fastest;
}

test("asking every structural path against every claim grows linearly with the claims, not quadratically", () => {
  const { sizes, depth, maximumGrowthRatio } = fixture.ladder as { sizes: number[]; depth: number; maximumGrowthRatio: number };
  const timings = sizes.map((size) => {
    const claims = Array.from({ length: size }, (_, index) => absolute([...Array.from({ length: depth }, (_, level) => `d${(index >> level) % 7}`), `f${index}.rs`]));
    const asked = Array.from({ length: size }, (_, index) => absolute(["absent", `p${index}`]));
    let kept = 0;
    const milliseconds = best(() => {
      const directories = claimDirectoriesV1(claims);
      kept = 0;
      for (const path of asked) if (directories.has(path)) kept++;
    });
    expect(kept).toBe(0);
    return milliseconds;
  });
  const growth = timings[1]! / Math.max(timings[0]!, 0.05), scale = sizes[1]! / sizes[0]!;
  expect(growth).toBeLessThan(maximumGrowthRatio);
  expect(scale).toBeLessThan(maximumGrowthRatio);
});

test("the completed-compilation roster is unchanged and its cost grows linearly with the units", () => {
  const build = (units: number, sources: number) => {
    const workspace = resolve(process.cwd(), "roster-fixture"), buildDirectory = join(workspace, "build"), manifest = join(workspace, "Cargo.toml");
    const digest = (value: number): string => value.toString(16).padStart(64, "0");
    const observed: any[] = [], scripts: any[] = [], resources: any[] = [], captures: any[] = [], custom = Math.floor(units / 4), captured = Math.floor(units / 6);
    for (let unit = 0; unit < units; unit++) {
      const isCustom = unit < custom, id = `path+file:///s#u${unit}@0.0.0`, dir = join(workspace, "src", `u${unit}`), artifact = join(buildDirectory, `libu${unit}.rlib`);
      const inputs = Array.from({ length: sources }, (_, index) => ({ path: join(dir, `m${index}.rs`), kind: "file" as const, sha256: digest(unit * 1000 + index + 1) }));
      const message = { package_id: id, manifest_path: manifest, target: { name: `u${unit}`, kind: [isCustom ? "custom-build" : "lib"], src_path: inputs[0]!.path }, filenames: [artifact] };
      observed.push({ message, observedAtMs: 0, depInfo: [{ path: join(buildDirectory, `libu${unit}.d`), sha256: digest(900000 + unit), baseDirectory: workspace, sources: inputs.map((row) => row.path), checksums: [] }], inputs, artifacts: [{ path: artifact, sha256: digest(800000 + unit) }] });
      if (isCustom) {
        const out = join(buildDirectory, "build", `u${unit}`, "out");
        scripts.push({ package_id: id, out_dir: out });
        resources.push({ package_id: id, out_dir: out, path: join(out, "inputs.jsonl"), sha256: digest(700000 + unit), observedAtMs: 0, resources: [] });
      }
    }
    for (let index = 0; index < captured; index++) captures.push({ path: join(buildDirectory, "semio-compiler-resources", `r${index}`, "observation.json"), sha256: digest(600000 + index), observedAtMs: 0, producer: null, caller: null, producerUnit: observed[custom + index].message, callerUnit: observed[custom + index + 1].message, resources: [] });
    const observation: any = { version: 1, manifest, cwd: workspace, command: "cargo", args: [], buildDirectory, builtAtMs: 0, status: 0, cancelled: false, units: observed, buildScripts: scripts, observedAtMs: 0, invocationInputs: [{ path: manifest, sha256: digest(1) }], buildResources: resources, compilerResourceRoot: null, compilerResources: captures };
    const origin = { version: 1, workspace, mode: "prototype", manifest, cwd: workspace, buildDirectory, targetDirectory: join(workspace, "target"), cargoHome: join(workspace, "cargo") };
    return () => completedCargoClosureRosterV1(observation, origin);
  };
  const { units, sources, rows, retained } = fixture.roster as { units: number; sources: number; rows: number; retained: number };
  const small = build(units, sources)();
  expect(small.length).toBe(rows);
  expect(small.filter((row) => row.custody === "retained").length).toBe(retained);
  const first = best(build(units * 4, sources), 3), second = best(build(units * 16, sources), 3);
  expect(second / Math.max(first, 0.05)).toBeLessThan(fixture.ladder.maximumGrowthRatio * 2);
});
