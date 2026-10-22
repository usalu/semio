/** 🏗️ The catalog root script of the hub stdio package with its fresh root and cargo environment helpers. */
import { withRequestedIssuedResourceControlV1, BundleScript } from "@semio-tech/process/routing";
import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync } from "node:fs";
import { isAbsolute, join, relative, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { devToolingEnv, resolveWorkspaceBin, cargoTargetDirectory, cargoBuildDirectory } from "@semio-tech/process/execution";
import { scriptCompilerAdmissionControlV1 } from "@semio-tech/process/compiler-capacity";
import { builtPluginModulesRootIn } from "@semio-tech/framework-os-dev/activation";
import { COMPONENT_RECEIPT_FILE, SHIPPED_COMPONENT_MAX_BYTES, SHIPPED_COMPONENT_PROFILE, componentDeliverableV1, componentReceiptFindingsV1, readComponentReceiptV1 } from "@semio-tech/framework-plugin/describe/component-receipt";
import { CATALOG_DEADLINE_MS, COMPONENT_MANIFEST, COMPONENT_PACKAGE, DESCRIPTOR_PACK, IO_CHUNK_BYTES, PLUGIN_ID, WASM_OUT } from "../🧮️component-constants/🟦️.ts";
import { assertControlled, catalogControlV1, componentPackageId, copyCatalogArtifact, pathIsWithin, publishCatalogCommitMarker, runControlled, verifyIndependentOracles, webSha256 } from "../🧾️catalog-artifact-io/🟦️.ts";

export function requireEmptyFreshRoot(repoRoot: string, value: string): string {
  if (!isAbsolute(value)) throw new Error("catalog-root --build-root must be absolute");
  const root = resolve(value);
  const info = lstatSync(root);
  if (info.isSymbolicLink() || !info.isDirectory()) throw new Error("catalog-root build root must be a regular non-symlink directory");
  if (readdirSync(root).length !== 0) throw new Error("catalog-root build root must be empty");
  const exact = realpathSync(root);
  const ambientTarget = cargoTargetDirectory(repoRoot);
  const ambientBuild = cargoBuildDirectory(repoRoot);
  const developmentCaches=(["prototype","release"]as const).map(mode=>builtPluginModulesRootIn(repoRoot,mode));
  if (pathIsWithin(ambientTarget, exact)) throw new Error("catalog-root refuses the ambient shared target");
  if (pathIsWithin(ambientBuild, exact)) throw new Error("catalog-root refuses the ambient shared build directory");
  if(developmentCaches.some(cache=>pathIsWithin(cache,exact))) throw new Error("catalog-root refuses the development cache");
  if (exact === resolve(repoRoot)) throw new Error("catalog-root requires a dedicated fresh directory");
  return exact;
}

/** 🏗️ Keeps staged Cargo deliverables and intermediate compiler files in one private owner root. */
export function catalogCargoEnvironment(target: string, inherited: NodeJS.ProcessEnv = process.env): NodeJS.ProcessEnv {
  return devToolingEnv({ ...inherited, CARGO_TARGET_DIR: target, CARGO_BUILD_BUILD_DIR: join(target, "build"), CARGO_INCREMENTAL: "0" });
}

/** 🌳 Stages and verifies the one strict stdio row in an empty caller-owned root: the shipped `release` component, the
 * committed semantic descriptor and the `ComponentReceiptV1` binding the two — the release pass's own bytes, never a rebuild. */
export class CatalogRootScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { CATALOG_COMMIT_MARKER_FILENAME } = await import("@semio-tech/plugin-registry/catalog-verification");
    const { auditPluginCatalogSources } = await import("@semio-tech/plugin-registry/catalog-verification/source-audit");
    const { createFreshCatalogBuildVerifier, createFreshCatalogCommitMarker } = await import("@semio-tech/plugin-registry/catalog-verification/fresh-build-verifier");
    const { componentStageTargetV1 } = await import("@semio-tech/framework-plugin/describe/component-stage");
    const option = (name: string): string | undefined => {
      const index = segments.indexOf(name);
      return index < 0 ? undefined : segments[index + 1];
    };
    const suppliedRoot = option("--build-root") ?? process.env.SEMIO_CATALOG_FRESH_BUILD_ROOT;
    const cancelFile = option("--cancel-file");
    if (!suppliedRoot) throw new Error("usage: catalog-root --build-root <absolute empty fresh root> [--cancel-file <path>]");
    const buildRoot = requireEmptyFreshRoot(this.repoRoot, suppliedRoot);
    const admission=scriptCompilerAdmissionControlV1(this.invocation,"stdio:catalog-root",CATALOG_DEADLINE_MS),cancel=new AbortController();
    const poll=()=>{if(cancelFile!==undefined&&existsSync(resolve(cancelFile)))cancel.abort(Error("stdio catalog-root cancelled"));};poll();
    const timer=cancelFile===undefined?undefined:setInterval(poll,100);
    try{return await withRequestedIssuedResourceControlV1(admission,0,async phase=>{
    const execution={invocation:this.invocation,phase},control=catalogControlV1(execution);
    const workRoot = join(buildRoot, `.stdio-work-${process.pid}`);
    const stageRoot = join(buildRoot, `.stdio-stage-${process.pid}`);
    const rowRoot = join(buildRoot, PLUGIN_ID);
    let rowPublished = false;
    try {
      mkdirSync(workRoot, { recursive: true });
      mkdirSync(stageRoot, { recursive: true });
      const packageId = componentPackageId(join(this.root, COMPONENT_MANIFEST));
      const target = componentStageTargetV1(this.repoRoot, join(this.root, COMPONENT_MANIFEST));
      const deliverable = componentDeliverableV1(target.crateRoot, target.packageName, SHIPPED_COMPONENT_PROFILE);
      const fix = "run repo run workspace:release, then repo run workspace:components describe stdio";
      if (!existsSync(deliverable.wasm) || !existsSync(deliverable.receipt)) throw new Error(`stdio catalog-root stages the shipped component and its receipt from ${relative(this.repoRoot, deliverable.directory)}; ${fix}`);
      const committedPack = join(target.ownerRoot, DESCRIPTOR_PACK);
      const findings = componentReceiptFindingsV1(readComponentReceiptV1(deliverable.receipt), { pluginId: PLUGIN_ID, packageId, profile: SHIPPED_COMPONENT_PROFILE }, readFileSync(deliverable.wasm), readFileSync(committedPack));
      if (findings.length > 0) throw new Error(`stdio shipped component refused: ${findings.join("; ")}; ${fix}`);
      assertControlled(control);
      const jco = resolveWorkspaceBin("@bytecodealliance/jco", this.repoRoot);
      if (!jco) throw new Error("missing @bytecodealliance/jco workspace binary; run bun install");
      const witPath = join(workRoot, "stdio.wit");
      await runControlled(execution,"node", [jco, "wit", deliverable.wasm, "--output", witPath], this.repoRoot, devToolingEnv(), control);
      const wit = readFileSync(witPath, "utf8");
      if (!/world\s+actor\s*\{/.test(wit) || !["reactor", "jobs", "checkpoint", "describe"].every((name) => new RegExp(`export\\s+${name}\\s*;`).test(wit))) {
        throw new Error("wasm-tools WIT oracle rejected the required actor exports");
      }
      const stagedRaw = join(stageRoot, "raw", WASM_OUT);
      const stagedDescriptor = join(stageRoot, "descriptor", DESCRIPTOR_PACK);
      const stagedReceipt = join(stageRoot, "receipt", COMPONENT_RECEIPT_FILE);
      const copiedRawHash = copyCatalogArtifact(deliverable.wasm, stagedRaw, "raw component", control);
      copyCatalogArtifact(committedPack, stagedDescriptor, "descriptor pack", control);
      copyCatalogArtifact(deliverable.receipt, stagedReceipt, "component receipt", control);
      const oracle = await verifyIndependentOracles(stagedRaw, stagedDescriptor, stagedReceipt);
      if (copiedRawHash !== oracle.rawSha256) throw new Error("bounded-copy hash disagrees with the WebCrypto oracle");
      assertControlled(control);
      renameSync(stageRoot, rowRoot);
      rowPublished = true;
      await runControlled(execution,"bun", ["repo", "run", "@semio-tech/plugin-registry:generate"], this.repoRoot, devToolingEnv(), control);
      const audit = auditPluginCatalogSources(this.repoRoot, { cancelled: control.cancelled });
      const stdioIssues = audit.issues.filter((issue) => issue.pluginId === PLUGIN_ID);
      if (stdioIssues.length > 0) throw new Error(`stdio source audit failed: ${stdioIssues.map((issue) => issue.diagnostic).join("; ")}`);
      const source = audit.sources.find(({ entry }) => entry.pluginId === PLUGIN_ID);
      if (!source) throw new Error("stdio strict descriptor source was not discovered");
      if (source.entry.packageName !== COMPONENT_PACKAGE || source.entry.wasmOut !== WASM_OUT) throw new Error("stdio source identity is not bijective with Cargo component identity");
      await runControlled(execution,"bun", ["repo", "run", "@semio-tech/plugin-registry:check-generated"], this.repoRoot, devToolingEnv(), control);
      if (source.descriptor.packageId !== packageId) throw new Error("stdio descriptor packageId does not match the Cargo component contract");
      publishCatalogCommitMarker(rowRoot, createFreshCatalogCommitMarker(source, buildRoot, { cancelled: control.cancelled }), CATALOG_COMMIT_MARKER_FILENAME);
      const strictReceipt = createFreshCatalogBuildVerifier(this.repoRoot, buildRoot).verify(source.entry, { cancelled: control.cancelled });
      const strictHashes = { pluginId: strictReceipt.pluginId, rawSha256: strictReceipt.rawSha256, descriptorSha256: strictReceipt.descriptorSha256 };
      if (!isDeepStrictEqual(strictHashes, { pluginId: PLUGIN_ID, rawSha256: oracle.rawSha256, descriptorSha256: oracle.descriptorSha256 }) || !isDeepStrictEqual({ ...strictReceipt.receipt }, oracle.receipt)) throw new Error("strict verifier and independent oracle receipts disagree");
      if (await webSha256(strictReceipt.rawBytes) !== oracle.rawSha256 || !Buffer.from(strictReceipt.descriptorBytes).equals(Buffer.from(source.packBytes))) throw new Error("strict verifier did not retain the exact admitted bytes");
      rmSync(workRoot, { recursive: true, force: true });
      console.log(JSON.stringify({ schemaVersion: 2, packageId, wasmOut: WASM_OUT, limits: { artifactBytes: SHIPPED_COMPONENT_MAX_BYTES, ioChunkBytes: IO_CHUNK_BYTES, deadlineMs: CATALOG_DEADLINE_MS }, receipt: strictHashes }));
    } catch (error) {
      if (rowPublished) rmSync(rowRoot, { recursive: true, force: true });
      for (const path of [stageRoot, workRoot]) rmSync(path, { recursive: true, force: true });
      throw error;
    }
    },cancel.signal);}finally{if(timer!==undefined)clearInterval(timer);}
  }
}
