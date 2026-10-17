/** 🗺️ Trusted bootstrap GIS proofs and staging: the cold map component proof, the closed browser actor proof, the isolated candidate current staging and the rotation source fixture proof.
 * @see ../../../../📦️packages/🦀️rust/📜️script.ts
 */
import { checkResourceLeaseControl } from "@semio-tech/process/leases";
import { type CompilerAdmissionControl } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { requireIssuedResourceControlInvocationV1, type ScriptInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { buildBudgetMs } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { createHash, randomBytes } from "node:crypto";
import { chmodSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, renameSync, rmSync } from "node:fs";
import { dirname, join } from "node:path";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES } from "@semio-tech/framework/directory/schema";
import { DOCUMENT_BROWSER_ACTOR_MAX_BYTES } from "@semio-tech/framework/directory/schema/browser-actor";
import { TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES, TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🧬️schema/🟦️.ts";
import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { trustedBootstrapBuildControl, type TrustedBootstrapBundleV1, trustedBootstrapFsyncDirectory, trustedBootstrapGenerationReceipt, type TrustedBootstrapGenerationReceiptV1, type TrustedBootstrapMaterializationV1, trustedBootstrapVerifyGeneration, trustedBootstrapWriteNew } from "../../🗃️bootstrap-bundle/🟦️.ts";
import { trustedBootstrapPluginModuleBlobPath } from "../../🧮️bootstrap-claims/🟦️.ts";
import { trustedBootstrapReadRegular } from "../../🧭️bootstrap-selection/🟦️.ts";
import { proveBrowserActorGisChildV1 } from "../../🌍️gis-browser-proof/🟦️.ts";
import { webSha256Hex } from "../../../../🚀️local-relay/🛰️servers/🟦️.ts";
import { trustedBootstrapReadCurrentBundle } from "../🧱️materialized-bundle/🟦️.ts";
import { stageTrustedBootstrapProbePointer, trustedBootstrapCurrent, trustedBootstrapVerifyPublication } from "../🔄️rotation-and-current-pointer/🟦️.ts";

/** 🌉️ Executes the cold-map laws against the exact verified GIS bytes retained by one generation. */
export async function proveTrustedGisColdMapComponentV1(invocation: ScriptInvocation, phaseControl:CompilerAdmissionControl, repoRoot: string, current: TrustedBootstrapMaterializationV1): Promise<void> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);
  const check = () => {
    if (phaseControl.signal.aborted || (checkResourceLeaseControl(phaseControl)??Infinity) <= 0) throw new Error("trusted GIS cold-map law cancelled");
  };
  {
    const bundle = trustedBootstrapReadCurrentBundle(current, check);
    const packages = new Map<string, Record<string, any>>((bundle.packages ?? []).map((record: Record<string, any>) => [String(record.pluginId), record]));
    const gis = packages.get("gis");
    if (
      packages.size === 0 ||
      !gis ||
      gis.packageId !== "semio:gis" ||
      gis.component?.path !== "packages/gis/component.wasm" ||
      gis.descriptor?.path !== "packages/gis/descriptor.semio" ||
      !/^[a-f0-9]{64}$/u.test(gis.component.sha256 ?? "") ||
      !/^[a-f0-9]{64}$/u.test(gis.descriptor.sha256 ?? "")
    ) {
      throw new Error("trusted GIS cold-map law requires the exact verified bundle owner");
    }
    const generationRoot = dirname(current.bundlePath);
    const componentPath = join(generationRoot, "packages/gis/component.wasm");
    const descriptorPath = join(generationRoot, "packages/gis/descriptor.semio");
    const verifyRetainedBytes = (): void => {
      trustedBootstrapVerifyPublication(dirname(dirname(dirname(generationRoot))), current);
      const component = trustedBootstrapReadRegular(componentPath, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, "trusted GIS cold-map component", check);
      const descriptor = trustedBootstrapReadRegular(descriptorPath, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, "trusted GIS cold-map descriptor", check);
      try {
        if (
          component.byteLength !== gis.component.byteLength ||
          descriptor.byteLength !== gis.descriptor.byteLength ||
          createHash("sha256").update(component).digest("hex") !== gis.component.sha256 ||
          createHash("sha256").update(descriptor).digest("hex") !== gis.descriptor.sha256
        ) {
          throw new Error("trusted GIS cold-map bytes differ from the retained generation receipt");
        }
      } finally {
        component.fill(0);
        descriptor.fill(0);
      }
    };
    verifyRetainedBytes();
    const receipts = await runRepositoryExactCargoLaws({control:phaseControl,preparation: configuredRepositoryCargoPreparationPolicyV1(),cwd: repoRoot,groups: [
        {
          package: "semio-hub-gis",
          target: { kind: "test", name: "component_cold_map_patch" },
          laws: ["genuine_gis_component_cold_loads_and_patches_the_exact_tiled_map_surface", "genuine_gis_component_rejects_stale_cold_authority_before_loading"],
        },
      ],env: {
        ...process.env,
        RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432",
        SEMIO_GIS_COMPONENT_WASM: componentPath,
        SEMIO_GIS_COMPONENT_SHA256: gis.component.sha256,
        SEMIO_GIS_DESCRIPTOR_SHA256: gis.descriptor.sha256,
      },nativeEnv: { RUST_MIN_STACK: process.env.SEMIO_TEST_NATIVE_RUST_MIN_STACK ?? "268435456" },invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 120_000, lawMilliseconds: 300_000 },progress(event) {
        console.log(`trusted-gis-cold-map ${event.stage}: ${event.law ?? event.package} artifacts=${event.artifactDir}`);
      }});
    verifyRetainedBytes();
    for (const receipt of receipts) console.log(`trusted-gis-cold-map-receipt: ${JSON.stringify(receipt)}`);
    console.log(`trusted-gis-cold-map-component: generation=${current.generationId} component=${gis.component.sha256} descriptor=${gis.descriptor.sha256} exact=2; candidate publication and browser remain separate`);
  }
}

/** 🪢️ Executes the closed GIS actor of the exact published generation without rebuilding its parents. */
export async function proveTrustedGisClosedActorV1(repoRoot: string, dataRoot: string, current: TrustedBootstrapMaterializationV1, artifactRoot: string): Promise<void> {
  const work = mkdtempSync(join(artifactRoot, "retained-gis-browser-"));
  const owner = trustedBootstrapBuildControl(Math.min(buildBudgetMs() || 300_000, 300_000));
  const check = (): void => {
    if (owner.control.cancelled() || owner.control.remainingMs() <= 0) throw new Error("retained GIS browser proof cancelled");
  };
  let bundleBytes: Uint8Array | undefined, descriptor: Uint8Array | undefined, actorBytes: Uint8Array | undefined;
  try {
    const initialCurrent = trustedBootstrapCurrent(dataRoot);
    if (!initialCurrent || initialCurrent.generationId !== current.generationId || initialCurrent.bundleSha256 !== current.bundleSha256) throw new Error("retained GIS browser has no exact published selection");
    const bundle = trustedBootstrapReadCurrentBundle(current, check);
    bundleBytes = trustedBootstrapReadRegular(current.bundlePath, 4 * 1024 * 1024, "retained GIS browser bundle", check);
    if (createHash("sha256").update(bundleBytes).digest("hex") !== current.bundleSha256) throw new Error("retained GIS browser bundle changed");
    const packages = new Map<string, Record<string, any>>(bundle.packages.map((record: Record<string, any>) => [String(record.pluginId), record]));
    const gis = packages.get("gis");
    if (
      packages.size === 0 || !packages.has("stdio") || !gis || gis.packageId !== "semio:gis" ||
      gis.component?.path !== "packages/gis/component.wasm" || gis.descriptor?.path !== "packages/gis/descriptor.semio" ||
      gis.browserActor?.kind !== "closed-browser-actor" || gis.browserActor.path !== "packages/gis/browser/closed-actor.mjs" ||
      gis.browserActor.sourceComponentSha256 !== gis.component.sha256 ||
      gis.browserActor.sourceDescriptorByteSha256 !== gis.descriptor.sha256
    ) throw new Error("retained GIS browser parent identity differs");
    const receipts = new Map<string, TrustedBootstrapGenerationReceiptV1>();
    for (const plugin of ["gis", "stdio"] as const) {
      const record = packages.get(plugin)!;
      if (record.packageId !== `semio:${plugin}` || record.component?.path !== `packages/${plugin}/component.wasm` || record.descriptor?.path !== `packages/${plugin}/descriptor.semio`)
        throw new Error("retained GIS browser generation closure differs");
      receipts.set(plugin, trustedBootstrapGenerationReceipt(record));
    }
    const generationRoot = dirname(current.bundlePath);
    const verifyRetained = (): void => {
      check();
      for (const root of [dataRoot, join(dataRoot, "trusted-catalog"), join(dataRoot, "trusted-catalog", "generations")]) {
        const info = lstatSync(root);
        if (info.isSymbolicLink() || !info.isDirectory()) throw new Error("retained GIS browser current traverses a non-directory");
      }
      const published = trustedBootstrapCurrent(dataRoot);
      if (!published || published.currentSha256 !== initialCurrent.currentSha256 || published.publicationRevision !== initialCurrent.publicationRevision || published.profileId !== current.profileId || published.generationId !== current.generationId || published.bundleSha256 !== current.bundleSha256 || published.bundlePath !== current.bundlePath)
        throw new Error("retained GIS browser current generation changed");
      trustedBootstrapVerifyGeneration(generationRoot, bundleBytes!, receipts, check);
    };
    verifyRetained();
    descriptor = trustedBootstrapReadRegular(join(generationRoot, "packages/gis/descriptor.semio"), DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, "retained GIS browser descriptor", check);
    actorBytes = trustedBootstrapReadRegular(join(generationRoot, "packages/gis/browser/closed-actor.mjs"), DOCUMENT_BROWSER_ACTOR_MAX_BYTES, "retained GIS browser actor", check);
    if (descriptor.byteLength !== gis.descriptor.byteLength) throw new Error("retained GIS browser descriptor length changed");
    const proof = await proveBrowserActorGisChildV1(repoRoot, work, {
      bytes: actorBytes, sha256: gis.browserActor.sha256, byteLength: gis.browserActor.byteLength,
      componentSha256: gis.component.sha256, descriptorByteSha256: gis.descriptor.sha256,
    }, descriptor, owner.control);
    verifyRetained();
    console.log("trusted-retained-gis-browser-receipt: " + JSON.stringify({
      profileId: current.profileId, generationId: current.generationId, bundleSha256: current.bundleSha256,
      componentSha256: gis.component.sha256, descriptorSha256: gis.descriptor.sha256,
      actorSha256: gis.browserActor.sha256, actorByteLength: gis.browserActor.byteLength,
      normalizedDescriptor: true, ...proof,
    }));
    console.log("trusted-retained-gis-browser: exact published actor described in Chromium; no authenticated worker, cold-pair application, Shell renderer, Map mutation or two-peer claim");
  } finally {
    actorBytes?.fill(0);
    descriptor?.fill(0);
    bundleBytes?.fill(0);
    owner.close();
    rmSync(work, { recursive: true, force: true });
  }
}

/** 🧳️ Copies one already verified immutable selection below an isolated candidate's own data root. */
export function stageTrustedBootstrapCandidateCurrent(candidateDataRoot: string, current: TrustedBootstrapBundleV1, check: () => void): void {
  const bundleBytes = trustedBootstrapReadRegular(current.bundlePath, 4 * 1024 * 1024, "trusted candidate source bundle", check);
  const stageRoot = join(candidateDataRoot, "trusted-catalog", `staging-${randomBytes(16).toString("hex")}`);
  try {
    if (createHash("sha256").update(bundleBytes).digest("hex") !== current.bundleSha256) throw new Error("trusted candidate source bundle differs from its retained digest");
    const bundle = trustedBootstrapReadCurrentBundle(current, check);
    const packages = new Map<string, any>(bundle.packages.map((record: any) => [record.pluginId, record]));
    if (packages.size === 0 || !packages.has("stdio")) throw new Error("trusted candidate source carries no package closure rooted at stdio");
    const receipts = new Map<string, TrustedBootstrapGenerationReceiptV1>();
    for (const plugin of packages.keys()) {
      const record = packages.get(plugin)!;
      if (record.packageId !== `semio:${plugin}` || record.component?.path !== `packages/${plugin}/component.wasm` || record.descriptor?.path !== `packages/${plugin}/descriptor.semio`)
        throw new Error(`trusted candidate ${plugin} path identity differs`);
      receipts.set(plugin, trustedBootstrapGenerationReceipt(record));
    }
    const sourceRoot = dirname(current.bundlePath);
    trustedBootstrapVerifyGeneration(sourceRoot, bundleBytes, receipts, check);
    const trustedRoot = join(candidateDataRoot, "trusted-catalog");
    const generationsRoot = join(trustedRoot, "generations");
    mkdirSync(generationsRoot, { recursive: true, mode: 0o700 });
    for (const directory of [candidateDataRoot, trustedRoot, generationsRoot]) {
      const info = lstatSync(directory);
      if (info.isSymbolicLink() || !info.isDirectory()) throw new Error("trusted candidate owner root traverses a link or non-directory");
      if (process.platform !== "win32") chmodSync(directory, 0o700);
    }
    const generationRoot = join(generationsRoot, current.generationId);
    if (existsSync(generationRoot)) {
      trustedBootstrapVerifyGeneration(generationRoot, bundleBytes, receipts, check);
    } else {
      // 🧳️ Derived from the receipts the bundle itself carries, exactly the way
      // `trustedBootstrapVerifyGeneration` derives the closure it then checks this staging against.
      // Spelled as the fixed gis+stdio pair (ticket 26/09/18 slice TC3c generalised
      // `withMaterializedTrustedCatalogBundleV1` and left this copy behind) the candidate staged two
      // packages out of a three-package selection and the very next line's fence answered
      // `generation directory is not the exact regular closure` — TC3e measured that at 13:48:13 on
      // 2026-09-22, seven minutes into a hold, with the note package already published upstream.
      const stagedPlugins = [...receipts.keys()].sort();
      const stagedCarriesActor = (plugin: string): boolean => receipts.get(plugin)!.browserActor.kind === "closed-browser-actor";
      const files: { relative: string; maximum: number }[] = [];
      mkdirSync(join(stageRoot, "plugin-modules"), { recursive: true, mode: 0o700 });
      for (const plugin of stagedPlugins) {
        mkdirSync(stagedCarriesActor(plugin) ? join(stageRoot, "packages", plugin, "browser") : join(stageRoot, "packages", plugin), { recursive: true, mode: 0o700 });
        files.push({ relative: `packages/${plugin}/component.wasm`, maximum: DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES });
        files.push({ relative: `packages/${plugin}/descriptor.semio`, maximum: DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES });
        files.push({ relative: receipts.get(plugin)!.pluginModule.path, maximum: TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES });
        if (stagedCarriesActor(plugin)) files.push({ relative: `packages/${plugin}/browser/closed-actor.mjs`, maximum: DOCUMENT_BROWSER_ACTOR_MAX_BYTES });
      }
      for (const blob of readdirSync(join(sourceRoot, "plugin-modules")).sort()) files.push({ relative: trustedBootstrapPluginModuleBlobPath(blob), maximum: TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES });
      trustedBootstrapWriteNew(join(stageRoot, "trusted-catalog.json"), bundleBytes, check);
      for (const file of files) {
        const bytes = trustedBootstrapReadRegular(join(sourceRoot, file.relative), file.maximum, `trusted candidate ${file.relative}`, check);
        try {
          trustedBootstrapWriteNew(join(stageRoot, file.relative), bytes, check);
        } finally {
          bytes.fill(0);
        }
      }
      for (const plugin of stagedPlugins) {
        if (stagedCarriesActor(plugin)) trustedBootstrapFsyncDirectory(join(stageRoot, "packages", plugin, "browser"));
        trustedBootstrapFsyncDirectory(join(stageRoot, "packages", plugin));
      }
      trustedBootstrapFsyncDirectory(join(stageRoot, "plugin-modules"));
      trustedBootstrapFsyncDirectory(join(stageRoot, "packages"));
      trustedBootstrapFsyncDirectory(stageRoot);
      trustedBootstrapVerifyGeneration(stageRoot, bundleBytes, receipts, check);
      renameSync(stageRoot, generationRoot);
      trustedBootstrapFsyncDirectory(generationsRoot);
    }
    stageTrustedBootstrapProbePointer(candidateDataRoot, current);
    const staged = trustedBootstrapCurrent(candidateDataRoot);
    if (!staged || staged.profileId !== current.profileId || staged.generationId !== current.generationId || staged.bundleSha256 !== current.bundleSha256 || staged.bundlePath !== join(generationRoot, "trusted-catalog.json"))
      throw new Error("trusted candidate current pointer differs from its isolated generation");
  } finally {
    bundleBytes.fill(0);
    rmSync(stageRoot, { recursive: true, force: true });
  }
}

/** 🧪️ Independent WebCrypto receipt oracle exercises the production rotation capture boundary. */
export async function proveTrustedRotationSourceFixture(fixture: Record<string, any>): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const { writeFileSync } = await import("node:fs");
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  assert(artifactRoot !== undefined && artifactRoot.includes("🗑️generated"));
  const evidence = mkdtempSync(join(artifactRoot, "rotation-source-"));
  for (const law of fixture.rotation.sourceCases) {
    const path = join(evidence, `${law.change}.json`);
    const bundle = { schemaVersion: 4, profiles: [{ id: fixture.profile.id, generationId: fixture.profile.generationId }], packages: [{}, {}] };
    const bytes = Buffer.from(JSON.stringify(bundle));
    const sha256 = await webSha256Hex(bytes);
    writeFileSync(path, bytes, { flag: "wx" });
    const receipt = { profileId: fixture.profile.id, generationId: fixture.profile.generationId, bundleSha256: sha256, bundlePath: path };
    if (law.change === "wrong-generation") receipt.generationId = "42".repeat(32);
    if (law.change === "wrong-profile") receipt.profileId = "foreign";
    if (law.change === "replaced-bundle") writeFileSync(path, JSON.stringify({ ...bundle, packages: [{ changed: true }, {}] }));
    let checks = 0,
      accepted = false;
    try {
      trustedBootstrapReadCurrentBundle(receipt, () => {
        checks++;
        if (law.change === "cancel-before-read") throw new Error("cancelled");
        if (law.change === "leaf-replaced-during-read" && checks === 2) {
          renameSync(path, `${path}.old`);
          writeFileSync(path, bytes, { flag: "wx" });
        }
      });
      accepted = true;
    } catch {}
    assert.equal(accepted, law.accepted, law.change);
  }
  for (const law of fixture.rotation.writeCases) {
    const path = join(evidence, `${law.id}.mjs`);
    const bytes = Buffer.alloc(law.byteLength, 42),
      previous = Buffer.from("previous owner");
    if (law.existing) writeFileSync(path, previous, { flag: "wx" });
    let checks = 0,
      accepted = false;
    try {
      trustedBootstrapWriteNew(path, bytes, () => {
        checks++;
        if (checks === law.cancelCheck) throw new Error("cancelled");
      });
      accepted = true;
    } catch {}
    assert.equal(accepted, law.accepted, law.id);
    assert.equal(existsSync(path), law.retained !== "absent", `${law.id} retained owner`);
    if (law.retained !== "absent") {
      const retained = readFileSync(path),
        expected = law.retained === "old" ? previous : bytes;
      assert.equal(createHash("sha256").update(retained).digest("hex"), await webSha256Hex(expected), `${law.id} exact bytes`);
    }
  }
  console.log(`trusted-rotation-source: WebCrypto=1 cases=${fixture.rotation.sourceCases.length} writes=${fixture.rotation.writeCases.length} evidence=${evidence}; capture/write ownership only, no generation publication`);
}
