#!/usr/bin/env repo
import { createIssuedResourceControlV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { typecheckAdmittedProjectV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runVitest } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧪️vitest/🟦️.ts";
/** 🧍️ `@teaching/staffage` router: `repo ./📜️script.ts <test [exhaustive] | typecheck | export-models [--lang en|de] [--out dir] [--scale n] [--asset a,b] [--view v] [--force]>`. */

/** 🧪️ `test` — every module's and the schema twin's unit suite. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "exhaustive");
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts", "grant");
  }
}

/** 🪁️ `typecheck` — the barrel, the schema twin, the modules and their suites against the compiler. */
class TypecheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await typecheckAdmittedProjectV1({ workspace: this.repoRoot, invocation: this.invocation, control: createIssuedResourceControlV1(this.invocation, "typecheck:entry", 0), cwd: this.root, project: "tsconfig.json", owner: "staffage:typescript", args: segments, timeoutMs: 120_000 });
  }
}

/** 📦️ `export-models` — the complete 3D model set of the catalog (GLB per pose, FBX per clip, model JSON, ZIP per asset, `manifest.json`) into `📤️dist/🧊️models` of this package, or `--out`; resumes, honours SIGINT and the exit of this process; progress on stderr. */
class ExportModelsScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { exportModels, parseBatchArguments } = await import("../../🔨️modules/📤️export/📦️batch/🟦️.ts");
    const command = parseBatchArguments(segments);
    const controller = new AbortController();
    for (const signal of process.platform === "win32" ? (["SIGINT", "SIGTERM", "SIGBREAK"] as const) : (["SIGINT", "SIGTERM", "SIGHUP"] as const)) process.on(signal, () => controller.abort());
    const started = Date.now();
    let last = -1;
    const { summary } = await exportModels({ ...command, started: (id, index, count) => process.stderr.write(`[asset ${index}/${count}] ${id}\n`) }, {
      signal: controller.signal,
      progress: (done, total, label) => {
        const percent = Math.floor((done / total) * 100);
        if (percent !== last) {
          last = percent;
          process.stderr.write(`[${percent} %] ${label[command.language]}\n`);
        }
      },
    });
    process.stdout.write(`${command.out}\n`);
    process.stderr.write(`[done] ${summary.assets} assets, ${summary.files} files, ${summary.bytes} bytes; wrote ${summary.written}, kept ${summary.skipped}; ${((Date.now() - started) / 1000).toFixed(1)} s; [peak] ${Math.round(process.resourceUsage().maxRSS / 1024)} MB resident\n`);
  }
}

const router = new ScriptRouter(import.meta.dirname).register("test", TestScript).register("typecheck", TypecheckScript).register("export-models", ExportModelsScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
