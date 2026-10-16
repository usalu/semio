#!/usr/bin/env bun
import {writeOwnedCommandOutputV1} from "../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {frameworkOsViteArtifactInputsV1} from "../🏗️builder/🌐️vite/📥️artifacts/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {repositoryCargoNetworkV1} from "../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { readBuildMode } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { loadDevContribution, resolveDevContributionFile } from "../🧩️contribution/📥️loading/🟦️.ts";
import { frameworkOsLockedPrefsEnv } from "../../../../🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🟦️.ts";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { checkScriptInvocation, Script, ScriptRouter } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { buildViteArtifact } from "../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts";
import { productionBrowserInputHash } from "./⚙️inputs/🟦️.ts";
import { buildComponentClosureV1, playgroundComponentClosureV1 } from "../../🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts";

/** 🔏️ Supplies Nx with output-affecting environment and ignored configuration bytes. */
class InputsScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Production browser input hashing accepts no arguments");
    console.log(await productionBrowserInputHash(resolve(import.meta.dir, "..")));
  }
}

/** 🏁️ Completes the selected mode after Nx satisfies its browser build prerequisites. */
class CompleteScript extends Script {
  run(args: string[]): void {
    if (args.length) throw new Error("Distribution completion accepts no arguments");
    const mode = readBuildMode(process.env.SEMIO_BUILD_MODE);
    checkScriptInvocation(this.invocation);
    console.log(`Nx browser distribution complete: ${mode}`);
  }
}

/** 🏗️ Builds exactly the variant's components through the build driver, then bundles the browser variant. */
class BuildScript extends Script {
  async run(args: string[]): Promise<void> {
    const [variant, renderer, profile] = args;
    if (args.length !== 3 || !variant || renderer !== "react" || (profile !== "prototype" && profile !== "release") || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(variant)) throw new Error("build <variant> react <prototype|release>");
    const workspace = getWorkspaceRoot(), root = resolve(import.meta.dir, "..");
    const catalog = JSON.parse(readFileSync(join(process.env.SEMIO_PLUGIN_CATALOG ?? (()=>{throw Error("Distribution requires a supplied catalog root");})(), "🎠️playgrounds.json"), "utf8")) as { variant: string; distDir?: string; brand?: string; devContribution?: string }[];
    const playground = catalog.find((row) => row.variant === variant);
    const contribution = playground ? loadDevContribution(workspace, playground) : undefined;
    if (!playground) throw new Error(`Unknown production playground: ${variant}`);
    const mode = readBuildMode(profile);
    const controller = new AbortController();
    let cancelled: NodeJS.Signals | undefined;
    const stop = (signal: NodeJS.Signals): void => { cancelled ??= signal; controller.abort(); };
    const interrupt = (): void => stop("SIGINT"), terminate = (): void => stop("SIGTERM");
    process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
    try {
      await buildComponentClosureV1(workspace, playgroundComponentClosureV1(variant, "full"), { facet:"browser",invocation:this.invocation,network:repositoryCargoNetworkV1(this.invocation,workspace),control: scriptCompilerAdmissionControlV1(this.invocation, "native:driver", 0) });
      await buildViteArtifact({control:scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), invocation: this.invocation, mode: mode, artifactInputs: frameworkOsViteArtifactInputsV1(repositoryCachePartitionRootV1(workspace, "products"), {version:1,mode:mode,renderer:"react",command:"build",engines:"distribution"}), root, workspace, config: contribution ? resolveDevContributionFile(workspace, contribution.viteConfig, contribution.ownerRoot) : join(root, "🏗️builder/🌐️vite/🟦️.ts"), owner: `os-dev:${variant}:react`, signal: this.invocation.control.signal, environment: { ...frameworkOsLockedPrefsEnv({ ...process.env, SEMIO_BRAND: undefined }), SEMIO_PLUGIN: variant, SEMIO_RENDERER: "react", SEMIO_BUILD_MODE: mode, SEMIO_BRAND: undefined, PLAYGROUND_APP_KIND: undefined } });
    } catch (error) { if (!cancelled) throw error; process.exitCode = cancelled === "SIGINT" ? 130 : 143; }
    finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); }
  }
}

/** 🧵️ Receives one carried Original and owns its actual finite distribution compiler scope. */
class CompileScript extends Script {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("compile accepts no free path or configuration arguments");
  const{compileDistributionProductV1}=await import("./🏗️compiler/🟦️.ts"),receipt=await compileDistributionProductV1(this.repoRoot,this.invocation),text=JSON.stringify(receipt),{readNativeCallerAuthorityV1}=await import("../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts");
  if(Buffer.byteLength(text)>readNativeCallerAuthorityV1(this.invocation.capabilities).transport.maximumBytes)throw Error("Original distribution receipt transport exceeded");await writeOwnedCommandOutputV1(process.stdout,text,scriptCompilerAdmissionControlV1(this.invocation,"vite:distribution-receipt",0));
 }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dir).register("compile", CompileScript).register("build", BuildScript).register("inputs", InputsScript).register("complete", CompleteScript)).run(process.argv.slice(2), original));
