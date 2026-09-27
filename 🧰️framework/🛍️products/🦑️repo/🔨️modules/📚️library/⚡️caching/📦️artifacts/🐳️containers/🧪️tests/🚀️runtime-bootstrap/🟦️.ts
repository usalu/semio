import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

/** 🐳️ The devcontainer fields this bootstrap contract reads. */
interface DevcontainerConfig {
  readonly postCreateCommand: readonly string[];
  readonly features: Readonly<Record<string, { readonly version?: string; readonly moby?: boolean }>>;
  readonly forwardPorts: readonly number[];
  readonly portsAttributes: Readonly<Record<string, { readonly label: string }>>;
}

/** 🚪️ The launch rows whose servers the devcontainer forwards: one port each, named in the row's environment. */
interface LaunchRow {
  readonly name: string;
  readonly env?: Readonly<Record<string, string>>;
}

/** 🚀️ Verifies the image's pinned runtime acquisition before application dependency synchronization. */
export function testContainerRuntimeBootstrap(workspace: string): void {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🚀️runtime-bootstrap/🔣️.json"), "utf8"));
  const dockerfile = readFileSync(join(workspace, ".devcontainer/Dockerfile"), "utf8"), configSource = readFileSync(join(workspace, ".devcontainer/devcontainer.json"), "utf8");
  const config = Bun.JSONC.parse(configSource) as DevcontainerConfig;
  assert.deepEqual(Bun.JSONC.parse(configSource), require("jsonc-parser").parse(configSource));
  assert.deepEqual(config.postCreateCommand, fixture.postCreateCommand);
  for (const path of fixture.retiredScripts) assert.equal(existsSync(join(workspace, path)), false, `Retired lifecycle bypass: ${path}`);
  const rootTargets = JSON.parse(readFileSync(join(workspace, "📋️project.json"), "utf8")).targets;
  const target = rootTargets[fixture.postCreateCommand[3].split(":")[1]];
  assert.equal(target.cache, false); assert.deepEqual(target.outputs, []);
  assert.ok(target.options.command.endsWith("📜️script.ts setup"));
  assert.deepEqual(target.dependsOn, fixture.postCreateDependsOn, "Container creation must reach every zero-touch prerequisite: language environments, generated sources, agent instruction aliases and both MCP binaries");
  for (const dependency of fixture.postCreateDependsOn) if (!dependency.includes(":")) assert.ok(rootTargets[dependency], `Zero-touch prerequisite ${dependency} is not a root target`);
  assert.ok(rootTargets["deps-wasm"].dependsOn.includes("deps-trunk"), "Both rust-toolchain.toml wasm targets must be installed by one prerequisite chain");
  assert.equal(config.features["ghcr.io/devcontainers/features/rust:1"]?.version, "none", "rust-toolchain.toml is the only toolchain pin");
  assert.ok(!Object.keys(config.features).some((key) => /\/nx:/.test(key)), "Nx must come from the repository's locked bootstrap");
  assert.deepEqual(config.features[fixture.dockerDaemonFeature.id], fixture.dockerDaemonFeature.options, "Hub backends publish on 127.0.0.1, so the devcontainer needs its own Docker daemon (docker-in-docker, Docker CE on Ubuntu noble), not the host's socket");
  const launchSource = readFileSync(join(workspace, ".vscode/launch.json"), "utf8"), launch = (Bun.JSONC.parse(launchSource) as { configurations: LaunchRow[] }).configurations;
  assert.deepEqual(launch.map((row) => row.name), (require("jsonc-parser").parse(launchSource) as { configurations: LaunchRow[] }).configurations.map((row) => row.name));
  const served = fixture.forwardedLaunchRows.map((name: string) => {
    const row = launch.find((candidate) => candidate.name === name);
    assert.ok(row, `Forwarded launch row ${name} is absent from .vscode/launch.json`);
    const ports = Object.entries(row.env ?? {}).filter(([key]) => key.endsWith("_PORT")).map(([, value]) => Number(value));
    assert.equal(ports.length, 1, `${name} must name exactly one server port`);
    return ports[0];
  });
  const expectedPorts = [...served, ...fixture.forwardedToolPorts].sort((left: number, right: number) => left - right);
  assert.deepEqual([...config.forwardPorts].sort((left, right) => left - right), expectedPorts, "The devcontainer forwards exactly the hub, the `s` serves and the tool ports its launch rows start");
  assert.deepEqual(Object.keys(config.portsAttributes).map(Number).sort((left, right) => left - right), expectedPorts, "Every forwarded port carries a label");
  assert.equal(JSON.parse(readFileSync(join(workspace, "package.json"), "utf8")).packageManager, `bun@${fixture.bun}`);
  assert.ok(dockerfile.includes(`ARG BUN_VERSION=${fixture.bun}\n`));
  assert.ok(dockerfile.includes(`ARG NODE_VERSION=${fixture.node}\n`));
  const runtime = dockerfile.match(/^RUN set -eu; \\\n[\s\S]*?(?=\n\n)/m)?.[0];
  assert.ok(runtime, "Runtime acquisition must be one checked image layer");
  const selector = runtime.replaceAll("\\\n", "\n").match(/case "\$runtime_arch" in[\s\S]*?esac;/)?.[0];
  assert.ok(selector, "Image must reject unsupported runtime architectures");
  const rows = [...selector.matchAll(/^\s*(amd64|arm64)\) bun_archive="([^"]+)"; bun_sha="([a-f0-9]{64})"; node_archive="([^"]+)"; node_sha="([a-f0-9]{64})";;/gm)].map(([, architecture, bunArchive, bunSha256, nodeArchive, nodeSha256]) => ({ architecture, bunArchive, bunSha256, nodeArchive, nodeSha256 }));
  assert.deepEqual(require("lodash/sortBy")(rows, "architecture"), require("lodash/sortBy")(fixture.platforms, "architecture"));
  for (const tool of ["bun", "node"]) {
    assert.ok(runtime.indexOf(`"$${tool}_sha`) < runtime.indexOf(tool === "bun" ? "unzip -p" : "tar -xJf"), `${tool} verification must precede extraction`);
    assert.ok(runtime.includes(`test "$(${tool} --version)" = "${tool === "node" ? "v" : ""}$${tool.toUpperCase()}_VERSION"`));
  }
  assert.equal((runtime.match(/sha256sum -c -/g) ?? []).length, 2);
  assert.equal((runtime.match(/--max-time 300/g) ?? []).length, 2);
  assert.ok(runtime.includes("https://github.com/oven-sh/bun/releases/download/bun-v$BUN_VERSION/"));
  assert.ok(runtime.includes("https://nodejs.org/dist/v$NODE_VERSION/"));
  assert.ok(!/bun install|npm|curl[^\n]*\|\s*(?:ba)?sh/.test(runtime));
  if (process.platform !== "win32") {
    const command = `${selector}\nprintf '%s|%s|%s|%s' "$bun_archive" "$bun_sha" "$node_archive" "$node_sha"`;
    for (const row of fixture.platforms) {
      const result = spawnSync("sh", ["-ec", command], { env: { ...process.env, runtime_arch: row.architecture }, encoding: "utf8", timeout: 5000 });
      assert.equal(result.status, 0, result.stderr);
      assert.equal(result.stdout, [row.bunArchive, row.bunSha256, row.nodeArchive, row.nodeSha256].join("|"));
    }
    for (const architecture of fixture.rejectedArchitectures) assert.notEqual(spawnSync("sh", ["-ec", command], { env: { ...process.env, runtime_arch: architecture }, encoding: "utf8", timeout: 5000 }).status, 0);
  }
  console.log("✅️ Container runtime pins, checksum-before-extraction, repository Nx ownership, forwarded launch-row ports and JSONC/lodash/native shell oracles PASS");
}
