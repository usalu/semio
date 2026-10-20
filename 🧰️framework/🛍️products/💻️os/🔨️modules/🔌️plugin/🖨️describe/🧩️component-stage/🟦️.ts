import {withDescriptorEmitterV1,withDescriptorEmitterTaskV1} from "../🛠️emitter/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {readArtifactBytesV1} from "../../🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";
import { scriptCompilerAdmissionControlV1, type CompilerAdmissionControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildModePolicy, readBuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { NATIVE_BUILD_DRIVER_POLICY_V1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { DESCRIPTOR_JSON_REL_PATH, findPluginCargoFiles, parseComponentSourceOwnerV1, type ComponentSourceOwnerV1 } from "../../📇️registry/🔎️discovery/🟦️.ts";
import { COMPONENT_PROFILE_POLICIES, COMPONENT_RECEIPT_SCHEMA_ID, COMPONENT_TARGET, SHIPPED_COMPONENT_PROFILE, componentDeliverableV1, componentProfileV1, componentProvenanceDigestsV1, rustToolchainIdentityV1, writeComponentReceiptV1, type ComponentDeliverableV1, type ComponentProfileV1, type ComponentReceiptV1 } from "../🏷️component-receipt/🟦️.ts";
import { decodeCargoProvenanceV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
import { fileSha256V1, STAGE_CACHE_POLICY_V1, stageCacheV1, type StageCacheV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts";
import { CATALOG_DESCRIPTOR_MAX_BYTES } from "../../📇️registry/✅️catalog-verification/🟦️.ts";
import { verifySemanticDescriptorPairBytesV1 } from "../../📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";
import { DESCRIPTOR_JSON_FILENAME, DESCRIPTOR_PACK_FILENAME } from "../🏗️component-build/🟦️.ts";
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
  profile = componentProfileV1(profile);
  const args = ["-p", packageName, "--lib", "--target", COMPONENT_TARGET, "--profile", profile];
  const environment = { CARGO_UNSTABLE_FEATURE_UNIFICATION: "true", CARGO_RESOLVER_FEATURE_UNIFICATION: NATIVE_BUILD_DRIVER_POLICY_V1.featureUnification };
  return process.env.SEMIO_PLUGIN_SYMBOLS === "1" ? { command: "rustc", args: [...args, "--", "-C", "strip=none"], environment } : { command: "build", args, environment };
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
export async function describeComponentStageV1(repoRoot: string, target: ComponentStageTargetV1, deliverable: ComponentDeliverableV1, cache: StageCacheV1, control: Omit<DescriptorEmissionControlV1,"emitter">): Promise<SemanticDescriptorEmissionV1> {
 return withDescriptorEmitterV1(repoRoot,control.invocation,control.compiler,emitter=>withDescriptorEmitterTaskV1(emitter,control.invocation,async actualTool=>{
  const component=await fileSha256V1(deliverable.wasm),tool=actualTool.sha256;
  const output = mkdtempSync(join(dirname(deliverable.directory), ".describe-"));let primaryFailed=false;
  try {
    await cache.run({ stage: "describe", schemaVersion: 1, inputs: { component }, tools: { emitter: tool } }, output, async (directory) => {
      const emission = await emitSemanticDescriptorV1(repoRoot, { component: deliverable.wasm, artifactRoot: dirname(deliverable.directory) }, {...control,emitter});
      writeFileSync(join(directory, DESCRIPTOR_PACK_FILENAME), emission.packBytes);
      writeFileSync(join(directory, DESCRIPTOR_JSON_FILENAME), emission.jsonBytes);
    }, control.compiler.signal);
    const packBytes = await readArtifactBytesV1(join(output, DESCRIPTOR_PACK_FILENAME),CATALOG_DESCRIPTOR_MAX_BYTES,control.compiler.signal), jsonBytes = await readArtifactBytesV1(join(output, DESCRIPTOR_JSON_FILENAME),CATALOG_DESCRIPTOR_MAX_BYTES,control.compiler.signal), pair = verifySemanticDescriptorPairBytesV1(jsonBytes, packBytes);
    if (pair.pluginId !== target.pluginId || pair.packageId !== target.packageId || pair.role !== target.role) throw new Error(`${target.pluginId}: the component describes ${pair.pluginId} (${pair.packageId}, ${pair.role})`);
    return Object.freeze({ pluginId: pair.pluginId, packageId: pair.packageId, role: pair.role, version: pair.version, componentSha256: component, jsonBytes, packBytes, semanticSha256: pair.semanticSha256 });
  } catch(error){primaryFailed=true;throw error;}
  finally {try{rmSync(output,{recursive:true,force:true});}catch(error){if(!primaryFailed)throw error;}}
 }));
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
  const command = `bun repo run workspace:components describe ${target.pluginId}`;
  if (mode === "verify" && !equal) throw new Error(`${target.pluginId}: the shipped component describes a different semantic descriptor than the committed pair at ${relative(repoRoot, target.ownerRoot)}:\n  ${(drift.length ? drift : ["$: byte encoding differs"]).join("\n  ")}\ncommit the shipped declaration with: ${command}`);
  if (mode === "write" && !equal) publishSemanticPair(target.ownerRoot, emission);
  if (mode === "local") {
    mkdirSync(deliverable.directory, { recursive: true });
    writeFileSync(join(deliverable.directory, DESCRIPTOR_PACK_FILENAME), emission.packBytes);
    writeFileSync(join(deliverable.directory, DESCRIPTOR_JSON_FILENAME), emission.jsonBytes);
    if (!equal) console.warn(`[local-iterate] ${target.pluginId}: the iterated declaration differs from the committed pair (uncommitted; commit with \`${command} --profile prototype\`):\n  ${drift.join("\n  ")}`);
  }
  return drift;
}

/** 🎛️ Parses `describe <plugin-id…>|--all [--profile release|prototype]` into a selection and a profile. */
export function componentsDescribeArgumentsV1(segments: readonly string[]): Readonly<{ all: boolean; pluginIds: readonly string[]; profile: ComponentProfileV1 }> {
  const flag = segments.indexOf("--profile");
  const profile = flag === -1 ? buildModePolicy(readBuildMode(process.env.SEMIO_BUILD_MODE)).cargoProfile : componentProfileV1(segments[flag + 1]);
  const rest = flag === -1 ? segments : [...segments.slice(0, flag), ...segments.slice(flag + 2)];
  const all = rest.length === 1 && rest[0] === "--all";
  if (!all && (rest.length === 0 || rest.some((value) => !/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(value)))) throw new Error("usage: components describe <plugin-id…>|--all [--profile release|prototype]");
  return Object.freeze({ all, pluginIds: Object.freeze(all ? [] : [...rest]), profile });
}

/** 🔁️ The rotating profile-independence sample: `count` components chosen by `seed`, deterministic for one catalog. */
export function profileIndependenceSampleV1(targets: readonly ComponentStageTargetV1[], count: number, seed: number): readonly ComponentStageTargetV1[] {
  const ordered = [...targets].sort((left, right) => left.pluginId.localeCompare(right.pluginId));
  const start = ((seed % ordered.length) + ordered.length) % Math.max(1, ordered.length);
  const stride = Math.max(1, Math.floor(ordered.length / Math.max(1, count)));
  return Object.freeze(Array.from({ length: Math.min(count, ordered.length) }, (_, index) => ordered[(start + index * stride) % ordered.length]!));
}

//#endregion 🔖️ComponentStage
