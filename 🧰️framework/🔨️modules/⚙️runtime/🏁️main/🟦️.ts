import { realpathSync } from "node:fs";
import { fileURLToPath, pathToFileURL } from "node:url";

/** 🏁️ Tells whether the module at `metaUrl` is the process entry point. */
export function isMainModuleV1(metaUrl: string): boolean {
  const entry = process.argv[1];
  if (entry === undefined || entry === "") return false;
  const canonical = (path: string): string => { const real = pathToFileURL(realpathSync(path)).href; return process.platform === "win32" ? real.toLowerCase() : real; };
  try { return canonical(entry) === canonical(fileURLToPath(metaUrl)); } catch { return false; }
}

/** 🛑️ The signals that ask a process to stop. Node delivers `SIGINT` (Ctrl+C) everywhere and `SIGTERM` on POSIX hosts only; a Windows process hears the console instead: `SIGBREAK` (Ctrl+Break) and `SIGHUP` (the console window closes). An external `taskkill` without `/F` posts the same close event, so listening to all four is what makes cancellation reach a Windows child at all. */
export function terminationSignalsV1(platform: string = process.platform): readonly NodeJS.Signals[] {
  return platform === "win32" ? ["SIGINT", "SIGTERM", "SIGBREAK", "SIGHUP"] : ["SIGINT", "SIGTERM"];
}

/** 🛑️ Runs `handler` once when a stop signal arrives and answers the function that removes the listeners again. */
export function onTerminationV1(handler: (signal: NodeJS.Signals) => void, platform: string = process.platform, target: Pick<NodeJS.Process, "once" | "removeListener"> = process): () => void {
  const listeners = terminationSignalsV1(platform).map((signal) => [signal, () => handler(signal)] as const);
  for (const [signal, listener] of listeners) target.once(signal, listener);
  return () => { for (const [signal, listener] of listeners) target.removeListener(signal, listener); };
}
