#!/usr/bin/env bun
/** 🌉️ `@semio-tech/framework-os-mcp-rs` task router: `bun ./📜️script.ts <build|check|test|dev>`. */
import { existsSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import {
  BundleScript,
  ScriptRouter,
  orchestratorBudgetOpts,
  resolveTestLevel,
  runBundleScriptMain,
  runCargo,
  runCargoTestBudgeted,
  runCmd,
  runProbe,
} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { MCP_BINARY_NAME, MCP_CARGO_PACKAGE, resolveBuiltMcpBinaryPath, resolveMcpTargetDirectory } from "../../🟦️.ts";

const binaryContract = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧱️binary-gate.json", import.meta.url), "utf8")) as { cargoPackage: string; cargoBinary: string; profile: "debug" };
if (binaryContract.cargoPackage !== MCP_CARGO_PACKAGE || binaryContract.cargoBinary !== MCP_BINARY_NAME || binaryContract.profile !== "debug") throw new Error("semio-os-mcp binary fixture disagrees with the shared path contract");

function buildMcpBinary(repoRoot: string, root: string): string {
  const targetDirectory = resolveMcpTargetDirectory(repoRoot);
  runCargo(["build", "--manifest-path", "Cargo.toml", "--package", binaryContract.cargoPackage, "--bin", binaryContract.cargoBinary, "--target-dir", targetDirectory], root);
  const binary = resolveBuiltMcpBinaryPath(repoRoot);
  if (!statSync(binary).isFile()) throw new Error(`cargo succeeded without producing ${binary}`);
  return binary;
}

function mcpEntrypointProbeEnvironment(marker?: string): NodeJS.ProcessEnv {
  const environment: NodeJS.ProcessEnv = {};
  for (const [key, value] of Object.entries(process.env)) {
    const normalized = key.toUpperCase();
    const protectedKey =
      normalized === "S_USER" ||
      normalized === "VITE_S_USER" ||
      normalized === "S_HUB_URL" ||
      normalized.includes("TOKEN") ||
      normalized.includes("SESSION") ||
      normalized.includes("CREDENTIAL") ||
      normalized.includes("BEARER") ||
      normalized.includes("CAPABILITY") ||
      normalized.includes("AUTHORIZATION") ||
      normalized.includes("COOKIE");
    if (!protectedKey) environment[key] = value;
  }
  environment.SEMIO_DIRECT_CHILD_BENIGN = "preserved";
  if (marker !== undefined) environment.S_LOCAL_CREDENTIAL_FD = marker;
  return environment;
}

function proveMcpEntrypointCredentialMarker(executable: string, root: string): void {
  const clean = runProbe(executable, ["--assert-no-local-credential-state"], { cwd: root, env: mcpEntrypointProbeEnvironment(), budgetMs: 30_000 });
  if (clean.status !== 0) throw new Error("MCP entrypoint clean descendant seal probe failed");
  const poison = `session.v1.${"a".repeat(32)}.${"b".repeat(64)}`;
  const rejected = runProbe(executable, ["--assert-no-local-credential-state"], { cwd: root, env: mcpEntrypointProbeEnvironment(poison), budgetMs: 30_000 });
  if (rejected.status === 0 || rejected.stdout.includes(poison) || rejected.stderr.includes(poison)) throw new Error("MCP entrypoint admitted or leaked a non-fd3 credential marker");
  console.log("mcp-entrypoint-credential-marker: clean=accepted non-fd3=rejected redacted=1");
}

class BuildScript extends BundleScript {
  run(): void {
    const binary = buildMcpBinary(this.repoRoot, this.root);
    console.log(`[build] ${binary}`);
  }
}

class CheckScript extends BundleScript {
  run(): void {
    runCargo(["check", "--manifest-path", "Cargo.toml"], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-os-mcp"], this.repoRoot, rest);
  }
}

class CanonicalPairCheckScript extends BundleScript {
  run(): void {
    const oracle = join(this.root, "..", "..", "🏠️workspace", "🔗️remote", "🧩️pair", "🧪️oracle", "🟦️.ts");
    const hub = join(this.repoRoot, "🌎️hub", "📦️packages", "🦀️rust");
    if (!existsSync(oracle)) throw new Error(`missing canonical pair oracle at ${oracle}`);
    if (!existsSync(join(hub, "Cargo.toml"))) throw new Error(`missing Hub manifest at ${hub}`);
    proveMcpEntrypointCredentialMarker(buildMcpBinary(this.repoRoot, this.root), this.root);
    const suffixes = [
      "canonical_pair_neutral_receiver_rejects_all_malformed_vectors_and_wipes_candidates",
      "canonical_pair_actor_keys_cache_and_mount_to_one_binding_and_evicts_by_fixed_credits",
      "canonical_pair_cache_hit_never_returns_after_binding_revocation",
      "canonical_pair_receipt_preflights_streams_cancels_expires_and_never_resurrects_after_invalidation",
    ];
    const laws = suffixes.map((suffix) => {
      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "--lib", suffix, "--", "--list"], { cwd: this.root, ...orchestratorBudgetOpts() });
      const matches = listed.stdout
        .split("\n")
        .filter((line) => line.endsWith(": test"))
        .map((line) => line.slice(0, -": test".length))
        .filter((name) => name.endsWith(suffix));
      if (listed.status !== 0 || matches.length !== 1) throw new Error(`canonical-pair-check expected exactly one ${suffix} law, selected ${matches.length}`);
      return matches[0]!;
    });
    console.log(`canonical-pair-laws: ${laws.join(" ")}`);
    for (const law of laws) runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", law, "--", "--exact", "--test-threads=1"], this.root);
    runCmd("bun", [oracle], { cwd: this.root, budgetMs: 120_000 });
    runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features"], this.root);
    runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], hub);
  }
}

/** ▶️ `bun ./📜️script.ts dev [-- stdio [flags...]]` — boots the real stdio server for local/manual
 *  smoke testing (`printf '<json-rpc line>' | bun ./📜️script.ts dev -- stdio | ...`). Defaults to
 *  `stdio` when no mode is given, matching `📦️bin.rs`'s own default-less argv contract. */
class DevScript extends BundleScript {
  run(segments: string[]): void {
    const args = segments.length > 0 ? segments : ["stdio"];
    runCmd(buildMcpBinary(this.repoRoot, this.root), args, { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("check", CheckScript).register("test", TestScript).register("canonical-pair-check", CanonicalPairCheckScript).register("dev", DevScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
