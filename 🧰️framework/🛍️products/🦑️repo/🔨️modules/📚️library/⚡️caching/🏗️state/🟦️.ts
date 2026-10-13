import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { createHash } from "node:crypto";
import { mkdir } from "node:fs/promises";
import { join } from "node:path";
import { readBuildMode, type BuildMode } from "../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { acquireResourceLease } from "../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";

/** 🗃️ Owns one compiler's mutable intermediates independently of its staged deliverable. */
export type CompilerStateOwner = Readonly<{ root: string; owner: string; compiler: string; configuration: string; mode: BuildMode; platform?: string; architecture?: string }>;

/** 🔑️ Partitions state by compiler, configuration, platform, architecture, owner and intent. */
export function compilerStateDirectory(options: CompilerStateOwner): string {
  const failures = validateJsonSchemaSubset(schema, Object.fromEntries(Object.entries(options).filter(([key, value]) => key !== "signal" && value !== undefined)));
  if (failures.length) throw Error("Invalid compiler state owner: " + failures.join("; "));
  const mode = readBuildMode(options.mode), platform = options.platform ?? process.platform, architecture = options.architecture ?? process.arch;
  const identity = createHash("sha256").update(JSON.stringify([options.owner, options.compiler, options.configuration, platform, architecture])).digest("hex").slice(0, 24);
  return join(options.root, "compiler", `${platform}-${architecture}`, mode, identity);
}

/** 🔒️ Serializes access to owned mutable compiler state while retaining warm intermediates. */
export async function withCompilerState<T>(options: CompilerStateOwner & { readonly signal: AbortSignal }, operation: (directory: string) => Promise<T>): Promise<T> {
  const directory = compilerStateDirectory(options);
  const lease = await acquireResourceLease({ directory: join(options.root, "agents/resource-leases"), resource: `compiler-state:${directory}`, mode: "exclusive", signal: options.signal });
  try { options.signal.throwIfAborted(); await mkdir(directory, { recursive: true }); return await operation(directory); }
  finally { lease.release(); }
}
