import { createRequire } from "node:module";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { readFile, writeFile, stat, readlink } from "node:fs/promises";
import { existsSync, realpathSync, statSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { collectArtifactFiles } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts";
import type { BuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";

/** 📦️ Projects Next's actual trace closure into portable, bounded standalone artifact paths. */
export async function nextPublicationFiles(workspace: string, packageRoot: string, compiled: string, destination: string, mode: BuildMode, temporary: string, signal: AbortSignal): Promise<Map<string, string>> {
  const emitted = await collectArtifactFiles(compiled, signal), files = new Map<string, string>(), traced = new Set<string>(), nextRoot = dirname(createRequire(join(packageRoot, "package.json")).resolve("next/package.json"));
  const packageRequire = createRequire(join(packageRoot, "package.json")), sdkRequire = createRequire(join(nextRoot, "package.json")), sdkManifest = sdkRequire("./package.json"), sdkRoots = new Map<string, string>([["next", nextRoot]]);
  for (const name of [...Object.keys(sdkManifest.dependencies ?? {}), ...Object.keys(sdkManifest.peerDependencies ?? {})]) {
    const require = Object.hasOwn(sdkManifest.dependencies ?? {}, name) ? sdkRequire : packageRequire;
    const record = (require.resolve.paths(name) ?? []).map(directory => join(directory, name, "package.json")).find(existsSync);
    if (record) sdkRoots.set(name, dirname(realpathSync(record)));
  }
  const normalize = (value: string) => value.replaceAll("\\", "/"), within = (root: string, value: string) => { const part = relative(root, value); return !isAbsolute(part) && part.split(/[\\/]/u)[0] !== ".."; };
  const add = async (name: string, source: string) => {
    if (join(destination, name).length > 256) throw Error("Next publication path exceeds 256 characters: " + name);
    const prior = files.get(name); if (prior && !Buffer.from(await readFile(prior)).equals(await readFile(source))) throw Error("Next trace has conflicting package versions: " + name);
    files.set(name, source);
  };
  for (const [name, source] of emitted) {
    signal.throwIfAborted(); if (name.startsWith("cache/")) continue;
    await add(name, source); await add(`standalone/dist/${mode}/next/${name}`, source);
    if (name.endsWith(".nft.json")) {
      const trace = JSON.parse(await readFile(source, "utf8"));
      if (validateJsonSchemaSubset(schema, trace).length) throw Error("Invalid Next compiler trace");
      for (const path of trace.files) traced.add(resolve(dirname(source), path));
    }
  }
  console.log("[next-build] tracing standalone startup runtime");
  const { nodeFileTrace, resolve: resolveDependency } = sdkRequire("next/dist/compiled/@vercel/nft"), startup = await nodeFileTrace([sdkRequire.resolve("next"), sdkRequire.resolve("next/dist/server/lib/start-server")], { base: workspace, processCwd: packageRoot, mixedModules: true, async resolve(id: string, parent: string, job: unknown, isCjs: boolean) { const parts = id.split("/"), name = id.startsWith("@") ? parts.slice(0, 2).join("/") : parts[0], root = sdkRoots.get(name); return root ? sdkRequire.resolve(join(root, id.slice(name.length + 1))) : id.startsWith(".") ? createRequire(parent).resolve(id) : resolveDependency(id, parent, job, isCjs); }, ignore(path: string) { const name = normalize(path); return /(?:\.map|\.d\.ts)$/u.test(name) || /\/next\/dist\/(?:compiled\/webpack\/|compiled\/next-server\/.*\.dev\.js|server\/lib\/route-resolver)/u.test(name) || /\/node_modules\/(?:webpack5|electron)\//u.test(name); }, async readFile(path: string) { signal.throwIfAborted(); return readFile(path, "utf8"); }, async stat(path: string) { signal.throwIfAborted(); try { return await stat(path); } catch (error) { if (["ENOENT", "ENOTDIR"].includes((error as NodeJS.ErrnoException).code ?? "")) return null; throw error; } }, async readlink(path: string) { signal.throwIfAborted(); try { return await readlink(path); } catch (error) { if (["EINVAL", "ENOENT", "ENOTDIR"].includes((error as NodeJS.ErrnoException).code ?? "")) return null; throw error; } } });
  for (const name of startup.fileList) traced.add(resolve(workspace, name));
  for (const path of traced) {
    signal.throwIfAborted(); if (within(compiled, path)) continue;
    let physical = realpathSync(path);
    const dependency = normalize(physical).slice(normalize(physical).lastIndexOf("/node_modules/") + "/node_modules/".length), parts = dependency.split("/"), name = parts[0].startsWith("@") ? parts.slice(0, 2).join("/") : parts[0], sdkRoot = sdkRoots.get(name);
    if (sdkRoot) physical = join(sdkRoot, dependency.slice(name.length + 1));
    const normalized = normalize(physical), node = normalized.lastIndexOf("/node_modules/");
    if (!within(workspace, physical)) throw Error("Next trace escaped the admitted workspace: " + path);
    if (statSync(physical).isDirectory()) continue;
    const artifactName = node >= 0 ? "standalone/node_modules/" + normalized.slice(node + "/node_modules/".length) : "standalone/" + normalize(relative(within(packageRoot, physical) ? packageRoot : workspace, physical));
    if (artifactName === "standalone/package.json") continue;
    await add(artifactName, physical);
    if (node >= 0) {
      const modules = normalized.slice(0, node + "/node_modules".length);
      for (let directory = dirname(physical); directory !== modules && within(modules, directory); directory = dirname(directory)) {
        const manifest = join(directory, "package.json");
        if (existsSync(manifest)) await add("standalone/node_modules/" + normalize(relative(modules, manifest)), manifest);
      }
    }
  }
  const required = JSON.parse(await readFile(join(compiled, "required-server-files.json"), "utf8"));
  required.config.distDir = `dist/${mode}/next`; required.config.output = "standalone"; required.config.typescript.tsconfigPath = "tsconfig.json";
  const record = join(temporary, "required-server-files-publication.json"); await writeFile(record, JSON.stringify(required));
  files.set("required-server-files.json", record); files.set(`standalone/dist/${mode}/next/required-server-files.json`, record);
  const server = join(temporary, "standalone-server.js"), manifest = join(temporary, "standalone-package.json");
  await writeFile(manifest, JSON.stringify({ private: true, type: "commonjs" }));
  await writeFile(server, `const path = require("node:path"); process.env.NODE_ENV = "production"; process.chdir(__dirname); const config = ${JSON.stringify(required.config)}; process.env.__NEXT_PRIVATE_STANDALONE_CONFIG = JSON.stringify(config); require("next"); require("next/dist/server/lib/start-server").startServer({dir:__dirname,isDev:false,config,hostname:process.env.HOSTNAME||"0.0.0.0",port:Number(process.env.PORT||3000),allowRetry:false});`);
  files.set("standalone/server.js", server); files.set("standalone/package.json", manifest);
  return files;
}
