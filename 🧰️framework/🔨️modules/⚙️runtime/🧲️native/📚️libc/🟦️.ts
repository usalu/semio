import { requireRuntimeMemberV1, runtimeMessageV1 } from "../../🧭️selection/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
const protocol = schema.definitions.Protocol.const;
/** 🔌️ Answers whether the dynamic loader can open one library by name; production binds it to the runtime, tests to a table. */
export type RuntimeNativeLoadsV1 = (name: string) => boolean;
/** 📚️ Lists the loader names of the C library of one platform and architecture, first choice first; the loader search path resolves every name, so no distribution layout is assumed. See https://man7.org/linux/man-pages/man3/dlopen.3.html */
export function runtimeNativeLibcCandidatesV1(platform: string, arch: string): readonly string[] {
  if (platform === "darwin") return Object.freeze([protocol.darwin]);
  if (platform !== "linux") throw new Error(runtimeMessageV1("nativeLibcPlatform", { platform }));
  const names = (protocol.linux as Readonly<Record<string, readonly string[]>>)[arch];
  if (names === undefined) throw new Error(runtimeMessageV1("nativeLibcArchitecture", { arch }));
  return Object.freeze([...names]);
}
/** 🧭️ Picks the first loader name that opens; a host with none is refused with the names it tried, in English and German. */
export function resolveRuntimeNativeLibcV1(platform: string, arch: string, loads: RuntimeNativeLoadsV1): string {
  const names = runtimeNativeLibcCandidatesV1(platform, arch);
  for (const name of names) if (loads(name)) return name;
  throw new Error(runtimeMessageV1("nativeLibcMissing", { names: names.join(", "), platform, arch }));
}
let resolved: string | undefined;
/** 🧲️ Names the C library this process can bind, probed once through the runtime's own foreign function interface; the name is a loader name, never a distribution path. */
export function runtimeNativeLibcV1(): string {
  if (resolved !== undefined) return resolved;
  const open = requireRuntimeMemberV1("ffiOpen");
  const loads: RuntimeNativeLoadsV1 = name => {
    try { open(name, { [protocol.probeSymbol]: { args: [], returns: "i32" } }).close(); return true; } catch { return false; }
  };
  resolved = resolveRuntimeNativeLibcV1(process.platform, process.arch, loads);
  return resolved;
}
