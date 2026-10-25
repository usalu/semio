import {readViteRuntimeChannelsV1,type ViteRuntimeChannelV1} from "../📥️environment/🏃️runtime/🟦️.ts";
import type {CompilerAdmissionControl} from "@semio-tech/process/compiler-capacity";
import {checkScriptInvocation,requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {readBuildMode} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import type {RepositoryArtifactRequestV1} from "../../📦️artifacts/👓️reading/🟦️.ts";
import {playPollingEnv} from "../../../🏃️process/🌿️environment/🟦️.ts";
import {isDevPortInUse,devServerUrl,devServerPlayEntry,describeDevPortOccupant,canReuseDevPort,resolveDevPort} from "../../../🌐️network/🔌️ports/🟦️.ts";
import type {ViteConfigLoaderV1} from "../🎛️loader/🟦️.ts";
import {resolve} from "node:path";
import type { RuntimeIdV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📋️contract/🟦️.ts";
/** ▶️Vite dev via `bunx` with an explicitly owned configuration source. */
export async function runViteBunxDev(
  invocation: ScriptInvocation,
  control:CompilerAdmissionControl,
  bundleRoot: string,
  segments: string[],
  opts: {
    runtimeChannels:readonly ViteRuntimeChannelV1[];
    config: string;
    configLoader?: ViteConfigLoaderV1;
    runtime?: RuntimeIdV1;
    artifactInputs?: readonly RepositoryArtifactRequestV1[];
    portEnv?: string;
    defaultPort?: string;
    strictPort?: boolean;
    /** 📌️ When true, bind only `defaultPort` / env — never bump into another playground port. */
    fixedPort?: boolean;
    reservedPorts?: ReadonlySet<number>;
    env?: NodeJS.ProcessEnv;
    /** 🎮️ When set with `fixedPort`, only reuse an existing listener serving this play entry. */
    expectedPlayEntry?: string;
  },
): Promise<void> {
  requireIssuedResourceControlInvocationV1(control,invocation);const runtimeChannels=readViteRuntimeChannelsV1(opts.runtimeChannels);checkScriptInvocation(invocation);
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  const preferredPort = Number(process.env[opts.portEnv ?? "VITE_PORT"] ?? opts.defaultPort ?? "5173");
  const spawnEnv = playPollingEnv(opts.env);
  if (opts.fixedPort && isDevPortInUse(host, preferredPort)) {
    const url = devServerUrl(host, preferredPort);
    if (canReuseDevPort(host, preferredPort, opts.expectedPlayEntry)) {
      console.log(`[dev] Port ${preferredPort} is already in use — dev server appears to be running at ${url}`);
      return Promise.resolve();
    }
    const occupant = describeDevPortOccupant(preferredPort);
    const servedEntry = devServerPlayEntry(host, preferredPort);
    if (servedEntry && opts.expectedPlayEntry && servedEntry !== opts.expectedPlayEntry) {
      console.error(`[dev] Port ${preferredPort} is serving play entry "${servedEntry}" but "${opts.expectedPlayEntry}" was requested. Stop that process or set ${opts.portEnv ?? "VITE_PORT"}.`);
      process.exit(1);
    }
    console.error(`[dev] Port ${preferredPort} is already in use${occupant ? ` by ${occupant}` : ""}. Stop that process or set ${opts.portEnv ?? "VITE_PORT"}.`);
    process.exit(1);
  }
  const port = (() => {
    if (opts.fixedPort) {
      return preferredPort;
    }
    const skip = opts.reservedPorts ?? new Set<number>();
    const resolved = resolveDevPort(host, preferredPort, 20, skip);
    if (resolved !== preferredPort) {
      console.warn(`[dev] Port ${preferredPort} is already in use — starting on ${resolved} instead.`);
      if (opts.portEnv) process.env[opts.portEnv] = String(resolved);
    }
    return resolved;
  })();
  let selectedHost=host,selectedPort=port,strictPort=opts.strictPort??true;
  const seen=new Set<string>();
  for(let index=0;index<segments.length;index++){
    const flag=segments[index]!;if(seen.has(flag))throw Error("Duplicate Vite service option");seen.add(flag);
    if(flag==="--strictPort"||flag==="--no-strictPort"){strictPort=flag==="--strictPort";continue;}
    const value=segments[++index];if(!value||!["--host","--port"].includes(flag))throw Error("Vite service accepts only explicit host, port and strictPort options");
    if(flag==="--host")selectedHost=value;else selectedPort=Number(value);
  }
  const {serveViteCommandV1}=await import("../🟦️.ts");
  await serveViteCommandV1({runtimeChannels,invocation,control,root:bundleRoot,config:resolve(bundleRoot,opts.config),configLoader:opts.configLoader,mode:readBuildMode(spawnEnv.SEMIO_BUILD_MODE),host:selectedHost,port:selectedPort,strictPort,environment:spawnEnv,artifactInputs:opts.artifactInputs,runtime:opts.runtime,signal:invocation.control.signal});
}

