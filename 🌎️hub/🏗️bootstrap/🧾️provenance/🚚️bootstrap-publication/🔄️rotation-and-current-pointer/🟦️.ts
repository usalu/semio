/** 🔄️ Trusted bootstrap rotation and current pointer: the stdio GIS rotation materialization and the publication revision, the current pointer read, the published current, its verification and the probe pointer staging.
 * @see ../../../../📦️packages/🦀️rust/📜️script.ts
 */
import { withTrustedCatalogCleanupV1 } from "../../🧹️settlement/🟦️.ts";
import { readTrustedCatalogPublicationProvenanceV1, trustedCatalogByteClaimV1, createTrustedCatalogProvenanceDirectoryV1, type PackageProducerV1, type GenerationProducerV1 } from "../../🟦️.ts";
import { checkResourceLeaseControl } from "@semio-tech/process/leases";
import { type CompilerAdmissionControl } from "@semio-tech/process/compiler-capacity";
import { requireIssuedResourceControlInvocationV1, type ScriptInvocation } from "@semio-tech/process/routing";
import { observePhysicalFileV1, readPhysicalFileV1, type FileObservationControlV1 } from "@semio-tech/framework-filesystem/observation";
import { createHash, randomBytes } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, renameSync, rmSync } from "node:fs";
import { dirname, join } from "node:path";
import { decodePackValue, encodePackValue, packValueToExactJson } from "@semio-tech/framework-os";
import type { PackValue } from "@semio-tech/framework-os";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { verifySemanticDescriptorPairBytesV1 } from "@semio-tech/plugin-registry/catalog-verification/descriptor-pair";
import { readStableBuildFile } from "@semio-tech/repo-lib/build";
import { type TrustedBootstrapBundleV1, trustedBootstrapCarryPluginModule, type TrustedBootstrapCurrentPointerV1, trustedBootstrapFsyncDirectory, trustedBootstrapGenerationReceipt, type TrustedBootstrapMaterializationV1, type TrustedBootstrapPublishedV1, trustedBootstrapRecordReceipt, trustedBootstrapVerifyGeneration, trustedBootstrapWriteNew } from "../../🗃️bootstrap-bundle/🟦️.ts";
import { documentOpenNeutralObject, documentOpenNeutralText, trustedBootstrapBrowserActorV1, type TrustedBootstrapCodec, trustedBootstrapDependencyEncoding, trustedBootstrapDescriptorClaims, trustedBootstrapExecutionProtocol, trustedBootstrapProfileEncoding, trustedBootstrapResolveDependencies } from "../../🧮️bootstrap-claims/🟦️.ts";
import { trustedBootstrapReadRegular } from "../../🧭️bootstrap-selection/🟦️.ts";
import { trustedBootstrapReadCurrentBundle } from "../🧱️materialized-bundle/🟦️.ts";

/** 🔄️ Reissues the zero-target Stdio descriptor as a distinct immutable, fully verified generation. */
export async function materializeTrustedStdioGisRotation(invocation:ScriptInvocation,phaseControl:CompilerAdmissionControl,repoRoot: string, dataRoot: string, current: TrustedBootstrapPublishedV1): Promise<TrustedBootstrapMaterializationV1> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);
  const observation:FileObservationControlV1={maxBytes:134217728,maxWork:65536,chunkBytes:1048576,cancelled:()=>phaseControl.signal.aborted,remainingMs:()=>checkResourceLeaseControl(phaseControl)??Infinity,onProgress:()=>{checkResourceLeaseControl(phaseControl);}};
  const sourceProducer=await readTrustedCatalogPublicationProvenanceV1(dataRoot,observation);
  const retained = trustedBootstrapCurrent(dataRoot);
  if (!retained || JSON.stringify(retained) !== JSON.stringify(current)) throw new Error("trusted stdio+GIS rotation did not start from the retained current generation");
  const trustedRoot = join(dataRoot, "trusted-catalog");
  const generationsRoot = join(trustedRoot, "generations");
  const stageRoot = join(trustedRoot, `rotation-${randomBytes(16).toString("hex")}`);

  const checkGeneration = () => {
    if (phaseControl.signal.aborted || (checkResourceLeaseControl(phaseControl)??Infinity) <= 0) throw new Error("trusted rotation generation cancelled before publication");
    checkResourceLeaseControl(phaseControl);
  };
  return withTrustedCatalogCleanupV1(async()=>{
    const bundle = trustedBootstrapReadCurrentBundle(current, checkGeneration);
    const profile = bundle.profiles[0]!;
    mkdirSync(stageRoot, { mode: 0o700 });
    const codecs = {} as Record<"gis" | "stdio", TrustedBootstrapCodec[]>;
    for (const plugin of ["gis", "stdio"] as const) {
      const record = bundle.packages.find((candidate: any) => candidate.pluginId === plugin);
      if (!record || record.packageId !== `semio:${plugin}` || record.role !== "plugin" || !Array.isArray(record.nativeCodecs)) throw new Error(`trusted rotation lost ${plugin} package authority`);
      codecs[plugin] = record.nativeCodecs;
      const sourceRoot = join(generationsRoot, current.generationId, "packages", plugin);
      const destinationRoot = join(stageRoot, "packages", plugin);
      mkdirSync(destinationRoot, { recursive: true, mode: 0o700 });
      const sourceReceipt = trustedBootstrapGenerationReceipt(record);
      const component = trustedBootstrapReadRegular(join(sourceRoot, "component.wasm"), DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, `${plugin} rotation component`, checkGeneration);
      let descriptor = trustedBootstrapReadRegular(join(sourceRoot, "descriptor.semio"), 4 * 1024 * 1024, `${plugin} rotation descriptor`, checkGeneration);
      let actorBytes: Uint8Array | undefined;
      try {
        if (
          component.byteLength !== record.component.byteLength ||
          createHash("sha256").update(component).digest("hex") !== record.component.sha256 ||
          descriptor.byteLength !== record.descriptor.byteLength ||
          createHash("sha256").update(descriptor).digest("hex") !== record.descriptor.sha256
        )
          throw new Error(`trusted rotation ${plugin} source differs from its retained receipt`);
        const claims = trustedBootstrapDescriptorClaims(descriptor);
        const dependencies = trustedBootstrapResolveDependencies(claims, profile.selectedClosure);
        if (claims.identity.pluginId !== plugin || claims.identity.packageId !== record.packageId || claims.identity.version !== record.version || claims.executionProtocol.appChannelVersion !== trustedBootstrapExecutionProtocol(record.executionProtocol).appChannelVersion || !trustedBootstrapDependencyEncoding(dependencies).equals(trustedBootstrapDependencyEncoding(record.dependencies))) throw new Error("trusted rotation descriptor identity, protocol or dependencies differ from its retained receipt");
        record.executionProtocol = claims.executionProtocol;
        record.dependencies = dependencies;
        const actor = trustedBootstrapBrowserActorV1(record.browserActor, { componentSha256: record.component.sha256, descriptorByteSha256: record.descriptor.sha256 }, plugin === "gis" ? "wasm" : "react");
        record.browserActor = actor;
        if (actor.kind === "closed-browser-actor") {
          if (actor.path !== "packages/gis/browser/closed-actor.mjs") throw new Error("rotation actor path differs from fixed GIS closure");
          actorBytes = readStableBuildFile(join(sourceRoot, "browser", "closed-actor.mjs"), actor.byteLength, { remaining: actor.byteLength }, checkGeneration);
          if (actorBytes.byteLength !== actor.byteLength || createHash("sha256").update(actorBytes).digest("hex") !== actor.sha256) throw new Error("rotation actor differs from its verified receipt");
          mkdirSync(join(destinationRoot, "browser"), { mode: 0o700 });
          trustedBootstrapWriteNew(join(destinationRoot, "browser", "closed-actor.mjs"), actorBytes, checkGeneration);
          trustedBootstrapFsyncDirectory(join(destinationRoot, "browser"));
        }
        if (plugin === "stdio") {
          const value = decodePackValue(descriptor) as unknown as Record<string, any>;
          if (value.packageId !== record.packageId || value.manifest?.pluginId !== plugin || value.manifest?.version !== record.version || value.role !== record.role || value.execution !== "isolated" || "hashes" in value)
            throw new Error("trusted rotation Stdio descriptor identity changed");
          value.manifest.label = `Stdio trusted rotation ${randomBytes(8).toString("hex")}`;
          const updatedDescriptor = Buffer.from(encodePackValue(value));
          descriptor.fill(0);
          descriptor = updatedDescriptor;
          const json = Buffer.from(JSON.stringify(packValueToExactJson(value as PackValue)), "utf8");
          try {
            const pair = verifySemanticDescriptorPairBytesV1(json, descriptor);
            if (pair.pluginId !== plugin || pair.packageId !== record.packageId || pair.version !== record.version || pair.role !== "plugin" || pair.execution !== "isolated") throw new Error("trusted rotation Stdio descriptor identity changed");
          } finally {
            json.fill(0);
          }
          record.descriptor.byteLength = descriptor.byteLength;
          record.descriptor.sha256 = createHash("sha256").update(descriptor).digest("hex");
          record.receipt = { ...trustedBootstrapRecordReceipt({ ...record, descriptor: sourceReceipt.descriptor }), semanticDescriptorSha256: record.descriptor.sha256 };
        }
        trustedBootstrapWriteNew(join(destinationRoot, "component.wasm"), component, checkGeneration);
        trustedBootstrapWriteNew(join(destinationRoot, "descriptor.semio"), descriptor, checkGeneration);
        record.pluginModule = trustedBootstrapCarryPluginModule(join(generationsRoot, current.generationId), stageRoot, sourceReceipt, record, descriptor, checkGeneration);
        trustedBootstrapFsyncDirectory(destinationRoot);
      } finally {
        component.fill(0);
        descriptor.fill(0);
        actorBytes?.fill(0);
      }
    }
    const packageSummary = bundle.packages.map((record: any) => ({
      pluginId: record.pluginId,
      packageId: record.packageId,
      version: record.version,
      role: record.role,
      componentSha256: record.component.sha256,
      componentBlake3: record.component.blake3,
      descriptorSha256: record.descriptor.sha256,
      receipt: record.receipt,
      dependencies: record.dependencies,
      executionProtocol: record.executionProtocol,
      browserActor: record.browserActor,
      pluginModule: record.pluginModule,
      codecCount: record.nativeCodecs.length,
      targetCount: record.openTargets.length,
    }));
    const packageTargets = bundle.packages.reduce((total: number, record: any) => total + record.openTargets.length, 0);
    if (!Array.isArray(profile.openTargets) || profile.openTargets.length !== packageTargets) throw new Error("trusted rotation source profile does not open every package target");
    const profileSummary = { id: profile.id, selectedClosure: profile.selectedClosure, packages: packageSummary, openTargets: profile.openTargets.map((selection: any) => ({ pluginId: selection.package.pluginId, ...selection.target })) };
    const generationId = createHash("sha256").update(trustedBootstrapProfileEncoding(profileSummary, codecs)).digest("hex");
    if (generationId === current.generationId) throw new Error("trusted rotation did not change the full profile generation");
    profile.generationId = generationId;
    const bundleBytes = Buffer.from(`${JSON.stringify(bundle)}\n`, "utf8");
    if (bundleBytes.byteLength > 4 * 1024 * 1024) throw new Error("trusted rotation bundle exceeds 4 MiB");
    trustedBootstrapWriteNew(join(stageRoot, "trusted-catalog.json"), bundleBytes, checkGeneration);
    trustedBootstrapFsyncDirectory(join(stageRoot, "packages"));
    trustedBootstrapFsyncDirectory(join(stageRoot, "plugin-modules"));
    trustedBootstrapFsyncDirectory(stageRoot);
    const generationRoot = join(generationsRoot, generationId);
    if (existsSync(generationRoot)) throw new Error("trusted rotation generation already exists");
    trustedBootstrapVerifyGeneration(stageRoot, bundleBytes, new Map(bundle.packages.map((record: any) => [record.pluginId, trustedBootstrapGenerationReceipt(record)])), checkGeneration);
    renameSync(stageRoot, generationRoot);
    trustedBootstrapFsyncDirectory(generationsRoot);
    const observationRoot=join(trustedRoot,"provenance","generations",generationId,randomBytes(16).toString("hex"));createTrustedCatalogProvenanceDirectoryV1(dataRoot,observationRoot);
    const packages:PackageProducerV1[]=[];
    for(const original of sourceProducer.generation.packages){const record=bundle.packages.find((row:any)=>row.pluginId===original.pluginId);if(!record)throw Error("Rotation lost actual original package producer");let browserActor:PackageProducerV1["browserActor"]=null;if(original.browserActor){const input=await readPhysicalFileV1(original.browserActor.path,observation);if(input.claim.sha256!==original.browserActor.sha256||input.claim.byteLength!==original.browserActor.byteLength)throw Error("Rotation original actor producer changed");const path=join(observationRoot,original.pluginId+"-browser.json");trustedBootstrapWriteNew(path,input.bytes,checkGeneration);browserActor=await observePhysicalFileV1(path,observation);}packages.push({...original,component:await trustedCatalogByteClaimV1(generationRoot,join(generationRoot,record.component.path),observation),descriptor:await trustedCatalogByteClaimV1(generationRoot,join(generationRoot,record.descriptor.path),observation),browserActor});}
    const producer:GenerationProducerV1={...sourceProducer.generation,generationId,bundle:await trustedCatalogByteClaimV1(generationRoot,join(generationRoot,"trusted-catalog.json"),observation),packages};
    const producerPath=join(observationRoot,"producer.json");trustedBootstrapWriteNew(producerPath,Buffer.from(JSON.stringify(producer)+"\n"),checkGeneration);
    return { profileId: profile.id, generationId, bundleSha256: createHash("sha256").update(bundleBytes).digest("hex"), bundlePath: join(generationRoot, "trusted-catalog.json"),producerPath };
  },[()=>rmSync(stageRoot,{recursive:true,force:true})]);
}

export function trustedBootstrapPublicationRevision(value: unknown): string {
  if (typeof value !== "string" || !/^[1-9][0-9]{0,19}$/u.test(value) || BigInt(value) > 18_446_744_073_709_551_615n) throw new Error("trusted publication revision is not canonical nonzero u64");
  return value;
}

/** 📌️ Captures one canonical pointer token without following a link or reading its selection. */
export function trustedBootstrapReadCurrentPointer(dataRoot: string): Readonly<{ pointer: TrustedBootstrapCurrentPointerV1; sha256: string }> | undefined {
  const pointerPath = join(dataRoot, "trusted-catalog", "current.json");
  try {
    const info = lstatSync(pointerPath);
    if (info.isSymbolicLink() || !info.isFile()) throw new Error("trusted catalog current pointer is not a regular leaf");
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return undefined;
    throw error;
  }
  const bytes = Buffer.from(trustedBootstrapReadRegular(pointerPath, 64 * 1024, "trusted catalog current pointer", () => {}));
  try {
    const row = documentOpenNeutralObject(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes)), ["profileId", "generationId", "bundleSha256", "publicationRevision"]);
    if (typeof row.generationId !== "string" || typeof row.bundleSha256 !== "string" || !/^[0-9a-f]{64}$/u.test(row.generationId) || !/^[0-9a-f]{64}$/u.test(row.bundleSha256)) throw new Error("trusted catalog current digest is invalid");
    const pointer = Object.freeze({ profileId: documentOpenNeutralText(row.profileId), generationId: row.generationId, bundleSha256: row.bundleSha256, publicationRevision: trustedBootstrapPublicationRevision(row.publicationRevision) });
    if (!bytes.equals(Buffer.from(`${JSON.stringify(pointer)}\n`, "utf8"))) throw new Error("trusted catalog current pointer is not exact canonical metadata");
    return Object.freeze({ pointer, sha256: createHash("sha256").update(bytes).digest("hex") });
  } finally {
    bytes.fill(0);
  }
}

export function trustedBootstrapCurrent(dataRoot: string): TrustedBootstrapPublishedV1 | undefined {
  const current = trustedBootstrapReadCurrentPointer(dataRoot);
  if (!current) return undefined;
  const pointer = current.pointer;
  if (!/^local-(?:[a-z0-9]+-)+open-v1$/u.test(pointer.profileId)) throw new Error("trusted catalog current profile is not a local selection this builder mints");
  const bundlePath = join(dataRoot, "trusted-catalog", "generations", pointer.generationId, "trusted-catalog.json");
  const receipt = Object.freeze({ profileId: pointer.profileId, generationId: pointer.generationId, bundleSha256: pointer.bundleSha256, bundlePath, publicationRevision: pointer.publicationRevision, currentSha256: current.sha256 });
  trustedBootstrapReadCurrentBundle(receipt, () => {});
  return receipt;
}

/** 🔎️ Rechecks all exact generation bytes without mutating any selection. */
export function trustedBootstrapVerifyPublication(dataRoot: string, receipt: TrustedBootstrapBundleV1): void {
  const bundlePath = join(dataRoot, "trusted-catalog", "generations", receipt.generationId, "trusted-catalog.json");
  const retained = { ...receipt, bundlePath };
  const selected = trustedBootstrapReadCurrentBundle(retained, () => {});
  const bundleBytes = trustedBootstrapReadRegular(bundlePath, 4 * 1024 * 1024, "trusted publication bundle", () => {});
  try {
    if (createHash("sha256").update(bundleBytes).digest("hex") !== receipt.bundleSha256) throw new Error("trusted publication bundle changed before its final fence");
    trustedBootstrapVerifyGeneration(dirname(bundlePath), bundleBytes, new Map(selected.packages.map((record: any) => [record.pluginId, trustedBootstrapGenerationReceipt(record)])), () => {});
  } finally {
    bundleBytes.fill(0);
  }
}

/** 🧪️ Seeds only a never-published isolated probe; this path cannot replace a current pointer. */
export function stageTrustedBootstrapProbePointer(dataRoot: string, receipt: TrustedBootstrapBundleV1): void {
  trustedBootstrapVerifyPublication(dataRoot, receipt);
  const pointer = Buffer.from(`${JSON.stringify({ profileId: receipt.profileId, generationId: receipt.generationId, bundleSha256: receipt.bundleSha256, publicationRevision: "1" })}\n`, "utf8");
  const trustedRoot = join(dataRoot, "trusted-catalog");
  trustedBootstrapWriteNew(join(trustedRoot, "current.json"), pointer, () => {});
  trustedBootstrapFsyncDirectory(trustedRoot);
}
