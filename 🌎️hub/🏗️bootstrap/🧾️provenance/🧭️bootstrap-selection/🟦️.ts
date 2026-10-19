/** 🧭️ Trusted bootstrap package selection: package specs, profile ids, declared and hosted kinds and the selection preflight.
 * @see ../../../📦️packages/🦀️rust/📜️script.ts
 */
import { checkResourceLeaseControl } from "@semio-tech/process/leases";
import { type CompilerAdmissionControl } from "@semio-tech/process/compiler-capacity";
import { requireIssuedResourceControlInvocationV1, scriptInvocationEnvironment, type ScriptInvocation } from "@semio-tech/process/routing";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
import { captureOwnedCommandResultV1 } from "@semio-tech/process/owned-execution";
import { DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES } from "@semio-tech/framework/directory/schema";
import { SHIPPED_COMPONENT_MAX_BYTES, SHIPPED_COMPONENT_PROFILE, componentDeliverableV1, componentReceiptFindingsV1, readComponentReceiptV1 } from "@semio-tech/framework-plugin/describe/component-receipt";
import { browserActorImportAdmissionV1 } from "@semio-tech/framework-plugin/browser-bundle/script";
import { readStableBuildFile } from "@semio-tech/repo-lib/build";
import { documentOpenNeutralObject, type TrustedBootstrapCodec, trustedBootstrapCodecOrder, trustedBootstrapDescriptorClaims, trustedBootstrapResolveClosure } from "../🧮️bootstrap-claims/🟦️.ts";

/** 🛫️ The selection-level rules the final staging and the hub's `validate_bundle` hold a publication to, judged from the package
 * rows and their COMMITTED descriptors alone, so a violation surfaces in seconds instead of after the release builds (ticket
 * 26/09/23 W4: a hosted-only stdio family and a 260-byte profile id each failed only after 15–44 min): the profile id is a
 * bounded `local-…-open-v1` name (`TRUSTED_IDENTITY_MAX_BYTES`); every package identity and declared kind id/schema is a bounded
 * trusted identity; a package without linked codecs declares an artifact kind of its own (its component answers the codec rows)
 * or HOSTS kinds (`manifest.hostedArtifactKinds`, p15) — the hub's own hosted-row rule (`trusted-catalog` descriptor decode): each
 * (id, schema) row once and never also owned, its explicit `owner` another package this one declares as a dependency and this catalog
 * selects BEFORE it (the publisher binds the row to the owner's already-published codec rows; a row the owner publishes no codec for —
 * stdio's definition-only kinds — opens nothing, as in the bootstrap); which kinds its apps OWN is the `describe`
 * gate's `first_owned_codec` law; a linked package's codec registry exists; no artifact kind is owned by editors of two packages (the hub's
 * `artifact_creation_selection` needs one unambiguous editor — c9's GIS probe got 409 when demonstrator re-owned `s.gis.gismap`). Law + fixture: `🌎️hub/🧪️tests/🛫️catalog-selection-preflight`. */
export function trustedBootstrapSelectionFindingsV1(profileId: string, packages: readonly Readonly<{ pluginId: string; componentPackageId: string; outputName: string; linkedCodecRegistry: string | null; linkedCodecRegistryPresent: boolean; descriptor: Record<string, any> }>[]): readonly string[] {
  const bounded = (value: unknown): boolean => typeof value === "string" && value.length > 0 && new TextEncoder().encode(value).length <= 256 && value.trim() === value;
  const findings: string[] = [];
  if (!bounded(profileId) || !/^local-(?:[a-z0-9]+-)+open-v1$/u.test(profileId)) findings.push(`profile id ${JSON.stringify(profileId.slice(0, 96))} is not a bounded local-…-open-v1 name (${new TextEncoder().encode(profileId).length} bytes, bound 256)`);
  const editorOwners = new Map<string, string[]>();
  for (const row of packages) {
    for (const [field, value] of [["pluginId", row.pluginId], ["componentPackageId", row.componentPackageId], ["outputName", row.outputName]] as const) if (!bounded(value)) findings.push(`${row.pluginId}: ${field} is not a bounded trusted identity`);
    const kinds = trustedBootstrapDescriptorKindsV1(row.descriptor);
    const editors = ((row.descriptor.manifest?.apps ?? []) as Record<string, any>[]).filter((app) => app.role === "editor");
    for (const kind of kinds) {
      if (!bounded(kind.id) || !bounded(kind.schema)) findings.push(`${row.pluginId}: artifact kind ${JSON.stringify(String(kind.id).slice(0, 96))} has an unbounded id or schema`);
      if (editors.some((app) => app.io?.artifactSchema === kind.schema || app.dialect?.artifactKind === kind.schema)) editorOwners.set(String(kind.id), [...(editorOwners.get(String(kind.id)) ?? []), row.pluginId]);
    }
    if (row.linkedCodecRegistry !== null) {
      if (!row.linkedCodecRegistryPresent) findings.push(`${row.pluginId}: linked codec registry ${row.linkedCodecRegistry} is missing`);
    }
    const manifest = (row.descriptor.manifest ?? {}) as Record<string, any>;
    const hosted = (manifest.hostedArtifactKinds ?? []) as Record<string, any>[];
    if (row.linkedCodecRegistry === null && kinds.length === 0 && hosted.length === 0) findings.push(`${row.pluginId}: declares no artifact kind, owned or hosted — a trusted-catalog package answers a codec row of its own or hosts kinds of a selected owner`);
    const dependencies = new Set(((manifest.dependencies ?? []) as Record<string, any>[]).map((dependency) => String(dependency.pluginId)));
    const owned = new Set(((manifest.artifactKinds ?? []) as Record<string, any>[]).map((kind) => String(kind.id)));
    const hostedPairs = new Set<string>();
    for (const kind of hosted) {
      const owner = typeof kind.owner === "string" && kind.owner !== "" ? kind.owner : undefined;
      if (!bounded(kind.id) || !bounded(kind.schema)) findings.push(`${row.pluginId}: hosted kind ${JSON.stringify(String(kind.id).slice(0, 96))} has an unbounded id or schema`);
      else if (hostedPairs.has(`${kind.id}\u0000${kind.schema}`) || owned.has(String(kind.id))) findings.push(`${row.pluginId}: hosts ${kind.id} (${kind.schema}) twice or also owns it`);
      else if (!owner || owner === row.pluginId || !dependencies.has(owner) || !packages.slice(0, packages.indexOf(row)).some((candidate) => candidate.pluginId === owner)) findings.push(`${row.pluginId}: hosts ${kind.id}, whose owner ${owner ?? "(none)"} is not another package it declares as a dependency and this catalog selects before it`);
      hostedPairs.add(`${kind.id}\u0000${kind.schema}`);
    }
  }
  for (const [kindId, owners] of editorOwners) if (owners.length > 1) findings.push(`artifact kind ${kindId} is owned by editors of ${owners.join(" and ")} — the hub creates a kind only through one unambiguous editor (host, don't own, another package's kind)`);
  return Object.freeze(findings);
}

/** 🛫️ [[trustedBootstrapSelectionFindingsV1]] over the selected packages' committed owner-root `🔣️.json` descriptors; throws every
 * finding at once. First step of `trusted-catalog-preflight` and `trusted-catalog-bootstrap`. */
export function trustedBootstrapPreflightSelectionV1(repoRoot: string, selection: readonly TrustedBootstrapPackageSpecV1[]): void {
  const registryPath = join(repoRoot, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json");
  if (!existsSync(registryPath)) throw new Error(`trusted catalog preflight needs the generated plugin registry ${relative(repoRoot, registryPath)}; run bun repo run @semio-tech/plugin-registry:generate`);
  const registry = JSON.parse(readFileSync(registryPath, "utf8")) as readonly { pluginId?: unknown; cratePath?: unknown }[];
  const packages = selection.map((spec) => {
    const row = registry.find((entry) => entry.pluginId === spec.pluginId);
    if (typeof row?.cratePath !== "string") throw new Error(`trusted catalog preflight: ${spec.pluginId} is not a row of the generated plugin registry`);
    const descriptor = JSON.parse(readFileSync(join(repoRoot, row.cratePath, "..", "..", "🔣️.json"), "utf8")) as Record<string, any>;
    return { ...spec, linkedCodecRegistryPresent: spec.linkedCodecRegistry !== null && existsSync(join(repoRoot, spec.linkedCodecRegistry)), descriptor };
  });
  const findings = trustedBootstrapSelectionFindingsV1(trustedBootstrapProfileIdV1(selection), packages);
  if (findings.length > 0) throw new Error(`trusted catalog preflight: selection refused before any build:\n${findings.map((finding) => `  - ${finding}`).join("\n")}`);
  console.log(`trusted-catalog-preflight: selection of ${selection.length} packages publishes as ${trustedBootstrapProfileIdV1(selection)} with bounded identities and a codec of its own per package`);
}

/** 🛫️ Fails a publication in seconds, before any build or codec probe: every selected package's COMMITTED owner-root
 * descriptor must pass the same claim rules its fresh build is held to later (canonical bounded Pack, exact `=x.y.z`
 * dependency pins, every dependency inside the selected closure at that package's own version). A stale or malformed
 * declaration therefore surfaces before an hour of fresh release builds, and `describe` is the fix it names. */
export function trustedBootstrapPreflightDescriptorsV1(repoRoot: string, selection: readonly TrustedBootstrapPackageSpecV1[]): void {
  const registryPath = join(repoRoot, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json");
  if (!existsSync(registryPath)) throw new Error(`trusted catalog preflight needs the generated plugin registry ${relative(repoRoot, registryPath)}; run bun repo run @semio-tech/plugin-registry:generate`);
  const registry = JSON.parse(readFileSync(registryPath, "utf8")) as readonly { pluginId?: unknown; cratePath?: unknown }[];
  const claims = selection.map((spec) => {
    const row = registry.find((entry) => entry.pluginId === spec.pluginId);
    if (typeof row?.cratePath !== "string") throw new Error(`trusted catalog preflight: ${spec.pluginId} is not a row of the generated plugin registry`);
    const bytes = trustedBootstrapReadRegular(join(repoRoot, row.cratePath, "..", "..", "🛂️.descriptor.semio"), DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, `committed ${spec.pluginId} descriptor`, () => {});
    try {
      return { spec, claims: trustedBootstrapDescriptorClaims(bytes) };
    } catch (error) {
      throw new Error(`trusted catalog preflight: committed ${spec.pluginId} descriptor refused (${error instanceof Error ? error.message : String(error)}); fix the declaration, then bun repo run @semio-tech/${spec.pluginId}-plugin:describe`);
    } finally {
      bytes.fill(0);
    }
  });
  const closure = claims.map((row) => ({ pluginId: row.spec.pluginId, packageId: row.spec.componentPackageId, version: row.claims.identity.version }));
  try {
    trustedBootstrapResolveClosure(trustedBootstrapProfileIdV1(selection), new Map(claims.map((row) => [row.spec.pluginId, row.claims])), closure);
  } catch (error) {
    throw new Error(`trusted catalog preflight: dependencies refused (${error instanceof Error ? error.message : String(error)})`);
  }
  console.log(`trusted-catalog-preflight: ${claims.length} committed descriptors carry exact, bounded dependencies inside the selected closure`);
}

/** 🛫️ Fails a publication in milliseconds, before the hub build and every release build: each selected package and each extension
 * of one has a shipped `release` deliverable whose `ComponentReceiptV1` names exactly those bytes and exactly the committed
 * semantic descriptor (ticket 26/09/23 wave A2), and that component imports only interfaces a closed browser actor admits — the rule
 * the actor build applies to the release component after its build (`browserActorImportAdmissionV1`). Every finding of every
 * component is reported at once, with the fix it needs. */
export function trustedBootstrapPreflightComponentsV1(repoRoot: string, selection: readonly TrustedBootstrapPackageSpecV1[]): void {
  const registryPath = join(repoRoot, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json");
  if (!existsSync(registryPath)) throw new Error(`trusted catalog preflight needs the generated plugin registry ${relative(repoRoot, registryPath)}; run bun repo run @semio-tech/plugin-registry:generate`);
  const registry = JSON.parse(readFileSync(registryPath, "utf8")) as readonly { pluginId?: unknown; packageId?: unknown; packageName?: unknown; cratePath?: unknown; role?: unknown; extends?: unknown }[];
  const selected = new Set(selection.map((spec) => spec.pluginId));
  const rows = registry.filter((row) => (row.role === "plugin" && selected.has(String(row.pluginId))) || (row.role === "extension" && selected.has(String(row.extends))));
  const missing = [...selected].filter((pluginId) => !rows.some((row) => row.pluginId === pluginId));
  if (missing.length > 0) throw new Error(`trusted catalog preflight: ${missing.join(", ")} not rows of the generated plugin registry`);
  const findings: string[] = [];
  for (const row of rows) {
    if (typeof row.cratePath !== "string" || typeof row.packageName !== "string" || typeof row.packageId !== "string") { findings.push(`${String(row.pluginId)}: registry row carries no crate path, package name or package id`); continue; }
    const pluginId = String(row.pluginId), fix = `run bun repo run workspace:release, then the crate's describe target`;
    const deliverable = componentDeliverableV1(join(repoRoot, row.cratePath), row.packageName, SHIPPED_COMPONENT_PROFILE);
    if (!existsSync(deliverable.wasm) || !existsSync(deliverable.receipt)) { findings.push(`${pluginId}: no shipped component with its receipt under ${relative(repoRoot, deliverable.directory)}; ${fix}`); continue; }
    const descriptor = trustedBootstrapReadRegular(join(repoRoot, row.cratePath, "..", "..", "🛂️.descriptor.semio"), DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, `committed ${pluginId} descriptor`, () => {});
    const component = trustedBootstrapReadRegular(deliverable.wasm, SHIPPED_COMPONENT_MAX_BYTES, `${pluginId} shipped component`, () => {});
    try {
      for (const finding of componentReceiptFindingsV1(readComponentReceiptV1(deliverable.receipt), { pluginId, packageId: row.packageId, profile: SHIPPED_COMPONENT_PROFILE }, component, descriptor)) findings.push(`${pluginId}: ${finding}; ${fix}`);
      const imports = browserActorImportAdmissionV1(component);
      if (imports.refused.length > 0) findings.push(`${pluginId}: imports ${imports.refused.join(", ")}, which no closed browser actor admits; the guest must not link that capability`);
    } catch (error) {
      findings.push(`${pluginId}: ${error instanceof Error ? error.message : String(error)}`);
    } finally {
      component.fill(0);
      descriptor.fill(0);
    }
  }
  if (findings.length > 0) throw new Error(`trusted catalog preflight refused ${findings.length} of ${rows.length} components:\n  ${findings.join("\n  ")}`);
  console.log(`trusted-catalog-preflight: ${rows.length} shipped components (${selected.size} packages + ${rows.length - selected.size} extensions) carry receipts naming their bytes and their committed semantic descriptors, and import only browser-actor-admitted interfaces`);
}

/** 📦️ One selectable package of the trusted bootstrap closure. Adding a fourth plugin to a
 * generation is one row here: everything else — its document kinds, their schemas, their pack
 * fingerprints, its creatable surfaces — comes out of the package's own compiled descriptor and its
 * own built component. `linkedCodecRegistry` is not a schema transcription: it names the file the
 * hub binary's COMPILED native-codec provider is generated from, so it is present for exactly the
 * packages `NativeCodecProviderSetV1` links Rust codecs for and null for every other one, whose rows
 * the component answers for itself through `world actor`'s `codec` interface. */
export type TrustedBootstrapPackageSpecV1 = Readonly<{
  pluginId: string;
  cargoPackage: string;
  componentPackageId: string;
  outputName: string;
  linkedCodecRegistry: string | null;
  opensDocuments: boolean;
}>;

/** 🎯️ What `trusted-catalog-bootstrap` publishes when no `--packages` list is given. */
export const TRUSTED_BOOTSTRAP_DEFAULT_PACKAGES = "stdio,gis,note";

/** 🌎️ Every selectable `s` plugin package — the top-level plugins and the stdio family components right after the `stdio`
 * package they depend on — in publication order: a host after every owner it hosts kinds of (the publisher binds a hosted row to
 * the owner's already-published codec rows), so `demonstrator` comes last. */
const TRUSTED_BOOTSTRAP_ALL_PACKAGES = "stdio,stdio-image,stdio-media,stdio-cad,stdio-bim,stdio-mesh,stdio-pdf,stdio-office,stdio-semio,stdio-binary,gis,animate,architect,bim,block,cad,dag,draw,energy,fem,flow,forms,imperative,layout,lowpoly,mathematical,norm,note,playbook,procedural,process,puzzle,raster,reasoning,remodel,sequence,shooting,sourcing,space,trinity,vcs,wfc,writer,demonstrator";

/** 🔗️ The closure every gate, rotation and process law in this file proves: exactly the packages
 * this hub binary links Rust codecs for. They keep minting `local-stdio-gis-open-v1`, so the hub's
 * own exactness fence on that profile id keeps guarding them. */
export const TRUSTED_BOOTSTRAP_LINKED_PACKAGES = "stdio,gis";

const TRUSTED_BOOTSTRAP_PACKAGES: readonly TrustedBootstrapPackageSpecV1[] = Object.freeze([
  Object.freeze({ pluginId: "stdio", cargoPackage: "semio-hub-stdio-component", componentPackageId: "semio:stdio", outputName: "semio_hub_stdio_component.wasm", linkedCodecRegistry: "🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📜️native-codec-factories.json", opensDocuments: true }),
  Object.freeze({ pluginId: "stdio-image", cargoPackage: "semio-hub-stdio-image-component", componentPackageId: "semio:stdio-image", outputName: "semio_hub_stdio_image_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "stdio-media", cargoPackage: "semio-hub-stdio-media-component", componentPackageId: "semio:stdio-media", outputName: "semio_hub_stdio_media_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "stdio-cad", cargoPackage: "semio-hub-stdio-cad-component", componentPackageId: "semio:stdio-cad", outputName: "semio_hub_stdio_cad_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "stdio-bim", cargoPackage: "semio-hub-stdio-bim-component", componentPackageId: "semio:stdio-bim", outputName: "semio_hub_stdio_bim_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "stdio-mesh", cargoPackage: "semio-hub-stdio-mesh-component", componentPackageId: "semio:stdio-mesh", outputName: "semio_hub_stdio_mesh_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "stdio-pdf", cargoPackage: "semio-hub-stdio-pdf-component", componentPackageId: "semio:stdio-pdf", outputName: "semio_hub_stdio_pdf_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "stdio-office", cargoPackage: "semio-hub-stdio-office-component", componentPackageId: "semio:stdio-office", outputName: "semio_hub_stdio_office_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "stdio-semio", cargoPackage: "semio-hub-stdio-semio-component", componentPackageId: "semio:stdio-semio", outputName: "semio_hub_stdio_semio_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "stdio-binary", cargoPackage: "semio-hub-stdio-binary-component", componentPackageId: "semio:stdio-binary", outputName: "semio_hub_stdio_binary_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "gis", cargoPackage: "semio-hub-gis-component", componentPackageId: "semio:gis", outputName: "semio_hub_gis_component.wasm", linkedCodecRegistry: "🌎️hub/🧩️compositions/🌍️gis/📇️native-codecs/🔣️.json", opensDocuments: true }),
  Object.freeze({ pluginId: "animate", cargoPackage: "semio-hub-animate-component", componentPackageId: "semio:animate", outputName: "semio_hub_animate_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "architect", cargoPackage: "semio-hub-architect-component", componentPackageId: "semio:architect", outputName: "semio_hub_architect_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "bim", cargoPackage: "semio-hub-bim-component", componentPackageId: "semio:bim", outputName: "semio_hub_bim_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "block", cargoPackage: "semio-hub-block-component", componentPackageId: "semio:block", outputName: "semio_hub_block_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "cad", cargoPackage: "semio-hub-cad-component", componentPackageId: "semio:cad", outputName: "semio_hub_cad_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "dag", cargoPackage: "semio-hub-dag-component", componentPackageId: "semio:dag", outputName: "semio_hub_dag_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "demonstrator", cargoPackage: "semio-hub-demonstrator-component", componentPackageId: "semio:demonstrator", outputName: "semio_hub_demonstrator_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "draw", cargoPackage: "semio-hub-draw-component", componentPackageId: "semio:draw", outputName: "semio_hub_draw_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "energy", cargoPackage: "semio-hub-energy-component", componentPackageId: "semio:energy", outputName: "semio_hub_energy_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "fem", cargoPackage: "semio-hub-fem-component", componentPackageId: "semio:fem", outputName: "semio_hub_fem_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "flow", cargoPackage: "semio-hub-flow-component", componentPackageId: "semio:flow", outputName: "semio_hub_flow_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "forms", cargoPackage: "semio-hub-forms-component", componentPackageId: "semio:forms", outputName: "semio_hub_forms_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "imperative", cargoPackage: "semio-hub-imperative-component", componentPackageId: "semio:imperative", outputName: "semio_hub_imperative_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "layout", cargoPackage: "semio-hub-layout-component", componentPackageId: "semio:layout", outputName: "semio_hub_layout_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "lowpoly", cargoPackage: "semio-hub-lowpoly-component", componentPackageId: "semio:lowpoly", outputName: "semio_hub_lowpoly_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "mathematical", cargoPackage: "semio-hub-mathematical-component", componentPackageId: "semio:mathematical", outputName: "semio_hub_mathematical_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "norm", cargoPackage: "semio-hub-norm-component", componentPackageId: "semio:norm", outputName: "semio_hub_norm_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "note", cargoPackage: "semio-hub-note-component", componentPackageId: "semio:note", outputName: "semio_hub_note_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "playbook", cargoPackage: "semio-hub-playbook-component", componentPackageId: "semio:playbook", outputName: "semio_hub_playbook_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "procedural", cargoPackage: "semio-hub-procedural-component", componentPackageId: "semio:procedural", outputName: "semio_hub_procedural_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "process", cargoPackage: "semio-hub-process-component", componentPackageId: "semio:process", outputName: "semio_hub_process_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "puzzle", cargoPackage: "semio-hub-puzzle-component", componentPackageId: "semio:puzzle", outputName: "semio_hub_puzzle_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "raster", cargoPackage: "semio-hub-raster-component", componentPackageId: "semio:raster", outputName: "semio_hub_raster_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "reasoning", cargoPackage: "semio-hub-reasoning-component", componentPackageId: "semio:reasoning", outputName: "semio_hub_reasoning_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "remodel", cargoPackage: "semio-hub-remodel-component", componentPackageId: "semio:remodel", outputName: "semio_hub_remodel_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "sequence", cargoPackage: "semio-hub-sequence-component", componentPackageId: "semio:sequence", outputName: "semio_hub_sequence_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "shooting", cargoPackage: "semio-hub-shooting-component", componentPackageId: "semio:shooting", outputName: "semio_hub_shooting_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "sourcing", cargoPackage: "semio-hub-sourcing-component", componentPackageId: "semio:sourcing", outputName: "semio_hub_sourcing_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "space", cargoPackage: "semio-hub-space-component", componentPackageId: "semio:space", outputName: "semio_hub_space_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "trinity", cargoPackage: "semio-hub-trinity-component", componentPackageId: "semio:trinity", outputName: "semio_hub_trinity_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "vcs", cargoPackage: "semio-hub-vcs-component", componentPackageId: "semio:vcs", outputName: "semio_hub_vcs_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "wfc", cargoPackage: "semio-hub-wfc-component", componentPackageId: "semio:wfc", outputName: "semio_hub_wfc_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
  Object.freeze({ pluginId: "writer", cargoPackage: "semio-hub-writer-component", componentPackageId: "semio:writer", outputName: "semio_hub_writer_component.wasm", linkedCodecRegistry: null, opensDocuments: true }),
]);

/** 🧾️ Resolves a comma-separated `--packages` list against the selectable closure, in list order. */
/** 📏️ The hub's bound on every trusted identity it holds, a profile id included (`TRUSTED_IDENTITY_MAX_BYTES`,
 * `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`). */
const TRUSTED_PROFILE_ID_MAX_BYTES = 256;

/** 🏷️ The profile name one selection publishes under — a name only: the bundle's profile carries the exact selected closure and its
 * digest. `local-all-open-v1` for the whole selectable closure ([[TRUSTED_BOOTSTRAP_ALL_PACKAGES]]); otherwise the REQUESTED order,
 * `local-<plugin>-…-open-v1` (`--packages stdio,gis` keeps minting `local-stdio-gis-open-v1`), while that fits the hub's identity
 * bound, and `local-<count>-packages-<16 hex of the requested list's sha256>-open-v1` beyond it (ticket 26/09/23 W4: a 34-package
 * list minted a 260-byte name the pointer reader refused after the whole publication). */
export function trustedBootstrapProfileIdV1(selection: readonly TrustedBootstrapPackageSpecV1[]): string {
  const all = TRUSTED_BOOTSTRAP_ALL_PACKAGES.split(",");
  const ids = selection.map((spec) => spec.pluginId);
  if (ids.length === all.length && all.every((pluginId) => ids.includes(pluginId))) return "local-all-open-v1";
  const listed = `local-${ids.join("-")}-open-v1`;
  return Buffer.byteLength(listed, "utf8") <= TRUSTED_PROFILE_ID_MAX_BYTES ? listed : `local-${ids.length}-packages-${createHash("sha256").update(ids.join(",")).digest("hex").slice(0, 16)}-open-v1`;
}

export function trustedBootstrapSelectPackages(list: string): readonly TrustedBootstrapPackageSpecV1[] {
  const requested = (list.trim() === "all" ? TRUSTED_BOOTSTRAP_ALL_PACKAGES : list)
    .split(",")
    .map((part) => part.trim())
    .filter((part) => part.length > 0);
  if (requested.length === 0 || new Set(requested).size !== requested.length) throw new Error("trusted catalog package list is empty or repeats a package");
  return Object.freeze(
    requested.map((pluginId) => {
      const spec = TRUSTED_BOOTSTRAP_PACKAGES.find((candidate) => candidate.pluginId === pluginId);
      if (!spec) throw new Error(`trusted catalog package ${pluginId} is outside the selectable closure (${TRUSTED_BOOTSTRAP_PACKAGES.map((candidate) => candidate.pluginId).join(",")})`);
      return spec;
    }),
  );
}

/** 🧬️ Asks ONE built component for the pack fingerprint of each document kind its OWN compiled
 * descriptor declares, through the `codecs` subcommand of the same descriptor emitter the build
 * already ran. `pairs` are that descriptor's `ArtifactKindSpec { id, schema }` rows: the kind and
 * the schema are the package's own published identities, and only the 32-byte fingerprint — the one
 * thing a manifest cannot state — comes from the component, on the exact bytes about to be
 * published. So a package the hub links no Rust codec for transcribes nothing. */
export async function trustedBootstrapComponentCodecRowsV1(invocation:ScriptInvocation,control:CompilerAdmissionControl,repoRoot:string,emitter:string,componentPath:string,outPath:string,pairs:readonly(readonly[string,string])[]):Promise<Readonly<{rows:readonly TrustedBootstrapCodec[];unowned:ReadonlySet<string>}>>{
  requireIssuedResourceControlInvocationV1(control,invocation);
  if (pairs.length === 0) throw new Error("component codec probe requires at least one declared artifact kind");
  const probe=await captureOwnedCommandResultV1(emitter,["codecs",componentPath,"--kinds",pairs.map(([kind,schema])=>`${kind}=${schema}`).join(","),"--out",outPath],repoRoot,{control,env:scriptInvocationEnvironment(invocation,process.env),output:{maximumBytes:4194304,maximumLines:65536}});
  if(probe.status!==0||probe.signal!==null)throw Error(`component codec probe failed (${probe.status??probe.signal}): ${probe.stderr.trim()||probe.stdout.trim()}`);
  const bytes=trustedBootstrapReadRegular(outPath,4194304,"actual component codec rows",()=>{checkResourceLeaseControl(control);});
  let document;try{document=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes));}finally{bytes.fill(0);}
  if (document?.schema !== "semio.plugin.component-codec-rows/v1" || !Array.isArray(document.rows) || !Array.isArray(document.unowned) || document.rows.length + document.unowned.length !== pairs.length) throw new Error("component codec probe answered outside its exact declared closure");
  const unowned = new Set<string>(document.unowned.map((value: unknown) => {
    const row = documentOpenNeutralObject(value, ["artifactKind", "artifactSchema"]);
    if (!pairs.some(([kind, schema]) => kind === row.artifactKind && schema === row.artifactSchema)) throw new Error("component codec probe reported an unowned kind the descriptor does not declare");
    return String(row.artifactKind);
  }));
  const rows = document.rows.map((value: unknown) => {
    const row = documentOpenNeutralObject(value, ["artifactKind", "artifactSchema", "packSchemaHash"]);
    if (!pairs.some(([kind, schema]) => kind === row.artifactKind && schema === row.artifactSchema) || !/^(?!0{64}$)[0-9a-f]{64}$/u.test(String(row.packSchemaHash)))
      throw new Error("component codec row is not a bounded nonzero identity the descriptor declares");
    return Object.freeze({ artifactKind: row.artifactKind as string, artifactSchema: row.artifactSchema as string, packSchemaHash: row.packSchemaHash as string });
  });
  if (new Set(rows.map((row: TrustedBootstrapCodec) => row.artifactKind)).size !== rows.length || rows.some((row: TrustedBootstrapCodec) => unowned.has(row.artifactKind))) throw new Error("component codec probe repeated an artifact kind");
  return Object.freeze({ rows: Object.freeze(rows.sort(trustedBootstrapCodecOrder)), unowned });
}

/** ⛓️ The declared kinds a LINKED package does not own. The hub executes a linked package's documents only through
 * the native codecs it links (`linkedCodecRegistry`), and every document-open target must bind one of them
 * (`validate_bundle`: "trusted document-open target is bound to no native codec of its own package"), so a kind the
 * descriptor declares without a linked row (an editor declaring a kind its package links no codec for) is unowned:
 * declared, never a hub open target; Stdio and GIS link a codec for every kind they declare. A declared kind whose linked row carries another
 * schema is an identity conflict, not an unowned kind. Law + fixture: `🌎️hub/🧪️tests/⛓️linked-codec-ownership`. */
export function trustedBootstrapLinkedUnownedKindsV1(pairs: readonly (readonly [string, string])[], linked: readonly TrustedBootstrapCodec[]): ReadonlySet<string> {
  const unowned = new Set<string>();
  for (const [kind, schema] of pairs) {
    const row = linked.find((codec) => codec.artifactKind === kind);
    if (!row) unowned.add(kind);
    else if (row.artifactSchema !== schema) throw new Error(`linked codec ${kind} carries schema ${row.artifactSchema}, its descriptor declares ${schema}`);
  }
  return Object.freeze(unowned);
}

/** 🗂️ Every `ArtifactKindSpec` one verified descriptor declares, deduplicated by kind id and in
 * declaration order. This is `validate_descriptor_open_target`'s own discoverability union: a spec
 * declared at PLUGIN level (`PluginBuilder::artifact_kind`, the channel GIS and Stdio still use) or
 * on the OWNING APP, which is where a package migrated onto the declaration tree stitches it. */
/** 🏠️ The kinds one package HOSTS (`manifest.hostedArtifactKinds`: `PluginBuilder::host_artifact` and embedded surfaces of
 * another plugin's artifact): each row's explicit `owner` must be a declared dependency already published in this catalog —
 * never read off the kind id (an embedded editor's `3d.cad` names no plugin); a hosted row binds the
 * owner's verified codec row (codec rows never move), and a row the owner publishes no codec for opens nothing — the owner's
 * own rule for its unlinked kinds (the owner publishes ONE codec row per kind, so a kind's other document schemas stay
 * unbound). */
export function trustedBootstrapHostedKindsV1(pluginId: string, descriptor: Record<string, any>, codecs: Readonly<Record<string, readonly TrustedBootstrapCodec[]>>): Readonly<{ pairs: readonly (readonly [string, string])[]; rows: readonly TrustedBootstrapCodec[]; unowned: ReadonlySet<string>; unbound: ReadonlySet<string> }> {
  const manifest = descriptor.manifest as Record<string, any>;
  const dependencies = new Set(((manifest.dependencies ?? []) as Record<string, any>[]).map((dependency) => String(dependency.pluginId)));
  const hosted = (manifest.hostedArtifactKinds ?? []) as Record<string, any>[];
  const pairs = hosted.map((kind) => [String(kind.id), String(kind.schema)] as const);
  const rows: TrustedBootstrapCodec[] = [];
  const unowned = new Set<string>();
  const unbound = new Set<string>();
  for (const kind of hosted) {
    const [id, schema] = [String(kind.id), String(kind.schema)];
    const owner = typeof kind.owner === "string" ? kind.owner : "";
    if (!owner || owner === pluginId || !dependencies.has(owner)) throw new Error(`trusted catalog package ${pluginId} hosts ${id}, whose owner ${JSON.stringify(owner)} is not one of its declared dependencies`);
    const published = codecs[owner];
    if (!published) throw new Error(`trusted catalog package ${pluginId} hosts ${id}, whose owner ${owner} is not published before it in this catalog`);
    const row = published.find((codec) => codec.artifactKind === id && codec.artifactSchema === schema);
    if (row) rows.push(row);
    else {
      unowned.add(id);
      unbound.add(`${id} ${schema}`);
    }
  }
  for (const row of rows) unowned.delete(row.artifactKind);
  return Object.freeze({ pairs: Object.freeze(pairs), rows: Object.freeze(rows), unowned, unbound });
}

export function trustedBootstrapDescriptorKindsV1(descriptor: Record<string, any>): readonly Record<string, any>[] {
  const manifest = descriptor.manifest as Record<string, any>;
  const apps = (manifest.apps ?? []) as Record<string, any>[];
  const declared = [...((manifest.artifactKinds ?? []) as Record<string, any>[]), ...apps.flatMap((app) => (app.artifactKinds ?? []) as Record<string, any>[])];
  const seen = new Set<string>();
  return Object.freeze(declared.filter((kind) => !seen.has(String(kind.id)) && seen.add(String(kind.id))));
}

export function trustedBootstrapReadRegular(path: string, maximum: number, label: string, check: () => void): Uint8Array {
  const bytes = readStableBuildFile(path, maximum, { remaining: maximum }, check);
  if (bytes.byteLength === 0) throw new Error(`${label} is empty`);
  return bytes;
}
