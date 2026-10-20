/** 🧱️ Trusted bootstrap materialized bundle: materializes the selected trusted catalog bundle into a verified directory and reads the current bundle back.
 * @see ../../../../📦️packages/🦀️rust/📜️script.ts
 */
import { withTrustedCatalogCleanupV1 } from "../../🧹️settlement/🟦️.ts";
import { trustedCargoCompilationPlanV1 } from "../../📦️cargo/🟦️.ts";
import { readCompletedCargoPublicationReferenceV1, type CompletedCargoPublicationReferenceV1 } from "@semio-tech/process/artifacts/native-build/receipt/retention/reference";
import { withTrustedCargoCompilationPacketsV1 } from "../../📦️cargo/📥️reading/🟦️.ts";
import { withRepositoryCacheStoreV1 } from "@semio-tech/repo-lib/storage";
import { withRepositoryArtifactDestinationV1, stageRepositoryArtifacts } from "@semio-tech/repo-lib/artifacts";
import { checkResourceLeaseControl } from "@semio-tech/process/leases";
import { withDescriptorEmitterV1, withDescriptorEmitterTaskV1 } from "@semio-tech/framework-plugin/describe/emitter";
import { type CompilerAdmissionControl } from "@semio-tech/process/compiler-capacity";
import { withRequestedIssuedResourceControlV1 } from "@semio-tech/process/routing";
import { trustedCatalogByteClaimV1, createTrustedCatalogProvenanceDirectoryV1, type ActorProducerV1, type PackageProducerV1, type GenerationProducerV1 } from "../../🟦️.ts";
import { observePhysicalFileV1 } from "@semio-tech/framework-filesystem/observation";
import { buildBudgetMs } from "@semio-tech/process/budget";
import { createHash, randomBytes } from "node:crypto";
import { chmodSync, existsSync, lstatSync, mkdirSync, renameSync, rmSync } from "node:fs";
import { join } from "node:path";
import { decodePackValue, packValueToExactJson } from "@semio-tech/framework-os";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES } from "@semio-tech/framework/directory/schema";
import { produceFreshComponentV1 } from "@semio-tech/framework-plugin/describe/fresh-component";
import { type FreshComponentReceiptV1 } from "@semio-tech/framework-plugin/describe/source-epoch";
import { buildClosedBrowserActorArtifactV1, type ClosedBrowserActorArtifactV1 } from "@semio-tech/framework-plugin/browser-bundle/script";
import { type LocalHubHostV1, withLocalHubHostTaskV1 } from "../../../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { captureTrustedBootstrapCodecsV1, type TrustedBootstrapBundleV1, trustedBootstrapFileObservationControlV1, trustedBootstrapFsyncDirectory, trustedBootstrapGenerationReceipt, trustedBootstrapHubOpenTargetsV1, type TrustedBootstrapMaterializationV1, trustedBootstrapPluginModuleV1, trustedBootstrapVerifyGeneration, trustedBootstrapWriteNew, UNLIMITED_TRUSTED_BUILD_DEADLINE_MS } from "../../🗃️bootstrap-bundle/🟦️.ts";
import { trustedBootstrapBrowserActorV1, type TrustedBootstrapBrowserActorV1, trustedBootstrapClosureEncoding, type TrustedBootstrapCodec, trustedBootstrapDescriptorClaims, type TrustedBootstrapDescriptorClaimsV1, type TrustedBootstrapOpenTargetV1, type TrustedBootstrapPluginModuleRecordV1, trustedBootstrapProfileEncoding, trustedBootstrapResolveClosure, trustedBootstrapTupleOrder } from "../../🧮️bootstrap-claims/🟦️.ts";
import { trustedBootstrapComponentCodecRowsV1, trustedBootstrapDescriptorKindsV1, trustedBootstrapHostedKindsV1, trustedBootstrapLinkedUnownedKindsV1, type TrustedBootstrapPackageSpecV1, trustedBootstrapPreflightDescriptorsV1, trustedBootstrapProfileIdV1, trustedBootstrapReadRegular } from "../../🧭️bootstrap-selection/🟦️.ts";

/** 🏗️ Produces one immutable closed N-package generation without loading or registering codecs. */

export async function withMaterializedTrustedCatalogBundleV1<T>(host:LocalHubHostV1,phaseControl:CompilerAdmissionControl,selection:readonly TrustedBootstrapPackageSpecV1[],consume:(receipt:TrustedBootstrapMaterializationV1,phase:CompilerAdmissionControl)=>Promise<T>):Promise<T> {
  if(typeof consume!=="function")throw Error("Actual retained trusted catalog consumer required");
  selection=Object.freeze(selection.map(spec=>Object.freeze({...spec})));
  return withLocalHubHostTaskV1(host,phaseControl,context=>{
  const {invocation,workspace:repoRoot,dataRoot,binaryPath:hubBinary}=context;
  const budget=buildBudgetMs(),limit=budget>0?Math.min(budget,UNLIMITED_TRUSTED_BUILD_DEADLINE_MS):UNLIMITED_TRUSTED_BUILD_DEADLINE_MS;
  return withRequestedIssuedResourceControlV1(context.control,limit,async compiler=>{
  trustedBootstrapPreflightDescriptorsV1(repoRoot, selection);
  const trustedRoot = join(dataRoot, "trusted-catalog");
  mkdirSync(trustedRoot, { recursive: true, mode: 0o700 });
  if (lstatSync(trustedRoot).isSymbolicLink() || !lstatSync(trustedRoot).isDirectory()) throw new Error("trusted catalog root must be a regular private directory");
  if (process.platform !== "win32") chmodSync(trustedRoot, 0o700);
  const nonce = randomBytes(16).toString("hex");
  const buildRoot = join(trustedRoot, `build-${nonce}`);
  const stageRoot = join(trustedRoot, `staging-${nonce}`);
  mkdirSync(buildRoot, { mode: 0o700 });
  mkdirSync(stageRoot, { mode: 0o700 });
  const observationStage = join(trustedRoot, `provenance-staging-${nonce}`);
  mkdirSync(observationStage, { mode: 0o700 });
  const control={invocation,compiler,diagnosticsRoot:buildRoot,cancelled:()=>compiler.signal.aborted,remainingMs:()=>{const remaining=checkResourceLeaseControl(compiler);if(remaining===null)throw Error("Finite actual catalog compiler phase required");return remaining;},checkpoint:(stage:string,completed:number,total:number)=>{checkResourceLeaseControl(compiler);}};
  const checkBuild = () => {
    checkResourceLeaseControl(control.compiler);
    if (control.cancelled() || control.remainingMs() <= 0) throw new Error("trusted catalog build cancelled");
  };
  const observationControl=trustedBootstrapFileObservationControlV1(control);
  return withTrustedCatalogCleanupV1(async()=>{
    const { codecs: linkedCodecs, gisVersion } = captureTrustedBootstrapCodecsV1(repoRoot, () => {
      if (control.cancelled() || control.remainingMs() <= 0) throw new Error("trusted codec capture cancelled");
      control.checkpoint("capture-codecs", 0, 8);
    });
    const requests = selection.map((spec) => ({ pluginId: spec.pluginId, cargoPackage: spec.cargoPackage, componentPackageId: spec.componentPackageId, outputName: spec.outputName, componentProfile: "release" as const }));
    const receipts = new Map<string, FreshComponentReceiptV1>();
    const compilerInvocations = new Map<string, readonly CompletedCargoPublicationReferenceV1[]>();
    const actorProducers = new Map<string, Omit<ActorProducerV1,"compiler"> & {compilerName:string}>();
    const descriptorClaims = new Map<string, TrustedBootstrapDescriptorClaimsV1>();
    const browserActors = new Map<string, TrustedBootstrapBrowserActorV1>();
    const pluginModules = new Map<string, TrustedBootstrapPluginModuleRecordV1>();
    const descriptorJson = new Map<string, Record<string, any>>();
    const codecs: Record<string, readonly TrustedBootstrapCodec[]> = {};
    const openTargets: TrustedBootstrapOpenTargetV1[] = [];
    const declaredOpenTargets = new Map<string, (codecs: readonly TrustedBootstrapCodec[], unowned: ReadonlySet<string>, unbound: ReadonlySet<string>) => Promise<readonly TrustedBootstrapOpenTargetV1[]>>();
    const unownedKinds = new Map<string, ReadonlySet<string>>();
    for (const request of requests) {
      const spec = selection.find((candidate) => candidate.pluginId === request.pluginId)!;
      const target = join(buildRoot, `${request.pluginId}-target`);
      const stage = join(stageRoot, "packages", request.pluginId);
      mkdirSync(target, { recursive: true, mode: 0o700 });
      mkdirSync(stage, { recursive: true, mode: 0o700 });
      let derivedActor: ClosedBrowserActorArtifactV1 | undefined;
      try {
        const {receipt,derived:componentSha256}=await produceFreshComponentV1(repoRoot,request,stage,control,async(lease,evidence)=>lease.consume(async component=>{
          const digest=createHash("sha256").update(component).digest("hex"),retained:CompletedCargoPublicationReferenceV1[]=[];
          for(const row of evidence.observations)await evidence.withCompilation(row.kind,row.pass,async view=>{
            if(view.observationSha256!==row.sha256||createHash("sha256").update(row.bytes).digest("hex")!==row.sha256||view.observation!==new TextDecoder("utf-8",{fatal:true}).decode(row.bytes))throw Error("Exact unchanged completed compiler observation required");
            const index=view.files.get("index.json"),receipt=view.files.get("receipt.json");if(!index||!receipt||receipt.sha256!==view.observationSha256)throw Error("Complete original compiler packet required");
            const reference=readCompletedCargoPublicationReferenceV1({version:1,mode:view.origin.mode,kind:row.kind,pass:row.pass,receiptSha256:receipt.sha256,indexSha256:index.sha256,originalArtifact:row.originalArtifact});
            await view.recheck();
            await withRepositoryCacheStoreV1(repoRoot,"products",invocation,view.control,(store,phase)=>{const plan=trustedCargoCompilationPlanV1(store.root,reference);return withRepositoryArtifactDestinationV1({workspace:repoRoot,invocation,storage:store.storage,control:store.control,phaseControl:phase,directory:plan.path},destination=>stageRepositoryArtifacts(destination,plan.owner,new Map([...view.files].map(([name,file])=>[name,file.path]))));});
            await view.recheck();retained.push(reference);
          });
          if(retained.filter(row=>row.kind==="component"&&row.originalArtifact?.sha256===digest).length!==1||retained.filter(row=>row.kind==="emitter"&&row.originalArtifact!==null).length!==1)throw Error("One actual bound component and emitter completion required");
          compilerInvocations.set(request.pluginId,Object.freeze(retained));return digest;
        }));
        if (componentSha256 !== receipt.component.sha256 || (derivedActor && derivedActor.componentSha256 !== componentSha256)) throw new Error(`fresh ${request.pluginId} derivation differs from its verified component`);
        if (receipt.pluginId !== request.pluginId || receipt.packageId !== request.componentPackageId) throw new Error(`fresh ${request.pluginId} receipt identity changed after production`);
        const descriptor = trustedBootstrapReadRegular(join(stage, "descriptor.semio"), DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, "fresh compiled descriptor protocol", checkBuild);
        try {
          if (descriptor.byteLength !== receipt.descriptor.byteLength || createHash("sha256").update(descriptor).digest("hex") !== receipt.descriptor.sha256) throw new Error("fresh descriptor protocol bytes differ from the producer receipt");
          descriptorClaims.set(request.pluginId, trustedBootstrapDescriptorClaims(descriptor));
          descriptorJson.set(request.pluginId, packValueToExactJson(decodePackValue(descriptor)) as Record<string, any>);
          const descriptorCopy = Uint8Array.from(descriptor);
          declaredOpenTargets.set(request.pluginId, async (rows, unowned, unbound) => {
            try {
              return await trustedBootstrapHubOpenTargetsV1(host,control.compiler,request.pluginId,descriptorCopy,rows,unowned,unbound);
            } finally {
              descriptorCopy.fill(0);
            }
          });
        } finally {
          descriptor.fill(0);
        }
        const manifestKinds = trustedBootstrapDescriptorKindsV1(descriptorJson.get(request.pluginId)!).map((kind) => [String(kind.id), String(kind.schema)] as const);
        const hostedKinds = trustedBootstrapHostedKindsV1(request.pluginId, descriptorJson.get(request.pluginId)!, codecs);
        if (spec.linkedCodecRegistry) {
          const linked = linkedCodecs[request.pluginId as "gis" | "stdio"];
          if (!linked || linked.length === 0) throw new Error(`trusted codec capture carries no linked closure for ${request.pluginId}`);
          codecs[request.pluginId] = linked;
          unownedKinds.set(request.pluginId, trustedBootstrapLinkedUnownedKindsV1(manifestKinds, linked));
        } else if (manifestKinds.length === 0 && hostedKinds.pairs.length > 0) {
          codecs[request.pluginId] = [];
          unownedKinds.set(request.pluginId, hostedKinds.unowned);
        } else {
          const answered=await withDescriptorEmitterV1(repoRoot,invocation,control.compiler,emitter=>withDescriptorEmitterTaskV1(emitter,invocation,async tool=>{const result=await trustedBootstrapComponentCodecRowsV1(invocation,tool.control,repoRoot,tool.path,join(stage,"component.wasm"),join(target,"component-codecs.json"),manifestKinds);await tool.recheck();return result;}));
          codecs[request.pluginId] = answered.rows;
          unownedKinds.set(request.pluginId, answered.unowned);
        }
        const openTargetsForPackage=declaredOpenTargets.get(request.pluginId);if(!openTargetsForPackage)throw Error("Actual retained descriptor answer required");
        for(const declared of await openTargetsForPackage([...codecs[request.pluginId]!,...hostedKinds.rows],unownedKinds.get(request.pluginId)??new Set(),hostedKinds.unbound))openTargets.push(declared);
        const pluginOpensDocuments = openTargets.some((declared) => declared.pluginId === request.pluginId);
        if (pluginOpensDocuments) {
          const componentBytes = trustedBootstrapReadRegular(join(stage, "component.wasm"), DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, `fresh ${request.pluginId} component for closed actor`, checkBuild);
          try {
            derivedActor = await buildClosedBrowserActorArtifactV1(componentBytes, invocation, control.compiler, { cancelled: () => control.cancelled() || control.remainingMs() <= 0, progress: (phase, completed, total) => control.checkpoint(`actor-${phase}`, completed, total) });
          } finally {
            componentBytes.fill(0);
          }
          if (derivedActor.componentSha256 !== componentSha256) throw new Error(`fresh ${request.pluginId} closed actor source differs from its verified component`);
        }
        const actor = trustedBootstrapBrowserActorV1(
          derivedActor
            ? {
                kind: "closed-browser-actor",
                schema: derivedActor.schema,
                codegenPolicy: derivedActor.codegenPolicy,
                sha256: derivedActor.sha256,
                sourceComponentSha256: componentSha256,
                sourceDescriptorByteSha256: receipt.descriptor.sha256,
                policySha256: derivedActor.policySha256,
                importInterfaces: derivedActor.importInterfaces,
                byteLength: derivedActor.byteLength,
                path: `packages/${request.pluginId}/browser/closed-actor.mjs`,
              }
            : { kind: "none" },
          { componentSha256, descriptorByteSha256: receipt.descriptor.sha256 },
          pluginOpensDocuments ? "wasm" : "react",
        );
        if (derivedActor) {
          mkdirSync(join(stage, "browser"), { mode: 0o700 });
          trustedBootstrapWriteNew(join(stage, "browser", "closed-actor.mjs"), derivedActor.bytes, checkBuild);
          trustedBootstrapFsyncDirectory(join(stage, "browser"));
          const compilerName = derivedActor.producer.compiler.sha256 + ".mjs";
          const compilerRoot = join(observationStage, "compilers"); mkdirSync(compilerRoot, {recursive:true,mode:0o700});
          const compilerPath = join(compilerRoot,compilerName);
          if (!existsSync(compilerPath)) trustedBootstrapWriteNew(compilerPath,derivedActor.producer.compiler.bytes,checkBuild);
          const compilerClaim = await observePhysicalFileV1(compilerPath,observationControl);
          if (compilerClaim.sha256 !== derivedActor.producer.compiler.sha256 || compilerClaim.byteLength !== derivedActor.producer.compiler.byteLength) throw new Error("trusted actor retained compiler changed");
          actorProducers.set(request.pluginId,{schema:"semio.os.closed-browser-actor-producer/v1",actor:await trustedCatalogByteClaimV1(stageRoot,join(stage,"browser","closed-actor.mjs"),observationControl),component:await trustedCatalogByteClaimV1(stageRoot,join(stage,"component.wasm"),observationControl),descriptor:await trustedCatalogByteClaimV1(stageRoot,join(stage,"descriptor.semio"),observationControl),policyCanonical:derivedActor.policyCanonical,policySha256:derivedActor.policySha256,runtime:derivedActor.producer.runtime,inputs:derivedActor.producer.inputs,compilerName});

        }
        browserActors.set(request.pluginId, actor);
        pluginModules.set(request.pluginId, await trustedBootstrapPluginModuleV1(repoRoot, request, stage, stageRoot, target, receipt, control, checkBuild));
        receipts.set(request.pluginId, receipt);
      } finally {
        derivedActor?.bytes.fill(0);
        derivedActor?.producer.compiler.bytes.fill(0);
      }
    }
    const gis = receipts.get("gis");
    if (gis && gis.version !== gisVersion) throw new Error("fresh GIS component version differs from captured native codecs");
    for (const spec of selection) {
      const rows = codecs[spec.pluginId] ?? [];
      if (rows.length === 0) throw new Error(`trusted ${spec.pluginId} closure carries no artifact codec`);
    }
    const selectedClosure = selection
      .map((spec) => ({ pluginId: spec.pluginId, packageId: spec.componentPackageId, version: receipts.get(spec.pluginId)!.version }))
      .sort((left, right) => trustedBootstrapTupleOrder([left.pluginId, left.packageId, left.version], [right.pluginId, right.packageId, right.version]));
    const dependencies = trustedBootstrapResolveClosure(trustedBootstrapProfileIdV1(selection), descriptorClaims, selectedClosure);
    const packageSummary = selectedClosure.map((identity) => {
      const receipt = receipts.get(identity.pluginId)!;
      return {
        pluginId: identity.pluginId,
        packageId: identity.packageId,
        version: identity.version,
        role: "plugin",
        componentSha256: receipt.component.sha256,
        componentBlake3: receipt.component.blake3,
        descriptorSha256: receipt.descriptor.sha256,
        receipt: receipt.componentReceipt,
        dependencies: dependencies.get(identity.pluginId)!,
        executionProtocol: descriptorClaims.get(identity.pluginId)!.executionProtocol,
        browserActor: browserActors.get(identity.pluginId)!,
        pluginModule: pluginModules.get(identity.pluginId)!,
        codecCount: codecs[identity.pluginId]!.length,
        targetCount: openTargets.filter((declared) => declared.pluginId === identity.pluginId).length,
      };
    });
    const profileSummary = { id: trustedBootstrapProfileIdV1(selection), selectedClosure, packages: packageSummary, openTargets };
    const selectedClosureSha256 = createHash("sha256").update(trustedBootstrapClosureEncoding(profileSummary)).digest("hex");
    const generationId = createHash("sha256").update(trustedBootstrapProfileEncoding(profileSummary, codecs)).digest("hex");
    const file = (plugin: string, receipt: FreshComponentReceiptV1) => ({
      pluginId: plugin,
      packageId: receipt.packageId,
      version: receipt.version,
      role: "plugin",
      dependencies: dependencies.get(plugin)!,
      component: { path: `packages/${plugin}/component.wasm`, byteLength: receipt.component.byteLength, sha256: receipt.component.sha256, blake3: receipt.component.blake3 },
      descriptor: { path: `packages/${plugin}/descriptor.semio`, byteLength: receipt.descriptor.byteLength, sha256: receipt.descriptor.sha256 },
      receipt: receipt.componentReceipt,
      executionProtocol: descriptorClaims.get(plugin)!.executionProtocol,
      browserActor: browserActors.get(plugin)!,
      pluginModule: pluginModules.get(plugin)!,
      nativeCodecs: codecs[plugin],
      openTargets: openTargets.filter((declared) => declared.pluginId === plugin).map(({ pluginId: _owner, ...target }) => target),
    });
    const bundle = {
      schemaVersion: 4,
      profiles: [
        {
          id: profileSummary.id,
          selectedClosure,
          selectedClosureSha256,
          openTargets: openTargets.map(({ pluginId: owner, ...target }) => ({ package: selectedClosure.find((identity) => identity.pluginId === owner)!, target })),
          generationId,
        },
      ],
      packages: selectedClosure.map((identity) => file(identity.pluginId, receipts.get(identity.pluginId)!)),
    };
    const bundleBytes = Buffer.from(`${JSON.stringify(bundle)}\n`, "utf8");
    if (bundleBytes.byteLength > 4 * 1024 * 1024) throw new Error("trusted catalog bundle exceeds 4 MiB");
    const bundlePath = join(stageRoot, "trusted-catalog.json");
    trustedBootstrapWriteNew(bundlePath, bundleBytes, checkBuild);
    for (const identity of selectedClosure) trustedBootstrapFsyncDirectory(join(stageRoot, "packages", identity.pluginId));
    trustedBootstrapFsyncDirectory(join(stageRoot, "packages"));
    trustedBootstrapFsyncDirectory(join(stageRoot, "plugin-modules"));
    trustedBootstrapFsyncDirectory(stageRoot);
    if (control.cancelled() || control.remainingMs() <= 0) throw new Error("trusted catalog bootstrap cancelled before publication");
    const references=[...compilerInvocations.values()].flat();
    return await withTrustedCargoCompilationPacketsV1(repoRoot,invocation,control.compiler,references,observationControl,dataRoot,async(packets,actualPhase)=>{
    for(const request of requests){const rows=compilerInvocations.get(request.pluginId);const receipt=receipts.get(request.pluginId);if(!rows||!receipt||rows.filter(row=>row.kind==="component"&&row.originalArtifact?.sha256===receipt.component.sha256).length!==1)throw Error("Exact actual model compiler binding required before adoption");}
    const generations = join(trustedRoot, "generations");
    mkdirSync(generations, { recursive: true, mode: 0o700 });
    if (lstatSync(generations).isSymbolicLink() || !lstatSync(generations).isDirectory()) throw new Error("trusted generations root must be a regular directory");
    const generationRoot = join(generations, generationId);
    const checkGeneration = () => {
      checkBuild();
      if (control.cancelled() || control.remainingMs() <= 0) throw new Error("trusted catalog generation cancelled before publication");
      control.checkpoint("verify-generation", 8, 8);
    };
    if (existsSync(generationRoot)) {
      trustedBootstrapVerifyGeneration(
        generationRoot,
        bundleBytes,
        new Map(bundle.packages.map((record) => [record.pluginId, trustedBootstrapGenerationReceipt(record)])),
        checkGeneration,
      );
      rmSync(stageRoot, { recursive: true, force: true });
    } else {
      trustedBootstrapVerifyGeneration(stageRoot, bundleBytes, new Map(bundle.packages.map((record) => [record.pluginId, trustedBootstrapGenerationReceipt(record)])), checkGeneration);
      renameSync(stageRoot, generationRoot);
      trustedBootstrapFsyncDirectory(generations);
    }
    const bundleSha256 = createHash("sha256").update(bundleBytes).digest("hex");
    const observationParent = join(trustedRoot,"provenance","generations",generationId); createTrustedCatalogProvenanceDirectoryV1(dataRoot,observationParent);
    const observationRoot = join(observationParent,nonce); renameSync(observationStage,observationRoot);
    const packageProducers: PackageProducerV1[] = [];
    for(const identity of selectedClosure){
      const request=requests.find(row=>row.pluginId===identity.pluginId)!,target=join(buildRoot,`${identity.pluginId}-target`),packageRoot=join(generationRoot,"packages",identity.pluginId),metadataRoot=join(observationRoot,"packages",identity.pluginId);
      const invocations=compilerInvocations.get(identity.pluginId);if(!invocations?.length)throw Error("Captured actual compiler observations required");
      const cargoInvocations=invocations;
      let browserActor: PackageProducerV1["browserActor"] = null;
      const actor=actorProducers.get(identity.pluginId);
      if(actor){const {compilerName,...record}=actor;const actorRecord:ActorProducerV1={...record,compiler:await observePhysicalFileV1(join(observationRoot,"compilers",compilerName),observationControl)};const actorPath=join(metadataRoot,"browser-producer.json");trustedBootstrapWriteNew(actorPath,Buffer.from(JSON.stringify(actorRecord)+"\n"),checkBuild);browserActor=await observePhysicalFileV1(actorPath,observationControl);}
      packageProducers.push({pluginId:identity.pluginId,packageId:identity.packageId,cargoPackage:request.cargoPackage,component:await trustedCatalogByteClaimV1(generationRoot,join(packageRoot,"component.wasm"),observationControl),descriptor:await trustedCatalogByteClaimV1(generationRoot,join(packageRoot,"descriptor.semio"),observationControl),cargoInvocations,browserActor});
    }
    const producer:GenerationProducerV1={schema:"semio.hub.trusted-catalog-producer/v1",profileId:profileSummary.id,generationId,bundle:await trustedCatalogByteClaimV1(generationRoot,join(generationRoot,"trusted-catalog.json"),observationControl),packages:packageProducers};
    const producerPath=join(observationRoot,"producer.json");trustedBootstrapWriteNew(producerPath,Buffer.from(JSON.stringify(producer)+"\n"),checkBuild);trustedBootstrapFsyncDirectory(observationRoot);
    const materialized={ profileId: profileSummary.id, generationId, bundleSha256, bundlePath: join(generationRoot, "trusted-catalog.json"), producerPath };
    for(const packet of packets)await packet.recheck();
    const result=await consume(materialized,actualPhase);
    for(const packet of packets)await packet.recheck();
    return result;
    });
  },[()=>rmSync(stageRoot,{recursive:true,force:true}),()=>rmSync(buildRoot,{recursive:true,force:true}),()=>rmSync(observationStage,{recursive:true,force:true})]);
  });
  });
}

/** 🧿️ Rebinds captured rotation bytes to the already retained current receipt before decoding. */
export function trustedBootstrapReadCurrentBundle(current: TrustedBootstrapBundleV1, check: () => void): Record<string, any> {
  const bytes = trustedBootstrapReadRegular(current.bundlePath, 4 * 1024 * 1024, "trusted rotation source bundle", check);
  try {
    if (createHash("sha256").update(bytes).digest("hex") !== current.bundleSha256) throw new Error("trusted rotation source bundle differs from its retained digest");
    const bundle = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
    if (bundle.schemaVersion !== 4 || bundle.profiles?.length !== 1 || !Array.isArray(bundle.packages) || bundle.packages.length === 0 || bundle.profiles[0]?.id !== current.profileId || bundle.profiles[0]?.generationId !== current.generationId)
      throw new Error("trusted rotation source differs from the exact retained profile generation");
    return bundle;
  } finally {
    bytes.fill(0);
  }
}
