import { type ColdDocumentPairFrontier, type ColdPairIngressStatus, coldDocumentPairFrontierEquals } from "@semio-tech/framework-actor/cold-pair";
import type { ArtifactActorConfig } from "../../../../🟦️";
import type { WireArtifactBootstrap } from "@semio-tech/framework-replication";
import { type ActorInstanceLifetime, actorInstanceLifetimeEquals } from "../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts";
import type { BrowserActorChildValue } from "@semio-tech/framework-plugin/browser-bundle/child";
import type { DocumentExecutionTargetLease } from "../🪪️execution-lease/🟦️.ts";
import { type ArtifactState, artifacts } from "../🧾️document-state/🟦️.ts";
import { executionTargetHex } from "../📥️execution-target-read/🟦️.ts";
import { equalByteArrays, equalFrontiers } from "../📮️hub-outbox/🟦️.ts";
import { documentBrowserActorLease } from "../🎭️actor-lifecycle/🟦️.ts";

const COLD_DOCUMENT_PAIR_PAGE_BYTES = 64 * 1024;
const COLD_DOCUMENT_PAIR_MAXIMUM_PAGES = 64;
const COLD_DOCUMENT_PAIR_MAXIMUM_BYTES = 4 * 1024 * 1024;

export const verifiedColdDocumentPairMintToken = Symbol("semio.os.verified-cold-document-pair.mint/v1");
let verifiedColdDocumentPairGeneration = 0n;

/** 🧊️ This module owns the `verifiedColdPair` slot of the artifact state. */
declare module "../🧾️document-state/🟦️.ts" {
  interface ArtifactOwnerSlots { verifiedColdPair: VerifiedColdDocumentPair }
}

export class VerifiedColdDocumentPair {
  readonly transferGeneration: bigint;
  /** 🧊️ Set once the guest applied this pair: from then on the document frontier advances with live `Commands`/`Ack`
   * frames past the baseline, so only the pair's ownership stays pinned, never its frontier. */
  private applied = false;
  readonly pageCount: number;
  readonly frontier: ColdDocumentPairFrontier;
  private readonly runtimeKey: string;
  private readonly config: ArtifactActorConfig;
  private socket: WebSocket | null;
  private linkSuspended = false;
  private readonly clientInstanceId: string;
  private readonly lease: DocumentExecutionTargetLease;
  private readonly publishedPack: Uint8Array;
  private readonly publishedSpr: Uint8Array;
  private readonly descriptorSha256: Uint8Array;
  private readonly packSha256: Uint8Array;
  private readonly sprSha256: Uint8Array;
  private readonly aggregateSha256: Uint8Array;
  private pack: Uint8Array | null;
  private spr: Uint8Array | null;

  constructor(
    token: symbol,
    private readonly state: ArtifactState,
    lease: DocumentExecutionTargetLease,
    bootstrap: WireArtifactBootstrap,
    pair: Readonly<{ pack: Uint8Array; spr: Uint8Array }>,
    published: Readonly<{ pack: Uint8Array; spr: Uint8Array }>,
  ) {
    if (token !== verifiedColdDocumentPairMintToken || verifiedColdDocumentPairGeneration === 0xffffffffffffffffn) throw new Error("cold document pair: private owner");
    const total = pair.pack.byteLength + pair.spr.byteLength;
    if (pair.pack.byteLength !== bootstrap.pack_length || pair.spr.byteLength !== bootstrap.spr_length || total < 2 || total > COLD_DOCUMENT_PAIR_MAXIMUM_BYTES || Math.ceil(total / COLD_DOCUMENT_PAIR_PAGE_BYTES) > COLD_DOCUMENT_PAIR_MAXIMUM_PAGES)
      throw new Error("cold document pair: invalid capacity");
    const fields = lease.fields(),
      checkpoint = fields.checkpoint;
    if (
      fields.browserActor.kind !== "closed-browser-actor" ||
      !checkpoint ||
      checkpoint.descriptorDigestV1 !== fields.descriptorDigestV1 ||
      checkpoint.aggregateSha256 !== executionTargetHex(new Uint8Array(bootstrap.aggregate_hash)) ||
      fields.descriptorDigestV1 !== executionTargetHex(new Uint8Array(bootstrap.descriptor_hash))
    )
      throw new Error("cold document pair: invalid lease");
    this.transferGeneration = ++verifiedColdDocumentPairGeneration;
    this.pageCount = Math.ceil(total / COLD_DOCUMENT_PAIR_PAGE_BYTES);
    this.frontier = Object.freeze({
      documentId: bootstrap.baseline_frontier.document_id,
      headEditOrdinal: BigInt(bootstrap.baseline_frontier.head_edit_ordinal),
      headEditId: bootstrap.baseline_frontier.head_edit_id,
      lastCommitSeq: BigInt(bootstrap.baseline_frontier.last_commit_seq),
      chainSha256: Uint8Array.from(bootstrap.baseline_frontier.chain_hash),
    });
    this.runtimeKey = state.runtimeKey;
    this.config = state.config;
    this.socket = state.socket;
    this.clientInstanceId = state.openClientInstanceId;
    this.lease = lease;
    this.publishedPack = published.pack;
    this.publishedSpr = published.spr;
    this.descriptorSha256 = Uint8Array.from(bootstrap.descriptor_hash);
    this.packSha256 = Uint8Array.from(bootstrap.pack_hash);
    this.sprSha256 = Uint8Array.from(bootstrap.spr_hash);
    this.aggregateSha256 = Uint8Array.from(bootstrap.aggregate_hash);
    this.pack = Uint8Array.from(pair.pack);
    this.spr = Uint8Array.from(pair.spr);
    Object.freeze(this.frontier);
  }

  assertCurrent(): void {
    const pack = this.pack,
      spr = this.spr;
    if (
      !pack ||
      !spr ||
      this.state.closed ||
      this.state.docAbort.signal.aborted ||
      this.state.verifiedColdPair !== this ||
      this.state.config !== this.config ||
      this.state.runtimeKey !== this.runtimeKey ||
      artifacts.get(this.runtimeKey) !== this.state ||
      (!this.linkSuspended && this.state.socket !== this.socket) ||
      this.state.openClientInstanceId !== this.clientInstanceId ||
      this.state.executionTargetLease !== this.lease ||
      !this.lease.live ||
      this.state.currentPack !== this.publishedPack ||
      this.state.currentSpr !== this.publishedSpr ||
      !this.state.frontier ||
      (!this.applied && !equalFrontiers(this.state.frontier, {
        document_id: this.frontier.documentId,
        head_edit_ordinal: Number(this.frontier.headEditOrdinal),
        head_edit_id: this.frontier.headEditId,
        last_commit_seq: Number(this.frontier.lastCommitSeq),
        chain_hash: Array.from(this.frontier.chainSha256),
      }))
    )
      throw new Error("cold document pair: stale owner");
    documentBrowserActorLease(this.state);
  }

  page(lifetime: ActorInstanceLifetime, pageIndex: number): BrowserActorChildValue {
    this.assertCurrent();
    const pack = this.pack!,
      spr = this.spr!;
    if (!Number.isInteger(pageIndex) || pageIndex < 0 || pageIndex >= this.pageCount) throw new Error("cold document pair: invalid cursor");
    const start = pageIndex * COLD_DOCUMENT_PAIR_PAGE_BYTES;
    const end = Math.min(start + COLD_DOCUMENT_PAIR_PAGE_BYTES, pack.byteLength + spr.byteLength);
    let bytes: Uint8Array;
    if (end <= pack.byteLength) bytes = pack.slice(start, end);
    else if (start >= pack.byteLength) bytes = spr.slice(start - pack.byteLength, end - pack.byteLength);
    else {
      bytes = new Uint8Array(end - start);
      bytes.set(pack.subarray(start), 0);
      bytes.set(spr.subarray(0, end - pack.byteLength), pack.byteLength - start);
    }
    return {
      header: {
        lifetime: { ...lifetime },
        transferGeneration: this.transferGeneration,
        descriptorSha256: Array.from(this.descriptorSha256),
        baselineFrontier: {
          documentId: this.frontier.documentId,
          headEditOrdinal: this.frontier.headEditOrdinal,
          headEditId: this.frontier.headEditId,
          lastCommitSeq: this.frontier.lastCommitSeq,
          chainSha256: Array.from(this.frontier.chainSha256),
        },
        packSha256: Array.from(this.packSha256),
        sprSha256: Array.from(this.sprSha256),
        aggregateSha256: Array.from(this.aggregateSha256),
        packLength: BigInt(pack.byteLength),
        sprLength: BigInt(spr.byteLength),
        pageCount: this.pageCount,
      },
      pageIndex,
      bytes,
    };
  }

  assertApplied(status: ColdPairIngressStatus, lifetime: ActorInstanceLifetime): void {
    this.assertCurrent();
    if (
      status.kind !== "applied" ||
      !actorInstanceLifetimeEquals(status.receipt.lifetime, lifetime) ||
      status.receipt.transferGeneration !== this.transferGeneration ||
      !coldDocumentPairFrontierEquals(status.receipt.baselineFrontier, this.frontier) ||
      !equalByteArrays(status.receipt.aggregateSha256, this.aggregateSha256)
    )
      throw new Error("cold document pair: invalid applied receipt");
    this.applied = true;
  }

  /** 🔌️ Keeps an APPLIED pair owned across a short link loss: the frontier already advances with live frames, so only
   * the socket identity is released until {@link resumeLink} binds the next socket. An unapplied pair never suspends. */
  suspendLink(): boolean {
    if (!this.applied || this.pack === null) return false;
    this.linkSuspended = true;
    this.socket = null;
    return true;
  }

  /** 🔌️ Binds the socket whose `Session` re-admitted the same hub actor after {@link suspendLink}. */
  resumeLink(socket: WebSocket): void {
    if (!this.linkSuspended) throw new Error("cold document pair: link not suspended");
    this.linkSuspended = false;
    this.socket = socket;
  }

  drop(): void {
    this.pack?.fill(0);
    this.spr?.fill(0);
    this.pack = null;
    this.spr = null;
  }
}
