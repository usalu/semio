import { dirname, join, resolve } from "node:path";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import type { SemioBuildMode } from "../../🏃️process/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import { readBuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";

/** 📎️ Reads `{ hash, items }` collection blocks from kit snapshot JSON. */
export function fixtureItemsOf<T = Record<string, unknown>>(node: unknown): readonly T[] {
  if (node && typeof node === "object" && Array.isArray((node as { items?: unknown[] }).items)) {
    return (node as { items: T[] }).items;
  }
  return [];
}

function defaultCliBin(root: string, mode: SemioBuildMode): string {
  const win = process.platform === "win32";
  return join(root, ".🧬semio/🦑️repo/⚡️cache/🗃️bin", mode, "repo", win ? "repo.exe" : "repo");
}

export function resolveCliBin(root = getWorkspaceRoot(), mode: SemioBuildMode = readBuildMode(process.env.SEMIO_BUILD_MODE ?? "prototype")): string {
  const fromEnv = process.env.REPO_CLI_BIN?.trim();
  if (fromEnv) return resolve(fromEnv);
  return defaultCliBin(root, readBuildMode(mode));
}

/** 🗃️ The MCP server is build output, so it lives in the marked repository cache and never in the tree. */
function defaultMcpBin(root: string, mode: SemioBuildMode): string {
  const win = process.platform === "win32";
  return join(root, ".🧬semio", "🦑️repo", "⚡️cache", "🗃️bin", mode, "repo", win ? "repo.exe" : "repo");
}

/** 🔌️Resolves the native repo MCP binary at the unified repository executable. */
export function resolveMcpBin(root = getWorkspaceRoot(), mode: SemioBuildMode = readBuildMode(process.env.SEMIO_BUILD_MODE ?? "prototype")): string {
  const fromEnv = process.env.REPO_MCP_BIN?.trim();
  if (fromEnv) return resolve(fromEnv);
  return defaultMcpBin(root, readBuildMode(mode));
}

/** 📡️Runs repo client with `--json` and returns parsed GraphQL payload (`data` object). */
export function runCliGraphql(query: string, variables: Record<string, unknown> = {}, options?: { cwd?: string; repoRoot?: string }): unknown {
  const root = options?.repoRoot ?? getWorkspaceRoot();
  const cwd = options?.cwd ?? root;
  const bin = resolveCliBin(root);
  const vars = JSON.stringify(variables ?? {});
  const args = ["--repo", root, "--json", "graphql", "--query", query, "-v", vars];
  let stdout: string;
  try {
    stdout = execFileSync(bin, args, {
      cwd,
      encoding: "utf8",
      maxBuffer: 64 * 1024 * 1024,
    });
  } catch (e: unknown) {
    const err = e as { stderr?: Buffer; status?: number; message?: string };
    const msg = err.stderr?.toString?.() ?? err.message ?? String(e);
    throw new Error(`[repo/cli] exit ${err.status ?? "?"}: ${msg}`);
  }
  const lines = stdout
    .split(/\r?\n/)
    .map((l) => l.trim())
    .filter(Boolean);
  let last: unknown;
  for (const line of lines) {
    try {
      last = JSON.parse(line) as { data?: unknown; errors?: { message: string }[] };
    } catch {
      continue;
    }
  }
  if (!last || typeof last !== "object") {
    throw new Error(`[repo/cli] no JSON lines in stdout: ${stdout.slice(0, 500)}`);
  }
  const payload = last as { data?: unknown; errors?: { message: string }[] };
  if (payload.errors?.length) {
    throw new Error(`[repo/cli] graphql errors: ${payload.errors.map((x) => x.message).join("; ")}`);
  }
  return payload.data ?? payload;
}

const __dirname = dirname(fileURLToPath(import.meta.url));

/** 🧭️Package dir for `@semio-tech/repo-lib` (…/repo/lib/js). */
export function getLibRoot(): string {
  return resolve(__dirname, "../../📦️packages");
}
