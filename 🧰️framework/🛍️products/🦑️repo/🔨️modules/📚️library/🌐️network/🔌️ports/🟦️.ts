import { spawnSync } from "node:child_process";
import { runToolSyncV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { runtimeCommandNameV1 } from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
/** 🔌️ True when host:port already accepts TCP (existing dev server). */
export function isDevPortInUse(host: string, port: number): boolean {
  const probe = `
import { createConnection } from "node:net";
const socket = createConnection({ host: ${JSON.stringify(host)}, port: ${port} });
socket.setTimeout(300);
socket.once("connect", () => process.exit(0));
socket.once("timeout", () => process.exit(1));
socket.once("error", () => process.exit(1));
`;
  const result = runToolSyncV1(runtimeCommandNameV1(), ["--input-type=module", "-e", probe], { timeout: 500 });
  return result.status === 0;
}

/** 🌐️ Loopback URL for a dev server bound to `host`/`port`. */
export function devServerUrl(host: string, port: number): string {
  const probeHost = host === "0.0.0.0" ? "127.0.0.1" : host;
  return `http://${probeHost}:${port}/`;
}

/** 🎯️ Reads `import.meta.env.PLAYGROUND_APP_KIND` baked into a running playground dev server. */
export function devServerPlayEntry(host: string, port: number): string | undefined {
  const url = `${devServerUrl(host, port)}index.ts`;
  const probe = `const res = await fetch(${JSON.stringify(url)}, { signal: AbortSignal.timeout(2000) });
if (!res.ok) process.exit(1);
const text = await res.text();
const match = text.match(/PLAYGROUND_APP_KIND\\":\\s*\\"([^\\"]+)\\"/);
process.stdout.write(match?.[1] ?? "");
`;
  const result = runToolSyncV1(runtimeCommandNameV1(), ["--input-type=module", "-e", probe], { encoding: "utf8", timeout: 3000 });
  const entry = result.stdout?.trim();
  return entry || undefined;
}

/** 🔎️ Best-effort description of the process listening on `port` (Unix/macOS/Linux). */
export function describeDevPortOccupant(port: number): string | undefined {
  if (process.platform === "win32") return undefined;
  const result = spawnSync("lsof", ["-nP", `-iTCP:${port}`, "-sTCP:LISTEN"], { encoding: "utf8" });
  const line = result.stdout
    ?.trim()
    .split("\n")
    .find((row, index) => index > 0 && row.trim().length > 0);
  if (!line) return undefined;
  const parts = line.trim().split(/\s+/);
  return parts.length >= 2 ? `${parts[0]} (PID ${parts[1]})` : line.trim();
}

/** ♻️ True when an HTTP server on `port` already responds successfully (reuse existing dev). */
export function canReuseDevPort(host: string, port: number, expectedPlayEntry?: string): boolean {
  const url = devServerUrl(host, port);
  const probe = `const res = await fetch(${JSON.stringify(url)}, { signal: AbortSignal.timeout(2000) });
process.exit(res.ok ? 0 : 1);`;
  const result = runToolSyncV1(runtimeCommandNameV1(), ["--input-type=module", "-e", probe], { timeout: 3000 });
  if (result.status !== 0) return false;
  if (!expectedPlayEntry) return true;
  return devServerPlayEntry(host, port) === expectedPlayEntry;
}

/** 🆓️ First free TCP port at or after `preferredPort` (up to `maxAttempts`), skipping `skipPorts`. */
export function resolveDevPort(host: string, preferredPort: number, maxAttempts = 20, skipPorts: ReadonlySet<number> = new Set()): number {
  for (let offset = 0; offset < maxAttempts; offset++) {
    const port = preferredPort + offset;
    if (skipPorts.has(port)) continue;
    if (!isDevPortInUse(host, port)) return port;
  }
  console.error(`[dev] No free port found in range ${preferredPort}-${preferredPort + maxAttempts - 1}.`);
  process.exit(1);
}

