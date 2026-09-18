#!/usr/bin/env bun
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { resolveTestLevel, runCargoTestBudgeted } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { runOwnedCommand } from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import { stageArtifacts } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";
import { FONT_ASSET, validateFontAsset } from "../../🖼️canvas/🔤️fonts/🟦️.ts";

const ROOT = import.meta.dir;
const FONT_TOOL = "dump-guestslim-typst-fonts";

class FontsScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("fonts takes no arguments");
    const binary = join(ROOT, "../../🖼️canvas/🔤️fonts/📦️packages/🦀️rust/dist/build", FONT_TOOL + (process.platform === "win32" ? ".exe" : ""));
    if (!existsSync(binary)) throw new Error("Missing font tool prerequisite; run fonts through Nx");
    mkdirSync(join(ROOT, "dist"), { recursive: true });
    const temporary = mkdtempSync(join(ROOT, "dist/.fonts-")), asset = join(temporary, FONT_ASSET);
    let cancelled = false;
    const cancel = (): void => { cancelled = true; };
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      await runOwnedCommand(binary, [asset], ROOT, "font-assets:publish", 60_000);
      if (cancelled) throw new Error("Font dump cancelled");
      const bytes = readFileSync(asset), count = validateFontAsset(bytes);
      await stageArtifacts(join(ROOT, "dist/fonts"), "infinite:fonts", new Map([[FONT_ASSET, asset]]));
      console.log(`Staged ${count} fonts (${bytes.byteLength} bytes)`);
    } finally {
      process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel);
      rmSync(temporary, { recursive: true, force: true });
    }
  }
}

/**
 * 🌍️ The two module trees the wgpu renderer actually paints from this crate — `world::` (board,
 * mesh3d, action queue, camera) and everything named `terrain`. The wgpu renderer parity lane runs
 * exactly this slice, so it is a target rather than a remembered command line; `--lib` because the
 * laws live in `#[cfg(test)]` modules of the library, not in `[[test]]` binaries.
 */
class TestWgpuWorldTerrainScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted([], this.root, ["--lib", ...rest, "world::", "terrain"]);
  }
}

const router = new ScriptRouter(ROOT).register("fonts", FontsScript).register("test-wgpu-world-terrain", TestWgpuWorldTerrainScript);
if (import.meta.main) await router.run(process.argv.slice(2));
