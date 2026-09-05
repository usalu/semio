#!/usr/bin/env bun
/** 🌉️ `@semio-tech/framework-os-mcp-rs` task router: `bun ./📜️script.ts <build|check|test|dev>`. */
import { existsSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { deepStrictEqual } from "node:assert";
import Ajv2020 from "ajv/dist/2020.js";
import {
  BundleScript,
  ScriptRouter,
  buildBudgetMs,
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

/** 🗺️ Independently validates the committed GIS roster without granting execution authority. */
class InferenceDiscoveryOracleScript extends BundleScript {
  run(): void {
    const identityRoot = join(this.repoRoot, "✏️s", "🔌️plugins", "🌍️gis", "🧪️fixtures", "🪪️artifact-identity");
    const identity = JSON.parse(readFileSync(join(identityRoot, "🔣️.json"), "utf8"));
    const validateIdentity = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(identityRoot, "🧬️.schema.json"), "utf8")));
    if (!validateIdentity(identity)) throw new Error(`invalid GIS identity fixture: ${JSON.stringify(validateIdentity.errors)}`);
    const kinds = new Set<string>();
    for (const artifact of identity.artifacts) {
      const segments = artifact.kind.split(".");
      if (segments.length !== 3 || segments[0] !== "s" || segments[1] !== identity.pluginId || kinds.has(artifact.kind)) throw new Error("GIS artifact identity must have one exact plugin owner");
      kinds.add(artifact.kind);
      if (artifact.nativeDialect !== `${artifact.kind}@1/*` || artifact.documentSchema !== (segments[2] === "gismap" ? "gis.map" : "gis.terrain")) throw new Error("GIS native identity and payload schema were conflated");
      if (artifact.extension !== segments[2] || artifact.codecExtension !== `${Buffer.byteLength(artifact.documentSchema, "utf8")}:${artifact.documentSchema}:${artifact.extension}`) throw new Error("GIS codec extension must bind its exact payload schema");
    }
    for (const kind of identity.hostileKinds) {
      const candidate = structuredClone(identity);
      candidate.artifacts[0].kind = kind;
      if (validateIdentity(candidate)) throw new Error(`GIS identity oracle admitted ${kind}`);
    }
    console.log(`gis-artifact-identity-oracle: canonical=${kinds.size} hostile=${identity.hostileKinds.length}; native assembly still requires Rust law`);
    const controlRoot = join(this.repoRoot, "✏️s", "🔌️plugins", "🌍️gis", "🧪️fixtures", "💡️inference-control");
    const control = JSON.parse(readFileSync(join(controlRoot, "🔣️.json"), "utf8"));
    const validateControl = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(controlRoot, "🧬️.schema.json"), "utf8")));
    if (!validateControl(control)) throw new Error(`invalid GIS control fixture: ${JSON.stringify(validateControl.errors)}`);
    const checkpoints = [0, 1];
    const coordinates: number[][] = [];
    let work = 1;
    const scan = (value: any): void => {
      checkpoints.push(++work);
      if (Array.isArray(value)) {
        if (value.length === 2 && value.every((item) => typeof item === "number")) coordinates.push(value);
        else value.forEach(scan);
      } else if (value !== null && typeof value === "object") {
        if (typeof value.lon === "number" && typeof value.lat === "number") coordinates.push([value.lon, value.lat]);
        Object.values(value).forEach(scan);
      }
    };
    for (const feature of [...control.snapshot.positions, ...control.snapshot.routes, ...control.snapshot.regions]) scan(feature.data);
    checkpoints.push(work);
    deepStrictEqual(checkpoints, control.checkpoints);
    deepStrictEqual({ positionCount: control.snapshot.positions.length, routeCount: control.snapshot.routes.length, regionCount: control.snapshot.regions.length, bounds: {
      lonMin: Math.min(...coordinates.map(([lon]) => lon!)), lonMax: Math.max(...coordinates.map(([lon]) => lon!)), latMin: Math.min(...coordinates.map(([, lat]) => lat!)), latMax: Math.max(...coordinates.map(([, lat]) => lat!)),
    } }, control.expected);
    for (const interruption of control.interruptions) {
      if (checkpoints.indexOf(interruption.at) + 1 !== interruption.calls) throw new Error(`control does not stop at first interruption ${interruption.name}`);
    }
    const { lonMin, lonMax, latMin, latMax } = control.expected.bounds;
    deepStrictEqual(control.proposal, { CreateRegion: { index: control.snapshot.regions.length, item: { id: `inference-${control.proposalJobId}`, data: { kind: "inference-bounds", ring: [[lonMin, latMin], [lonMax, latMin], [lonMax, latMax], [lonMin, latMax], [lonMin, latMin]] } } } });
    console.log(`gis-inference-control-oracle: checkpoints=${checkpoints.length} interruptions=${control.interruptions.length} typed-proposal=1; no hub execution claim`);
    const fixtureRoot = join(this.root, "..", "..", "💡️inference", "🧪️fixtures", "🗺️gis-discovery");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(fixtureRoot, "🧬️.schema.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    if (!validate(fixture.expected)) throw new Error(`invalid neutral GIS roster: ${JSON.stringify(validate.errors)}`);
    for (const hostile of fixture.hostile) {
      const candidate = structuredClone(fixture.expected);
      if (hostile.operation === "remove") candidate.declared = [];
      else if (hostile.operation === "duplicate") candidate.declared.push(structuredClone(candidate.declared[0]));
      else candidate.declared[0][hostile.field] = hostile.value;
      if (validate(candidate)) throw new Error(`GIS discovery oracle admitted ${hostile.name}`);
    }
    const descriptor = JSON.parse(readFileSync(join(this.repoRoot, "✏️s", "🔌️plugins", "🌍️gis", "🔣️.json"), "utf8"));
    const contributions = descriptor.contributions;
    const declared = [...(contributions.inferenceServices ?? []), ...(contributions.artifactContributions ?? []).flatMap((row: { inferences?: unknown[] }) => row.inferences ?? [])];
    const actual = { declared };
    if (!validate(actual)) throw new Error(`committed GIS descriptor discovery drift: ${JSON.stringify(validate.errors)}`);
    deepStrictEqual(actual, fixture.expected);
    console.log(`gis-inference-discovery-oracle: exact=1 hostile=${fixture.hostile.length} execution-authority=0`);
  }
}

/** 🌉️ Proves the literal neutral trace through the registered MCP discovery tool. */
class InferenceDiscoveryCheckScript extends BundleScript {
  run(): void {
    runCmd("bun", ["./📜️script.ts", "inference-discovery-oracle"], { cwd: this.root, budgetMs: 60_000 });
    const packets = [
      { root: this.root, suffix: "gis_inference_discovery_reads_committed_descriptor_through_registered_mcp_tool_without_execution_authority" },
      { root: join(this.repoRoot, "✏️s", "🔌️plugins", "🌍️gis", "📦️packages", "🦀️rust"), suffix: "gis_component_assembly_declares_exact_package_identity_before_descriptor_emission" },
      { root: join(this.repoRoot, "✏️s", "🔌️plugins", "🌍️gis", "📦️packages", "🦀️rust"), suffix: "gis_native_controlled_inference_executes_literal_progress_cancel_and_deadline_trace" },
    ];
    for (const packet of packets) {
      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "--lib", packet.suffix, "--", "--list"], { cwd: packet.root, budgetMs: buildBudgetMs() });
      const matches = listed.stdout.split("\n").filter((line) => line.endsWith(": test")).map((line) => line.slice(0, -6)).filter((name) => name.endsWith(packet.suffix));
      if (listed.status !== 0 || matches.length !== 1) throw new Error(`GIS discovery exact-one preflight failed ${packet.suffix}: status=${listed.status} matches=${matches.length} diagnostic=${listed.stderr.slice(-4000)}`);
      runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", matches[0]!, "--", "--exact", "--test-threads=1"], packet.root);
    }
    runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features"], this.root);
    console.log("gis-inference-discovery-check: committed descriptor, exact MCP tool trace, all-feature compile; no execution claim");
  }
}

/** ▶️ `bun ./📜️script.ts dev [-- stdio [flags...]]` — boots the real stdio server for local/manual
 *  smoke testing (`printf '<json-rpc line>' | bun ./📜️script.ts dev -- stdio | ...`). Defaults to
 *  `stdio` when no mode is given, matching `🚀️bin.rs`'s own default-less argv contract. */
class DevScript extends BundleScript {
  run(segments: string[]): void {
    const args = segments.length > 0 ? segments : ["stdio"];
    runCmd(buildMcpBinary(this.repoRoot, this.root), args, { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("check", CheckScript).register("test", TestScript).register("canonical-pair-check", CanonicalPairCheckScript).register("inference-discovery-oracle", InferenceDiscoveryOracleScript).register("inference-discovery-check", InferenceDiscoveryCheckScript).register("dev", DevScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
