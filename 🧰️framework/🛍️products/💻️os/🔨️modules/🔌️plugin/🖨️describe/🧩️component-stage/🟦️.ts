import { buildModePolicy, readBuildMode } from "../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { cargoBuildDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { buildRepositoryCargoArtifacts } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { RELEASE_DRIVER_POLICY_V1 } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { DESCRIPTOR_JSON_REL_PATH, findPluginCargoFiles, parseComponentSourceOwnerV1, type ComponentSourceOwnerV1 } from "../../📇️registry/🔎️discovery/🟦️.ts";
import { COMPONENT_PROFILE_POLICIES, COMPONENT_RECEIPT_SCHEMA_ID, COMPONENT_TARGET, SHIPPED_COMPONENT_PROFILE, componentDeliverableV1, componentProfileV1, componentProvenanceDigestsV1, rustToolchainIdentityV1, writeComponentReceiptV1, type ComponentDeliverableV1, type ComponentProfileV1, type ComponentReceiptV1 } from "../🏷️component-receipt/🟦️.ts";
import { decodeCargoProvenanceV1 } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
import { fileSha256V1, STAGE_CACHE_POLICY_V1, stageCacheV1, type StageCacheV1 } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts";
import { repoCacheDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { verifySemanticDescriptorPairBytesV1 } from "../../📇️registry/✅️catalog-verification/🟦️.ts";
import { DESCRIPTOR_JSON_FILENAME, DESCRIPTOR_PACK_FILENAME, freshWasmArtifactSize, publishedDescriptorEmitter } from "../🏗️component-build/🟦️.ts";
import { emitSemanticDescriptorV1, type DescriptorEmissionControlV1, type SemanticDescriptorEmissionV1 } from "../🛂️descriptor-emission/🟦️.ts";

//#region 🔖️ComponentStage
/** 🪪️ One plugin or extension component crate as the describe stage sees it: its discovered source identity plus the
 * crate root its deliverables live under and the owner root its committed semantic descriptor pair lives at. */
export type ComponentStageTargetV1 = ComponentSourceOwnerV1 & Readonly<{ manifestPath: string; crateRoot: string; ownerRoot: string }>;

/** 🔍️ Admits one component manifest through registry discovery, the single source of component identity. */
export function componentStageTargetV1(repoRoot: string, manifest: string): ComponentStageTargetV1 {
  const manifestPath = resolve(repoRoot, manifest);
  const source = parseComponentSourceOwnerV1(manifestPath, repoRoot);
  const crateRoot = dirname(manifestPath);
  return Object.freeze({ ...source, manifestPath, crateRoot, ownerRoot: dirname(resolve(repoRoot, source.cratePath, ...DESCRIPTOR_JSON_REL_PATH)) });
}

/** 🗂️ Every discovered component crate, in plugin-id order — the selection `components describe --all` walks. */
export function componentStageTargetsV1(repoRoot: string): readonly ComponentStageTargetV1[] {
  return findPluginCargoFiles(repoRoot).map((path) => componentStageTargetV1(repoRoot, path)).sort((left, right) => left.pluginId.localeCompare(right.pluginId));
}

/** 🦀️ The one cargo invocation that links one component under one profile: exactly the unit the release driver's pass links
 * (`--lib`, the same target, the same per-package feature unification), so after a driver pass it is a fresh check. Only the
 * diagnostic `SEMIO_PLUGIN_SYMBOLS=1` build needs compiler arguments, hence `cargo rustc`. */
export function componentCargoV1(packageName: string, profile: ComponentProfileV1): Readonly<{ command: "build" | "rustc"; args: readonly string[]; environment: Readonly<Record<string, string>> }> {
  const args = ["-p", packageName, "--lib", "--target", COMPONENT_TARGET, "--profile", profile];
  const environment = { CARGO_UNSTABLE_FEATURE_UNIFICATION: "true", CARGO_RESOLVER_FEATURE_UNIFICATION: RELEASE_DRIVER_POLICY_V1.featureUnification };
  return process.env.SEMIO_PLUGIN_SYMBOLS === "1" ? { command: "rustc", args: [...args, "--", "-C", "strip=none"], environment } : { command: "build", args, environment };
}

/** 🏗️ Builds one component crate under one profile through the repository cargo port and stages exactly its linked
 * component into the profile's deliverable directory; resolves with that deliverable and the cargo provenance receipt
 * the build wrote, the evidence the component receipt's `sourceTreeDigest`/`provenanceSha256` are derived from. */
export async function buildComponentDeliverableV1(repoRoot: string, target: ComponentStageTargetV1, profile: ComponentProfileV1, signal?: AbortSignal): Promise<Readonly<{ deliverable: ComponentDeliverableV1; provenance: string }>> {
  const deliverable = componentDeliverableV1(target.crateRoot, target.packageName, profile);
  const cargo = componentCargoV1(target.packageName, profile);
  let provenance: string | undefined;
  await buildRepositoryCargoArtifacts(target.manifestPath, [...cargo.args], repoRoot, {
    command: cargo.command,
    environment: { ...process.env, ...cargo.environment },
    deliverables: [".wasm"],
    output: relative(target.crateRoot, deliverable.directory),
    ...(signal ? { signal } : {}),
    provenance: (path) => { provenance = path; },
    validate: (files) => {
      if (files.size !== 1 || !files.has(target.wasmOut)) throw new Error(`${target.pluginId}: a component deliverable holds exactly ${target.wasmOut}, not ${[...files.keys()].join(", ")}`);
      const path = files.get(target.wasmOut)!;
      if (!Buffer.from(readFileSync(path).subarray(0, 8)).equals(Buffer.from([0, 97, 115, 109, 13, 0, 1, 0]))) throw new Error(`${target.pluginId}: ${target.wasmOut} is not a WASI component`);
      freshWasmArtifactSize(path, COMPONENT_PROFILE_POLICIES[profile].maximumBytes, `${target.pluginId} ${profile} component`);
    },
  });
  if (!provenance || !existsSync(provenance)) throw new Error(`${target.pluginId}: the ${profile} build wrote no cargo provenance receipt`);
  return Object.freeze({ deliverable, provenance });
}

/** 🧭️ A bounded, path-addressed difference between two descriptor values, the canonical diff a refused describe prints. */
export function canonicalDescriptorDiffV1(committed: unknown, emitted: unknown, limit = 40): readonly string[] {
  const rows: string[] = [];
  const walk = (left: unknown, right: unknown, path: string): void => {
    if (rows.length >= limit || isDeepStrictEqual(left, right)) return;
    const objects = left !== null && right !== null && typeof left === "object" && typeof right === "object" && Array.isArray(left) === Array.isArray(right);
    if (!objects) {
      rows.push(`${path || "$"}: committed ${JSON.stringify(left)?.slice(0, 160) ?? "(absent)"} → shipped ${JSON.stringify(right)?.slice(0, 160) ?? "(absent)"}`);
      return;
    }
    const keys = Array.isArray(left) ? Array.from({ length: Math.max(left.length, (right as unknown[]).length) }, (_, index) => String(index)) : [...new Set([...Object.keys(left as object), ...Object.keys(right as object)])].sort();
    for (const key of keys) walk((left as Record<string, unknown>)[key], (right as Record<string, unknown>)[key], Array.isArray(left) ? `${path}[${key}]` : `${path}.${key}`);
  };
  walk(committed, emitted, "");
  return rows;
}

/** 🛂️ How the describe stage treats the committed pair: `verify` refuses any difference with a canonical diff (the shipped
 * `describe` target), `write` replaces the committed pair (`components describe`), `local` keeps the emitted pair beside
 * the iterate bytes and only reports a difference (`components iterate`). */
export type ComponentDescribeModeV1 = "verify" | "write" | "local";

/** 🖋️ Publishes one semantic pair at the owner root as two atomic renames, never a half-written pair. */
function publishSemanticPair(ownerRoot: string, emission: SemanticDescriptorEmissionV1): void {
  const pack = join(ownerRoot, DESCRIPTOR_PACK_FILENAME), json = join(ownerRoot, DESCRIPTOR_JSON_FILENAME), suffix = `.${process.pid}.describe`;
  try {
    writeFileSync(pack + suffix, emission.packBytes, { flag: "wx" });
    writeFileSync(json + suffix, emission.jsonBytes, { flag: "wx" });
    renameSync(pack + suffix, pack);
    renameSync(json + suffix, json);
  } finally {
    rmSync(pack + suffix, { force: true });
    rmSync(json + suffix, { force: true });
  }
}

/** 🛂️ The cached describe stage over one staged deliverable: the semantic descriptor pair exactly these bytes answer, keyed by the component's digest and the
 * emitter binary's digest, strict-decoded and refused when it names another component. */
export async function describeComponentStageV1(repoRoot: string, target: ComponentStageTargetV1, deliverable: ComponentDeliverableV1, cache: StageCacheV1, control: DescriptorEmissionControlV1 = {}): Promise<SemanticDescriptorEmissionV1> {
  const emitter = publishedDescriptorEmitter(repoRoot), [component, tool] = await Promise.all([fileSha256V1(deliverable.wasm), emitterDigest(emitter)]);
  const output = mkdtempSync(join(dirname(deliverable.directory), ".describe-"));
  try {
    await cache.run({ stage: "describe", schemaVersion: 1, inputs: { component }, tools: { emitter: tool } }, output, async (directory) => {
      const emission = await emitSemanticDescriptorV1(repoRoot, { component: deliverable.wasm, artifactRoot: dirname(deliverable.directory) }, control);
      writeFileSync(join(directory, DESCRIPTOR_PACK_FILENAME), emission.packBytes);
      writeFileSync(join(directory, DESCRIPTOR_JSON_FILENAME), emission.jsonBytes);
    }, control.signal);
    const packBytes = readFileSync(join(output, DESCRIPTOR_PACK_FILENAME)), jsonBytes = readFileSync(join(output, DESCRIPTOR_JSON_FILENAME)), pair = verifySemanticDescriptorPairBytesV1(jsonBytes, packBytes);
    if (pair.pluginId !== target.pluginId || pair.packageId !== target.packageId || pair.role !== target.role) throw new Error(`${target.pluginId}: the component describes ${pair.pluginId} (${pair.packageId}, ${pair.role})`);
    return Object.freeze({ pluginId: pair.pluginId, packageId: pair.packageId, role: pair.role, version: pair.version, componentSha256: component, jsonBytes, packBytes, semanticSha256: pair.semanticSha256 });
  } finally {
    rmSync(output, { recursive: true, force: true });
  }
}

const emitterDigests = new Map<string, Promise<string>>();
/** 🪪️ The emitter binary's digest, once per binary identity. */
function emitterDigest(path: string): Promise<string> {
  const stat = lstatSync(path), key = `${path}\0${stat.size}\0${stat.mtimeMs}`;
  let known = emitterDigests.get(key);
  if (!known) { known = fileSha256V1(path); emitterDigests.set(key, known); }
  return known;
}

/** ⚖️ Settles an emitted semantic pair against the committed pair by `mode` and resolves with the canonical drift (empty when equal). A `verify` refusal names
 * every differing path and the command that commits the shipped declaration. */
export function settleComponentDescriptorV1(repoRoot: string, target: ComponentStageTargetV1, deliverable: ComponentDeliverableV1, emission: SemanticDescriptorEmissionV1, mode: ComponentDescribeModeV1): readonly string[] {
  if (mode === "verify" && deliverable.profile !== SHIPPED_COMPONENT_PROFILE) throw new Error(`${target.pluginId}: only shipped ${SHIPPED_COMPONENT_PROFILE} bytes are verified against the committed descriptor`);
  if (mode === "local" && COMPONENT_PROFILE_POLICIES[deliverable.profile].shipped) throw new Error(`${target.pluginId}: a local describe is for the iterate profile only`);
  const committedPack = join(target.ownerRoot, DESCRIPTOR_PACK_FILENAME), committedJson = join(target.ownerRoot, DESCRIPTOR_JSON_FILENAME);
  const committed = existsSync(committedPack) && existsSync(committedJson) ? { pack: readFileSync(committedPack), json: readFileSync(committedJson) } : undefined;
  const equal = committed !== undefined && Buffer.from(committed.pack).equals(Buffer.from(emission.packBytes)) && Buffer.from(committed.json).equals(Buffer.from(emission.jsonBytes));
  const drift = equal ? [] : committed === undefined ? ["$: no committed descriptor pair"] : canonicalDescriptorDiffV1(JSON.parse(committed.json.toString("utf8")), JSON.parse(Buffer.from(emission.jsonBytes).toString("utf8")));
  const command = `bun nx run workspace:components describe ${target.pluginId}`;
  if (mode === "verify" && !equal) throw new Error(`${target.pluginId}: the shipped component describes a different semantic descriptor than the committed pair at ${relative(repoRoot, target.ownerRoot)}:\n  ${(drift.length ? drift : ["$: byte encoding differs"]).join("\n  ")}\ncommit the shipped declaration with: ${command}`);
  if (mode === "write" && !equal) publishSemanticPair(target.ownerRoot, emission);
  if (mode === "local") {
    mkdirSync(deliverable.directory, { recursive: true });
    writeFileSync(join(deliverable.directory, DESCRIPTOR_PACK_FILENAME), emission.packBytes);
    writeFileSync(join(deliverable.directory, DESCRIPTOR_JSON_FILENAME), emission.jsonBytes);
    if (!equal) console.warn(`[local-iterate] ${target.pluginId}: the iterated declaration differs from the committed pair (uncommitted; commit with \`${command} --profile wasm-prototype\`):\n  ${drift.join("\n  ")}`);
  }
  return drift;
}

/** 🧾️ Writes the `ComponentReceiptV1` that binds a settled semantic descriptor to the deliverable's bytes and to the provenance of the units that built them. */
export function writeComponentStageReceiptV1(repoRoot: string, target: ComponentStageTargetV1, deliverable: ComponentDeliverableV1, emission: SemanticDescriptorEmissionV1, provenance: Readonly<{ sourceTreeDigest: string; provenanceSha256: string }>): ComponentReceiptV1 {
  const receipt: ComponentReceiptV1 = Object.freeze({
    schema: COMPONENT_RECEIPT_SCHEMA_ID,
    pluginId: target.pluginId,
    packageId: target.packageId,
    profile: deliverable.profile,
    target: COMPONENT_TARGET,
    wasmSha256: emission.componentSha256,
    wasmBytes: lstatSync(deliverable.wasm).size,
    semanticDescriptorSha256: emission.semanticSha256,
    toolchain: rustToolchainIdentityV1(repoRoot),
    sourceTreeDigest: provenance.sourceTreeDigest,
    provenanceSha256: provenance.provenanceSha256,
  });
  writeComponentReceiptV1(deliverable.receipt, receipt);
  return receipt;
}

/** 🧾️ The describe stage over one narrowly built deliverable: emits (cached) the semantic descriptor of exactly these bytes, settles it against the committed
 * pair by `mode`, then writes the receipt whose provenance is the Merkle root over every unit of that narrow build. */
export async function describeComponentDeliverableV1(repoRoot: string, target: ComponentStageTargetV1, built: Readonly<{ deliverable: ComponentDeliverableV1; provenance: string }>, mode: ComponentDescribeModeV1, control: DescriptorEmissionControlV1 = {}): Promise<Readonly<{ receipt: ComponentReceiptV1; drift: readonly string[] }>> {
  const { deliverable } = built;
  rmSync(deliverable.receipt, { force: true });
  const emission = await describeComponentStageV1(repoRoot, target, deliverable, stageCacheV1(repoCacheDirectory(repoRoot, STAGE_CACHE_POLICY_V1.warmRoot)), control);
  const drift = settleComponentDescriptorV1(repoRoot, target, deliverable, emission, mode);
  const observed = decodeCargoProvenanceV1(readFileSync(built.provenance, "utf8"));
  const receipt = writeComponentStageReceiptV1(repoRoot, target, deliverable, emission, componentProvenanceDigestsV1(observed, { repository: repoRoot, build: cargoBuildDirectory(repoRoot), cargoHome: resolve(process.env.CARGO_HOME ?? join(homedir(), ".cargo")) }));
  return Object.freeze({ receipt, drift });
}

/** 🧩️ The complete stage chain of one component under one profile: build, stage, describe, receipt. */
export async function stageComponentV1(repoRoot: string, target: ComponentStageTargetV1, profile: ComponentProfileV1, mode: ComponentDescribeModeV1, signal?: AbortSignal): Promise<Readonly<{ deliverable: ComponentDeliverableV1; receipt: ComponentReceiptV1; drift: readonly string[] }>> {
  const built = await buildComponentDeliverableV1(repoRoot, target, profile, signal);
  const described = await describeComponentDeliverableV1(repoRoot, target, built, mode, { cancelled: () => signal?.aborted ?? false, ...(signal ? { signal } : {}) });
  return Object.freeze({ deliverable: built.deliverable, ...described });
}

/** 📦️ `component --manifest <Cargo.toml>`: the inferred Nx `describe` target of every plugin and extension crate — the
 * shipped `wasm-release` bytes (a fresh check after the release driver linked them), verified against the committed
 * semantic descriptor, with their receipt. */
export class ComponentScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 2 || segments[0] !== "--manifest") throw new Error("usage: component --manifest <Cargo.toml>");
    const controller = new AbortController(), abort = (): void => controller.abort();
    process.once("SIGINT", abort);
    process.once("SIGTERM", abort);
    try {
      const target = componentStageTargetV1(this.repoRoot, segments[1]!);
      const staged = await stageComponentV1(this.repoRoot, target, SHIPPED_COMPONENT_PROFILE, "verify", controller.signal);
      console.log(`component ${target.pluginId}: ${staged.receipt.wasmBytes} B ${staged.receipt.profile} wasm ${staged.receipt.wasmSha256.slice(0, 12)}… describes the committed semantic descriptor ${staged.receipt.semanticDescriptorSha256.slice(0, 12)}… -> ${relative(this.repoRoot, staged.deliverable.receipt)}`);
    } finally {
      process.removeListener("SIGINT", abort);
      process.removeListener("SIGTERM", abort);
    }
  }
}

/** 🎛️ Parses `describe <plugin-id…>|--all [--profile wasm-release|wasm-prototype]` into a selection and a profile. */
export function componentsDescribeArgumentsV1(segments: readonly string[]): Readonly<{ all: boolean; pluginIds: readonly string[]; profile: ComponentProfileV1 }> {
  const flag = segments.indexOf("--profile");
  const profile = flag === -1 ? buildModePolicy(readBuildMode(process.env.SEMIO_BUILD_MODE)).wasmProfile : componentProfileV1(segments[flag + 1]);
  const rest = flag === -1 ? segments : [...segments.slice(0, flag), ...segments.slice(flag + 2)];
  const all = rest.length === 1 && rest[0] === "--all";
  if (!all && (rest.length === 0 || rest.some((value) => !/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(value)))) throw new Error("usage: components describe <plugin-id…>|--all [--profile wasm-release|wasm-prototype]");
  return Object.freeze({ all, pluginIds: Object.freeze(all ? [] : [...rest]), profile });
}

/** ✍️ `components describe --profile wasm-prototype`: rebuilds each selected component under the iterate profile and writes its committed
 * semantic pair from exactly those bytes, one receipt per component, a summary at the end. Every component is attempted; the command fails after
 * the summary when any of them failed. Shipped bytes are described through the build driver instead.
 * @see ../../🏗️build/🧩️driver-stages/🟦️.ts */
export async function describeIterateComponentsV1(repoRoot: string, segments: readonly string[], signal?: AbortSignal): Promise<void> {
  const { all, pluginIds, profile } = componentsDescribeArgumentsV1(segments);
  const targets = componentStageTargetsV1(repoRoot);
  const unknown = pluginIds.filter((id) => !targets.some((target) => target.pluginId === id));
  if (unknown.length) throw new Error(`components describe: ${unknown.join(", ")} name no discovered component`);
  const selected = all ? targets : targets.filter((target) => pluginIds.includes(target.pluginId));
  const failures: string[] = [];
  let rewritten = 0;
  for (const [index, target] of selected.entries()) {
    signal?.throwIfAborted();
    const started = Date.now();
    try {
      const staged = await stageComponentV1(repoRoot, target, profile, "write", signal);
      if (staged.drift.length) rewritten++;
      console.log(`components describe ${index + 1}/${selected.length} ${target.pluginId}: ${staged.drift.length ? `rewrote the committed pair (${staged.drift.length} path(s))` : "committed pair unchanged"} from ${profile} bytes in ${Math.round((Date.now() - started) / 1000)} s`);
    } catch (error) {
      failures.push(`${target.pluginId}: ${(error as Error).message.split("\n")[0]}`);
      console.error(`components describe ${index + 1}/${selected.length} ${target.pluginId}: FAILED — ${(error as Error).message}`);
    }
  }
  console.log(`components describe: ${selected.length - failures.length}/${selected.length} described from ${profile} bytes, ${rewritten} committed pair(s) rewritten`);
  if (failures.length) throw new Error(`components describe failed for ${failures.length} component(s):\n  ${failures.join("\n  ")}`);
}

/** 🔁️ The rotating profile-independence sample: `count` components chosen by `seed`, deterministic for one catalog. */
export function profileIndependenceSampleV1(targets: readonly ComponentStageTargetV1[], count: number, seed: number): readonly ComponentStageTargetV1[] {
  const ordered = [...targets].sort((left, right) => left.pluginId.localeCompare(right.pluginId));
  const start = ((seed % ordered.length) + ordered.length) % Math.max(1, ordered.length);
  const stride = Math.max(1, Math.floor(ordered.length / Math.max(1, count)));
  return Object.freeze(Array.from({ length: Math.min(count, ordered.length) }, (_, index) => ordered[(start + index * stride) % ordered.length]!));
}

/** ⚖️ `components profile-independence [--sample <n>] [--seed <n>] | <plugin-id…>`: the semantic descriptor of each component
 * must be byte-equal under `wasm-release` and `wasm-prototype` — the law that makes an iterate-written descriptor valid for
 * release. Release bytes are verified against the committed pair; iterate bytes are described locally and compared. */
export async function profileIndependenceV1(repoRoot: string, segments: readonly string[], signal?: AbortSignal): Promise<void> {
  const option = (name: string, fallback: number): number => {
    const index = segments.indexOf(name);
    const value = index === -1 ? fallback : Number(segments[index + 1]);
    if (!Number.isSafeInteger(value) || value < 0) throw new Error(`components profile-independence: ${name} must be a non-negative integer`);
    return value;
  };
  const targets = componentStageTargetsV1(repoRoot);
  const named = segments.filter((value, index) => !value.startsWith("--") && !segments[index - 1]?.startsWith("--"));
  const selected = named.length ? targets.filter((target) => named.includes(target.pluginId)) : profileIndependenceSampleV1(targets, option("--sample", 8), option("--seed", Math.floor(Date.now() / 86_400_000)));
  if (named.length && selected.length !== named.length) throw new Error(`components profile-independence: ${named.filter((id) => !selected.some((target) => target.pluginId === id)).join(", ")} name no discovered component`);
  const failures: string[] = [];
  for (const target of selected) {
    signal?.throwIfAborted();
    const release = await stageComponentV1(repoRoot, target, SHIPPED_COMPONENT_PROFILE, "verify", signal).catch((error: Error) => error);
    const iterate = await stageComponentV1(repoRoot, target, "wasm-prototype", "local", signal).catch((error: Error) => error);
    if (release instanceof Error || iterate instanceof Error) failures.push(`${target.pluginId}: ${(release instanceof Error ? release : iterate as Error).message.split("\n")[0]}`);
    else if (release.receipt.semanticDescriptorSha256 !== iterate.receipt.semanticDescriptorSha256) failures.push(`${target.pluginId}: wasm-release describes ${release.receipt.semanticDescriptorSha256}, wasm-prototype ${iterate.receipt.semanticDescriptorSha256}:\n    ${iterate.drift.join("\n    ")}`);
    console.log(`profile-independence ${target.pluginId}: ${release instanceof Error || iterate instanceof Error ? "FAILED" : release.receipt.semanticDescriptorSha256 === iterate.receipt.semanticDescriptorSha256 ? `equal ${release.receipt.semanticDescriptorSha256.slice(0, 12)}… (release ${release.receipt.wasmBytes} B, iterate ${iterate.receipt.wasmBytes} B)` : "DIFFERENT"}`);
  }
  console.log(`profile-independence: ${selected.length - failures.length}/${selected.length} components describe equally under wasm-release and wasm-prototype`);
  if (failures.length) throw new Error(`profile-independence failed for ${failures.length} component(s):\n  ${failures.join("\n  ")}`);
}
//#endregion 🔖️ComponentStage
