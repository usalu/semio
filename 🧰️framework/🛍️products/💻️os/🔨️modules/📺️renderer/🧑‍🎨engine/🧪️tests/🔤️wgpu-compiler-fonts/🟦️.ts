import { describe, expect, it } from "vitest";
import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { COMPILER_FONT_FILES, COMPILER_FONTS_DIRECTORY, COMPILER_FONTS_ROUTE, installCompilerFonts, type CompilerFontBindings } from "../../../../../../../🔨️modules/📚️compiler/🌍️world/🟦️.ts";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../..");
const compilerWorld = join(repoRoot, "🧰️framework/🔨️modules/📚️compiler/🌍️world");
const fixture = JSON.parse(readFileSync(join(compilerWorld, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8")) as { fonts: { role: string; file: string; bytes: number }[] };
const frameWorker = readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts"), "utf8");

const page = "https://play.invalid/os/index.html";
const payload = (file: string): Uint8Array => Uint8Array.from(Array.from(file, ch => ch.codePointAt(0)! & 255));

describe("compiler font delivery to the wgpu renderer", () => {
  it("names exactly the Rust port's five assets, in role order, and every one exists with the fixture's size", () => {
    expect([...COMPILER_FONT_FILES]).toEqual(fixture.fonts.map(row => row.file));
    const rust = readFileSync(join(compilerWorld, "🦀️.rs"), "utf8");
    const declared = [...rust.matchAll(/FontRole::\w+ => "([^"]+)",/gu)].map(match => match[1]);
    expect(declared).toEqual([...COMPILER_FONT_FILES]);
    expect(existsSync(join(repoRoot, COMPILER_FONTS_DIRECTORY))).toBe(true);
    for (const row of fixture.fonts) expect(statSync(join(repoRoot, COMPILER_FONTS_DIRECTORY, row.file)).size, row.role).toBe(row.bytes);
  });

  it("fetches the five served files and installs them in one call, in role order", async () => {
    const requested: string[] = [];
    const calls: Uint8Array[][] = [];
    const bindings: CompilerFontBindings = { semioWgpuSetCompilerFonts: (...fonts) => void calls.push(fonts) };
    const installed = await installCompilerFonts(bindings, page, async url => {
      requested.push(url);
      const file = decodeURIComponent(url.slice(url.lastIndexOf("/") + 1));
      return new Response(payload(file));
    });
    expect(installed).toBe(true);
    expect([...requested].sort()).toEqual(COMPILER_FONT_FILES.map(file => new URL(`${COMPILER_FONTS_ROUTE}/${encodeURIComponent(file)}`, page).href).sort());
    expect(calls).toHaveLength(1);
    expect(calls[0]).toEqual(COMPILER_FONT_FILES.map(payload));
  });

  it("installs nothing and fetches nothing when the renderer module has no font door", async () => {
    let fetched = 0;
    expect(await installCompilerFonts({}, page, async () => (fetched++, new Response(new Uint8Array(1))))).toBe(false);
    expect(fetched).toBe(0);
  });

  it("rejects a missing font and never installs a partial set", async () => {
    let installs = 0;
    const bindings: CompilerFontBindings = { semioWgpuSetCompilerFonts: () => void installs++ };
    await expect(installCompilerFonts(bindings, page, async url => (url.includes(encodeURIComponent(COMPILER_FONT_FILES[4])) ? new Response("no", { status: 404 }) : new Response(new Uint8Array(2))))).rejects.toThrow(/compiler-fonts: 404/u);
    expect(installs).toBe(0);
  });

  it("is served by every host that serves the wgpu renderer module", () => {
    const storybook = readFileSync(join(repoRoot, ".storybook/📖️stories/🧭️coordination/🟦️.ts"), "utf8");
    expect(storybook).toContain(`{ kind: "static-dir", route: "${COMPILER_FONTS_ROUTE}", root: "${COMPILER_FONTS_DIRECTORY}" }`);
    const devBuilder = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts"), "utf8");
    expect(devBuilder).toContain("route: COMPILER_FONTS_ROUTE, root: COMPILER_FONTS_DIRECTORY");
  });

  it("keeps booting, and says why, when the fonts cannot be fetched", () => {
    expect(frameWorker).toMatch(/catch \(error\) \{\s+if \(closed \|\| closing\) throw error;\s+progress\(`compiler-fonts-fault: /u);
  });

  it("boots the renderer wasm, then installs the fonts, then calls the renderer bootstrap that first resolves an icon", () => {
    const instantiate = frameWorker.indexOf("await instantiateRendererWasm(loaded, message.bindingsWasmUrl)");
    const install = frameWorker.indexOf("installCompilerFonts(loaded, scope.location.href");
    const bootstrap = frameWorker.indexOf("loaded.semioWgpuWorkerBootstrap!(");
    expect(instantiate).toBeGreaterThan(-1);
    expect(install).toBeGreaterThan(instantiate);
    expect(bootstrap).toBeGreaterThan(install);
    expect(frameWorker.indexOf("installCompilerFonts(loaded")).toBe(frameWorker.lastIndexOf("installCompilerFonts(loaded"));
  });
});
