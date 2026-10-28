import { RuntimeCapabilityMissingV1, runtimeMessageV1 } from "../../../../🗣️messages/🟦️.ts";
import { defaultNativeToolchainPortsV1, nativeToolchainHostV1, probeNativeToolchainV1, type NativeToolProbeV1 } from "../../../../🧲️native/🧰️toolchain/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import { defaultNodeNativePortsV1, ensureNodeNativeAddonV1, planNodeNativeAddonV1, type NodeNativePortsV1 } from "../🟦️.ts";

const environment = schema.definitions.Protocol.const.addon.environment;

/** 🏷️ Whether the Node native core of this source exists, where, and whether a prebuilt core named by `SEMIO_RUNTIME_NATIVE_ADDON` stands in for it. */
export type NodeNativeCoreStateV1 = Readonly<{ built: boolean; file: string; designated: boolean }>;
/** 🔬️ What the machine offers for building the core: the verdict on each prerequisite of its platform, read lazily because a built core needs none. */
export type NodeNativeToolchainV1 = () => readonly NativeToolProbeV1[];

/** 🗺️ Reads, without building or locking anything, whether the core of the current source is in the cache. */
export function nodeNativeCoreStateV1(ports: NodeNativePortsV1): NodeNativeCoreStateV1 {
  const named = ports.host.env[environment.addon];
  if (named !== undefined && named !== "") return Object.freeze({ built: ports.exists(named), file: named, designated: true });
  const plan = planNodeNativeAddonV1(ports.host, ports.read(ports.host.source));
  return Object.freeze({ built: ports.exists(plan.file), file: plan.file, designated: false });
}

/** 🛠️ Builds the core ahead of its first use, once per source: a built core is left alone; otherwise every missing prerequisite (Rust compiler, C linker) is named at once in both languages before any compiler is started, and then the one locked, atomically published build of the addon module runs. */
export function prepareNodeNativeCoreV1(ports: NodeNativePortsV1, toolchain: NodeNativeToolchainV1): NodeNativeCoreStateV1 {
  const state = nodeNativeCoreStateV1(ports);
  if (state.built || state.designated) return state;
  const missing = toolchain().filter(probe => !probe.found).map(probe => probe.id);
  if (missing.length > 0) throw new RuntimeCapabilityMissingV1("ffi", "node", runtimeMessageV1("nativePrebuildToolchain", { platform: ports.host.platform, arch: ports.host.arch, missing: missing.join(", ") }));
  const file = ensureNodeNativeAddonV1(ports);
  return Object.freeze({ built: ports.exists(file), file, designated: false });
}

/** 📟️ Runs `prepare` or `verify` and answers the exit status: `verify` is 1 while the core is not built, `prepare` is 1 with the refusal printed. The build itself is one synchronous compiler call with its own timeout, so a stop signal ends the process between steps and the half-built staging directory is never published. */
export function runNodeNativePrepareV1(arguments_: readonly string[], ports: NodeNativePortsV1 = defaultNodeNativePortsV1(), toolchain: NodeNativeToolchainV1 = () => probeNativeToolchainV1(nativeToolchainHostV1(ports.host.env), defaultNativeToolchainPortsV1()), write: (line: string) => void = line => { process.stdout.write(`${line}\n`); }): number {
  const [command, ...rest] = arguments_;
  if ((command !== "prepare" && command !== "verify") || rest.length > 0) { write(runtimeMessageV1("nativePrebuildUsage", {})); return 2; }
  try {
    const state = command === "verify" ? nodeNativeCoreStateV1(ports) : prepareNodeNativeCoreV1(ports, toolchain);
    if (!state.built) { write(runtimeMessageV1("nativePrebuildMissing", { file: state.file })); return 1; }
    write(runtimeMessageV1(state.designated ? "nativePrebuildDesignated" : "nativePrebuildReady", { file: state.file }));
    return 0;
  } catch (error) { write((error as Error).message); return 1; }
}
