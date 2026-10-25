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
