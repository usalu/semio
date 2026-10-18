import { existsSync, rmSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { coverageDir } from "../../🏃️process/📋️context/🔣️policy/🟦️.ts";
import { repoCacheDirectory } from "../../⚡️caching/🟦️.ts";
import { loadCatalogTaxonomy } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { orchestratorBudgetOpts, runCmd, runCmdStatus } from "../../🏃️process/🟦️.ts";
import { processTableSnapshot, ACTIVE_SEMIO_TECH_BUILD } from "../../🏃️process/📋️process-table/🟦️.ts";
import { Script } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runTaxonomyCliWorkflow } from "../../🧹️normalization/🎮️command-contract/🔁️workflow/🟦️.ts";
import { cleanCollectEmptyFolderRemovals } from "../🔍️empty-folders/🟦️.ts";
import { cleanCollectMarkerOnlyFolderRemovals } from "../🔍️marker-only-folders/🟦️.ts";
import { cleanProtectedPrefixes, cleanProjectRemovals, CLEAN_PROTECTION_VIEW } from "../🛡️protection/🟦️.ts";
import { cleanRemovePath, runWorkspaceClean } from "../🗑️removal/🟦️.ts";

/**
 * 🧹Workspace cleaner: misplaced emoji mounts, ticket junk, oversized build artifacts, and rustc
 * incremental directories larger than 1 GiB — never map/hub/space, and never the rest of the cache.
 *
 * Two different mechanisms used to share the word "clean". This is the WORKSPACE one. Generated
 * test state is removed by `clean test`, which is owned by the testing domain and refuses to touch
 * anything that is not a marked test output; the architecture rule that used to be called
 * `clean-mechanism` is now the `taxonomy/owner-shape` policy and removes nothing at all.
 */
export class CleanScript extends Script {
  run(segments: string[]): void {
    const entry = readWorkspaceCleanupEntryV1(segments);
    segments = [entry.mode, ...entry.arguments];
    if (segments[0] === "taxonomy") {
      runTaxonomyCliWorkflow(this.root, segments.slice(1));
      return;
    }
    const dry = entry.dry;
    if (segments[0] === "marker-only-folders") {
      const protectedPrefixes = cleanProtectedPrefixes(this.root);
      const pending = cleanCollectMarkerOnlyFolderRemovals(this.root, protectedPrefixes);
      const blocked: string[] = [];
      const removals = cleanProjectRemovals(this.root, pending, protectedPrefixes, CLEAN_PROTECTION_VIEW, (path) => blocked.push(path));
      let bytes = 0;
      for (const row of removals) {
        if (cleanRemovePath(this.root, resolve(this.root, row.path), dry, protectedPrefixes)) bytes += row.bytes;
        else blocked.push(row.path);
      }
      console.log(`[clean marker-only-folders] ${dry ? "dry-run" : "applied"} removals=${removals.length} bytes=${bytes}`);
      for (const row of removals) console.log(`[clean marker-only-folders] ${dry ? "would-remove" : "removed"} ${row.path} (${row.bytes})`);
      for (const path of blocked) console.log(`[clean marker-only-folders] protected ${path}`);
      return;
    }
    if (segments[0] === "empty-folders") {
      const protectedPrefixes = cleanProtectedPrefixes(this.root);
      let totalBytes = 0;
      let totalRemovals = 0;
      const blocked: string[] = [];
      for (;;) {
        const pending = cleanCollectEmptyFolderRemovals(this.root, protectedPrefixes);
        if (pending.length === 0) break;
        const removals = cleanProjectRemovals(this.root, pending, protectedPrefixes, CLEAN_PROTECTION_VIEW, (path) => blocked.push(path));
        let passBytes = 0;
        let passCount = 0;
        for (const row of removals) {
          if (cleanRemovePath(this.root, resolve(this.root, row.path), dry, protectedPrefixes)) {
            passBytes += row.bytes;
            passCount += 1;
            console.log(`[clean empty-folders] ${dry ? "would-remove" : "removed"} ${row.path} (${row.bytes})`);
          } else blocked.push(row.path);
        }
        totalBytes += passBytes;
        totalRemovals += passCount;
        if (passCount === 0) break;
      }
      console.log(`[clean empty-folders] ${dry ? "dry-run" : "applied"} removals=${totalRemovals} bytes=${totalBytes}`);
      for (const path of blocked) console.log(`[clean empty-folders] protected ${path}`);
      return;
    }
    if (segments[0] === "test") {
      // 🧪️ Marker-guarded removal of generated test state, delegated to its owner. Never descends
      // into `compose/`, never follows a symlink, never deletes an unmarked directory.
      const domain = String((loadCatalogTaxonomy() as unknown as Readonly<{ testDomainPath?: string }>).testDomainPath ?? "");
      if (domain === "") throw new Error("🔣️taxonomy.json declares no testDomainPath — `clean test` has no owner to delegate to.");
      runCmd("bun", [join(this.root, domain, "📜️script.ts"), "clean", ...segments.slice(1)], { cwd: join(this.root, domain), ...orchestratorBudgetOpts() });
      return;
    }
    if (segments[0] === "coverage") {
      // 📊️ Generated coverage reports only — no source, no fixture, no other cache entry.
      const removed: string[] = [];
      for (const kind of ["js", "rust", "go", "py", "dotnet"] as const) {
        const dir = coverageDir(this.root, kind);
        if (!existsSync(dir)) continue;
        removed.push(relative(this.root, dir));
        if (!dry) rmSync(dir, { recursive: true, force: true });
      }
      console.log(`[clean coverage] ${dry ? "dry-run" : "applied"} removals=${removed.length}`);
      for (const path of removed) console.log(`[clean coverage] ${dry ? "would-remove" : "removed"} ${path}`);
      return;
    }
    const report = runWorkspaceClean(this.root, dry);
    const totalBytes = report.removals.reduce((n, r) => n + r.bytes, 0);
    const lines = [
      `[clean] ${dry ? "dry-run" : "applied"} removals=${report.removals.length} bytes=${totalBytes}`,
      ...(["misplaced", "root-transient", "gitignore", "ticket-file", "ticket-dir", "ticket-generated", "build-artifact", "cargo-incremental", "windows-illegal"] as const).map((kind) => {
        const rows = report.removals.filter((r) => r.kind === kind);
        return `[clean] ${kind}: ${rows.length} (bytes=${rows.reduce((n, r) => n + r.bytes, 0)})`;
      }),
      ...report.removals.map((r) => `[clean] ${dry ? "would-remove" : "removed"} ${r.kind} ${r.path} (${r.bytes})`),
      ...report.skippedProtected.map((p) => `[clean] protected ${p}`),
    ];
    for (const line of lines) console.log(line);
    this.runCachePrune(dry);
  }

  /** ⚡️Bounds the shared cache root through its own owner instead of size-sweeping it — `clean` never walks or deletes under it directly. */
  private runCachePrune(dry: boolean): void {
    const processes = processTableSnapshot();
    if (!processes) {
      console.log(`[clean] cache-prune skipped: the process table is unreadable, so an active Semio Tech build cannot be ruled out`);
      return;
    }
    if (processes.some((row) => ACTIVE_SEMIO_TECH_BUILD.test(row.command))) {
      console.log(`[clean] cache-prune skipped active Semio Tech build`);
      return;
    }
    const cachingScript = join(this.root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts");
    const status = runCmdStatus("bun", [cachingScript, "cache-prune", ...(dry ? ["--dry-run"] : [])], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log(`[clean] cache-prune ${status === 0 ? "ok" : `unavailable (exit ${status})`} ${repoCacheDirectory(this.root)}`);
  }
}

/** 🧭️ Decodes the complete workspace cleanup entry before observing or changing resources. */
export function readWorkspaceCleanupEntryV1(segments:readonly string[]):Readonly<{mode:"workspace"|"taxonomy"|"test"|"coverage"|"empty-folders"|"marker-only-folders";dry:boolean;arguments:readonly string[]}>{if(!Array.isArray(segments)||segments.some(value=>typeof value!=="string"))throw Error("Closed workspace cleanup command required");const first=segments[0];if(first==="taxonomy"||first==="test")return Object.freeze({mode:first,dry:false,arguments:Object.freeze(segments.slice(1))});const marker=(value:string)=>value==="--dry"||value==="dry",mode=first===undefined||marker(first)?"workspace":first,tail=first===undefined?[]:marker(first)?segments:segments.slice(1);if(!["workspace","coverage","empty-folders","marker-only-folders"].includes(mode)||tail.length>1||tail.some(value=>!marker(value)))throw Error("Closed workspace cleanup command required");return Object.freeze({mode:mode as "workspace"|"coverage"|"empty-folders"|"marker-only-folders",dry:tail.length===1,arguments:Object.freeze([])});}
