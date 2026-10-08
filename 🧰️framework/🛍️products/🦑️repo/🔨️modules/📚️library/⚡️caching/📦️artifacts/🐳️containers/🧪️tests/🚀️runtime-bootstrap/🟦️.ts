import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

/** 🐳️ The devcontainer fields this bootstrap contract reads. */
interface DevcontainerConfig {
  readonly workspaceFolder: string;
  readonly remoteUser: string;
  readonly mounts: readonly string[];
  readonly onCreateCommand: readonly string[];
  readonly postCreateCommand: readonly string[];
  readonly features: Readonly<Record<string, { readonly version?: string; readonly moby?: boolean }>>;
  readonly forwardPorts: readonly number[];
  readonly portsAttributes: Readonly<Record<string, { readonly label: string }>>;
}

/** 🚪️ One dashboard command whose server the devcontainer forwards: a target or tool ready port of a project manifest, or a port of the generated playground catalog. */
interface ForwardedCommand {
  readonly label: string;
  readonly manifest?: string;
  readonly target?: string;
  readonly tool?: string;
  readonly playground?: string;
  readonly surface?: "react" | "wgpu";
  readonly slot?: number;
}

interface ProjectManifest {
  readonly targets?: Readonly<Record<string, { readonly metadata?: { readonly semio?: { readonly dashboard?: { readonly ready?: { readonly port?: number } } } } }>>;
  readonly metadata?: { readonly semio?: { readonly dashboard?: { readonly tools?: readonly { readonly id: string; readonly ready?: { readonly port?: number } }[] } } };
}

interface PlaygroundRow {
  readonly variant: string;
  readonly ports: Readonly<Record<"react" | "wgpu", number>>;
  readonly userPorts?: Readonly<Record<"react" | "wgpu", readonly number[]>>;
}

/** 🚀️ Verifies the image's pinned runtime acquisition before application dependency synchronization, and that every named
 * volume nested in the checkout is private to that checkout (`${devcontainerId}`: two clones with the same folder name must
 * not share `node_modules` or build caches) and, created root-owned by Docker, is handed to the remote user before
 * `postCreateCommand` installs into it. https://containers.dev/implementors/json_reference/#variables-in-devcontainerjson
 * https://code.visualstudio.com/remote/advancedcontainers/improve-performance#_use-a-targeted-named-volume */
export function testContainerRuntimeBootstrap(workspace: string): void {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🚀️runtime-bootstrap/🔣️.json"), "utf8"));
  const dockerfile = readFileSync(join(workspace, ".devcontainer/Dockerfile"), "utf8"), configSource = readFileSync(join(workspace, ".devcontainer/devcontainer.json"), "utf8");
  const config = Bun.JSONC.parse(configSource) as DevcontainerConfig;
  assert.deepEqual(Bun.JSONC.parse(configSource), require("jsonc-parser").parse(configSource));
  assert.deepEqual(config.postCreateCommand, fixture.postCreateCommand);
  assert.ok(dockerfile.includes("> /usr/local/bin/semio") && dockerfile.includes('exec bun run --cwd /workspaces/semio dashboard "$@"'), "Native hosts have no `semio` on PATH (`semio …` is `bun run dashboard …`); the container image puts a shim on PATH so the shorthand works literally there");
  const nestedMounts = (mounts: readonly Readonly<Record<string, string>>[]) => mounts.filter((mount) => mount.type === "volume" && mount.target!.startsWith("${containerWorkspaceFolder}/"));
  const nested = (mounts: readonly Readonly<Record<string, string>>[]) => nestedMounts(mounts).map((mount) => mount.target!.replace("${containerWorkspaceFolder}", config.workspaceFolder));
  const parsedMounts = config.mounts.map((mount) => Object.fromEntries(mount.split(",").map((pair) => [pair.slice(0, pair.indexOf("=")), pair.slice(pair.indexOf("=") + 1)])));
  const volumes = nested(parsedMounts);
  for (const mount of nestedMounts(parsedMounts)) assert.ok(mount.source!.includes("${devcontainerId}"), `The volume at ${mount.target} is nested in the checkout, so it must be private to it (\${devcontainerId})`);
  assert.deepEqual(volumes, nested(require("lodash").map(config.mounts, (mount: string) => require("lodash").fromPairs(require("lodash").map(mount.split(","), (pair: string) => require("lodash").split(pair, /=(.*)/s, 2))))));
  assert.deepEqual(volumes, fixture.workspaceVolumeTargets);
  assert.deepEqual(config.onCreateCommand, ["sudo", "chown", `${config.remoteUser}:${config.remoteUser}`, ...volumes], "Docker creates a named volume nested in the checkout owned by root; the remote user must own it before postCreateCommand installs into it");
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
  const lodash = require("lodash"), readManifest = (path: string): ProjectManifest => JSON.parse(readFileSync(join(workspace, path), "utf8"));
  const catalogPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🚀️playgrounds.json";
  const catalog: readonly PlaygroundRow[] = JSON.parse(readFileSync(join(workspace, catalogPath), "utf8"));
  assert.deepEqual(catalog, require("jsonc-parser").parse(readFileSync(join(workspace, catalogPath), "utf8")));
  const declaredPort = (command: ForwardedCommand): number => {
    if (command.playground !== undefined) {
      const row = catalog.find((candidate) => candidate.variant === command.playground);
      assert.ok(row, `Forwarded playground ${command.playground} is absent from the generated playground catalog`);
      const port = command.slot === 0 ? row.ports[command.surface!] : row.userPorts?.[command.surface!]?.[command.slot! - 1];
      assert.equal(port, command.slot === 0 ? lodash.get(row, ["ports", command.surface!]) : lodash.get(row, ["userPorts", command.surface!, command.slot! - 1]));
      assert.ok(port, `${command.label} names no catalog port`);
      return port;
    }
    const manifest = readManifest(command.manifest!);
    const ready = command.tool !== undefined ? manifest.metadata?.semio?.dashboard?.tools?.find((tool) => tool.id === command.tool)?.ready : manifest.targets?.[command.target!]?.metadata?.semio?.dashboard?.ready;
    const oracle = command.tool !== undefined ? lodash.get(lodash.find(lodash.get(manifest, ["metadata", "semio", "dashboard", "tools"]), { id: command.tool }), ["ready", "port"]) : lodash.get(manifest, ["targets", command.target!, "metadata", "semio", "dashboard", "ready", "port"]);
    assert.ok(ready?.port, `${command.label} declares no ready port in metadata.semio.dashboard of ${command.manifest}`);
    assert.equal(ready.port, oracle);
    return ready.port;
  };
  const served = (fixture.forwardedCommands as ForwardedCommand[]).map((command) => ({ label: command.label, port: declaredPort(command) }));
  const forwarded = [...served, ...fixture.forwardedUndeclaredPorts as { label: string; port: number }[]];
  assert.equal(new Set(forwarded.map((entry) => entry.port)).size, forwarded.length, "Every forwarded dashboard command owns a distinct port");
  const expectedPorts = forwarded.map((entry) => entry.port).sort((left, right) => left - right);
  assert.deepEqual([...config.forwardPorts].sort((left, right) => left - right), expectedPorts, "The devcontainer forwards exactly the ports the declared dashboard commands start (ready ports of metadata.semio.dashboard and playground catalog ports)");
  assert.deepEqual(Object.keys(config.portsAttributes).map(Number).sort((left, right) => left - right), expectedPorts, "Every forwarded port carries a label");
  for (const entry of forwarded) assert.equal(config.portsAttributes[String(entry.port)]?.label, entry.label, `Port ${entry.port} is labelled after its dashboard command`);
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
  assert.ok(!/(?:curl|wget)[^\n]*\|[^\n]*\bsh\b/.test(dockerfile), "The image pipes no unpinned installer into a shell; tools come from pinned features or checksum-verified archives");
  assert.equal(config.features[fixture.uvFeature.id]?.version, fixture.uvFeature.version, "uv comes from its pinned feature only");
  if (process.platform !== "win32") {
    const command = `${selector}\nprintf '%s|%s|%s|%s' "$bun_archive" "$bun_sha" "$node_archive" "$node_sha"`;
    for (const row of fixture.platforms) {
      const result = spawnSync("sh", ["-ec", command], { env: { ...process.env, runtime_arch: row.architecture }, encoding: "utf8", timeout: 5000 });
      assert.equal(result.status, 0, result.stderr);
      assert.equal(result.stdout, [row.bunArchive, row.bunSha256, row.nodeArchive, row.nodeSha256].join("|"));
    }
    for (const architecture of fixture.rejectedArchitectures) assert.notEqual(spawnSync("sh", ["-ec", command], { env: { ...process.env, runtime_arch: architecture }, encoding: "utf8", timeout: 5000 }).status, 0);
  }
  console.log("✅️ Container runtime pins, checksum-before-extraction, repository Nx ownership, forwarded dashboard-command ports, volume ownership before creation and JSONC/lodash/native shell oracles PASS");
}
