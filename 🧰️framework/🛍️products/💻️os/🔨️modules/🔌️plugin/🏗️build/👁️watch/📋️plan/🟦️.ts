import { createHash } from "node:crypto";
import { watch, lstatSync, existsSync, realpathSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";

/** 👁️ The admitted native source and generator inputs projected by the actual build producer. */
export type NativeSourceWatchPlanV1 = Readonly<{ schema: "semio.framework.os.plugin.source-watch-plan/v1"; includes: readonly string[]; excludes: readonly string[]; files: readonly string[] }>;

/** 🧬️ Checks the closed source-watch envelope before any filesystem observation. */
export function parseNativeSourceWatchPlanV1(value: unknown): NativeSourceWatchPlanV1 {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw Error("Invalid native source-watch plan");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).sort().join(",") !== "excludes,files,includes,schema" || row.schema !== "semio.framework.os.plugin.source-watch-plan/v1") throw Error("Invalid native source-watch schema");
  for (const key of ["includes", "excludes", "files"]) {
    const paths = row[key];
    if (!Array.isArray(paths) || paths.length > 32768 || key === "includes" && !paths.length || paths.some(path => typeof path !== "string" || !path || /^[\/\\]|^[A-Za-z]:/u.test(path) || path.includes("\\") || path.split("/").includes("..")) || new Set(paths).size !== paths.length) throw Error("Invalid native source-watch paths");
  }
  return row as NativeSourceWatchPlanV1;
}

/** 🔎️ Selects an exact source coordinate using the first-party runtime glob implementation. */
export function nativeSourceWatchSelectedV1(plan: NativeSourceWatchPlanV1, path: string): boolean {
  return plan.files.includes(path) || plan.includes.some(glob => new Bun.Glob(glob).match(path)) && !plan.excludes.some(glob => new Bun.Glob(glob).match(path));
}

/** 👀️ Observes admitted owner files, collapsing unchanged writes and retiring all watchers on cancellation. */
export async function startNativeSourceWatchV1(workspace: string, authored: NativeSourceWatchPlanV1, changed: (path: string) => void, options: Readonly<{ signal?: AbortSignal; onProgress?: (completed: number) => void; selected?: (path: string) => boolean }> = {}): Promise<Readonly<{ close(): Promise<void> }>> {
  const plan = parseNativeSourceWatchPlanV1(authored), root = realpathSync(workspace), hashes = new Map<string, string>(), watchers = new Map<string, ReturnType<typeof watch>>(), pending = new Set<string>();
  const selected = options.selected ?? ((path: string) => nativeSourceWatchSelectedV1(plan, path));
  let stopped = false, running: Promise<void> | undefined;
  const cancelled = (): void => { if (stopped || options.signal?.aborted) throw Error("Native source watch cancelled"); };
  const digest = async (path: string): Promise<string> => {
    const full = resolve(root, path), local = relative(root, full);
    if (isAbsolute(local) || local === ".." || local.startsWith("../") || local.startsWith("..\\")) throw Error("Native watch file escapes workspace");
    if (!existsSync(full)) return "missing";
    const info = lstatSync(full);
    if (info.isSymbolicLink() || !info.isFile() || info.size > 32 * 1024 * 1024) throw Error("Native watch file is not a bounded regular source");
    const physical = relative(root, realpathSync(full));
    if (isAbsolute(physical) || physical.startsWith("..")) throw Error("Native watch source escapes workspace through a link");
    return createHash("sha256").update(await readFile(full, { signal: options.signal })).digest("hex");
  };
  const observe = async (path: string): Promise<void> => {
    cancelled();
    if (!selected(path)) return;
    const hash = await digest(path);
    cancelled();
    if (hashes.get(path) !== hash) { hashes.set(path, hash); changed(path); }
  };
  const scan = async (initial: boolean): Promise<void> => {
    const files = new Set<string>();
    for (const pattern of [...plan.includes, ...plan.files]) for await (const path of new Bun.Glob(pattern).scan({ cwd: root, dot: true, onlyFiles: true, followSymlinks: false })) { cancelled(); if (selected(path)) files.add(path); }
    if (files.size > 32768) throw Error("Native watch source count exceeds its boundary");
    for (const [directory, watcher] of watchers) if (!existsSync(directory)) { watcher.close(); watchers.delete(directory); }
    if (!initial) for (const path of hashes.keys()) if (!files.has(path)) await observe(path);
    let completed = 0;
    for (const path of files) {
      cancelled();
      if (initial) hashes.set(path, await digest(path)); else if (!hashes.has(path) || hashes.get(path) === "missing") await observe(path);
      let directory = dirname(join(root, path));
      while (true) {
        if (!watchers.has(directory)) {
          const sourceDirectory = directory;
          watchers.set(directory, watch(directory, (_kind, file) => {
            if (stopped) return;
            if (file !== null) pending.add(relative(root, join(sourceDirectory, String(file))).replaceAll("\\", "/"));
            schedule();
          }));
        }
        if (directory === root) break;
        directory = dirname(directory);
      }
      options.onProgress?.(++completed);
    }
  };
  const schedule = (): void => {
    if (running || stopped) return;
    running = (async () => {
      while (pending.size && !stopped) {
        const paths = [...pending]; pending.clear();
        for (const path of paths) {
          const full = join(root, path);
          if (!existsSync(full) || lstatSync(full).isFile()) await observe(path);
        }
        await scan(false);
      }
    })().catch(error => { if (!stopped) console.error("Native source observation failed", error); }).finally(() => { running = undefined; if (pending.size) schedule(); });
  };
  const stop = (): void => { stopped = true; for (const watcher of watchers.values()) watcher.close(); watchers.clear(); pending.clear(); };
  options.signal?.addEventListener("abort", stop, { once: true });
  try { await scan(true); cancelled(); }
  catch (error) { stop(); options.signal?.removeEventListener("abort", stop); throw error; }
  return { async close() { stop(); options.signal?.removeEventListener("abort", stop); await running; hashes.clear(); } };
}
