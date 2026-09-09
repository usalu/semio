#!/usr/bin/env bun
/** 🌉️ `@semio-tech/framework-os-mcp-rs` task router: `bun ./📜️script.ts <build|check|test|dev>`. */
import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { deepStrictEqual } from "node:assert";
import { createHash } from "node:crypto";
import Ajv from "ajv";
import {
  BundleScript,
  ScriptRouter,
  buildBudgetMs,
  daemonBudgetOpts,
  orchestratorBudgetOpts,
  resolveTestLevel,
  runBundleScriptMain,
  runCargo,
  runCargoTestBudgeted,
  runCmd,
  runProbe,
} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { MCP_BINARY_NAME, MCP_CARGO_PACKAGE, resolveBuiltMcpBinaryPath, resolveMcpTargetDirectory, requireMcpBinary } from "../../🟦️.ts";

import { buildCargoArtifacts } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts";

const binaryContract = JSON.parse(readFileSync(new URL("../../🎚️config/🧱️binary-gate.json", import.meta.url), "utf8")) as { cargoPackage: string; cargoBinary: string; profile: "debug" };
if (binaryContract.cargoPackage !== MCP_CARGO_PACKAGE || binaryContract.cargoBinary !== MCP_BINARY_NAME || binaryContract.profile !== "debug") throw new Error("semio-os-mcp binary fixture disagrees with the shared path contract");

/** 🧬️ One compiled export of the GIS plugin's own module contract (`✏️s/🔌️plugins/🌍️gis/🧬️schema/🔣️.json`).
 * The GIS scope owns these shapes; this crate is a reader, never a second declaration site. */
function gisContract(repoRoot: string, exportId: string) {
  const schema = JSON.parse(readFileSync(join(repoRoot, "✏️s", "🔌️plugins", "🌍️gis", "🧬️schema", "🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const validate = ajv.getSchema(`${schema.$id}#/$defs/${exportId}`);
  if (!validate) throw new Error(`the GIS scope publishes no ${exportId} export`);
  return validate;
}

/** 🧬️ One compiled export of the `os.mcp.workspace` module contract — draft-07, `$defs`-addressed,
 * read straight off `🏠️workspace/🧬️schema/🔣️.json` so no oracle carries a schema of its own. */
function workspaceContract(root: string, exportId: string) {
  const schema = JSON.parse(readFileSync(join(root, "..", "..", "🏠️workspace", "🧬️schema", "🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const validate = ajv.getSchema(`${schema.$id}#/$defs/${exportId}`);
  if (!validate) throw new Error(`os.mcp.workspace publishes no ${exportId} export`);
  return validate;
}

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
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("MCP build has a fixed binary output contract");
    await buildCargoArtifacts(join(this.root, "Cargo.toml"), ["--package", MCP_CARGO_PACKAGE, "--bin", MCP_BINARY_NAME], this.repoRoot);
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
    const oracle = join(this.root, "..", "..", "🏠️workspace", "🔗️remote", "🧩️pair", "🧪️tests", "🧪️canonical-pair-oracle", "🟦️.ts");
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
    const identityRoot = join(this.repoRoot, "✏️s", "🔌️plugins", "🌍️gis", "🧫️fixtures", "🪪️artifact-identity");
    const identity = JSON.parse(readFileSync(join(identityRoot, "🔣️.json"), "utf8"));
    const validateIdentity = gisContract(this.repoRoot, "GisArtifactIdentity");
    if (!validateIdentity(identity)) throw new Error(`invalid GIS identity fixture: ${JSON.stringify(validateIdentity.errors)}`);
    const kinds = new Set<string>();
    for (const artifact of identity.artifacts) {
      const segments = artifact.kind.split(".");
      if (segments.length !== 3 || segments[0] !== "s" || segments[1] !== identity.pluginId || kinds.has(artifact.kind)) throw new Error("GIS artifact identity must have one exact plugin owner");
      kinds.add(artifact.kind);
      if (artifact.nativeDialect !== `${artifact.kind}@1/*` || artifact.documentSchema !== (segments[2] === "gismap" ? "gis.map" : "gis.terrain")) throw new Error("GIS native identity and payload schema were conflated");
      if (artifact.extension !== segments[2] || artifact.codecExtension !== `${Buffer.byteLength(artifact.documentSchema, "utf8")}:${artifact.documentSchema}:${artifact.extension}`)
        throw new Error("GIS codec extension must bind its exact payload schema");
    }
    for (const kind of identity.hostileKinds) {
      const candidate = structuredClone(identity);
      candidate.artifacts[0].kind = kind;
      if (validateIdentity(candidate)) throw new Error(`GIS identity oracle admitted ${kind}`);
    }
    console.log(`gis-artifact-identity-oracle: canonical=${kinds.size} hostile=${identity.hostileKinds.length}; native assembly still requires Rust law`);
    const controlRoot = join(this.repoRoot, "✏️s", "🔌️plugins", "🌍️gis", "🧫️fixtures", "💡️inference-control");
    const control = JSON.parse(readFileSync(join(controlRoot, "🔣️.json"), "utf8"));
    const validateControl = gisContract(this.repoRoot, "GisInferenceControl");
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
    deepStrictEqual(
      {
        positionCount: control.snapshot.positions.length,
        routeCount: control.snapshot.routes.length,
        regionCount: control.snapshot.regions.length,
        bounds: {
          lonMin: Math.min(...coordinates.map(([lon]) => lon!)),
          lonMax: Math.max(...coordinates.map(([lon]) => lon!)),
          latMin: Math.min(...coordinates.map(([, lat]) => lat!)),
          latMax: Math.max(...coordinates.map(([, lat]) => lat!)),
        },
      },
      control.expected,
    );
    for (const interruption of control.interruptions) {
      if (checkpoints.indexOf(interruption.at) + 1 !== interruption.calls) throw new Error(`control does not stop at first interruption ${interruption.name}`);
    }
    const { lonMin, lonMax, latMin, latMax } = control.expected.bounds;
    deepStrictEqual(control.proposal, {
      CreateRegion: {
        index: control.snapshot.regions.length,
        item: {
          id: `inference-${control.proposalJobId}`,
          data: {
            // 🆔️ The GIS `CreateRegion` payload carries the region id inside `data` as well as on the
            // item — both derived here from `proposalJobId` alone, never copied out of `control.proposal`.
            id: `inference-${control.proposalJobId}`,
            kind: "inference-bounds",
            ring: [
              [lonMin, latMin],
              [lonMax, latMin],
              [lonMax, latMax],
              [lonMin, latMax],
              [lonMin, latMin],
            ],
          },
        },
      },
    });
    console.log(`gis-inference-control-oracle: checkpoints=${checkpoints.length} interruptions=${control.interruptions.length} typed-proposal=1; no hub execution claim`);
    const fixtureRoot = join(this.root, "..", "..", "💡️inference", "🧫️fixtures", "🗺️gis-discovery");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    // 🧬️ Shape comes from the module contract `💡️inference/🧬️schema/🔣️.json` (real value space —
    // any owner, any dotted service id, any version ≥ 1); EXACTNESS comes from the fixture itself.
    // Splitting the two is what lets a hostile vector be rejected for the right reason: `matchesRoster`
    // rejects a forged identity, the schema rejects a malformed or smuggled field.
    const schema = JSON.parse(readFileSync(join(this.root, "..", "..", "💡️inference", "🧬️schema", "🔣️.json"), "utf8"));
    const inferenceAjv = new Ajv({ strict: true, allErrors: true });
    inferenceAjv.addSchema(schema);
    const validate = inferenceAjv.getSchema(`${schema.$id}#/$defs/InferenceDiscoveryRosterV1`)!;
    if (!validate(fixture.expected)) throw new Error(`invalid neutral GIS roster: ${JSON.stringify(validate.errors)}`);
    const matchesRoster = (candidate: unknown): boolean => JSON.stringify(candidate) === JSON.stringify(fixture.expected);
    if (!matchesRoster(fixture.expected)) throw new Error("the neutral GIS roster did not match itself");
    let shapeRejections = 0;
    let identityRejections = 0;
    for (const hostile of fixture.hostile) {
      const candidate = structuredClone(fixture.expected);
      if (hostile.operation === "remove") candidate.declared = [];
      else if (hostile.operation === "duplicate") candidate.declared.push(structuredClone(candidate.declared[0]));
      else candidate.declared[0][hostile.field] = hostile.value;
      if (!validate(candidate)) shapeRejections += 1;
      else if (!matchesRoster(candidate)) identityRejections += 1;
      else throw new Error(`GIS discovery oracle admitted ${hostile.name}`);
    }
    const descriptor = JSON.parse(readFileSync(join(this.repoRoot, "✏️s", "🔌️plugins", "🌍️gis", "🔣️.json"), "utf8"));
    const contributions = descriptor.contributions;
    // 🔗️ A descriptor omits `dependsOn` when a service declares none; `ContributedInferenceMetadata`
    // decodes that absence to an empty `Vec`, which is what the MCP tool then answers with. Defaulting
    // it here reads the SAME roster the Rust law compares against, instead of two shapes of "none".
    const declared = [...(contributions.inferenceServices ?? []), ...(contributions.artifactContributions ?? []).flatMap((row: { inferences?: unknown[] }) => row.inferences ?? [])].map((row: Record<string, unknown>) => ({
      ...row,
      dependsOn: row.dependsOn ?? [],
    }));
    const actual = { declared };
    if (!validate(actual)) throw new Error(`committed GIS descriptor discovery drift: ${JSON.stringify(validate.errors)}`);
    deepStrictEqual(actual, fixture.expected);
    console.log(`gis-inference-discovery-oracle: exact=1 hostile=${fixture.hostile.length} shape-rejected=${shapeRejections} identity-rejected=${identityRejections} execution-authority=0`);
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
      const matches = listed.stdout
        .split("\n")
        .filter((line) => line.endsWith(": test"))
        .map((line) => line.slice(0, -6))
        .filter((name) => name.endsWith(packet.suffix));
      if (listed.status !== 0 || matches.length !== 1) throw new Error(`GIS discovery exact-one preflight failed ${packet.suffix}: status=${listed.status} matches=${matches.length} diagnostic=${listed.stderr.slice(-4000)}`);
      runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", matches[0]!, "--", "--exact", "--test-threads=1"], packet.root);
    }
    runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features"], this.root);
    console.log("gis-inference-discovery-check: committed descriptor, exact MCP tool trace, all-feature compile; no execution claim");
  }
}

/** 🔐 Independently validates the fail-closed Hub-selected discovery contract. */
class HubLiveCatalogOracleScript extends BundleScript {
  run(): void {
    const fixtureRoot = join(this.root, "..", "..", "🏠️workspace", "🧫️fixtures", "🔐️hub-live-catalog");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const validate = workspaceContract(this.root, "HubLiveCatalogContractV1");
    if (!validate(fixture)) throw new Error("invalid Hub live-catalog fixture: " + JSON.stringify(validate.errors));
    const project = (binding: string): string[] => (binding === "ready" ? [fixture.selection.package.pluginId] : []);
    for (const state of fixture.states) {
      deepStrictEqual(project(state.binding), state.selectedPlugins);
      if (state.selectedPlugins.includes(fixture.localOnly.pluginId) || state.localFallback) throw new Error(state.binding + " admitted installed fallback");
    }
    const matches = (candidate: any): boolean => {
      const packageIdentity = candidate.package;
      return (
        candidate.scope.spaceId === fixture.selection.scope.spaceId &&
        candidate.scope.documentId === fixture.selection.scope.documentId &&
        candidate.descriptorDigestV1 === fixture.selection.descriptorDigestV1 &&
        packageIdentity.pluginId === fixture.selection.package.pluginId &&
        packageIdentity.packageId === fixture.selection.package.packageId &&
        packageIdentity.version === fixture.selection.package.version &&
        packageIdentity.componentSha256 === fixture.selection.package.componentSha256 &&
        packageIdentity.descriptorByteSha256 === fixture.selection.package.descriptorByteSha256 &&
        packageIdentity.executionProtocol?.appChannelVersion === fixture.selection.package.executionProtocol.appChannelVersion
      );
    };
    if (!matches(fixture.selection)) throw new Error("positive Hub selection did not match itself");
    for (const hostile of fixture.hostile) {
      const candidate = structuredClone(fixture.selection);
      if (hostile.field === "scope" || hostile.field === "descriptorDigestV1") candidate[hostile.field] = hostile.value;
      else candidate.package[hostile.field] = hostile.value;
      if (matches(candidate)) throw new Error("Hub live-catalog oracle admitted " + hostile.name);
    }
    console.log("hub-live-catalog-oracle: AJV=1 states=" + fixture.states.length + " hostile=" + fixture.hostile.length + " execution-protocol=compiled-lease local-fallback=denied");
  }
}

/** 🧪 Runs source/neutral checks without claiming native transport execution. */
class HubLiveCatalogCheckScript extends BundleScript {
  run(): void {
    runCmd("bun", ["./📜️script.ts", "hub-live-catalog-oracle"], { cwd: this.root, budgetMs: 60_000 });
    const workspace = readFileSync(join(this.root, "..", "..", "🏠️workspace", "🦀️.rs"), "utf8");
    const remote = readFileSync(join(this.root, "..", "..", "🏠️workspace", "🔗️remote", "🦀️.rs"), "utf8");
    const inference = readFileSync(join(this.root, "..", "..", "💡️inference", "🦀️.rs"), "utf8");
    const root = readFileSync(join(this.root, "..", "..", "🦀️.rs"), "utf8");
    for (const marker of ["verified_hub_catalog_selections", "discovery_descriptors", "ready_catalog_snapshot"]) if (!workspace.includes(marker)) throw new Error("workspace live-catalog source missing " + marker);
    for (const marker of ["document_execution_target_manifest", "document_execution_target_descriptor", "authority_generation", "descriptor_byte_sha256"]) if (!remote.includes(marker)) throw new Error("remote live-catalog source missing " + marker);
    const inferenceDiscovery = inference.slice(inference.indexOf("pub fn declared_inferences_for_workspace"), inference.indexOf("fn resolve_artifact_schema"));
    if (inferenceDiscovery.includes("find_plugin_entry") || inferenceDiscovery.includes("load_plugin_registry") || !inferenceDiscovery.includes("workspace.discovery_descriptors()"))
      throw new Error("Hub inference discovery still owns a registry fallback");
    if (!root.includes("struct WorkspaceToolRegistry") || !root.includes("workspace.discovery_catalog()") || !root.includes("workspace_tool_catalog_meta") || !root.includes("hubSelectedPackages") || !root.includes("selection.lease.package.execution_protocol.app_channel_version"))
      throw new Error("tools/list is not projected from live workspace discovery");
    console.log("hub-live-catalog-source: workspace=3 remote=4 inference=no-registry-fallback tools=live-selection-projection");
  }
}

/** 🦀 Runs the exact native Hub selection/revocation laws. */
class HubLiveCatalogNativeCheckScript extends BundleScript {
  run(): void {
    const suffixes = ["authenticated_hub_catalog_hydrates_exact_selected_descriptor_and_revocation_removes_it", "authenticated_hub_discovery_uses_retained_selection_and_never_installed_fallback"];
    for (const suffix of suffixes) {
      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "--lib", suffix, "--", "--list"], { cwd: this.root, ...orchestratorBudgetOpts() });
      const matches = listed.stdout
        .split("\n")
        .filter((line) => line.endsWith(": test"))
        .map((line) => line.slice(0, -6))
        .filter((name) => name.endsWith(suffix));
      if (listed.status !== 0 || matches.length !== 1) throw new Error("Hub live-catalog exact-one preflight failed " + suffix + ": status=" + listed.status + " matches=" + matches.length + " diagnostic=" + listed.stderr.slice(-16_000));
      const executed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "--lib", matches[0]!, "--", "--exact", "--test-threads=1"], { cwd: this.root, ...orchestratorBudgetOpts() });
      if (executed.status !== 0) throw new Error("Hub live-catalog exact law failed " + matches[0] + ": status=" + executed.status + " stdout=" + executed.stdout.slice(-8_000) + " stderr=" + executed.stderr.slice(-8_000));
    }
    console.log("hub-live-catalog-native: laws=" + suffixes.length + " authenticated-selection=1 revoked-fallback=denied");
  }
}

class CanonicalCheckpointResourceOracleScript extends BundleScript {
  run(): void {
    const fixtureRoot = join(this.root, "..", "..", "🏠️workspace", "🧫️fixtures", "🔐️canonical-checkpoint-resource");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const validate = workspaceContract(this.root, "CanonicalCheckpointResourceContractV1");
    if (!validate(fixture)) throw new Error("invalid canonical checkpoint resource fixture: " + JSON.stringify(validate.errors));
    const digest = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
    const pack = Buffer.from(fixture.resource.value.pack.base64, "base64");
    const spr = Buffer.from(fixture.resource.value.spr.base64, "base64");
    deepStrictEqual(
      {
        pack: { byteLength: pack.byteLength, sha256: digest(pack), base64: pack.toString("base64") },
        spr: { byteLength: spr.byteLength, sha256: digest(spr), base64: spr.toString("base64") },
      },
      { pack: fixture.resource.value.pack, spr: fixture.resource.value.spr },
    );
    const scopedUri = (spaceId: string, documentId: string): string => `semio://workspace/scopes/${encodeURIComponent(spaceId)}/${encodeURIComponent(documentId)}/checkpoint`;
    if (scopedUri(fixture.resource.value.scope.spaceId, fixture.resource.value.scope.documentId) !== fixture.resource.uri) throw new Error("checkpoint URI is not exact percent-encoded scope");
    const matchesGis = (candidate: { artifactKind: string; artifactSchema: string }): boolean => candidate.artifactKind === fixture.selector.artifactKind && candidate.artifactSchema === fixture.selector.artifactSchema;
    if (!matchesGis(fixture.selector) || fixture.selector.hostile.some(matchesGis)) throw new Error("GIS kind/schema selector admitted a partial identity");
    const rawLength = pack.byteLength + spr.byteLength;
    const base64Length = (length: number): number => Math.ceil(length / 3) * 4;
    if (rawLength > fixture.limits.pairBytes || base64Length(pack.byteLength) + base64Length(spr.byteLength) + fixture.limits.metadataBytes > fixture.limits.textBytes)
      throw new Error("fixture violates checkpoint resource budgets");
    console.log(`canonical-checkpoint-resource-oracle: AJV=1 parts=2 hostile=${fixture.hostile.length} lifecycle=${fixture.lifecycle.length} selector=${fixture.selector.hostile.length}`);
  }
}

class CanonicalCheckpointResourceCheckScript extends BundleScript {
  run(): void {
    runCmd("bun", ["./📜️script.ts", "canonical-checkpoint-resource-oracle"], { cwd: this.root, budgetMs: 60_000 });
    const workspace = readFileSync(join(this.root, "..", "..", "🏠️workspace", "🦀️.rs"), "utf8");
    const remote = readFileSync(join(this.root, "..", "..", "🏠️workspace", "🔗️remote", "🦀️.rs"), "utf8");
    const pair = readFileSync(join(this.root, "..", "..", "🏠️workspace", "🔗️remote", "🧩️pair", "🦀️.rs"), "utf8");
    const context = readFileSync(join(this.root, "..", "..", "🧠️context", "🦀️.rs"), "utf8");
    for (const marker of ["checkpoint_resource_uri", "parse_checkpoint_resource_uri", "read_canonical_checkpoint", "CANONICAL_CHECKPOINT_RESOURCE_MAX_TEXT_BYTES"])
      if (!remote.includes(marker)) throw new Error("remote checkpoint source missing " + marker);
    for (const marker of ["project_mounted_canonical_pair", "project_mounted", "validate_mount_current"])
      if (!pair.includes(marker)) throw new Error("retained pair projection missing " + marker);
    if (!workspace.includes("parse_checkpoint_resource_uri") || !workspace.includes("checkpoint_resource_uri") || !workspace.includes("is_gis_map_descriptor"))
      throw new Error("workspace checkpoint routing or exact GIS selector is missing");
    if (!context.includes('uri.starts_with("semio://workspace/scopes/")')) throw new Error("workspace resource registry rejects exact scoped checkpoint URIs");
    console.log("canonical-checkpoint-resource-source: uri=scope-exact retained-pair=private final-fence=2 raw-limit=4MiB text-limit=6MiB GIS-selector=exact");
  }
}

class CanonicalCheckpointResourceNativeCheckScript extends BundleScript {
  run(): void {
    const suffixes = ["authenticated_hub_checkpoint_resource_projects_exact_verified_pair_and_never_crosses_scope", "gis_map_inference_selector_requires_the_exact_kind_and_schema_pair"];
    for (const suffix of suffixes) {
      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "--lib", suffix, "--", "--list"], { cwd: this.root, ...orchestratorBudgetOpts() });
      const matches = listed.stdout
        .split("\n")
        .filter((line) => line.endsWith(": test"))
        .map((line) => line.slice(0, -6))
        .filter((name) => name.endsWith(suffix));
      if (listed.status !== 0 || matches.length !== 1) throw new Error(`canonical checkpoint resource exact-one preflight failed ${suffix}: status=${listed.status} matches=${matches.length} diagnostic=${listed.stderr.slice(-16_000)}`);
      const executed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "--lib", matches[0]!, "--", "--exact", "--test-threads=1"], { cwd: this.root, ...orchestratorBudgetOpts() });
      if (executed.status !== 0) throw new Error(`canonical checkpoint resource exact law failed ${matches[0]}: status=${executed.status} stdout=${executed.stdout.slice(-8_000)} stderr=${executed.stderr.slice(-8_000)}`);
    }
    console.log(`canonical-checkpoint-resource-native: laws=${suffixes.length} retained-pair=1 GIS-selector=exact`);
  }
}

/** ▶️ `bun ./📜️script.ts dev [-- stdio [flags...]]` — boots the real stdio server for local/manual
 *  smoke testing (`printf '<json-rpc line>' | bun ./📜️script.ts dev -- stdio | ...`). Defaults to
 *  `stdio` when no mode is given, matching `🚀️bin.rs`'s own default-less argv contract. */
class DevScript extends BundleScript {
  run(segments: string[]): void {
    const args = segments.length > 0 ? segments : ["stdio"];
    runCmd(requireMcpBinary(this.repoRoot), args, { cwd: this.root, ...daemonBudgetOpts() });
  }
}

/** 🪞️ `bun ./📜️script.ts schema-mirror [--check]` — regenerates (or verifies) the `os.mcp` scope's
 *  two language mirrors from the ONE Rust registry, by running the built binary's own
 *  `semio-os-mcp schemas` emitter. `--check` writes nothing and fails on any drift, so a stale
 *  mirror is a red gate rather than a silently divergent contract. */
class SchemaMirrorScript extends BundleScript {
  run(segments: string[]): void {
    const check = segments.includes("--check");
    if (segments.some((segment) => segment !== "--check")) throw new Error("schema-mirror accepts only --check");
    const schemaRoot = join(this.root, "..", "..", "🧬️schema");
    const emitted = runProbe(requireMcpBinary(this.repoRoot), ["schemas"], { cwd: this.root, budgetMs: 60_000 });
    if (emitted.status !== 0) throw new Error(`semio-os-mcp schemas failed: status=${emitted.status} stderr=${emitted.stderr.slice(-4_000)}`);
    const json = emitted.stdout;
    const document = JSON.parse(json) as SchemaDocument;
    const typescript = renderSchemaTypescript(document);
    const jsonPath = join(schemaRoot, "🔣️.json");
    const typescriptPath = join(schemaRoot, "🟦️.ts");
    if (check) {
      for (const [path, expected] of [
        [jsonPath, json],
        [typescriptPath, typescript],
      ] as const) {
        if (readFileSync(path, "utf8") !== expected) throw new Error(`${path} is stale — run \`bun nx run @semio-tech/framework-os-mcp-rs:schema-mirror\``);
      }
    } else {
      writeFileSync(jsonPath, json);
      writeFileSync(typescriptPath, typescript);
    }
    const ajv = new Ajv({ strict: true, allErrors: true });
    ajv.addSchema(document);
    for (const exportId of Object.keys(document.$defs)) {
      if (!ajv.getSchema(`${document.$id}#/$defs/${exportId}`)) throw new Error(`AJV draft-07 could not resolve ${exportId}`);
    }
    console.log(`schema-mirror${check ? "-check" : ""}: exports=${Object.keys(document.$defs).length} ajv-draft07-resolved=${Object.keys(document.$defs).length} json=1 typescript=1`);
  }
}

type SchemaNode = Record<string, unknown>;
type SchemaDocument = { $id: string; $defs: Record<string, SchemaNode> };

const PASCAL = /^[A-Z][A-Za-z0-9]*$/;

/** 🔤️ `RevisionStamp` → `parseRevisionStamp`. */
const parserName = (exportId: string): string => `parse${exportId}`;

/** 🧾️ TypeScript for one schema node — precise where the document is precise, `JsonValue` where it
 * is deliberately permissive (a free-form `{}` or an untyped property). Never invents a shape. */
function renderType(node: SchemaNode | boolean, indent: string): string {
  if (typeof node === "boolean") return node ? "JsonValue" : "never";
  const reference = node.$ref;
  if (typeof reference === "string") return reference.startsWith("#/$defs/") ? reference.slice("#/$defs/".length) : "JsonValue";
  if (Array.isArray(node.enum)) return node.enum.map((member) => JSON.stringify(member)).join(" | ");
  if ("const" in node) return JSON.stringify(node.const);
  for (const key of ["anyOf", "oneOf"] as const) {
    const branches = node[key];
    if (Array.isArray(branches)) return branches.map((branch) => renderType(branch as SchemaNode, indent)).join(" | ");
  }
  if (Array.isArray(node.allOf)) return (node.allOf as SchemaNode[]).map((branch) => renderType(branch, indent)).join(" & ");
  const declared = node.type;
  if (Array.isArray(declared)) return declared.map((member) => renderType({ ...node, type: member }, indent)).join(" | ");
  switch (declared) {
    case "null":
      return "null";
    case "boolean":
      return "boolean";
    case "string":
      return "string";
    case "integer":
    case "number":
      return "number";
    case "array":
      return `readonly ${renderType((node.items as SchemaNode | boolean | undefined) ?? true, indent)}[]`;
    case "object": {
      const properties = node.properties as Record<string, SchemaNode> | undefined;
      if (!properties || Object.keys(properties).length === 0) return "{ readonly [key: string]: JsonValue }";
      const required = new Set((node.required as string[] | undefined) ?? []);
      const inner = `${indent}  `;
      const fields = Object.entries(properties).map(([name, member]) => `${inner}readonly ${JSON.stringify(name)}${required.has(name) ? "" : "?"}: ${renderType(member, inner)};`);
      // 📐️ No `[key: string]: JsonValue` catch-all even where the schema permits extra properties:
      // TypeScript requires every optional member to be assignable to the index type, which a
      // `T | undefined` never is, and the committed `🔣️.json` — not this type — is the authority on
      // what else a document may carry.
      return `{\n${fields.join("\n")}\n${indent}}`;
    }
    default:
      return "JsonValue";
  }
}

/** 🖨️ The whole generated `🧬️schema/🟦️.ts`: a dependency-free draft-07 walker over the committed
 * `🔣️.json`, one exported type per `$defs` entry, one `parse<ExportId>` per type. */
function renderSchemaTypescript(document: SchemaDocument): string {
  const exportIds = Object.keys(document.$defs);
  for (const exportId of exportIds) if (!PASCAL.test(exportId)) throw new Error(`${exportId} is not a PascalCase ExportId`);
  const types = exportIds.map((exportId) => `export type ${exportId} = ${renderType(document.$defs[exportId]!, "")};`);
  const parsers = exportIds.map((exportId) => `export const ${parserName(exportId)} = (value: unknown): ${exportId} => parseExport("${exportId}", value) as ${exportId};`);
  return `${GENERATED_PRELUDE}
//#region 🔖️Exports
/** 🆔️ Every ExportId this scope publishes, in the document's own (key-sorted) order. */
export const OS_MCP_EXPORT_IDS = [${exportIds.map((exportId) => `"${exportId}"`).join(", ")}] as const;

export type OsMcpExportId = (typeof OS_MCP_EXPORT_IDS)[number];

${types.join("\n\n")}
//#endregion 🔖️Exports

//#region 🔖️Parsers
${parsers.join("\n")}
//#endregion 🔖️Parsers
`;
}

const GENERATED_PRELUDE = `/** 🧬️ \`@generated\` — do NOT edit. Regenerate with
 * \`bun nx run @semio-tech/framework-os-mcp-rs:schema-mirror\`, which runs \`semio-os-mcp schemas\`
 * over \`🌉️mcp/🧬️schema/🦀️.rs\`'s \`schemas()\` — the single registry every \`os.mcp\` schema lives in.
 *
 * This file validates against the committed \`./🔣️.json\` with its own walker rather than an
 * external validator: a \`🔨️modules/*\` component file must not take a runtime dependency on a
 * package (CLAUDE.md, "no runtime dependencies on external libraries"). The third-party
 * cross-check lives in \`📦️packages/🟦️typescript\`, which validates the SAME document with AJV. */
import { readFileSync } from "node:fs";

//#region 🔖️Document
export type JsonValue = null | boolean | number | string | readonly JsonValue[] | { readonly [key: string]: JsonValue };

type SchemaNode = boolean | { readonly [keyword: string]: JsonValue };

/** 📄️ The committed draft-07 document, read once. */
export const OS_MCP_SCHEMA_DOCUMENT = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")) as { readonly $id: string; readonly $defs: Record<string, SchemaNode> };

/** 🆔️ \`$id\` of the document above — the base every \`#/$defs/<ExportId>\` pointer resolves against. */
export const OS_MCP_SCHEMA_ID = OS_MCP_SCHEMA_DOCUMENT.$id;

/** 🔍️ One named export's raw schema node, for a consumer that wants to hand it to its own validator. */
export function osMcpSchema(exportId: string): SchemaNode {
  const node = OS_MCP_SCHEMA_DOCUMENT.$defs[exportId];
  if (node === undefined) throw new Error(\`os.mcp publishes no \${exportId} schema\`);
  return node;
}
//#endregion 🔖️Document

//#region 🔖️Walker
const typeOf = (value: unknown): string => (value === null ? "null" : Array.isArray(value) ? "array" : Number.isInteger(value) ? "integer" : typeof value);

const sameJson = (left: unknown, right: unknown): boolean => JSON.stringify(left) === JSON.stringify(right);

function violations(node: SchemaNode, value: unknown, path: string): string[] {
  if (node === true) return [];
  if (node === false) return [\`\${path}: no value is valid here\`];
  const found: string[] = [];
  const reference = node.$ref;
  if (typeof reference === "string") return violations(osMcpSchema(reference.slice("#/$defs/".length)), value, path);
  const actual = typeOf(value);
  const declared = node.type;
  const admits = (candidate: JsonValue): boolean => candidate === actual || (candidate === "number" && actual === "integer");
  if (typeof declared === "string" && !admits(declared)) found.push(\`\${path}: expected \${declared}, found \${actual}\`);
  if (Array.isArray(declared) && !declared.some(admits)) found.push(\`\${path}: expected one of \${declared.join("|")}, found \${actual}\`);
  if ("const" in node && !sameJson(node.const, value)) found.push(\`\${path}: must equal \${JSON.stringify(node.const)}\`);
  if (Array.isArray(node.enum) && !node.enum.some((member) => sameJson(member, value))) found.push(\`\${path}: not one of \${JSON.stringify(node.enum)}\`);
  if (Array.isArray(node.anyOf) && !node.anyOf.some((branch) => violations(branch as SchemaNode, value, path).length === 0)) found.push(\`\${path}: matched no anyOf branch\`);
  if (Array.isArray(node.oneOf) && (node.oneOf as SchemaNode[]).filter((branch) => violations(branch, value, path).length === 0).length !== 1) found.push(\`\${path}: must match exactly one oneOf branch\`);
  if (Array.isArray(node.allOf)) for (const branch of node.allOf as SchemaNode[]) found.push(...violations(branch, value, path));
  if (node.not !== undefined && violations(node.not as SchemaNode, value, path).length === 0) found.push(\`\${path}: matched a forbidden schema\`);
  if (typeof value === "string") {
    if (typeof node.minLength === "number" && value.length < node.minLength) found.push(\`\${path}: shorter than \${node.minLength}\`);
    if (typeof node.maxLength === "number" && value.length > node.maxLength) found.push(\`\${path}: longer than \${node.maxLength}\`);
    if (typeof node.pattern === "string" && !new RegExp(node.pattern, "u").test(value)) found.push(\`\${path}: does not match \${node.pattern}\`);
  }
  if (typeof value === "number") {
    if (typeof node.minimum === "number" && value < node.minimum) found.push(\`\${path}: below \${node.minimum}\`);
    if (typeof node.maximum === "number" && value > node.maximum) found.push(\`\${path}: above \${node.maximum}\`);
  }
  if (Array.isArray(value)) {
    if (typeof node.minItems === "number" && value.length < node.minItems) found.push(\`\${path}: fewer than \${node.minItems} items\`);
    if (typeof node.maxItems === "number" && value.length > node.maxItems) found.push(\`\${path}: more than \${node.maxItems} items\`);
    if (node.uniqueItems === true && new Set(value.map((item) => JSON.stringify(item))).size !== value.length) found.push(\`\${path}: items are not unique\`);
    const items = node.items;
    if (items !== undefined && !Array.isArray(items)) for (const [index, item] of value.entries()) found.push(...violations(items as SchemaNode, item, \`\${path}/\${index}\`));
    if (Array.isArray(items)) for (const [index, member] of items.entries()) if (index < value.length) found.push(...violations(member as SchemaNode, value[index], \`\${path}/\${index}\`));
  }
  if (actual === "object") {
    const record = value as Record<string, unknown>;
    const properties = (node.properties ?? {}) as Record<string, SchemaNode>;
    for (const name of (node.required ?? []) as string[]) if (!(name in record)) found.push(\`\${path}/\${name}: required\`);
    if (typeof node.minProperties === "number" && Object.keys(record).length < node.minProperties) found.push(\`\${path}: fewer than \${node.minProperties} properties\`);
    if (typeof node.maxProperties === "number" && Object.keys(record).length > node.maxProperties) found.push(\`\${path}: more than \${node.maxProperties} properties\`);
    for (const [name, member] of Object.entries(record)) {
      const declaredMember = properties[name];
      if (declaredMember !== undefined) found.push(...violations(declaredMember, member, \`\${path}/\${name}\`));
      else if (node.additionalProperties === false) found.push(\`\${path}/\${name}: not permitted\`);
      else if (typeof node.additionalProperties === "object" && node.additionalProperties !== null) found.push(...violations(node.additionalProperties as SchemaNode, member, \`\${path}/\${name}\`));
    }
  }
  return found;
}

/** ✅️ Every reason \`value\` fails one named export — empty means it conforms. */
export function osMcpViolations(exportId: string, value: unknown): readonly string[] {
  return violations(osMcpSchema(exportId), value, "");
}

function parseExport(exportId: string, value: unknown): unknown {
  const found = osMcpViolations(exportId, value);
  if (found.length > 0) throw new Error(\`\${exportId}: \${found.join("; ")}\`);
  return value;
}
//#endregion 🔖️Walker
`;

const router = new ScriptRouter(import.meta.dir)
  .register("build", BuildScript)
  .register("check", CheckScript)
  .register("test", TestScript)
  .register("canonical-pair-check", CanonicalPairCheckScript)
  .register("inference-discovery-oracle", InferenceDiscoveryOracleScript)
  .register("inference-discovery-check", InferenceDiscoveryCheckScript)
  .register("hub-live-catalog-oracle", HubLiveCatalogOracleScript)
  .register("hub-live-catalog-check", HubLiveCatalogCheckScript)
  .register("hub-live-catalog-native-check", HubLiveCatalogNativeCheckScript)
  .register("canonical-checkpoint-resource-oracle", CanonicalCheckpointResourceOracleScript)
  .register("canonical-checkpoint-resource-check", CanonicalCheckpointResourceCheckScript)
  .register("canonical-checkpoint-resource-native-check", CanonicalCheckpointResourceNativeCheckScript)
  .register("schema-mirror", SchemaMirrorScript)
  .register("dev", DevScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
