import {captureOwnedCommandV1,runOwnedCommand} from "../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import{fleetFetchContextV1,pinnedFetchV1}from"../../../../🛰️fleet-fetch/🟦️.ts";
import {readIssuedResourceControlCapabilitiesV1,requireIssuedResourceControlV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import type {NativeToolPortV1} from "../../🟦️.ts";

/** 🎟️ Runs archive extraction and pin probes under the exact held Original control and caller network authority. Lives apart from the system port so the compiler store never reaches the JavaScript dependency bootstrap that the system port runs tools through.
 * @see ../🟦️.ts — `systemNativeToolPortV1`, the system half */
export function ownedNativeToolPortV1(environment: Readonly<Record<string, string | undefined>>, control: ResourceLeaseControl): NativeToolPortV1 {
  requireIssuedResourceControlV1(control);
  const authority = readNativeCallerAuthorityV1(readIssuedResourceControlCapabilitiesV1(control)), output = {maximumBytes: Math.min(65536, authority.transport.maximumBytes), maximumLines: Math.min(256, authority.transport.maximumLines)};
  const check = (signal: AbortSignal): void => { requireIssuedResourceControlV1(control); if (signal !== control.signal) throw Error("Exact Original native tool signal required"); signal.throwIfAborted(); };
  return Object.freeze({
    platform: process.platform,
    offline: authority.network.offline,
    environment,
    fetch: async (url: string, signal: AbortSignal, pin?: Readonly<{sha256: string; bytes: number}>) => { check(signal); if (authority.network.offline) throw Error("Offline Original native tool network refused"); return pin ? pinnedFetchV1(fleetFetchContextV1(environment), {url, ...pin}, {signal}) : fetch(url, {signal}); },
    run: async (command: string, args: readonly string[], cwd: string, signal: AbortSignal) => { check(signal); await runOwnedCommand(command, [...args], cwd, "native-tools:" + command, 0, {control, signal, env: environment, output}); },
    capture: async (command: string, args: readonly string[], cwd: string, signal: AbortSignal) => { check(signal); return captureOwnedCommandV1(command, args, cwd, {control, signal, env: environment, output}); },
    log: (text: string) => console.log(text),
  });
}
