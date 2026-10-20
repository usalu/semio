/** 🧱️ Checked-in asset contracts: the repository files the Repo client relies on but does not own (MCP bootstrap assets, the Postgres kit schema, the build-artifact ignore rules and the micro-commit hook sources) keep their agreed shape. They read the checkout on purpose and never write to it, which is why they live in the repository laws and not in the hermetic client Go suite. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";

const root = resolve(import.meta.dir, "../..");
const modules = "🧰️framework/🛍️products/🦑️repo/🔨️modules";
const read = (path: string): string => readFileSync(join(root, path), "utf8");
const compact = (value: string): string => value.replace(/\s+/gu, "");

type Asset = { name: string; path: string; required: string[]; forbidden: string[] };

const assets: Asset[] = [
  { name: "codex starts the repo MCP server through the repo command", path: ".codex/config.toml", required: ['command = "repo"', 'args = ["mcp"]', 'SEMIO_REPO_MCP_CLIENT = "codex"'], forbidden: ["repo/client/client", "./📜️script.ts"] },
  { name: "generic clients start the repo MCP server through the repo command", path: ".mcp.json", required: ['"command": "repo"', '"args": ["mcp"]', '"SEMIO_REPO_MCP_CLIENT": "client"'], forbidden: ["repo/client/client", "./📜️script.ts"] },
  { name: "devcontainer provisions through the cross-platform workspace setup", path: ".devcontainer/devcontainer.json", required: ['"postCreateCommand": ["bun", "repo", "run", "workspace:setup"]'], forbidden: ["repo/client/client", "./repo/client/mcp/go", "post-create.sh"] },
  { name: "native shell bootstrap installs the repo command and runs the workspace setup through it", path: `${modules}/🔩️native/🥾️bootstrap/🐚️.sh`, required: ["bun repo run repo-mcp:install", "repo run workspace:setup", "repo run @semio-tech/repo-developer-tools:setup-debugger"], forbidden: ["repo/client/client", "./repo/client/mcp/go", "💻️client"] },
  { name: "native windows bootstrap installs the repo command and runs the workspace setup through it", path: `${modules}/🔩️native/🥾️bootstrap/🔵️.ps1`, required: ['@("repo", "run", "repo-mcp:install")', '@("run", "workspace:setup")', '@("run", "@semio-tech/repo-developer-tools:setup-debugger")'], forbidden: ["repo/client/client.exe", "./repo/client/mcp/go"] },
];

for (const asset of assets) {
  test(`MCP bootstrap asset: ${asset.name}`, () => {
    const text = read(asset.path);
    for (const fragment of asset.required) expect(compact(text).includes(compact(fragment)), `${asset.path} must contain ${fragment}`).toBe(true);
    for (const fragment of asset.forbidden) expect(text.includes(fragment), `${asset.path} must exclude ${fragment}`).toBe(false);
  });
}

test("Postgres schema declares the kit version-control tables", () => {
  const schema = read(`${modules}/🖥️server/🧬️schema/🐘️postgres/🗄️.sql`);
  const tables = ["kits", "kit_snapshots", "kit_checkpoints", "kit_alternatives", "kit_sessions", "kit_drafts", "kit_transactions", "kit_releases", "kit_snapshot_families", "kit_snapshot_kind_entities", "kit_snapshot_layouts", "kit_snapshot_layout_pieces", "kit_snapshot_layout_connections", "kit_snapshot_properties", "kit_snapshot_attributes"];
  for (const table of tables) expect(schema.includes(`CREATE TABLE IF NOT EXISTS ${table} (`), table).toBe(true);
  for (const snippet of [
    "CREATE UNIQUE INDEX IF NOT EXISTS idx_kit_transactions_one_open_per_draft ON kit_transactions(draft_id) WHERE state = 'open';",
    "source_json          JSONB NOT NULL DEFAULT '{}'",
    "before_snapshot_json  JSONB NOT NULL DEFAULT '{}'",
    "snapshot_kind        TEXT NOT NULL CHECK (snapshot_kind IN ('initial', 'materialized', 'draft-base', 'session-cache'))",
    "state       TEXT NOT NULL DEFAULT 'open' CHECK (state IN ('open', 'finalized', 'aborted'))",
  ]) expect(schema.includes(snippet), snippet).toBe(true);
});

test("the root ignore rules cover every build binary artifact", () => {
  const artifacts = [
    ".🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo",
    ".🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo.exe",
    ".🧬semio/🦑️repo/⚡️cache/🗃️bin/repo",
    ".🧬semio/🦑️repo/⚡️cache/🗃️bin/repo.exe",
    `${modules}/🖥️server/🎛️coordinator/⚡️implementations/🐹️go/server`,
    `${modules}/🖥️server/🎛️coordinator/⚡️implementations/🐹️go/server.exe`,
    "coda/example/compose-blnbo-roomprogram/.coda/validators/programming.exe",
  ];
  for (const artifact of artifacts) {
    const result = spawnSync("git", ["-C", root, "check-ignore", "-q", "--no-index", "--", artifact], { encoding: "utf8" });
    expect(result.status, `${artifact} must be gitignored`).toBe(0);
  }
});

test("the post-commit micro-commit hook source resets the commit templates", () => {
  const hook = join(root, "🧰️framework/🛍️products/🦑️repo/🪝️hooks/post-commit");
  expect(existsSync(hook)).toBe(true);
  const repository = mkdtempSync(join(tmpdir(), "semio-micro-commit-"));
  try {
    const git = (...args: string[]) => spawnSync("git", args, { cwd: repository, encoding: "utf8" });
    expect(git("init", "-q").status).toBe(0);
    const gitDirectory = join(repository, ".git");
    const template = join(gitDirectory, "gkcommittemplate.txt");
    const message = join(gitDirectory, "COMMIT_EDITMSG");
    for (const path of [template, message]) writeFileSync(path, "draft\n");
    const run = spawnSync(hook, [], { cwd: repository, env: { ...process.env, GIT_DIR: gitDirectory, GIT_WORK_TREE: repository }, encoding: "utf8" });
    expect(run.status, run.stderr).toBe(0);
    expect(readFileSync(message, "utf8")).toBe("");
    expect(readFileSync(template, "utf8")).toBe("");
    expect(git("config", "--local", "--get", "commit.template").stdout).toContain("gkcommittemplate.txt");
  } finally {
    rmSync(repository, { recursive: true, force: true });
  }
});

test("the configured repo MCP server answers the initialize handshake", async () => {
  const server = JSON.parse(read(".mcp.json")).mcpServers.repo as { command: string; args: string[]; env: Record<string, string> };
  const child = Bun.spawn([server.command, ...server.args], { cwd: root, env: { ...process.env, ...server.env }, stdin: "pipe", stdout: "pipe", stderr: "inherit" });
  try {
    child.stdin.write('{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0"}}}\n');
    await child.stdin.flush();
    const reader = child.stdout.getReader();
    const decoder = new TextDecoder();
    let line = "";
    const deadline = Date.now() + 90_000;
    while (!line.includes("\n") && Date.now() < deadline) {
      const chunk = await reader.read();
      if (chunk.done) break;
      line += decoder.decode(chunk.value);
    }
    expect(line, "initialize response").toContain('"result"');
    expect(line).toContain("jsonrpc");
    expect(line).toContain('"name":"repo"');
  } finally {
    child.kill();
  }
}, 120_000);
