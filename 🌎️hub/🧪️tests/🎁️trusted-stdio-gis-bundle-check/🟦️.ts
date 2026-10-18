/** 🧭️ The trusted stdio and GIS bundle gate: source, native, process, browser and two-author modes over the published trusted catalog.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { repositoryCargoNetworkV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import { verifyCapturedCargoArtifactsV1 } from "@semio-tech/process/native-build";
import { readNativeCallerAuthorityV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { buildBudgetMs } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { mkdirSync, readFileSync } from "node:fs";
import { isAbsolute, join, relative, resolve } from "node:path";
import { withRepositoryPublishedCargoExecutableV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/📥️published/🟦️.ts";
import { buildRepositoryCargoArtifacts } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { proveGisComponentColdMapPatch } from "../../🧩️compositions/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🟦️.ts";
import { runRepositoryCargoTests } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { withCapturedLocalHubHostV1 } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { readHubScriptModule } from "../../🧪️testing/📜️script-sources/🟦️.ts";
import { proveTrustedPublicationCli, validateAndPublishTrustedStdioGisCandidate } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🟦️.ts";
import { withMaterializedTrustedCatalogBundleV1 } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🧱️materialized-bundle/🟦️.ts";
import { proveTrustedGisClosedActorV1 } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🗺️gis-proofs-and-staging/🟦️.ts";
import { materializeTrustedStdioGisRotation, trustedBootstrapCurrent } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🔄️rotation-and-current-pointer/🟦️.ts";
import { TRUSTED_BOOTSTRAP_LINKED_PACKAGES, trustedBootstrapSelectPackages } from "../../🏗️bootstrap/🧾️provenance/🧭️bootstrap-selection/🟦️.ts";
import { proveGisMapTwoAuthorCompositionFixture } from "../🌍️gis-map-fixtures/🟦️.ts";
import { proveGisMapTwoAuthorShellProcess } from "../🕹️gis-map-process/🟦️.ts";
import { readHubBinSource, readTrustedCatalogSource } from "../../🧪️testing/📖️source-readers/🟦️.ts";
import { proveTrustedStdioGisBootstrapFixture } from "../🏛️trusted-catalog-fixtures/🚀️stdio-gis-bootstrap-and-compiled-dependencies-fixtures/🟦️.ts";
import { proveTrustedGenerationStageFixture, proveTrustedPublicationFixture } from "../📮️trusted-publication-fixtures/🟦️.ts";

export class TrustedStdioGisBundleCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length === 1 && segments[0] === "--two-author-source") {
      await proveGisMapTwoAuthorCompositionFixture(this.invocation, this.repoRoot);
      return;
    }
    if (segments.length === 1 && segments[0] === "--publication-cli") {
      await proveTrustedPublicationFixture(this.repoRoot);
      await proveTrustedPublicationCli(this.invocation,exactCargoCallerControlV1(this.invocation,"trusted-publication:cli",0), this.repoRoot, this.root);
      return;
    }
    if (segments.length === 1 && segments[0] === "--publication-native") {
      await proveTrustedGenerationStageFixture(this.repoRoot);
      await proveTrustedPublicationFixture(this.repoRoot);
      await runRepositoryCargoTests(["semio-hub-trusted-catalog-contract-laws", "semio-hub-trusted-catalog-laws"], this.repoRoot, this.invocation, ["--no-fail-fast", "--lib", "trusted_publication", "--", "--nocapture"]);
      return;
    }
    if (segments.length === 1 && segments[0] === "--publication-source") {
      await proveTrustedGenerationStageFixture(this.repoRoot);
      await proveTrustedPublicationFixture(this.repoRoot);
      console.log("trusted-publication-source: final file/protocol fences only; no native loader, OS lock, CAS, or current Hub activation claim");
      return;
    }
    if (segments.length > 1 || (segments[0] !== undefined && !["--source", "--native", "--process", "--browser", "--two-author-shell"].includes(segments[0]))) throw new Error("usage: trusted-stdio-gis-bundle-check [--source|--two-author-source|--publication-source|--publication-native|--publication-cli|--native|--process|--browser|--two-author-shell]");
    await proveTrustedStdioGisBootstrapFixture(this.invocation, this.repoRoot);
    await proveGisComponentColdMapPatch(this.repoRoot);
    await proveGisMapTwoAuthorCompositionFixture(this.invocation, this.repoRoot);
    const describeSource = readFileSync(resolve(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts"), "utf8");
    const producerStart = describeSource.indexOf("function freshStage(");
    const producerEnd = describeSource.length;
    const producer = describeSource.slice(producerStart, producerEnd);
    if (
      producerStart < 0 ||
      producerEnd < 0 ||
      !producer.includes('CARGO_INCREMENTAL: "0"') ||
      !producer.includes("pluginWasmArtifactPath(") ||
      !producer.includes("verifySemanticDescriptorPairBytesV1(") ||
      !producer.includes("freshComponentReceiptV1(") ||
      !producer.includes("readStableBuildFile(cargoComponent") ||
      !producer.includes("freshStage(snapshot.descriptorBytes") ||
      !producer.includes("snapshot?.descriptorBytes.fill(0)") ||
      !producer.includes("blake3Hex(componentBytes)") ||
      !producer.includes("if (!complete) rmSync(destination") ||
      producer.indexOf("closeSync(output)") > producer.indexOf("if (!complete) rmSync(destination") ||
      producer.includes("freshCopy(") ||
      producer.includes("atomicDescriptorPair") ||
      producer.includes("plugin-registry:generate") ||
      producer.includes("ownerRoot")
    )
      throw new Error("fresh component producer is not isolated, descriptor-verified, receipt-bound, bounded, close-before-cleanup, or side-effect free");
    const catalogSource = readTrustedCatalogSource(this.repoRoot);
    const providerSource = readFileSync(resolve(this.repoRoot, "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs"), "utf8");
    const runtimeSource = readHubBinSource(this.repoRoot).split("\nmod tests {")[0]!;
    const registrySource = readFileSync(resolve(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts"), "utf8");
    const runtimeCompact = runtimeSource.replace(/\s+/g, "");
    const receiptResolution = runtimeCompact.indexOf(".authority_for_authenticated_exchange(&intent.plan_receipt");
    const targetResolution = runtimeCompact.indexOf("catalog.resolve_document_open(&authority.descriptor", receiptResolution);
    if (
      !catalogSource.includes("bundle.schema_version != TRUSTED_BUNDLE_SCHEMA_VERSION") ||
      !catalogSource.includes("record.receipt.binds(directory::os_component_receipt::ComponentProfileV1::Release, &record.plugin_id, &record.package_id, &record.component.sha256, &record.descriptor.sha256)") ||
      !catalogSource.includes("trusted_profile_generation(&bundle, &profile)") ||
      !catalogSource.includes("selected profile must resolve at least one document-open target") ||
      !catalogSource.includes("selected profile resolved a different number of document-open targets than it declares") ||
      !providerSource.includes("pub fn native_openable_provider_receipt_count()") ||
      !providerSource.includes("receipt.package_version != version") ||
      !registrySource.includes("CATALOG_DESCRIPTOR_MAX_BYTES = 4 * 1024 * 1024") ||
      runtimeSource.split("catalog.generation_id() != authority.catalog.generation_id").length - 1 !== 2 ||
      receiptResolution < 0 ||
      targetResolution < receiptResolution
    )
      throw new Error("trusted stdio+GIS runtime/source boundary is incomplete");
    const modules: Readonly<Record<string, string>> = { publication: "🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication", bundle: "🏗️bootstrap/🧾️provenance/🗃️bootstrap-bundle", dev: "🎛️commands/🏗️build-and-dev", commands: "🎛️commands/🏷️trusted-catalog", check: "🧪️tests/🎁️trusted-stdio-gis-bundle-check" };
    const scriptSource = readHubScriptModule(this.repoRoot, modules.check!);
    const body = (module: string, start: string, end: string): string => {
      const source = readHubScriptModule(this.repoRoot, modules[module]!);
      const first = source.indexOf(start),
        last = source.indexOf(end, first);
      if (first < 0 || last < 0) throw new Error(`trusted stdio+GIS source boundary is missing ${start}`);
      return source.slice(first, last);
    };
    const materializer = body("publication", "\nasync function withMaterializedTrustedCatalogBundleV1", "\n}\n");
    const writer = body("bundle", "\nfunction trustedBootstrapWriteNew", "\nfunction trustedBootstrapFsyncDirectory");
    const publisher = body("publication", "\nasync function publishTrustedBootstrapCurrent", "\nasync function proveTrustedStdioGisCandidatePlan");
    const stalePlan = body("publication", "\nasync function proveTrustedStdioGisStalePlanRejected", "\n/** 🟢️ Publishes current metadata");
    const candidate = body("publication", "\nasync function validateAndPublishTrustedStdioGisCandidate", "\n}\n");
    const rotation = body("publication", "\nasync function materializeTrustedStdioGisRotation", "\nfunction trustedBootstrapCurrent");
    const bootstrap = body("commands", "\nclass TrustedCatalogBootstrapScript", "\n}\n");
    const dev = body("dev", "\nclass DevScript", "\n}\n");
    const gisColdMap = body("publication", "\nasync function proveTrustedGisColdMapComponentV1", "\n/** 🧳️ Copies one already verified immutable selection");
    const nativeGate = body("check", "\nclass TrustedStdioGisBundleCheckScript", "\n}\n");
    const nativeMaterialized = nativeGate.slice(nativeGate.lastIndexOf("return withMaterializedTrustedCatalogBundleV1"));
    const processGate = nativeGate.slice(nativeGate.lastIndexOf('if (segments[0] === "--process") {'));
    const ordered = (source: string, earlier: string, later: string): boolean => source.indexOf(earlier) >= 0 && source.indexOf(earlier) < source.indexOf(later);
    const missingFence = [
      ["materializer publication", !materializer.includes("publishTrustedBootstrapCurrent")],
      ["materializer bounded budget",materializer.includes("budget=buildBudgetMs()")&&materializer.includes("withRequestedIssuedResourceControlV1(context.control,limit")&&!materializer.includes("trustedBootstrapBuildControl(")],
      ["codec capture before component production", ordered(materializer, "captureTrustedBootstrapCodecsV1(", "await produceFreshComponentV1(") && !materializer.includes("trustedBootstrapSourceCodecs(")],
      ["independent versions", !materializer.includes("stdio.version !== gis.version")],
      ["development publication", !dev.includes("publishTrustedBootstrapCurrent")],
      ["writer cleanup", ordered(writer, "closeSync(output)", "if (!complete) rmSync(path")],
      ["native publisher authority", publisher.includes('["trusted-catalog", "publish"]') && publisher.includes("expectedCurrentSha256: expected?.sha256 ?? null") && publisher.includes("trustedPublicationChildReceipt(processResult") && !publisher.includes("renameSync(")],
      ["candidate readiness", ordered(candidate, "await waitForReadiness(candidate,false,TRUSTED_CATALOG_READINESS_STALL_BOUND_MS)","const plan=await proveTrustedStdioGisCandidatePlan(candidate,receipt,envelope)")],
      ["candidate publication", ordered(candidate, "const plan=await proveTrustedStdioGisCandidatePlan(candidate,receipt,envelope)","await publishTrustedBootstrapCurrent(host,actual,receipt,validationRoot,expectedCurrent)")],
      ["candidate compare-current snapshot", ordered(candidate, "expectedCurrent=trustedBootstrapReadCurrentPointer(dataRoot)","await proveTrustedGisColdMapComponentV1")],
      ["candidate stale plan", ordered(candidate, "if(stalePlan)await proveTrustedStdioGisStalePlanRejected","const plan=await proveTrustedStdioGisCandidatePlan")],
      ["stale receipt cleanup", /finally\s*\{\s*stalePlan\.receipt\s*=\s*"";\s*\}/u.test(stalePlan)],
      ["rotation descriptor", rotation.includes("verifySemanticDescriptorPairBytesV1(") && rotation.includes("trustedBootstrapProfileEncoding(")],
      ["server-owned rotation", rotation.includes('value.manifest.label = `Stdio trusted rotation ${randomBytes(8).toString("hex")}`')],
      ["rotation publication", ordered(rotation, "renameSync(stageRoot, generationRoot)", "trustedBootstrapFsyncDirectory(generationsRoot)")],
      ["bootstrap candidate", ordered(bootstrap, "withMaterializedTrustedCatalogBundleV1", "validateAndPublishTrustedStdioGisCandidate")],
      ["development candidate", ordered(dev, "await withMaterializedTrustedCatalogBundleV1", "await validateAndPublishTrustedStdioGisCandidate")],
      ["development launch", ordered(dev, "await validateAndPublishTrustedStdioGisCandidate", "const run = await startLocalHub")],
      [
        "GIS cold-map retained owner",
        gisColdMap.includes('gis.component?.path !== "packages/gis/component.wasm"') &&
          gisColdMap.includes('gis.descriptor?.path !== "packages/gis/descriptor.semio"') &&
          gisColdMap.includes("trustedBootstrapReadCurrentBundle(current, check)") &&
          gisColdMap.includes("verifyRetainedBytes();") &&
          gisColdMap.includes("runRepositoryExactCargoLaws({") &&
          gisColdMap.includes("genuine_gis_component_cold_loads_and_patches_the_exact_tiled_map_surface") &&
          gisColdMap.includes("genuine_gis_component_rejects_stale_cold_authority_before_loading"),
      ],
      ["GIS cold-map pre-publication", nativeMaterialized.length < nativeGate.length && ordered(candidate, "await proveTrustedGisColdMapComponentV1", "stageTrustedBootstrapCandidateCurrent") && ordered(candidate, "await proveTrustedGisColdMapComponentV1", "await startLocalHub")],
      ["GIS cold-map neutral source", nativeGate.includes("await proveGisComponentColdMapPatch(this.repoRoot)") && scriptSource.includes('from "../../../🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🟦️.ts"')],
      [
        "native target",
        nativeGate.includes("consume:capture=>withCapturedLocalHubHostV1") &&
          nativeGate.includes("buildRepositoryCargoArtifacts(join(this.root,") &&
          candidate.includes("stageTrustedBootstrapCandidateCurrent(candidateContext.dataRoot,receipt") &&
          candidate.includes("withLocalHubHostCandidateV1(host,control"),
      ],
      ["process mode", processGate.length > 0],
      ["failed candidate", ordered(processGate, "const retained = trustedBootstrapCurrent(dataRoot)", 'profileId: "missing-profile"')],
      [
        "restart candidate",
        ordered(processGate, 'profileId: "missing-profile"', "const rotated = await materializeTrustedStdioGisRotation") &&
          ordered(processGate, "const rotated = await materializeTrustedStdioGisRotation", "const freshPlan = await validateAndPublishTrustedStdioGisCandidate"),
      ],
    ].find(([, present]) => !present)?.[0];
    if (missingFence) throw new Error(`trusted stdio+GIS source fence is incomplete: ${missingFence}`);
    if (segments[0] === "--native" || segments[0] === "--process" || segments[0] === "--browser" || segments[0] === "--two-author-shell") {
      const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
      const ticketsRoot = resolve(this.repoRoot, ".🧬semio", "🦑️repo", "🎫️tickets");
      const artifactPath = artifactRoot ? resolve(artifactRoot) : "";
      const ticketRelative = artifactPath ? relative(ticketsRoot, artifactPath) : "";
      if (!artifactRoot || !isAbsolute(artifactRoot) || ticketRelative === "" || ticketRelative.startsWith("..") || isAbsolute(ticketRelative))
        throw new Error("trusted stdio+GIS native gate requires an absolute ticket-owned SEMIO_TEST_ARTIFACT_DIR");
      mkdirSync(artifactRoot, { recursive: true, mode: 0o700 });
      const phaseControl=exactCargoCallerControlV1(this.invocation,"hub:trusted-native",0),hubEnv={...process.env};
      const targets=segments[0]==="--two-author-shell"?["-p","semio-hub","--bins","--features","semio-hub/integration-fixtures"]:["--bin","os-hub"];
      if(segments[0]==="--two-author-shell")await buildRepositoryCargoArtifacts(join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml"),["--bins"],this.repoRoot,{producer:"artifact",invocation:this.invocation,control:phaseControl,network:repositoryCargoNetworkV1(this.invocation,this.repoRoot),consume:verifyCapturedCargoArtifactsV1});
      if(segments[0]==="--two-author-shell")await runRepositoryExactCargoLaws({control:phaseControl,preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:buildBudgetMs(),listMilliseconds:60000,lawMilliseconds:120000},cwd:this.root,env:hubEnv,nativeEnv:{RUST_MIN_STACK:"268435456"},groups:[{package:"semio-hub-boot-laws",target:{kind:"lib"},cargoArgs:["--features","integration-fixtures"],laws:["check_in_process_fixture_emits_verified_gis_ledger_and_catalog"]}],progress(event){}});
      await buildRepositoryCargoArtifacts(join(this.root,"Cargo.toml"),targets,this.repoRoot,{producer:"artifact",invocation:this.invocation,control:phaseControl,network:repositoryCargoNetworkV1(this.invocation,this.repoRoot),consume:capture=>withCapturedLocalHubHostV1(this.invocation,phaseControl,capture,async(host,context)=>{
      const {dataRoot,binaryPath:binary}=context;
      return withMaterializedTrustedCatalogBundleV1(host,context.control,trustedBootstrapSelectPackages(TRUSTED_BOOTSTRAP_LINKED_PACKAGES),async(receipt,actualPhase)=>{
      const initialPlan = await validateAndPublishTrustedStdioGisCandidate(host,actualPhase,receipt);
      if (segments[0] === "--browser" || segments[0] === "--two-author-shell") await proveTrustedGisClosedActorV1(this.repoRoot, dataRoot, receipt, artifactPath);
      if (segments[0] === "--two-author-shell") {
        const current = trustedBootstrapCurrent(dataRoot);
        if (!current) throw new Error("two-author Shell composition has no prepared verified current");
        initialPlan.receipt = "";
        await withRepositoryPublishedCargoExecutableV1(join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml"),process.platform==="win32"?"semio-os-mcp.exe":"semio-os-mcp",this.repoRoot,readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode,this.invocation,actualPhase,(mcp,mcpPhase)=>proveGisMapTwoAuthorShellProcess(host,mcpPhase,{artifactRoot:artifactPath,dataRoot,receipt,current,binaryPath:binary,mcpBinaryPath:mcp.path}));
        return;
      }
      if (segments[0] === "--process") {
        const retained = trustedBootstrapCurrent(dataRoot);
        if (!retained) throw new Error("trusted stdio+GIS process gate has no retained current generation");
        let rejected = false;
        try {
          await validateAndPublishTrustedStdioGisCandidate(host,actualPhase,{...receipt,profileId:"missing-profile"});
        } catch {
          rejected = true;
        }
        if (!rejected || JSON.stringify(trustedBootstrapCurrent(dataRoot)) !== JSON.stringify(retained)) throw new Error("failed trusted stdio+GIS candidate changed the retained current generation");
        const rotated = await materializeTrustedStdioGisRotation(this.invocation, actualPhase, this.repoRoot, dataRoot, retained);
        const freshPlan=await validateAndPublishTrustedStdioGisCandidate(host,actualPhase,rotated,initialPlan);
        const published = trustedBootstrapCurrent(dataRoot);
        if (
          !published || published.profileId !== rotated.profileId || published.generationId !== rotated.generationId || published.bundleSha256 !== rotated.bundleSha256 || published.bundlePath !== rotated.bundlePath ||
          published.publicationRevision !== (BigInt(retained.publicationRevision) + 1n).toString() || published.currentSha256 === retained.currentSha256 ||
          initialPlan.catalog.generationId !== retained.generationId ||
          freshPlan.catalog.generationId !== rotated.generationId ||
          retained.generationId === rotated.generationId
        )
          throw new Error("trusted stdio+GIS process rotation did not retain exact distinct plan generations");
        console.log("trusted-stdio-gis-bundle-process-check: failed candidate preserved current; a real next-generation candidate denied the old authenticated plan before issuing the fresh exact GIS Map plan");
      }
      console.log(`trusted-stdio-gis-bundle-native-receipt: ${JSON.stringify(receipt)}`);
      console.log("trusted-stdio-gis-bundle-check: actual isolated stdio+GIS generation loaded by a candidate hub and published current after readiness; client execution remains unclaimed");
      });
      })});
      return;
    }
    console.log("trusted-stdio-gis-bundle-check: source+neutral exact closure passed; native materialization, candidate hub, and current pointer remain unclaimed");
  }
}
