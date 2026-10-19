import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { createIssuedResourceControlV1, createRequestedIssuedResourceControlV1, scriptInvocationEnvironment } from "@semio-tech/framework/process/routing";
import { runOwnedCommand } from "@semio-tech/framework/process/execution";
import { prepareCargoWorkspaceInvocation } from "@semio-tech/framework/process/native-build";
import { devToolingEnv } from "@semio-tech/framework/process/execution";
import { buildBudgetMs } from "@semio-tech/framework/process/budget";
import { acquireCargoBuildLeaseV1 } from "@semio-tech/framework/process/native-build";
import { repoCacheDirectory } from "@semio-tech/framework/process/execution";
import { isAbsolute, join, relative, resolve, sep } from "node:path";
import { lstatSync, mkdirSync, readFileSync, realpathSync } from "node:fs";
import { resolve as scriptDirectoryResolve } from "node:path";

const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dir, "../../📦️packages/🦀️rust");

const familyRoot = resolve(SCRIPT_DIR, "../..");
export const familyPackages = (Bun.TOML.parse(readFileSync(join(familyRoot, "Cargo.toml"), "utf8")) as { workspace: { members: string[] } }).workspace.members.map((member) => (Bun.TOML.parse(readFileSync(join(familyRoot, member, "Cargo.toml"), "utf8")) as { package: { name: string } }).package.name);

/** 🏃️ Runs the `NativeSchemaScript` command under the caller's roots and invocation. */
export async function runNativeSchemaScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts || !isAbsolute(artifacts) || segments.length > 1) throw new Error("Native GLTF schema proof requires an absolute caller artifact directory and at most one copied workspace");
  mkdirSync(artifacts, { recursive: true });
  const workspace = segments[0] ? realpathSync(segments[0]) : context.repoRoot;
  if (segments[0]) {
    const local = relative(realpathSync(artifacts), workspace);
    if (!local || isAbsolute(local) || local === ".." || local.startsWith(".." + sep) || workspace === context.repoRoot || lstatSync(segments[0]).isSymbolicLink()) throw new Error("Native GLTF copied workspace must be contained in caller artifacts");
  }
  const manifest = resolve(workspace, relative(context.repoRoot, context.root), "Cargo.toml");
  const target = segments[0] ? join(workspace, "target") : join(artifacts, "gltf-native-target");
  const env = devToolingEnv({ SEMIO_WORKSPACE_ROOT: workspace, CARGO_TARGET_DIR: target, CARGO_BUILD_BUILD_DIR: join(target, "intermediate") });
  const laws: readonly (readonly [string, string])[] = [
    ["semio-s-artifact-stdio-gltf", "definition_tests::native_codec_identity_matches_structural_graph_and_independent_blake3"],
    ["semio-s-artifact-stdio-gltf-model", "standards::v2_0::subsets::any::io::component::binary::snapshot::owned_pack::borrowed_carrier_tests::borrowed_gltf_carriers_match_the_neutral_owned_metadata_and_serde_oracle"],
    ["semio-s-artifact-stdio-gltf-inferences", "standards::v2_0::subsets::any::io::text::inferences::tests::canonical_json_bytes_matches_the_portable_byte_array_contract"],
    ["semio-s-artifact-stdio-gltf-model", "standards::v2_0::subsets::any::io::component::sqlite::snapshot::tests::sqlite_snapshot_gltf_empty_image_and_texture_entities_survive_logical_native_lists"],
    ["semio-s-artifact-stdio-gltf-model", "standards::v2_0::subsets::any::io::component::sqlite::snapshot::tests::sqlite_snapshot_gltf_controlled_binding_retires_deep_completed_fields_after_failure_and_cancellation"],
    ["semio-s-artifact-stdio-gltf-model", "standards::v2_0::subsets::any::io::component::sqlite::snapshot::tests::sqlite_snapshot_gltf_independent_bad_later_json_owner_retires_deep_completed_sibling"],
  ];
  const args = ["test", "--offline", "--manifest-path", manifest, ...[...new Set(laws.map(([name]) => name))].flatMap((name) => ["-p", name]), "--lib"];
  const budget = buildBudgetMs() || 1_200_000, phaseControl = createIssuedResourceControlV1(context.invocation, "gltf:native-schema", 0);
  await prepareCargoWorkspaceInvocation(workspace,args,workspace,scriptInvocationEnvironment(context.invocation, process.env),createRequestedIssuedResourceControlV1(phaseControl, budget));
  const controller = new AbortController(), abort = () => controller.abort(), timer = setTimeout(abort, budget);
  process.once("SIGINT", abort); process.once("SIGTERM", abort);
  let lease: Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>> | undefined;
  try {
    lease = await acquireCargoBuildLeaseV1({ directory: repoCacheDirectory(workspace, "agents", "resource-leases"), buildDirectory: env.CARGO_BUILD_BUILD_DIR!, targetDirectory: target, args, signal: controller.signal });
    for (const [name, law] of laws) {
      let observed = false;
      await runOwnedCommand("cargo", ["test", "--offline", "--manifest-path", manifest, "-p", name, "--lib", law, "--", "--exact", "--nocapture"], workspace, "gltf-native-schema", budget, { env, signal: controller.signal, onLine: line => { if (line.includes(`test ${law} ... ok`)) observed = true; } });
      if (!observed) throw new Error(`Native GLTF law did not execute: ${law}`);
    }
  } finally {
    clearTimeout(timer); process.off("SIGINT", abort); process.off("SIGTERM", abort); if (lease) await lease.release();
  }
}
