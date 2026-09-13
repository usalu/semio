#!/usr/bin/env bun
import { once } from "node:events";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import type { Server } from "node:http";
import { dirname, join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { runTool } from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts";
import { getWorkspaceRoot } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { startAssetServer } from "../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
import { nativeRendererBinary } from "../🏗️compiler/🦀️native/📜️script.ts";
import { nativeRuntimeDirectory } from "./📦️modules/🟦️.ts";

const ownerPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript";
const registryPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";

/** 🔐️ Excludes parent credentials from ordinary native renderer processes. */
export function nativeRunnerEnvironment(source: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  return Object.fromEntries(Object.entries(source).filter(([key]) => {
    const normalized = key.toUpperCase();
    return !["S_USER", "VITE_S_USER", "S_HUB_URL"].includes(normalized) && !/TOKEN|SESSION|CREDENTIAL|BEARER|CAPABILITY|AUTHORIZATION|COOKIE/.test(normalized);
  }).concat([["SEMIO_DIRECT_CHILD_BENIGN", "preserved"]]));
}

/** 🏃️ Keeps the owning JavaScript event loop responsive while the native child runs. */
export async function runNativeBinary(executable: string, args: readonly string[], environment: NodeJS.ProcessEnv, cwd = getWorkspaceRoot(), signal: AbortSignal = new AbortController().signal): Promise<void> {
  await runTool(executable, [...args], cwd, signal, false, nativeRunnerEnvironment(environment));
}

/** 🌐️ Starts one ready asset listener, runs the native consumer and closes all owned connections. */
export async function runNativeSession(executable: string, args: readonly string[], environment: NodeJS.ProcessEnv, cwd: string, signal: AbortSignal, startAssets?: () => Server): Promise<void> {
  signal.throwIfAborted();
  const server = startAssets?.();
  try {
    if (server && !server.listening) await once(server, "listening", { signal });
    const address = server?.address();
    if (server && (!address || typeof address === "string")) throw new Error("Native asset server needs a TCP address");
    const env = { ...environment, ...(address && typeof address !== "string" ? { SEMIO_ASSET_BASE_URL: `http://127.0.0.1:${address.port}` } : {}) };
    await runNativeBinary(executable, args, env, cwd, signal);
  } finally {
    if (server) {
      server.closeAllConnections();
      await new Promise<void>(accept => server.close(() => accept()));
    }
  }
}

/** 🎛️ Rejects undeclared profiles and variants before reading completed artifacts. */
function selection(args: string[], smoke = false): { variant: string; profile: "dev" | "release" } {
  const [variant, profile, ...rest] = args;
  nativeRuntimeDirectory(".", variant, profile);
  if (rest.length && !(smoke && rest.length === 1 && rest[0] === "--smoke")) throw new Error("Native runtime accepts a variant, profile and optional --smoke");
  return { variant, profile: profile as "dev" | "release" };
}

/** 🖥️ Runs completed native artifacts without compiling or invoking another Nx graph. */
class RunScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const { variant, profile } = selection(args, true), repo = this.repoRoot, runtime = nativeRuntimeDirectory(join(repo, ownerPath), variant, profile);
    const manifest = JSON.parse(readFileSync(join(runtime, "🔣️runtime.json"), "utf8"));
    if (manifest.version !== 1 || manifest.variant !== variant || manifest.profile !== profile || !existsSync(join(runtime, ".nx-artifact.json"))) throw new Error("Missing or mismatched Nx native runtime artifact");
    const catalog = JSON.parse(readFileSync(join(repo, registryPath, "🤖️generated/🎠️playgrounds.json"), "utf8"));
    const row = catalog.find((entry: { variant: string }) => entry.variant === variant);
    if (!row) throw new Error(`Unknown native playground: ${variant}`);
    const controller = new AbortController(), cancel = () => controller.abort();
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      const binary = nativeRendererBinary(resolve(import.meta.dir, "../📦️packages/🦀️rust"), profile);
      await runNativeSession(binary, ["--plugin", variant, ...(row.app ? ["--app", row.app] : []), ...(args.includes("--smoke") ? ["--smoke"] : [])], { ...process.env, SEMIO_PLUGIN: variant, SEMIO_RENDERER: "wgpu", SEMIO_BUILD_MODE: profile === "release" ? "ship" : "dev", SEMIO_PLUGIN_MODULES: runtime }, repo, controller.signal, row.assets?.length ? () => startAssetServer(repo, 0, row.assets) : undefined);
    } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  }
}

/** ⚖️ Runs the native scale consumer with only its binary and fixture prerequisites. */
class ScaleScript extends BundleScript {
  async run([profile, ...args]: string[]): Promise<void> {
    nativeRuntimeDirectory(".", "scale", profile);
    if (!args.length || args.length % 2 !== 0 || args.some((value, index) => index % 2 === 0 && !["--scale", "--scale-wasm", "--shards", "--report"].includes(value))) throw new Error("Native scale accepts --scale, --scale-wasm, --shards and --report value pairs");
    for (const flag of ["--scale", "--scale-wasm", "--report"]) if (args.filter(value => value === flag).length !== 1) throw new Error(`Native scale requires exactly one ${flag}`);
    mkdirSync(dirname(resolve(this.repoRoot, args[args.indexOf("--report") + 1])), { recursive: true });
    const controller = new AbortController(), cancel = () => controller.abort();
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try { await runNativeBinary(nativeRendererBinary(resolve(import.meta.dir, "../📦️packages/🦀️rust"), profile), args, process.env, this.repoRoot, controller.signal); }
    finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  }
}

if (import.meta.main) await new ScriptRouter(import.meta.dir).register("run", RunScript).register("scale", ScaleScript).run(process.argv.slice(2));
