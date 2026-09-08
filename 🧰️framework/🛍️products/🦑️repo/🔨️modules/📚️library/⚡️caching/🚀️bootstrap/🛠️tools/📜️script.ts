import { createHash, randomUUID } from "node:crypto";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, renameSync, rmSync, symlinkSync, unlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { basename, dirname, isAbsolute, join, relative } from "node:path";
import { runBun } from "../📦️dependencies/📜️script.ts";
import { withResourceLeases } from "../../🔒️leases/🟦️.ts";

export type NxTooling = { readonly cli: string; readonly modulePath: string };
const RECIPE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools";
const STORE = ".🧬semio/🦑️repo/⚡️cache/tools/nx-tooling";

/** 🛣️ Rejects substituted directories before creating a tooling installation. */
function regularDirectory(path: string): void {
  for (let current = path; ; current = dirname(current)) {
    try { const stat = lstatSync(current); if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Invalid tooling directory: ${current}`); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
    if (dirname(current) === current) break;
  }
  mkdirSync(path, { recursive: true });
}

/** 🔗️ Selects an immutable toolset through Nx's native installation directory. */
export async function activateNxTools(workspace: string, tooling: NxTooling, signal: AbortSignal): Promise<void> {
  signal.throwIfAborted();
  const root = realpathSync(workspace), nx = join(root, ".nx"), pointer = join(nx, "installation"), selected = realpathSync(dirname(tooling.modulePath));
  if (dirname(selected) !== join(root, STORE) || !/^[a-f0-9]{64}$/.test(basename(selected))) throw new Error("Nx tooling must be an owned immutable installation");
  regularDirectory(nx);
  await withResourceLeases({ directory: join(root, ".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"), signal, resources: [{ resource: "nx-tooling-activation", mode: "exclusive" }] }, async () => {
    let previous: string | undefined;
    let present = true;
    try { lstatSync(pointer); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; present = false; }
    if (present) {
      if (!lstatSync(pointer).isSymbolicLink()) throw new Error("The Nx installation directory has another owner");
      previous = realpathSync(pointer);
      if (dirname(previous) !== join(root, STORE)) throw new Error("The Nx installation link has another owner");
      if (previous === selected) return;
    }
    const staging = join(nx, `.installation-${randomUUID()}`), kind = process.platform === "win32" ? "junction" : "dir";
    try {
      symlinkSync(selected, staging, kind);
      signal.throwIfAborted();
      try { renameSync(staging, pointer); }
      catch (error) {
        if (process.platform !== "win32" || !previous || !["EEXIST", "EPERM", "EACCES"].includes((error as NodeJS.ErrnoException).code ?? "")) throw error;
        unlinkSync(pointer);
        try { renameSync(staging, pointer); }
        catch (failure) { symlinkSync(previous, pointer, kind); throw failure; }
      }
    } finally { try { unlinkSync(staging); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; } }
  });
}

/** 🛠️ Acquires the frozen Nx toolchain independently of application dependency synchronization. */
export async function provisionNxTools(workspace: string, signal: AbortSignal): Promise<NxTooling> {
  signal.throwIfAborted();
  const root = realpathSync(workspace), recipe = join(root, RECIPE), files = new Map<string, Buffer>();
  for (const name of ["package.json", "bun.lock"]) files.set(name, readFileSync(join(recipe, name)));
  const manifest = JSON.parse(files.get("package.json")!.toString());
  if (manifest.packageManager !== `bun@${process.versions.bun}`) throw new Error(`Nx bootstrap requires ${manifest.packageManager}`);
  for (const path of Object.values(manifest.patchedDependencies) as string[]) {
    const canonical = realpathSync(join(root, path)), local = relative(root, canonical);
    if (isAbsolute(path) || path.includes("\\") || path.split("/").some(part => !part || part === "." || part === "..") || isAbsolute(local) || local.startsWith("..")) throw new Error(`Invalid tooling patch: ${path}`);
    files.set(path, readFileSync(canonical));
  }
  const report = process.platform === "linux" ? process.report?.getReport?.() as { header?: { glibcVersionRuntime?: string } } : undefined;
  const libc = process.platform === "linux" ? report?.header?.glibcVersionRuntime ? "gnu" : "musl" : "";
  const hash = createHash("sha256").update(`${process.platform}\0${process.arch}\0${libc}\0`);
  for (const [name, bytes] of files) hash.update(name + "\0").update(bytes).update("\0");
  const digest = hash.digest("hex"), base = join(root, STORE), destination = join(base, digest);
  regularDirectory(base);
  const installed = (directory: string): NxTooling => {
    regularDirectory(directory);
    const marker = JSON.parse(readFileSync(join(directory, ".semio-nx-tooling.json"), "utf8"));
    if (marker.version !== 1 || marker.digest !== digest) throw new Error("Nx tooling ownership mismatch");
    const modulePath = join(directory, "node_modules"), require = createRequire(join(directory, "package.json"));
    for (const [name, version] of Object.entries(manifest.dependencies)) {
      const path = realpathSync(require.resolve(name + "/package.json")), local = relative(modulePath, path);
      if (isAbsolute(local) || local.startsWith("..") || JSON.parse(readFileSync(path, "utf8")).version !== version) throw new Error(`Incomplete Nx tooling: ${name}`);
    }
    return { cli: require.resolve("nx/bin/nx.js"), modulePath };
  };
  let present = true;
  try { lstatSync(destination); }
  catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; present = false; }
  if (present) return installed(destination);
  const staging = mkdtempSync(join(base, ".stage-"));
  try {
    for (const [name, bytes] of files) { mkdirSync(dirname(join(staging, name)), { recursive: true }); writeFileSync(join(staging, name), bytes); }
    console.log(`Acquiring pinned Nx ${manifest.dependencies.nx} tooling…`);
    await runBun(["install", "--frozen-lockfile", "--ignore-scripts"], staging, signal);
    signal.throwIfAborted();
    writeFileSync(join(staging, ".semio-nx-tooling.json"), JSON.stringify({ version: 1, digest }));
    installed(staging);
    try { renameSync(staging, destination); }
    catch (error) { if (!["EEXIST", "ENOTEMPTY", "EPERM"].includes((error as NodeJS.ErrnoException).code ?? "")) throw error; installed(destination); }
    return installed(destination);
  } finally { rmSync(staging, { recursive: true, force: true }); }
}
