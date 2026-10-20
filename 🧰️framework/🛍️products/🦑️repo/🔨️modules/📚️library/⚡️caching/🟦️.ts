import{capturePhysicalSourceTextV1,physicalSourceOriginV1,type PhysicalSourceModuleMetaV1}from"../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import { join } from "node:path";
import {requireDeclaredCacheSegmentsV1} from "./💾️storage/🧾️ledger/🟦️.ts";

let generatedDirectories: ReadonlySet<string> | undefined;

/** ⚡️ The single repository cache root shared by every tool, agent and dev (`.🧬semio/🦑️repo/⚡️cache/…`); it refuses every home the cache ledger does not declare. */
export function repoCacheDirectory(repoRoot: string, ...segments: string[]): string {
  requireDeclaredCacheSegmentsV1(segments);
  return join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", ...segments);
}

/** 📁️ True when one directory name is a policy-declared generated directory (`dist`, `🗑️generated`, …). */
export function isGeneratedDirectoryName(name: string): boolean {
  generatedDirectories ??= new Set(JSON.parse(capturePhysicalSourceTextV1(new URL("./🔣️policy.json", physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),65536)).generatedDirectories);
  return generatedDirectories.has(name);
}

/** 🗑️ True when a path lies inside a policy-declared generated directory: disposable output that never feeds source inputs. */
export function isGeneratedPath(path: string): boolean {
  return path.split(/[\\/]/u).some(isGeneratedDirectoryName);
}
