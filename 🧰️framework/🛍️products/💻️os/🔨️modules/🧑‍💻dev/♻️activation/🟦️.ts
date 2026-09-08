import { existsSync, lstatSync, mkdirSync, readFileSync, renameSync, rmSync, watch, writeFileSync } from "node:fs";
import { randomUUID } from "node:crypto";
import { join } from "node:path";

export const ACTIVATION_RECEIPT_FILE = "🔣️receipt.json";
export type ActivationArtifact = { readonly pluginId: string; readonly artifactSha256: string };
export type ActivationReceipt = {
  readonly schema: "semio.dev.activation/v1";
  readonly variant: string;
  readonly profile: "dev" | "release";
  readonly plugins: readonly (ActivationArtifact & { readonly rebuiltAt: number })[];
};

const identity = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;
const keys = (value: unknown, expected: readonly string[]): value is Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value) && Object.keys(value).sort().join() === [...expected].sort().join();

/** 🧾️ Validates the completed runtime receipt defined by {@link ./🧬️schema/🔣️.json} `#/$defs/DevActivationV1`. */
export function parseActivationReceipt(value: unknown): ActivationReceipt {
  if (!keys(value, ["schema", "variant", "profile", "plugins"]) || value.schema !== "semio.dev.activation/v1" || typeof value.variant !== "string" || !identity.test(value.variant) || !["dev", "release"].includes(String(value.profile)) || !Array.isArray(value.plugins)) throw new Error("Invalid activation receipt");
  const seen = new Set<string>();
  for (const row of value.plugins) {
    if (!keys(row, ["pluginId", "artifactSha256", "rebuiltAt"]) || typeof row.pluginId !== "string" || !identity.test(row.pluginId) || typeof row.artifactSha256 !== "string" || !/^[a-f0-9]{64}$/.test(row.artifactSha256) || !Number.isSafeInteger(row.rebuiltAt) || Number(row.rebuiltAt) < 1) throw new Error("Invalid activation plugin");
    if (seen.has(row.pluginId)) throw new Error(`Duplicate activation plugin: ${row.pluginId}`);
    seen.add(row.pluginId);
  }
  return value as unknown as ActivationReceipt;
}

/** 🕰️ Keeps warm activations unchanged and advances changed content despite clock rollback. */
export function nextActivationReceipt(variant: string, profile: "dev" | "release", completed: readonly ActivationArtifact[], previous?: ActivationReceipt, now = Date.now()): ActivationReceipt {
  if (previous) parseActivationReceipt(previous);
  if (previous && (previous.variant !== variant || previous.profile !== profile)) throw new Error("Activation receipt identity mismatch");
  if (!Number.isSafeInteger(now)) throw new Error("Invalid activation clock");
  const prior = new Map(previous?.plugins.map((row) => [row.pluginId, row]));
  const timestamp = Math.max(now, 1, ...[...prior.values()].map((row) => row.rebuiltAt + 1));
  return parseActivationReceipt({ schema: "semio.dev.activation/v1", variant, profile, plugins: [...completed].sort((a, b) => a.pluginId < b.pluginId ? -1 : a.pluginId > b.pluginId ? 1 : 0).map((row) => ({ ...row, rebuiltAt: prior.get(row.pluginId)?.artifactSha256 === row.artifactSha256 ? prior.get(row.pluginId)!.rebuiltAt : timestamp })) });
}

/** 📖️ Reads only explicit activation completion, never the existence or mtime of cached outputs. */
export function readActivationReceipt(directory: string): ActivationReceipt {
  return parseActivationReceipt(JSON.parse(readFileSync(join(directory, ACTIVATION_RECEIPT_FILE), "utf8")));
}

/** 🗂️ Separates each variant/profile's ephemeral activation and installations from cached artifacts. */
export function developmentRuntimeRoot(packageRoot: string, variant: string, profile: "dev" | "release"): string {
  parseActivationReceipt({ schema: "semio.dev.activation/v1", variant, profile, plugins: [] });
  return join(packageRoot, "dist", "runtime", profile, variant);
}

/** 📬️ Atomically announces completed preparation without rewriting a warm receipt. */
export function publishActivationReceipt(directory: string, receipt: ActivationReceipt): boolean {
  const text = JSON.stringify(parseActivationReceipt(receipt)) + "\n", destination = join(directory, ACTIVATION_RECEIPT_FILE);
  if (existsSync(destination)) {
    if (lstatSync(destination).isSymbolicLink()) throw new Error("Invalid activation receipt path");
    const previous = readActivationReceipt(directory);
    if (previous.variant !== receipt.variant || previous.profile !== receipt.profile) throw new Error("Activation receipt identity mismatch");
    if (JSON.stringify(previous) + "\n" === text) return false;
  }
  mkdirSync(directory, { recursive: true });
  const temporary = join(directory, `.receipt-${randomUUID()}.stage`);
  try { writeFileSync(temporary, text, { flag: "wx" }); renameSync(temporary, destination); }
  finally { rmSync(temporary, { force: true }); }
  return true;
}

/** 👀️ Observes atomic completion receipts and owns its filesystem subscription. */
export function observeActivationReceipts(directory: string, listener: (receipt: ActivationReceipt) => void, onError: (error: unknown) => void): { snapshot: () => ActivationReceipt; close: () => void } {
  let current: ActivationReceipt, serialized: string | undefined, closed = false;
  const refresh = (): void => {
    if (closed) return;
    const next = readActivationReceipt(directory), text = JSON.stringify(next);
    if (serialized === text) return;
    current = next;
    serialized = text;
    listener(next);
  };
  const watcher = watch(directory, () => {
    try { refresh(); } catch (error) { onError(error); }
  });
  watcher.on("error", onError);
  const close = (): void => { if (!closed) { closed = true; watcher.close(); } };
  try { refresh(); } catch (error) { close(); throw error; }
  return { snapshot: () => current, close };
}
