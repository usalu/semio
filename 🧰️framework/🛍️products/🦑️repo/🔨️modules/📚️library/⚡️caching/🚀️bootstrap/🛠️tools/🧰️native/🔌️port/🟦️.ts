import {captureOwnedCommandV1,runOwnedCommand} from "../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {readIssuedResourceControlCapabilitiesV1,requireIssuedResourceControlV1} from "../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {repositoryCargoPreparationStorageV1} from "../../../../../📦️packages/🟦️typescript/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import {runTool} from "../../../../../📦️packages/🟦️typescript/💾️storage/⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts";
import type {NativeToolPortV1} from "../🟦️.ts";

/** 🔌️ The system port of provisioning: the platform network, child programs through the repository runner, the standard log. */
export function systemNativeToolPortV1(workspace: string, offline: boolean): NativeToolPortV1 {
  const storage = repositoryCargoPreparationStorageV1(workspace);
  return Object.freeze({
    platform: process.platform,
    offline,
    environment: process.env,
    fetch: (url: string, signal: AbortSignal) => fetch(url, {signal}),
    run: async (command: string, args: readonly string[], cwd: string, signal: AbortSignal) => { await runTool(command, [...args], cwd, signal, false, process.env, storage); },
    capture: (command: string, args: readonly string[], cwd: string, signal: AbortSignal) => runTool(command, [...args], cwd, signal, true, process.env, storage),
    log: (text: string) => console.log(text),
  });
}

/** 🎟️ Runs archive extraction and pin probes under the exact held Original control and caller network authority. */
export function ownedNativeToolPortV1(environment: Readonly<Record<string, string | undefined>>, control: ResourceLeaseControl): NativeToolPortV1 {
  requireIssuedResourceControlV1(control);
  const authority = readNativeCallerAuthorityV1(readIssuedResourceControlCapabilitiesV1(control)), output = {maximumBytes: Math.min(65536, authority.transport.maximumBytes), maximumLines: Math.min(256, authority.transport.maximumLines)};
  const check = (signal: AbortSignal): void => { requireIssuedResourceControlV1(control); if (signal !== control.signal) throw Error("Exact Original native tool signal required"); signal.throwIfAborted(); };
  return Object.freeze({
    platform: process.platform,
    offline: authority.network.offline,
    environment,
    fetch: async (url: string, signal: AbortSignal) => { check(signal); if (authority.network.offline) throw Error("Offline Original native tool network refused"); return fetch(url, {signal}); },
    run: async (command: string, args: readonly string[], cwd: string, signal: AbortSignal) => { check(signal); await runOwnedCommand(command, [...args], cwd, "native-tools:" + command, 0, {control, signal, env: environment, output}); },
    capture: async (command: string, args: readonly string[], cwd: string, signal: AbortSignal) => { check(signal); return captureOwnedCommandV1(command, args, cwd, {control, signal, env: environment, output}); },
    log: (text: string) => console.log(text),
  });
}
