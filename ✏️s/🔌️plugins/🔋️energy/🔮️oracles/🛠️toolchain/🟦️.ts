import { createHash } from "node:crypto";
import { createReadStream, existsSync, mkdirSync, readFileSync, statSync } from "node:fs";
import { arch, platform } from "node:os";
import { join, resolve } from "node:path";
import { getRepoMetaDir, runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

export type OraclePlatformAsset = { asset: string; bytes: number; sha256: string };
export type OracleToolManifest = { tool: string; version: string; bundles: Record<string, string>; downloadUrlTemplate: string; stripComponents: number; binaries: Record<string, string>; platforms: Record<string, OraclePlatformAsset> };
export type OracleManifest = { tools: OracleToolManifest[]; python: { version: string } };
const ORACLE_PLATFORMS = ["darwin-arm64", "darwin-x64", "linux-arm64", "linux-x64", "win32-arm64", "win32-x64"] as const;
const ORACLE_MANIFEST = "✏️s/🔌️plugins/🔋️energy/🔮️oracles/🛠️toolchain/🔣️.json";

/** 🛂️ Admits only complete, literal, cross-platform toolchain pins. */
export function validateOracleManifest(value: unknown): OracleManifest {
  const manifest = value as any;
  if (!manifest || !Array.isArray(manifest.tools) || manifest.tools.length === 0 || manifest.python?.version !== "3.12") throw new Error("Energy oracle manifest identity is invalid");
  const tools = new Set<string>();
  for (const tool of manifest.tools) {
    if (typeof tool.tool !== "string" || tools.has(tool.tool) || typeof tool.version !== "string" || !tool.downloadUrlTemplate?.includes("{asset}") || !Number.isSafeInteger(tool.stripComponents) || tool.stripComponents < 0) throw new Error("Energy oracle tool identity is invalid");
    tools.add(tool.tool);
    if (JSON.stringify(Object.keys(tool.platforms ?? {}).sort()) !== JSON.stringify([...ORACLE_PLATFORMS].sort()) || Object.keys(tool.binaries ?? {}).length === 0) throw new Error(`${tool.tool} platform or binary closure is incomplete`);
    for (const relative of Object.values(tool.binaries) as string[]) if (!relative || relative.includes("\\") || relative.split("/").some((part) => !part || part === "." || part === "..")) throw new Error(`${tool.tool} binary path is not literal-relative`);
    for (const pin of Object.values(tool.platforms) as OraclePlatformAsset[]) if (!pin.asset || pin.asset.includes("/") || pin.asset.includes("\\") || !Number.isSafeInteger(pin.bytes) || pin.bytes <= 0 || !/^(?!0{64}$)[0-9a-f]{64}$/u.test(pin.sha256)) throw new Error(`${tool.tool} archive pin is invalid`);
  }
  return manifest as OracleManifest;
}

/** 🖥️ Returns the exact platform key used by the pinned oracle asset table. */
export function oraclePlatformKey(): string {
  return `${platform()}-${arch()}`;
}

/** 📇️ Reads the committed toolchain pin table from its semantic owner. */
export function oracleManifest(repoRoot: string): OracleManifest {
  return validateOracleManifest(JSON.parse(readFileSync(resolve(repoRoot, ORACLE_MANIFEST), "utf8")));
}

/** 🗂️ Resolves one pinned toolchain's repository cache owner. */
export function oracleToolDirectory(repoRoot: string, tool: OracleToolManifest): string {
  return join(getRepoMetaDir(repoRoot), "⚡️cache", "oracles", `${tool.tool}-${tool.version}-${oraclePlatformKey()}`);
}

/** 🔐️ Computes the streaming SHA-256 of an archive. */
export async function fileSha256(path: string): Promise<string> {
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(path)) hash.update(chunk as Uint8Array);
  return hash.digest("hex");
}

/** 📥️ Materializes and verifies one pinned oracle tool only when its local files are absent. */
export async function ensureOracleTool(repoRoot: string, tool: OracleToolManifest): Promise<string> {
  const key = oraclePlatformKey();
  const pin = tool.platforms[key];
  if (!pin) throw new Error(`${tool.tool} ${tool.version} has no pinned asset for ${key} (see 🔣️.json)`);
  const directory = oracleToolDirectory(repoRoot, tool);
  const archive = join(directory, pin.asset);
  const marker = join(directory, Object.values(tool.binaries)[0]!);
  mkdirSync(directory, { recursive: true });
  if (!existsSync(archive) || statSync(archive).size !== pin.bytes) {
    const url = tool.downloadUrlTemplate.replace("{asset}", encodeURIComponent(pin.asset));
    console.log(`[oracle] downloading ${pin.asset} (${(pin.bytes / 1e6).toFixed(1)} MB) from ${url}`);
    runCmd("curl", ["-fSL", "--retry", "3", "-o", archive, url], { cwd: directory });
  }
  const digest = await fileSha256(archive);
  if (digest !== pin.sha256) throw new Error(`${pin.asset} sha256 mismatch: expected ${pin.sha256}, got ${digest}`);
  console.log(`[oracle] verified ${pin.asset} sha256 ${digest}`);
  if (!existsSync(marker)) {
    console.log(`[oracle] extracting into ${directory}`);
    runCmd("tar", ["-xzf", archive, "-C", directory, "--strip-components", String(tool.stripComponents)], { cwd: directory });
  }
  if (!existsSync(marker)) throw new Error(`${tool.tool} archive extracted but ${marker} is missing`);
  return directory;
}

/** 🌍️ Binds every declared tool root into an oracle process environment. */
export function oracleEnvironment(repoRoot: string): NodeJS.ProcessEnv {
  const resolved: NodeJS.ProcessEnv = { ...process.env };
  for (const tool of oracleManifest(repoRoot).tools) resolved[`SEMIO_ORACLE_${tool.tool.toUpperCase()}_ROOT`] = oracleToolDirectory(repoRoot, tool);
  return resolved;
}
