#!/usr/bin/env bun
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { join, relative } from "node:path";
import { BundleScript, ScriptRouter, getWorkspaceRoot, runBundleScriptMain, runExactCargoLawProcess } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { buildCargoArtifacts } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts";
import { stageArtifacts } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";

const ROOT = import.meta.dir;
const FONT_TOOL = "dump-guestslim-typst-fonts";
export const FONT_ASSET = "🔤️guestslim-typst-fonts.bin";

/** 🔤️ Validates every packed font extent before publishing the complete asset. */
export function validateFontAsset(bytes: Uint8Array): number {
  if (bytes.byteLength < 4) throw new Error("Missing packed font count");
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength), count = view.getUint32(0, true);
  if (count < 1 || count > 1024) throw new Error("Invalid packed font count");
  let offset = 4;
  for (let index = 0; index < count; index++) {
    if (offset + 4 > bytes.byteLength) throw new Error("Missing packed font extent");
    const size = view.getUint32(offset, true);
    offset += 4;
    if (size < 12 || offset + size > bytes.byteLength) throw new Error("Invalid packed font extent");
    offset += size;
  }
  if (offset !== bytes.byteLength) throw new Error("Trailing packed font bytes");
  return count;
}

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
