import { randomBytes } from "node:crypto";
import { existsSync, realpathSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { acquireQueuedResourceLease, type LeaseWait, type ResourceLease } from "../../../🔒️leases/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🪪️ One canonical compiler directory and profile protected before any Cargo unit lock. */
export type CargoBuildLeaseIdentityV1 = { readonly buildDirectory: string; readonly profile: string; readonly mode: "exclusive" };

/** 🧭️ Resolves actual Cargo profile aliases and physical cache identity from first-party schema rules. */
export function cargoBuildLeaseIdentityV1(buildDirectory: string, args: readonly string[]): CargoBuildLeaseIdentityV1 {
  const argumentsBeforeCompiler = args.slice(0, args.includes("--") ? args.indexOf("--") : args.length);
  const profiles: string[] = [];
  for (let index = 0; index < argumentsBeforeCompiler.length; index++) {
    const arg = argumentsBeforeCompiler[index]!;
    if (arg === "--release") profiles.push("release");
    else if (arg === "--profile") { const value = argumentsBeforeCompiler[++index]; if (!value) throw Error("Cargo profile requires a value"); profiles.push(value); }
    else if (arg.startsWith("--profile=")) profiles.push(arg.slice(10));
  }
  if (profiles.length > 1) throw Error("Cargo build declares multiple profiles");
  const authored = profiles[0] ?? schema.definitions.Protocol.const.defaultProfile;
  if (!new RegExp(schema.properties.profile.pattern, "u").test(authored)) throw Error("Invalid Cargo build profile");
  const aliases = schema.definitions.Protocol.const.profileDirectories as Record<string, string>;
  const profile = aliases[authored] ?? authored;
  let existing = resolve(buildDirectory);
  const suffix: string[] = [];
  while (!existsSync(existing)) { const parent = dirname(existing); if (parent === existing) throw Error("Cargo build directory has no physical ancestor"); suffix.unshift(basename(existing)); existing = parent; }
  const canonical = join(realpathSync(existing), ...suffix).replaceAll("\\", "/").normalize("NFC");
  if (canonical.length > schema.properties.buildDirectory.maxLength || canonical.includes("\0")) throw Error("Invalid Cargo build directory");
  const mode = schema.properties.mode.const;
  if (mode !== "exclusive") throw Error("Cargo build lease requires exclusive compiler ownership");
  return { buildDirectory: canonical, profile, mode };
}

/** 🔒️ Queues one compiler process while keeping different profile caches independent. */
export async function acquireCargoBuildLeaseV1(options: { readonly directory: string; readonly buildDirectory: string; readonly args: readonly string[]; readonly signal: AbortSignal; readonly onWait?: (progress: LeaseWait) => void }): Promise<ResourceLease> {
  const identity = cargoBuildLeaseIdentityV1(options.buildDirectory, options.args);
  const resource = JSON.stringify([schema.definitions.Protocol.const.namespace, identity.buildDirectory, identity.profile]);
  return acquireQueuedResourceLease({ directory: options.directory, resource, mode: identity.mode, owner: randomBytes(16).toString("base64url"), signal: options.signal, onWait: options.onWait });
}
