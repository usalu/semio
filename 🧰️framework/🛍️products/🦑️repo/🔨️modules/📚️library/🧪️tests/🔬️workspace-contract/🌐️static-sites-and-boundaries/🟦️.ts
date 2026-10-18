import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { playgroundStaticSiteBuildOptions } from "../../../../../../../🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
import { PLAYGROUND_LOCKED_EXAMPLE_ENV, PLAYGROUND_PORTS, allPlaygroundReservedPorts, playgroundDevPort, playgroundPlayViteDefine } from "../../../🎮️playground/🟦️.ts";
import { findWorkspaceRoot } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

describe("playground static sites", () => {
  test("PLAYGROUND_PORTS keeps cad and dag on distinct reserved dev ports", () => {
    expect(playgroundDevPort("cad")).toBe(6020);
    expect(playgroundDevPort("dag")).toBe(6017);
    expect(allPlaygroundReservedPorts().size).toBeGreaterThanOrEqual(Object.keys(PLAYGROUND_PORTS).length);
  });

  test("assigns a unique port per dev and test slot", () => {
    const seen = new Set<number>();
    for (const spec of Object.values(PLAYGROUND_PORTS)) {
      expect(seen.has(spec.dev)).toBe(false);
      seen.add(spec.dev);
      if (spec.test !== undefined) {
        expect(seen.has(spec.test)).toBe(false);
        seen.add(spec.test);
      }
    }
  }, 30_000);

  test("playgroundPlayViteDefine embeds locked fixture env", () => {
    const prev = process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
    try {
      delete process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
      expect(playgroundPlayViteDefine()["import.meta.env.PLAYGROUND_LOCKED_EXAMPLE_ID"]).toBe('""');
      process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV] = "concrete-forest";
      expect(playgroundPlayViteDefine()["import.meta.env.PLAYGROUND_LOCKED_EXAMPLE_ID"]).toBe('"concrete-forest"');
    } finally {
      if (prev === undefined) delete process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
      else process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV] = prev;
    }
  });

  test("playgroundStaticSiteBuildOptions uses relative-base dist output", () => {
    expect(playgroundStaticSiteBuildOptions()).toEqual({
      target: "esnext",
      outDir: "dist",
      emptyOutDir: true,
    });
    expect(playgroundStaticSiteBuildOptions({ sourcemap: true }).sourcemap).toBe(true);
  });
});

describe("package boundary guards", () => {
  const repoRoot = findWorkspaceRoot();

  test("vite and vitest configs avoid cross-package @semio-tech aliases", () => {
    const offenders: string[] = [];
    const listed = spawnSync("rg", ["--files", "--glob", "**/*vite*.config.ts", "--glob", "**/🧪️tests/🟦️.ts", "--glob", "!**/.🧬semio/**", "--glob", "!**/.claude/**"], { cwd: repoRoot, encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
    for (const relativePath of (listed.stdout ?? "").split("\n").map((line) => line.trim()).filter(Boolean)) {
      const full = join(repoRoot, relativePath);
      const pkgDir = findNearestPackageDir(full, repoRoot);
      const text = readFileSync(full, "utf8");
      for (const match of text.matchAll(/find:\s*(?:\/\^)?["']@semio-tech\/[^"']+["']/g)) {
        const line = text.slice(0, match.index).split("\n").length;
        const snippet = text.split("\n")[line - 1] ?? "";
        if (/path\.resolve\(__dirname,\s*["']\.\./.test(snippet) && pkgDir && !snippet.includes("node_modules")) {
          offenders.push(`${relativePath}:${line}`);
        }
      }
    }
    expect<readonly (number | null)[]>([0, 1]).toContain(listed.status);
    expect(offenders).toEqual([]);
  });

  test("legacy package aliases are absent from source imports", () => {
    const result = spawnSync("rg", ["-l", "@compose/ui|@ui/react|@elements/", "--glob", "*.{ts,tsx}", "--glob", "!**/.🧬semio/**", "--glob", "!**/🔬️workspace-contract/🟦️.ts"], {
      cwd: repoRoot,
      encoding: "utf8",
    });
    const files = (result.stdout ?? "")
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean);
    expect(files).toEqual([]);
  });

  test("framework renderer host has no per-technology registerUi surface host APIs", () => {
    const indexPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx");
    const indexSource = readFileSync(indexPath, "utf8");
    expect(indexSource).not.toMatch(/registerUi(?:Draw|Flow|Layout|Note|Puzzle2d|Puzzle3d|Puzzle5d|Sequence|Writer|Raster|Forms|Trinity|Procedural|Shooting|Gis|Cad|Dag|Lowpoly|Imperative|S)SurfaceHost/);
    expect(indexSource).toContain("bootFrameworkOs");
  });
});

function findNearestPackageDir(filePath: string, repoRoot: string): string | undefined {
  let dir = join(filePath, "..");
  while (dir.startsWith(repoRoot)) {
    if (existsSync(join(dir, "package.json"))) return dir;
    const parent = join(dir, "..");
    if (parent === dir) break;
    dir = parent;
  }
  return undefined;
}
