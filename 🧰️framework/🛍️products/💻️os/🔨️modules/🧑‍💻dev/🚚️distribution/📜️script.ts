#!/usr/bin/env bun
import { frameworkOsLockedPrefsEnv } from "../../../../🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🟦️.ts";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { Script, ScriptRouter } from "../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { buildViteArtifact } from "../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts";
import { playgroundReactReleaseOutputPath } from "./📍️output/🟦️.ts";
import { productionBrowserInputHash } from "./⚙️inputs/🟦️.ts";

/** 🔏️ Supplies Nx with output-affecting environment and ignored configuration bytes. */
class InputsScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Production browser input hashing accepts no arguments");
    console.log(await productionBrowserInputHash(resolve(import.meta.dir, "..")));
  }
}

/** 🏗️ Bundles one prepared browser variant without executing its prerequisite producers. */
class BuildScript extends Script {
  async run(args: string[]): Promise<void> {
    const [variant, renderer, profile] = args;
    if (args.length !== 3 || renderer !== "react" || profile !== "release" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(variant)) throw new Error("build <variant> react release");
    const workspace = getWorkspaceRoot(), root = resolve(import.meta.dir, ".."), packageRoot = join(root, "📦️packages/🟦️typescript");
    const catalog = JSON.parse(readFileSync(join(root, "../🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json"), "utf8")) as { variant: string; distDir?: string }[];
    const playground = catalog.find((row) => row.variant === variant);
    if (!playground) throw new Error(`Unknown production playground: ${variant}`);
    const output = playgroundReactReleaseOutputPath(workspace, packageRoot, playground);
    const controller = new AbortController();
    let cancelled: NodeJS.Signals | undefined;
    const stop = (signal: NodeJS.Signals): void => { cancelled ??= signal; controller.abort(); };
    const interrupt = (): void => stop("SIGINT"), terminate = (): void => stop("SIGTERM");
    process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
    try {
      await buildViteArtifact({ root, workspace, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), output, owner: `os-dev:${variant}:react:release`, signal: controller.signal, environment: { ...frameworkOsLockedPrefsEnv({ ...process.env, SEMIO_BRAND: undefined }), SEMIO_PLUGIN: variant, SEMIO_RENDERER: "react", SEMIO_BUILD_MODE: "ship", SEMIO_BRAND: undefined, PLAYGROUND_APP_KIND: undefined } });
    } catch (error) { if (!cancelled) throw error; process.exitCode = cancelled === "SIGINT" ? 130 : 143; }
    finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); }
  }
}

if (import.meta.main) await new ScriptRouter(import.meta.dir).register("build", BuildScript).register("inputs", InputsScript).run(process.argv.slice(2));
