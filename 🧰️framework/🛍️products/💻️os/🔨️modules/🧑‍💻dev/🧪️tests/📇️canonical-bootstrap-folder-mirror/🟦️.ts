/** 🧩️ Semantic canonical bootstrap verification owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { createHash } from "node:crypto";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import {
  BundleScript,
  ScriptRouter,
  buildBudgetMs,
  daemonBudgetOpts,
  describeDevPortOccupant,
  devServerUrl,
  getWorkspaceRoot,
  getRepoMetaDir,
  isDevPortInUse,
  loadFrameworkOsPlaygroundCatalog,
  wgpuDevPlayUrl,
  runBundleScriptMain,
  runCmd,
  runCmdStatus,
  runBunxStatus,
  runNodeBinStatus,
  runProbe,
  runVitest,
  spawnDaemon,
  type SpawnDaemonHandle,
  runViteBunxDev,
  frameworkOsPlaygroundDefaultPort,
  frameworkOsLockedPrefsEnv,
  resolveTestLevel,
  atTestLevel,
  cargoProfileDir,
  selectComponentWasmProfile,
  semioBuildMode,
  semioShipEnv,
} from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

import { decodeDocumentPackBytes, decodePackValue, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, encodeDocumentArchiveBytes, encodePackValue, packValueToExactJson } from "@semio-tech/framework-os";

import {
  CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BYTES,
  PLUGIN_SOURCE_WATCH_PATH,
  backboneDbHandleFor,
  descriptorRouteDecision,
  publishCanonicalBootstrapFolderMirror,
  readBackbonePayload,
  reserveCanonicalBootstrapFolderMirror,
  retireCanonicalBootstrapFolderMirror,
  scanBuiltPluginModules,
  stageCanonicalBootstrapFolderMirror,
  writeBackbonePayload,
  type CanonicalBootstrapFolderMirrorReserveV1,
} from "../../🔌️vite-plugins/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { devContract } from "../../🧬️schema/🛂️validation/🟦️.ts";



//#endregion 🔖️Bench

//#region 🔖️CanonicalBootstrapFolderMirror
type CanonicalBootstrapFolderMirrorCorpusV1 = {
  readonly schema: "semio.backbone.canonical-bootstrap-folder-mirror-corpus/v1";
  readonly documentId: string;
  readonly artifactSchema: string;
  readonly descriptorDigestV1: string;
  readonly baselineFrontier: CanonicalBootstrapFolderMirrorReserveV1["baselineFrontier"];
  readonly pairs: readonly { readonly name: "a" | "b"; readonly packHex: string; readonly sprHex: string; readonly aggregateSha256: string }[];
  readonly hostile: readonly { readonly name: string; readonly expected: "invalid" | "conflict" | "too-large" }[];
};

type CanonicalPairFrontierCaseV1 = {
  readonly id: string;
  readonly documentId: string;
  readonly headEditOrdinal: number;
  readonly headEditId: string;
  readonly lastCommitSeq: number;
  readonly chainHash: string;
  readonly accepted: boolean;
};

//#endregion 🧬️Contracts

function canonicalBootstrapFolderMirrorFixtureRoot(repoRoot: string): string {
  return join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/📇️folder/📣️canonical-bootstrap-folder-mirror-v1");
}

function canonicalBootstrapFolderMirrorReserve(corpus: CanonicalBootstrapFolderMirrorCorpusV1, aggregateSha256: string, documentId = corpus.documentId): CanonicalBootstrapFolderMirrorReserveV1 {
  return {
    schema: "semio.backbone.canonical-bootstrap-folder-mirror-reserve/v1",
    artifactSchema: corpus.artifactSchema,
    descriptorDigestV1: corpus.descriptorDigestV1,
    aggregateSha256,
    baselineFrontier: { ...corpus.baselineFrontier, documentId },
  };
}

async function expectCanonicalBootstrapFolderMirrorFailure(operation: Promise<unknown>, expected: string): Promise<void> {
  try {
    await operation;
  } catch (error) {
    if (typeof error === "object" && error !== null && "code" in error && (error as { code?: unknown }).code === expected) return;
    throw error;
  }
  throw new Error(`canonical bootstrap folder mirror expected ${expected}`);
}

class CanonicalBootstrapFolderMirrorCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (!['source', 'process'].includes(phase) || segments.length !== 1) throw new Error("canonical-bootstrap-folder-mirror-check expects source|process");
    const fixtureRoot = canonicalBootstrapFolderMirrorFixtureRoot(this.repoRoot);
    const corpus = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as CanonicalBootstrapFolderMirrorCorpusV1;
    const canonicalPairRoot = join(this.repoRoot, "🌎️hub", "🛰️lag-rebootstrap", "🧫️fixtures", "🪢️canonical-pair");
    const canonicalPairCorpus = JSON.parse(readFileSync(join(canonicalPairRoot, "🔣️.json"), "utf8")) as { selection: { documentId: string; baseline: Record<string, unknown> }; frontierCases: readonly CanonicalPairFrontierCaseV1[] };
    const lagModule = JSON.parse(readFileSync(join(this.repoRoot, "🌎️hub", "🛰️lag-rebootstrap", "🧬️schema", "🔣️.json"), "utf8")) as { $id: string };
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true, allErrors: true });
    const validate = await devContract("CanonicalBootstrapFolderMirrorCorpusV1");
    if (!validate(corpus)) throw new Error("canonical bootstrap folder mirror corpus rejected by its owned contract");
    const lagAjv = new Ajv({ strict: true, allErrors: true });
    lagAjv.addSchema(lagModule);
    const validateCanonicalPairSelection = lagAjv.getSchema(`${lagModule.$id}#/$defs/CanonicalCheckpointPairSelectionV1`)!;
    const validateCanonicalPairBaseline = lagAjv.getSchema(`${lagModule.$id}#/$defs/CanonicalCheckpointPairBaselineV1`)!;
    if (!validateCanonicalPairSelection(canonicalPairCorpus.selection)) throw new Error(`canonical pair selection: ${JSON.stringify(validateCanonicalPairSelection.errors)}`);
    for (const { id, accepted, ...frontier } of canonicalPairCorpus.frontierCases)
      if (!validateCanonicalPairBaseline(frontier)) throw new Error(`canonical pair frontier ${id}: ${JSON.stringify(validateCanonicalPairBaseline.errors)}`);
    const own = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts"), "utf8");
    const worker = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"), "utf8");
    const markers = [
      "canonical_bootstrap_owner",
      "canonical_bootstrap_archive_stage",
      "reserveCanonicalBootstrapFolderMirror",
      "stageCanonicalBootstrapFolderMirror",
      "publishCanonicalBootstrapFolderMirror",
      "retireCanonicalBootstrapFolderMirror",
      "await retireCurrentFolderCanonicalBootstrapMirror(state)",
      "state.canonicalFolderMirror = mirror",
    ];
    if (!markers.slice(0, 6).every((marker) => own.includes(marker)) || !markers.slice(6).every((marker) => worker.includes(marker))) throw new Error("canonical bootstrap folder mirror source boundary drift");
    if (phase === "source") {
      console.log(`canonical-bootstrap-folder-mirror: phase=source ajv=1 markers=${markers.length}`);
      return;
    }
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifactRoot) throw new Error("canonical bootstrap folder mirror process requires SEMIO_TEST_ARTIFACT_DIR");
    mkdirSync(resolve(artifactRoot), { recursive: true });
    const root = mkdtempSync(join(resolve(artifactRoot), "folder-mirror-"));
    const uri = `folder://${root}`;
    const pair = (name: "a" | "b") => {
      const row = corpus.pairs.find((candidate) => candidate.name === name);
      if (!row) throw new Error(`canonical bootstrap folder mirror fixture pair ${name} missing`);
      const pack = Buffer.from(row.packHex, "hex"), spr = Buffer.from(row.sprHex, "hex");
      return { row, pack, spr, bundle: encodeDocumentArchiveBytes({ parent_pack: Array.from(pack), parent_spr: Array.from(spr), members: [] }) };
    };
    const a = pair("a"), b = pair("b");
    for (const item of [a, b]) {
      const firstParty = createHash("sha256").update(item.pack).update(item.spr).digest("hex");
      const thirdParty = Buffer.from(await crypto.subtle.digest("SHA-256", Buffer.concat([item.pack, item.spr]))).toString("hex");
      if (firstParty !== item.row.aggregateSha256 || thirdParty !== firstParty) throw new Error(`canonical bootstrap folder mirror ${item.row.name} aggregate oracle mismatch`);
    }

    for (const row of canonicalPairCorpus.frontierCases) {
      const request: CanonicalBootstrapFolderMirrorReserveV1 = {
        schema: "semio.backbone.canonical-bootstrap-folder-mirror-reserve/v1",
        artifactSchema: corpus.artifactSchema,
        descriptorDigestV1: corpus.descriptorDigestV1,
        aggregateSha256: a.row.aggregateSha256,
        baselineFrontier: { documentId: row.documentId, headEditOrdinal: row.headEditOrdinal, headEditId: row.headEditId, lastCommitSeq: row.lastCommitSeq, chainSha256: row.chainHash },
      };
      const operation = reserveCanonicalBootstrapFolderMirror(uri, canonicalPairCorpus.selection.documentId, request);
      if (row.accepted) {
        const owner = await operation;
        await retireCanonicalBootstrapFolderMirror(uri, canonicalPairCorpus.selection.documentId, owner);
      } else {
        await expectCanonicalBootstrapFolderMirrorFailure(operation, "invalid");
      }
    }

    const genericId = `${corpus.documentId}-generic`;
    await writeBackbonePayload(uri, genericId, corpus.artifactSchema, a.bundle);
    const genericRead = await readBackbonePayload(uri, genericId);
    if (!genericRead || !Buffer.from(genericRead).equals(a.bundle)) throw new Error("canonical bootstrap folder mirror changed pure-folder PUT/GET");

    const ownerA = await reserveCanonicalBootstrapFolderMirror(uri, corpus.documentId, canonicalBootstrapFolderMirrorReserve(corpus, a.row.aggregateSha256));
    await stageCanonicalBootstrapFolderMirror(uri, corpus.documentId, ownerA, a.bundle);
    await publishCanonicalBootstrapFolderMirror(uri, corpus.documentId, ownerA);
    const visibleA = await readBackbonePayload(uri, corpus.documentId);
    if (!visibleA || !Buffer.from(visibleA).equals(a.bundle)) throw new Error("canonical bootstrap folder mirror did not publish A");
    const ownerB = await reserveCanonicalBootstrapFolderMirror(uri, corpus.documentId, canonicalBootstrapFolderMirrorReserve(corpus, b.row.aggregateSha256));
    if ((await readBackbonePayload(uri, corpus.documentId)) !== null) throw new Error("canonical bootstrap folder mirror exposed published A during B reservation");
    await expectCanonicalBootstrapFolderMirrorFailure(stageCanonicalBootstrapFolderMirror(uri, corpus.documentId, ownerA, a.bundle), "conflict");
    await expectCanonicalBootstrapFolderMirrorFailure(publishCanonicalBootstrapFolderMirror(uri, corpus.documentId, ownerA), "conflict");
    await expectCanonicalBootstrapFolderMirrorFailure(retireCanonicalBootstrapFolderMirror(uri, corpus.documentId, ownerA), "conflict");
    await stageCanonicalBootstrapFolderMirror(uri, corpus.documentId, ownerB, b.bundle);
    await publishCanonicalBootstrapFolderMirror(uri, corpus.documentId, ownerB);
    const visibleB = await readBackbonePayload(uri, corpus.documentId);
    if (!visibleB || !Buffer.from(visibleB).equals(b.bundle)) throw new Error("canonical bootstrap folder mirror did not publish B");

    const raceId = `${corpus.documentId}-race`;
    const raceA = await reserveCanonicalBootstrapFolderMirror(uri, raceId, canonicalBootstrapFolderMirrorReserve(corpus, a.row.aggregateSha256, raceId));
    await stageCanonicalBootstrapFolderMirror(uri, raceId, raceA, a.bundle);
    const raceB = await reserveCanonicalBootstrapFolderMirror(uri, raceId, canonicalBootstrapFolderMirrorReserve(corpus, b.row.aggregateSha256, raceId));
    await expectCanonicalBootstrapFolderMirrorFailure(publishCanonicalBootstrapFolderMirror(uri, raceId, raceA), "conflict");
    if ((await readBackbonePayload(uri, raceId)) !== null) throw new Error("canonical bootstrap folder mirror exposed late A during B reservation");
    await stageCanonicalBootstrapFolderMirror(uri, raceId, raceB, b.bundle);
    await publishCanonicalBootstrapFolderMirror(uri, raceId, raceB);
    const visibleRaceB = await readBackbonePayload(uri, raceId);
    if (!visibleRaceB || !Buffer.from(visibleRaceB).equals(b.bundle)) throw new Error("canonical bootstrap folder mirror did not publish race B");
    await expectCanonicalBootstrapFolderMirrorFailure(stageCanonicalBootstrapFolderMirror(uri, corpus.documentId, { ...ownerA, capability: "f".repeat(64) }, a.bundle), "conflict");
    await expectCanonicalBootstrapFolderMirrorFailure(stageCanonicalBootstrapFolderMirror(uri, corpus.documentId, { ...ownerB, epoch: ownerB.epoch + 1 }, b.bundle), "conflict");

    const mismatchId = `${corpus.documentId}-mismatch`;
    const mismatch = await reserveCanonicalBootstrapFolderMirror(uri, mismatchId, canonicalBootstrapFolderMirrorReserve(corpus, a.row.aggregateSha256, mismatchId));
    await expectCanonicalBootstrapFolderMirrorFailure(stageCanonicalBootstrapFolderMirror(uri, mismatchId, mismatch, b.bundle), "conflict");
    await expectCanonicalBootstrapFolderMirrorFailure(stageCanonicalBootstrapFolderMirror(uri, mismatchId, mismatch, Uint8Array.of(0x80)), "invalid");
    await expectCanonicalBootstrapFolderMirrorFailure(stageCanonicalBootstrapFolderMirror(uri, mismatchId, mismatch, new Uint8Array(CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BYTES + 11)), "too-large");

    await expectCanonicalBootstrapFolderMirrorFailure(writeBackbonePayload(uri, corpus.documentId, corpus.artifactSchema, a.bundle), "conflict");
    await retireCanonicalBootstrapFolderMirror(uri, corpus.documentId, ownerB);
    if ((await readBackbonePayload(uri, corpus.documentId)) !== null) throw new Error("canonical bootstrap folder mirror retirement remained visible");
    await writeBackbonePayload(uri, corpus.documentId, corpus.artifactSchema, a.bundle);
    const mixedRead = await readBackbonePayload(uri, corpus.documentId);
    if (!mixedRead || !Buffer.from(mixedRead).equals(a.bundle)) throw new Error("canonical bootstrap folder mirror did not admit explicit retired-to-generic transition");
    const successor = await reserveCanonicalBootstrapFolderMirror(uri, corpus.documentId, canonicalBootstrapFolderMirrorReserve(corpus, b.row.aggregateSha256));
    if (successor.epoch !== ownerB.epoch + 1 || (await readBackbonePayload(uri, corpus.documentId)) !== null) throw new Error("canonical bootstrap folder mirror generic transition reset or bypassed the server epoch");
    await retireCanonicalBootstrapFolderMirror(uri, corpus.documentId, successor);
    console.log(`canonical-bootstrap-folder-mirror: phase=process ajv=2 sqlite=1 sha256=2 frontiers=${canonicalPairCorpus.frontierCases.length} stale=7 hostile=${corpus.hostile.length} mixed=1 monotonic-epoch=1`);
  }
}

export { CanonicalBootstrapFolderMirrorCheckScript, CanonicalBootstrapFolderMirrorCorpusV1, CanonicalPairFrontierCaseV1, canonicalBootstrapFolderMirrorFixtureRoot, canonicalBootstrapFolderMirrorReserve, expectCanonicalBootstrapFolderMirrorFailure };
