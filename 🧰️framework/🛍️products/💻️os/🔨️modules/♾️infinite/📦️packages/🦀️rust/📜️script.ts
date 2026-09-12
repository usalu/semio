#!/usr/bin/env bun
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { join, relative } from "node:path";
import { BundleScript, ScriptRouter, getWorkspaceRoot, runBundleScriptMain, runExactCargoLawProcess } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { buildCargoArtifacts } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts";
import { stageArtifacts } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";
import { FONT_ASSET, validateFontAsset } from "../../🖼️canvas/🔤️fonts/🟦️.ts";

const ROOT = import.meta.dir;
const FONT_TOOL = "dump-guestslim-typst-fonts";

class FontToolScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("font-tool takes no arguments");
    await buildCargoArtifacts(relative(getWorkspaceRoot(), join(ROOT, "Cargo.toml")), ["--bin", FONT_TOOL, "--features", "render"], getWorkspaceRoot(), { output: "dist/font-tool" });
  }
}

class FontsScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("fonts takes no arguments");
    const binary = join(ROOT, "dist/font-tool", FONT_TOOL + (process.platform === "win32" ? ".exe" : ""));
    if (!existsSync(binary)) throw new Error("Missing font tool prerequisite; run fonts through Nx");
    mkdirSync(join(ROOT, "dist"), { recursive: true });
    const temporary = mkdtempSync(join(ROOT, "dist/.fonts-")), asset = join(temporary, FONT_ASSET);
    let cancelled = false;
    const cancel = (): void => { cancelled = true; };
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      const result = await runExactCargoLawProcess(binary, [asset], { cwd: ROOT, env: process.env, budgetMs: 60_000, maxOutputBytes: 1024 * 1024, stdoutPath: join(temporary, "stdout"), stderrPath: join(temporary, "stderr"), cancelled: () => cancelled });
      if (result.status !== 0 || cancelled) throw new Error(`Font dump failed: ${result.stderr}`);
      const bytes = readFileSync(asset), count = validateFontAsset(bytes);
      stageArtifacts(join(ROOT, "dist/fonts"), "infinite:fonts", new Map([[FONT_ASSET, asset]]));
      console.log(`Staged ${count} fonts (${bytes.byteLength} bytes)`);
    } finally {
      process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel);
      rmSync(temporary, { recursive: true, force: true });
    }
  }
}

const router = new ScriptRouter(ROOT).register("font-tool", FontToolScript).register("fonts", FontsScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url);
