/** 📦️ `publish`, `components` and `release` commands.
 * @see 📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { orchestratorBudgetOpts, runCmd } from "../../🏃️process/🟦️.ts";
import { Script } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readFileSync } from "node:fs";
import { join } from "node:path";

//#region 🔖️PublishScript
export class PublishScript extends Script {
  run(segments: string[]): void {
    const slice = segments[0];
    /** 🚚️ Every slice that has a real, versioned, checksummed local deliverable today. Each target
     * depends on its own release build and writes `<project>/dist/publish/<name>-<version>-<platform>-<arch>.tar.gz`
     * plus a `.sha256`; none of them uploads or pushes anything anywhere. */
    const map: Record<string, string> = {
      "os-hub": "os-hub:publish",
      "os-mcp": "@semio-tech/framework-os-mcp-rs:publish",
    };
    if (!slice) {
      console.error(`[publish] usage: bun ./📜️script.ts publish <${Object.keys(map).join(" | ")}>`);
      process.exit(1);
    }
    const target = map[slice];
    if (!target) {
      console.error(`[publish] unknown slice ${JSON.stringify(slice)}`);
      process.exit(1);
    }
    runCmd("bun", ["repo", "run", target], { cwd: this.root, ...orchestratorBudgetOpts() });
  }
}
//#endregion 🔖️PublishScript

//#region 🔖️ComponentsScript
/** 🧩️ `components describe <plugin-id…>|--all [--profile release|prototype]` writes the committed semantic descriptor
 * pairs from freshly built bytes (`describe --write`; shipped bytes through one build driver run); `components iterate <plugin-id>` is the single-plugin `prototype`
 * browser loop; `components profile-independence [--sample <n>] [--seed <n>]|<plugin-id…>` proves a semantic descriptor is
 * equal under both profiles. Every subcommand is cancellable (SIGINT/SIGTERM).
 * @see ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧩️component-stage/🟦️.ts */
export class ComponentsScript extends Script {
  async run(segments: string[]): Promise<void> {
    const [subcommand, ...rest] = segments;
    if (subcommand === "describe") return await (await import("../../../../../💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🏗️build/🧩️driver-stages/🟦️.ts")).describeComponentsV1(this.repoRoot,this.invocation, rest, scriptCompilerAdmissionControlV1(this.invocation, "components:describe", 0), (await import("../../🏃️process/🦀️cargo/📥️dependencies/🟦️.ts")).repositoryCargoNetworkV1(this.invocation, this.repoRoot));
    if (subcommand === "profile-independence") return await (await import("../../../../../💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📦️model/⚖️comparison/🟦️.ts")).profileIndependenceV1(this.repoRoot,this.invocation,rest,scriptCompilerAdmissionControlV1(this.invocation,"components:profile",0));
    if (subcommand === "iterate" && rest.length === 1) return await (await import("../../../../../💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🏗️build/📦️materialization/🟦️.ts")).iterateComponentV1(this.repoRoot,this.invocation,rest[0]!, scriptCompilerAdmissionControlV1(this.invocation, "components:iterate", 0),(await import("../../🏃️process/🦀️cargo/📥️dependencies/🟦️.ts")).repositoryCargoNetworkV1(this.invocation,this.repoRoot));
    throw new Error("usage: components describe <plugin-id…>|--all [--profile release|prototype] | components iterate <plugin-id> | components profile-independence [--sample <n>] [--seed <n>]|<plugin-id…>");
  }
}
//#endregion 🔖️ComponentsScript

//#region 🔖️ReleaseScript
/** 🚀️ `release` (`workspace:release`): the one release build driver over every discovered component. It owns every component Cargo pass (one per workspace) and
 * the jobs this project's `release` target declares (descriptor emitter, fonts, browser support) under one jobserver, memory admission and telemetry, and
 * streams each component through describe, transpile, `wasm-opt`, its module probe and publication as Cargo links it; `SEMIO_NATIVE_BUILD_GENERATION=fresh`
 * compiles cold into a new generation.
 * @see ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts
 * @see ./🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts */
export class ReleaseScript extends Script {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("The release driver accepts no arguments; SEMIO_NATIVE_BUILD_GENERATION=fresh selects a cold generation");
    const [{ buildReleasePipelineV1 }, { repositoryCargoNetworkV1 }] = await Promise.all([import("../../../../../💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🏗️build/🧩️driver-stages/🟦️.ts"), import("../../🏃️process/🦀️cargo/📥️dependencies/🟦️.ts")]);
    await buildReleasePipelineV1(this.repoRoot, JSON.parse(readFileSync(join(this.repoRoot, "📋️project.json"), "utf8")).targets.release.metadata.semio.release, { facet:"browser",invocation:this.invocation,network: repositoryCargoNetworkV1(this.invocation, this.repoRoot), control: scriptCompilerAdmissionControlV1(this.invocation, "native:driver", 0) });
  }
}
//#endregion 🔖️ReleaseScript
