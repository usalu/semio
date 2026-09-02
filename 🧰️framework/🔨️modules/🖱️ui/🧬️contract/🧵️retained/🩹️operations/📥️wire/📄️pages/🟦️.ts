//#region 📄️PrivatePagedInputAuthority
import { OwnedUiResidentPayload, OwnedUiResidentPayloadReader, OwnedUiResidentReaderRetirement, type OwnedUiResidentReaderAdmission, type OwnedUiResidentReaderStep, type OwnedUiResidentPage } from "../../../💾️resident/🟦️.ts";
import type { OwnedUiInstance } from "../../../🏘️instance/🟦️.ts";
import type { NumericIndexGrant } from "../../../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
import type { RetainedUiWireStep } from "../../../📦️wire/🟦️.ts";
import type { ActorInstanceLifetime } from "../../../../../../🎭️actor/🚪️lifetime/🟦️.ts";
import type { ShardActorActivationLease } from "../../../../../../🎭️actor/🧵️shard-client/🟦️.ts";
import { OwnedKernelReturnInputField, OwnedKernelReturnInputFragment, OwnedKernelReturnInputRelease } from "../../../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts";

const MINT = Object.freeze({});
const NO_FAILURE = Object.freeze({});
type Evidence = { fragment: object | null; field: object | null; builder: OwnedUiOperationPayloadBuilder | null; readonly offset: bigint; readonly length: number; release: OwnedKernelReturnInputRelease | null; phase: "constructed" | "available" | "issued" | "source-consuming" | "source-observing" | "source-detached" | "source-settled" | "domain-retired"; token: OwnedUiOperationInputCopied | OwnedUiOperationInputCancelled | null };
const admitted = (grant: NumericIndexGrant, bytes: number): boolean => Number.isSafeInteger(grant.maxItems) && grant.maxItems >= 1 && Number.isSafeInteger(grant.maxBytes) && grant.maxBytes >= bytes;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
type InputToken = OwnedUiOperationInputCopied | OwnedUiOperationInputCancelled;
let cancelledProof: (builder: OwnedUiOperationPayloadBuilder, resident: OwnedUiResidentPayload, fragment: OwnedKernelReturnInputFragment) => OwnedUiOperationInputCancelled;
let copiedProof: (builder: OwnedUiOperationPayloadBuilder, resident: OwnedUiResidentPayload, fragment: OwnedKernelReturnInputFragment) => OwnedUiOperationInputCopied;
let copiedState: (token: unknown) => Evidence | OwnedUiOperationPayloadBuilder | null;
let cancelledState: (token: unknown) => Evidence | OwnedUiOperationPayloadBuilder | null;
let builderBrand: (value: unknown) => value is OwnedUiOperationPayloadBuilder;
function evidenceState(token: unknown): Evidence | null { const value = copiedState(token) ?? cancelledState(token); return value !== null && !builderBrand(value) ? value : null; }
export type OwnedUiOperationPayloadAdmission = { readonly step: RetainedUiWireStep; readonly builder: OwnedUiOperationPayloadBuilder | null };
export type OwnedUiOperationPayloadReadAdmission = OwnedUiResidentReaderAdmission;
export type OwnedUiOperationPayloadReadStep = OwnedUiResidentReaderStep;
function childStep(current: RetainedUiWireStep, grant: NumericIndexGrant): RetainedUiWireStep {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes) return { ...current, kind: "rejected" };
  return current.kind === "ready" || current.kind === "complete" ? { ...current, kind: "pending" } : current;
}
function exact(proof: Evidence, fragment: object, field: object, builder: object, offset: bigint, length: number): boolean {
  return typeof offset === "bigint" && offset >= 0n && offset <= 18446744073709551615n && Number.isSafeInteger(length) && length >= 0 && length <= 4096 && offset + BigInt(length) <= 18446744073709551615n
    && (proof.phase === "available" || proof.phase === "issued" || proof.phase === "source-consuming" || proof.phase === "source-observing") && proof.fragment === fragment && proof.field === field && proof.builder === builder && proof.offset === offset && proof.length === length && OwnedUiOperationPayloadBuilder.matchesField(builder, field);
}
function evidenceRelease(proof: Evidence, token: object, release: unknown): boolean { return proof.token === token && proof.release !== null && proof.release === release && (proof.phase === "issued" || proof.phase === "source-consuming" || proof.phase === "source-observing" || proof.phase === "source-detached" || proof.phase === "source-settled"); }

/** 📄️ Exact native field admission is retained by its resident parent before exposure to a caller. */
export class OwnedUiOperationPayloadBuilder {
  #field: OwnedKernelReturnInputField | null;
  #resident: OwnedUiResidentPayload | null;
  #bound = false;
  #closing = false;
  #phase: "open" | "proof" | "release" | "receipt" | "pages" | "owner" | "closed" = "open";
  #fragment: OwnedKernelReturnInputFragment | null = null;
  #proof: OwnedUiOperationInputCancelled | null = null;
  #release: OwnedKernelReturnInputRelease | null = null;
  #failure: unknown = NO_FAILURE;
  #copyPhase: "idle" | "page" | "page-observe" | "allocate" | "allocate-observe" | "copy" | "write" | "seal" | "seal-observe" | "proof" | "receipt" | "range-observe" | "ready" = "idle";
  #sourceKind: "pending" | "complete" | number | null = null;
  #input: OwnedKernelReturnInputFragment | null = null;
  #copyFragment: OwnedKernelReturnInputFragment | null = null;
  #lastFragment: OwnedKernelReturnInputFragment | null = null;
  #inputOffset = 0;
  #copied = 0n;
  #copyProof: OwnedUiOperationInputCopied | null = null;
  #copyRelease: OwnedKernelReturnInputRelease | null = null;
  #head: OwnedUiResidentPage | null = null;
  #tail: OwnedUiResidentPage | null = null;
  #writer: OwnedUiResidentPage | null = null;
  #written = 0;
  #reader: OwnedUiResidentPayloadReader | OwnedUiResidentReaderRetirement | null = null;
  #readerPhase: "none" | "held" | "detached" | "settled" = "none";
  static { builderBrand = (value): value is OwnedUiOperationPayloadBuilder => value !== null && typeof value === "object" && #field in value; }
  private constructor(mint: object, field: OwnedKernelReturnInputField, resident: OwnedUiResidentPayload) {
    if (mint !== MINT) throw new Error("Invalid paged builder authority"); this.#field = field; this.#resident = resident;
    const installed = resident.installBuilder(this, { maxItems: 1, maxBytes: 64 }); if (installed.kind !== "ready" || installed.bytes !== 64) throw new Error("Paged builder registration refused");
  }
  static matchesField(builder: unknown, field: object): builder is OwnedUiOperationPayloadBuilder {
    return builder !== null && typeof builder === "object" && #field in builder && builder.#field === field && builder.#resident !== null;
  }
  static matchesResident(builder: unknown, resident: OwnedUiResidentPayload): builder is OwnedUiOperationPayloadBuilder { return builder !== null && typeof builder === "object" && #resident in builder && builder.#resident === resident; }
  static hasBrand(value: unknown): value is OwnedUiOperationPayloadBuilder { return builderBrand(value); }
  static activeInput(builder: OwnedUiOperationPayloadBuilder): boolean { return builder.#phase === "open" && (builder.#input !== null || builder.#copyFragment !== null); }
  static cancellationPrepared(builder: OwnedUiOperationPayloadBuilder): boolean { return builder.#closing && builder.#phase === "proof" && builder.#fragment !== null && !builder.#input && !builder.#copyFragment && builder.#sourceKind === null; }
  static prepareInputCancellation(builder: OwnedUiOperationPayloadBuilder, resident: OwnedUiResidentPayload, grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 128)) return step("blocked", "paged-active-input-detach");
    if (!OwnedUiResidentPayload.matchesInputCancellation(resident, builder) || builder.#resident !== resident || builder.#failure !== NO_FAILURE || !OwnedUiOperationPayloadBuilder.activeInput(builder) || builder.#proof || builder.#release || builder.#copyProof || builder.#copyRelease) return step("rejected", "paged-active-input-owner");
    const fragment = builder.#copyFragment ?? builder.#input; if (!fragment || !builder.#field || !OwnedKernelReturnInputFragment.matches(fragment, builder.#field) || builder.#input !== null && builder.#input !== fragment) return step("rejected", "paged-active-input-fragment");
    builder.#closing = true; builder.#fragment = fragment; builder.#input = null; builder.#copyFragment = null; builder.#sourceKind = null; builder.#copyPhase = "idle"; builder.#phase = "proof"; return step("pending", "paged-active-input-detach", 128);
  }
  static evidenceEligible(builder: unknown, resident: OwnedUiResidentPayload): builder is OwnedUiOperationPayloadBuilder { return builderBrand(builder) && builder.#resident === resident && builder.#failure === NO_FAILURE && builder.#bound && builder.#field !== null && (builder.#closing && !builder.#input && !builder.#copyProof || builder.#copyPhase === "proof" && builder.#input === null) && (builder.#copyFragment !== null || builder.#fragment !== null || builder.#field.fragment !== null); }
  static matchesEvidenceConstruction(token: unknown, builder: OwnedUiOperationPayloadBuilder): token is InputToken { return copiedState(token) === builder || cancelledState(token) === builder; }
  static matchesEvidence(token: unknown, builder: OwnedUiOperationPayloadBuilder): token is InputToken { const proof = evidenceState(token); return proof !== null && proof.token === token && proof.builder === builder; }
  static constructEvidence(builder: OwnedUiOperationPayloadBuilder, resident: OwnedUiResidentPayload, grant: NumericIndexGrant): InputToken {
    if (!admitted(grant, 168) || !OwnedUiResidentPayload.matchesEvidencePhase(resident, builder, "constructing") || !OwnedUiOperationPayloadBuilder.evidenceEligible(builder, resident)) throw new Error("Invalid original evidence reservation");
    const fragment = builder.#copyFragment ?? builder.#fragment ?? builder.#field!.fragment; if (!fragment || !OwnedKernelReturnInputFragment.matches(fragment, builder.#field!)) throw new Error("Missing exact evidence fragment");
    return builder.#copyPhase === "proof" ? copiedProof(builder, resident, fragment) : cancelledProof(builder, resident, fragment);
  }
  static finalizeEvidence(token: InputToken, builder: OwnedUiOperationPayloadBuilder, resident: OwnedUiResidentPayload, grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 64)) return step("blocked", "paged-evidence-finalization"); if (!OwnedUiResidentPayload.matchesEvidencePhase(resident, builder, "witness-ready") || !OwnedUiOperationPayloadBuilder.matchesEvidence(token, builder)) return step("rejected", "paged-evidence-finalization"); Object.freeze(token); return step("pending", "paged-evidence-finalization", 64);
  }
  static publishEvidence(token: InputToken, builder: OwnedUiOperationPayloadBuilder, resident: OwnedUiResidentPayload): void { const proof = evidenceState(token); if (!OwnedUiResidentPayload.matchesEvidencePhase(resident, builder, "finalized") || !proof || proof.builder !== builder || proof.token !== token || proof.phase !== "constructed") throw new Error("Invalid evidence publication"); proof.phase = "available"; }
  static evidenceEmpty(token: unknown): boolean { const proof = evidenceState(token); return proof !== null && proof.phase === "domain-retired" && !proof.fragment && !proof.field && !proof.builder && !proof.release && !proof.token; }
  static advanceEvidence(token: InputToken, builder: OwnedUiOperationPayloadBuilder, resident: OwnedUiResidentPayload, grant: NumericIndexGrant): RetainedUiWireStep {
    const proof = evidenceState(token); if (!proof || !OwnedUiResidentPayload.matchesEvidenceRetirement(resident, builder, token) || builder.#failure !== NO_FAILURE || !builder.#field) return step("rejected", "paged-evidence-owner");
    const cancelling = OwnedUiResidentPayload.matchesEvidenceCancellation(resident, builder, token);
    if (!admitted(grant, proof.phase === "available" ? 128 : proof.phase === "source-consuming" && !cancelling ? 1 : 64)) return step("blocked", "paged-evidence-retirement"); const field = builder.#field;
    if (proof.phase === "available") { const fragment = proof.fragment; if (!OwnedKernelReturnInputFragment.matches(fragment, field) || proof.builder !== builder || proof.token !== token) return step("rejected", "paged-evidence-release-owner"); const receipt = fragment.release(token); if (!receipt || !OwnedKernelReturnInputRelease.matches(receipt, fragment, token)) return step("rejected", "paged-evidence-release-refused", 128); proof.release = receipt; if (copiedState(token) === proof) { builder.#inputOffset = 0; proof.phase = "source-consuming"; } else proof.phase = "issued"; return step("pending", "paged-evidence-release", 128); }
    if (cancelling && (proof.phase === "source-consuming" || proof.phase === "source-observing")) { field.beginClose(); builder.#sourceKind = null; proof.phase = "issued"; return step("pending", "paged-evidence-source-close", 64); }
    if (proof.phase === "source-consuming") {
      if (copiedState(token) !== proof || proof.builder !== builder || !proof.release || builder.#input || builder.#inputOffset > proof.length) return step("rejected", "paged-evidence-source-owner");
      const current = field.advance(grant, builder); const forwarded = childStep({ ...current, phase: "paged-evidence-source-advance" }, grant); if (forwarded.kind === "rejected") { builder.#retainFailure(current); return forwarded; } if (forwarded.kind === "blocked") return forwarded;
      if (current.kind !== "pending" && current.kind !== "complete") { builder.#retainFailure(current); return { ...forwarded, kind: "rejected" }; } builder.#sourceKind = current.kind; proof.phase = "source-observing"; return forwarded;
    }
    if (proof.phase === "source-observing") {
      const next = builder.#inputOffset + (builder.#inputOffset < proof.length ? 1 : 0); const consumed = proof.offset + BigInt(next);
      if (copiedState(token) !== proof || proof.builder !== builder || field.consumed !== consumed || field.complete !== (builder.#sourceKind === "complete") || field.complete !== (consumed === field.value.byteLength) || consumed > builder.#copied) { const failure = step("rejected", "paged-evidence-source-observation"); builder.#retainFailure(failure); return failure; }
      builder.#inputOffset = next; builder.#sourceKind = null; proof.phase = next === proof.length ? "issued" : "source-consuming"; return step("pending", "paged-evidence-source-observe", 64);
    }
    if (proof.phase === "issued") {
      if (!proof.release) return step("rejected", "paged-evidence-release-missing");
      if (OwnedKernelReturnInputRelease.matchesSourceDetached(proof.release, field, token)) { const fragment = proof.fragment; if (builder.#input === fragment && fragment !== null) return step("rejected", "paged-evidence-input-held"); if (builder.#fragment === fragment) builder.#fragment = null; if (builder.#copyFragment === fragment) builder.#copyFragment = null; if (builder.#lastFragment === fragment) builder.#lastFragment = null; if (builder.#proof === token) builder.#proof = null; if (builder.#copyProof === token) builder.#copyProof = null; if (builder.#release === proof.release) builder.#release = null; if (builder.#copyRelease === proof.release) builder.#copyRelease = null; proof.fragment = null; proof.field = null; proof.builder = null; proof.phase = "source-detached"; return step("pending", "paged-evidence-ui-detach", 64); }
      return childStep({ ...field.detachInputRelease(proof.release, token, grant), phase: "paged-evidence-source-detach" }, grant);
    }
    if (proof.phase === "source-detached") {
      if (!proof.release) return step("rejected", "paged-evidence-release-missing");
      if (OwnedKernelReturnInputRelease.matchesSettled(proof.release, token)) { proof.phase = "source-settled"; return step("pending", "paged-evidence-ui-settle", 64); }
      return childStep({ ...field.settleInputRelease(proof.release, token, grant), phase: "paged-evidence-source-settle" }, grant);
    }
    if (proof.phase === "source-settled") { if (!proof.release || !OwnedKernelReturnInputRelease.matchesSettled(proof.release, token)) return step("rejected", "paged-evidence-settled-owner"); proof.release = null; proof.token = null; proof.phase = "domain-retired"; return step("pending", "paged-evidence-capsule-clear", 64); }
    return step(OwnedUiOperationPayloadBuilder.evidenceEmpty(token) ? "complete" : "rejected", "paged-evidence-body");
  }
  static matchesPage(builder: OwnedUiOperationPayloadBuilder, page: OwnedUiResidentPage, resident: OwnedUiResidentPayload): boolean { return builder.#resident === resident && builder.#head === page && builder.#tail === page && (builder.#writer === page || builder.#writer === null); }
  static installPage(builder: OwnedUiOperationPayloadBuilder, page: OwnedUiResidentPage, resident: OwnedUiResidentPayload, grant: NumericIndexGrant): RetainedUiWireStep { if (!admitted(grant, 64)) return step("blocked", "paged-page-install"); if (builder.#resident !== resident || builder.#head || builder.#tail || builder.#writer || builder.#failure !== NO_FAILURE || builder.#closing || !OwnedUiResidentPayload.matchesPageConstruction(resident, page)) return step("rejected", "paged-page-install"); builder.#head = page; builder.#tail = page; builder.#writer = page; builder.#written = 0; return step("pending", "paged-page-install", 64); }
  static matchesPageDetached(builder: OwnedUiOperationPayloadBuilder, page: OwnedUiResidentPage, proof: unknown, resident: OwnedUiResidentPayload): boolean { return builder.#resident === resident && builder.#head === null && builder.#tail === null && builder.#writer === null && builder.#failure === NO_FAILURE && OwnedUiResidentPayload.matchesPageRetirement(resident, page, proof); }
  static detachPage(builder: OwnedUiOperationPayloadBuilder, page: OwnedUiResidentPage, proof: unknown, resident: OwnedUiResidentPayload, grant: NumericIndexGrant): RetainedUiWireStep { if (!admitted(grant, 64)) return step("blocked", "paged-page-detach"); if (builder.#resident !== resident || !OwnedUiOperationPayloadBuilder.matchesPage(builder, page, resident) || builder.#failure !== NO_FAILURE || !OwnedUiResidentPayload.matchesPageRetirement(resident, page, proof)) return step("rejected", "paged-page-detach"); builder.#head = null; builder.#tail = null; builder.#writer = null; return step("complete", "paged-page-detach", 64); }
  static readerIsEmpty(builder: OwnedUiOperationPayloadBuilder): boolean { return builder.#reader === null; }
  static readerAvailable(builder: OwnedUiOperationPayloadBuilder, resident: OwnedUiResidentPayload): boolean { return builder.#resident === resident && builder.#reader === null && builder.#readerPhase === "none" && builder.#failure === NO_FAILURE && !builder.#closing; }
  static matchesReader(builder: OwnedUiOperationPayloadBuilder, reader: OwnedUiResidentPayloadReader, resident: OwnedUiResidentPayload): boolean { return builder.#resident === resident && builder.#reader === reader && builder.#readerPhase === "held"; }
  static installReader(builder: OwnedUiOperationPayloadBuilder, reader: OwnedUiResidentPayloadReader, resident: OwnedUiResidentPayload, grant: NumericIndexGrant): RetainedUiWireStep { if (!admitted(grant, 64)) return step("blocked", "paged-reader-install"); if (!OwnedUiOperationPayloadBuilder.readerAvailable(builder, resident) || !OwnedUiResidentPayload.matchesReaderConstruction(resident, reader)) return step("rejected", "paged-reader-install"); builder.#reader = reader; builder.#readerPhase = "held"; return step("pending", "paged-reader-install", 64); }
  static readerEof(builder: OwnedUiOperationPayloadBuilder, resident: OwnedUiResidentPayload, consumed: bigint): boolean { return builder.#resident === resident && builder.#failure === NO_FAILURE && builder.#copyPhase === "ready" && builder.#field !== null && builder.#field.complete && consumed === builder.#field.value.byteLength && consumed === builder.#copied; }
  static detachReader(builder: OwnedUiOperationPayloadBuilder, reader: OwnedUiResidentPayloadReader, proof: OwnedUiResidentReaderRetirement, resident: OwnedUiResidentPayload, grant: NumericIndexGrant): RetainedUiWireStep { if (!admitted(grant, 64)) return step("blocked", "paged-reader-detach"); if (builder.#resident !== resident || builder.#failure !== NO_FAILURE || builder.#reader !== reader && !(builder.#reader === null && builder.#readerPhase === "none") || !OwnedUiResidentReaderRetirement.matchesBody(proof, reader, resident)) return step("rejected", "paged-reader-detach"); builder.#reader = proof; builder.#readerPhase = "detached"; return step("pending", "paged-reader-detach", 64); }
  static matchesReaderDetached(builder: OwnedUiOperationPayloadBuilder, reader: OwnedUiResidentPayloadReader, proof: OwnedUiResidentReaderRetirement, resident: OwnedUiResidentPayload): boolean { return builder.#resident === resident && builder.#reader === proof && builder.#readerPhase === "detached" && OwnedUiResidentPayload.matchesReaderBinding(resident, reader, proof); }
  static settleReader(builder: OwnedUiOperationPayloadBuilder, reader: OwnedUiResidentPayloadReader, proof: OwnedUiResidentReaderRetirement, resident: OwnedUiResidentPayload, grant: NumericIndexGrant): RetainedUiWireStep { if (!admitted(grant, 64)) return step("blocked", "paged-reader-settle"); if (!OwnedUiOperationPayloadBuilder.matchesReaderDetached(builder, reader, proof, resident) || !OwnedUiResidentReaderRetirement.matchesDetached(proof, reader, resident)) return step("rejected", "paged-reader-settle"); builder.#reader = null; builder.#readerPhase = "settled"; return step("complete", "paged-reader-settle", 64); }
  static matchesReaderSettled(builder: OwnedUiOperationPayloadBuilder, reader: OwnedUiResidentPayloadReader, proof: OwnedUiResidentReaderRetirement, resident: OwnedUiResidentPayload): boolean { return builder.#resident === resident && builder.#reader === null && builder.#readerPhase === "settled" && OwnedUiResidentPayload.matchesReaderBinding(resident, reader, proof); }
  static construct(field: OwnedKernelReturnInputField, resident: OwnedUiResidentPayload, grant: NumericIndexGrant): OwnedUiOperationPayloadBuilder {
    if (!admitted(grant, 272) || !OwnedUiResidentPayload.matchesBuilderConstruction(resident, field)) throw new Error("Invalid original builder reservation"); return new OwnedUiOperationPayloadBuilder(MINT, field, resident);
  }
  static bindSource(builder: OwnedUiOperationPayloadBuilder, resident: OwnedUiResidentPayload, grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 64)) return step("blocked", "paged-source-bind"); if (!OwnedUiResidentPayload.matchesBuilderPhase(resident, builder, "source-installing") || builder.#closing || builder.#failure !== NO_FAILURE || !builder.#field) return step("rejected", "paged-source-bind");
    if (!builder.#field.bind(builder)) return step("rejected", "paged-source-bind", 64); builder.#bound = OwnedKernelReturnInputField.matchesBuilder(builder.#field, builder); return step(builder.#bound ? "pending" : "rejected", "paged-source-bind", 64);
  }
  static finalize(builder: OwnedUiOperationPayloadBuilder, resident: OwnedUiResidentPayload, grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 64)) return step("blocked", "paged-finalization"); if (!OwnedUiResidentPayload.matchesBuilderPhase(resident, builder, "witness-ready") || !builder.#bound || builder.#closing || builder.#failure !== NO_FAILURE) return step("rejected", "paged-finalization"); Object.freeze(builder); return step("pending", "paged-finalization", 64);
  }
  static healthy(builder: OwnedUiOperationPayloadBuilder): boolean { return !builder.#closing && builder.#failure === NO_FAILURE && builder.#bound; }
  static empty(builder: OwnedUiOperationPayloadBuilder): boolean { return builder.#empty(); }
  static bodyEmpty(builder: OwnedUiOperationPayloadBuilder): boolean { return builder.#failure === NO_FAILURE && builder.#closing && !builder.#fragment && !builder.#proof && !builder.#release && !builder.#input && !builder.#copyFragment && !builder.#lastFragment && !builder.#copyProof && !builder.#copyRelease && !builder.#head && !builder.#tail && !builder.#writer && !builder.#reader && builder.#readerPhase !== "held" && builder.#readerPhase !== "detached"; }
  static sourceDetached(builder: OwnedUiOperationPayloadBuilder): boolean { return builder.#field === null && !builder.#bound && OwnedUiOperationPayloadBuilder.bodyEmpty(builder); }
  static matchesRetirementOwner(builder: OwnedUiOperationPayloadBuilder, field: unknown, witness: unknown): boolean { return builder.#resident !== null && OwnedUiResidentPayload.matchesBuilderRetirement(builder.#resident, builder, field, witness); }
  static detachRetirementSource(builder: OwnedUiOperationPayloadBuilder, resident: OwnedUiResidentPayload, grant: NumericIndexGrant): RetainedUiWireStep { if (!admitted(grant, 64)) return step("blocked", "paged-builder-source-detach"); if (builder.#resident !== resident || !OwnedUiOperationPayloadBuilder.bodyEmpty(builder) || !OwnedUiResidentPayload.matchesBuilderRetirementPhase(resident, builder, "binding-detaching")) return step("rejected", "paged-builder-source-detach"); builder.#field = null; builder.#bound = false; return step("pending", "paged-builder-source-detach", 64); }
  static finishRetirement(builder: OwnedUiOperationPayloadBuilder, resident: OwnedUiResidentPayload, grant: NumericIndexGrant): RetainedUiWireStep { if (!admitted(grant, 64)) return step("blocked", "paged-builder-binding-finish"); if (builder.#resident !== resident || !OwnedUiOperationPayloadBuilder.sourceDetached(builder) || !OwnedUiResidentPayload.matchesBuilderRetirementPhase(resident, builder, "binding-settled")) return step("rejected", "paged-builder-binding-finish"); builder.#resident = null; builder.#sourceKind = null; builder.#phase = "closed"; return step("complete", "paged-builder-binding-finish", 64); }
  static begin(owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime, field: unknown, resident: unknown, grant: NumericIndexGrant): OwnedUiOperationPayloadAdmission {
    if (!admitted(grant, 32)) return { step: step("blocked", "paged-admission"), builder: null };
    if (!OwnedKernelReturnInputField.matchesOwner(field, owner, activation, lifetime) || !OwnedUiResidentPayload.matchesOwner(resident, owner, activation, lifetime)) return { step: step("rejected", "paged-admission-owner"), builder: null };
    return resident.beginBuilder(field, grant);
  }
  get failure(): unknown { return this.#failure === NO_FAILURE ? null : this.#failure; }
  #retainFailure(error: unknown): void { if (this.#failure !== NO_FAILURE && !Object.is(this.#failure, error)) throw error; this.#failure = error; }

  //#region 📥️FixedPageCopy
  #copyChild(current: RetainedUiWireStep, grant: NumericIndexGrant): RetainedUiWireStep {
    const result = childStep(current, grant); if (result.kind === "rejected" && this.#failure === NO_FAILURE) this.#retainFailure(current); return result;
  }
  #settleCopy(grant: NumericIndexGrant): RetainedUiWireStep {
    if (this.#copyPhase === "proof") { const current = this.#resident!.beginEvidence(this, grant); const forwarded = this.#copyChild(current.step, grant); if (forwarded.kind !== "rejected" && current.step.kind === "ready") { const fragment = this.#copyFragment; if (!fragment || !this.#field || !OwnedUiOperationInputCopied.matches(current.evidence, fragment, this.#field, this, fragment.offset, fragment.length)) throw new Error("Missing original copied evidence"); this.#copyProof = current.evidence; this.#copyPhase = "receipt"; } return forwarded; }
    if (this.#copyPhase === "receipt") { const current = this.#resident!.advanceEvidence(this, grant); const forwarded = this.#copyChild(current, grant); if (forwarded.kind !== "rejected" && current.kind === "complete") this.#copyPhase = "range-observe"; return forwarded; }
    if (!admitted(grant, 64)) return step("blocked", "paged-range-observation");
    if (!this.#field || this.#input || this.#copyFragment || this.#copyProof || this.#copyRelease || this.#sourceKind !== null || this.#field.consumed !== this.#copied || this.#field.complete !== (this.#copied === this.#field.value.byteLength)) throw new Error("Copied range differs from original source");
    this.#inputOffset = 0; this.#copyPhase = this.#field.complete ? "ready" : "idle"; return step("pending", "paged-range-observation", 64);
  }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 1)) return step("blocked", "paged-copy"); if (this.#closing || !this.#bound || this.#failure !== NO_FAILURE || !OwnedUiResidentPayload.matchesBuilderLive(this.#resident, this)) return step("rejected", "paged-copy");
    try {
      if (this.#copyPhase === "ready") return step("ready", "paged-copy");
      if (this.#copyPhase === "proof" || this.#copyPhase === "receipt" || this.#copyPhase === "range-observe") return this.#settleCopy(grant);
      if (this.#copyPhase === "idle") {
        if (!admitted(grant, 128)) return step("blocked", "paged-input-admit");
        const fragment = this.#field!.fragment; if (!fragment || fragment === this.#lastFragment) return step("blocked", "paged-source-continuation");
        if (!OwnedKernelReturnInputFragment.matches(fragment, this.#field!) || fragment.offset !== this.#copied || fragment.offset + BigInt(fragment.length) > this.#field!.value.byteLength) return step("rejected", "paged-source-range");
        this.#input = fragment; this.#copyFragment = fragment; this.#inputOffset = 0; this.#copyPhase = "copy"; return step("pending", "paged-input-admit", 128);
      }
      if (this.#copyPhase === "copy" && this.#inputOffset !== this.#input!.length && !this.#writer) { if (this.#head) return step("blocked", "paged-page-window"); this.#copyPhase = "page"; }
      if (this.#copyPhase === "page") { const remaining = this.#field!.value.byteLength - this.#copied; const length = Number(remaining < 256n ? remaining : 256n); const current = this.#resident!.beginPage(this, length, grant); const result = this.#copyChild(current.step, grant); if (result.kind !== "rejected" && current.step.kind === "ready") this.#copyPhase = "page-observe"; return result; }
      if (this.#copyPhase === "page-observe") { if (!admitted(grant, 64)) return step("blocked", "paged-page-observation"); if (!this.#writer || !OwnedUiOperationPayloadBuilder.matchesPage(this, this.#writer, this.#resident!) || OwnedUiResidentPayload.pageLength(this.#resident, this, this.#writer) === null) return step("rejected", "paged-page-owner"); this.#copyPhase = "allocate"; return step("pending", "paged-page-observation", 64); }
      if (this.#copyPhase === "allocate") { const current = this.#writer!.allocate(grant); const result = this.#copyChild(current, grant); if (result.kind !== "rejected" && current.kind === "ready") this.#copyPhase = "allocate-observe"; return result; }
      if (this.#copyPhase === "allocate-observe") { if (!admitted(grant, 64)) return step("blocked", "paged-allocation-observation"); this.#copyPhase = "copy"; return step("pending", "paged-allocation-observation", 64); }
      if (this.#copyPhase === "seal") { const current = this.#writer!.seal(grant); const result = this.#copyChild(current, grant); if (result.kind !== "rejected" && current.kind === "ready") this.#copyPhase = "seal-observe"; return result; }
      if (this.#copyPhase === "seal-observe") { if (!admitted(grant, 64)) return step("blocked", "paged-seal-observation"); this.#writer = null; this.#copyPhase = "copy"; return step("pending", "paged-seal-observation", 64); }
      if (this.#copyPhase === "write") {
        if (typeof this.#sourceKind !== "number") return step("rejected", "paged-byte-latch"); const current = this.#writer!.writeByte(this.#sourceKind, grant); const result = this.#copyChild(current, grant);
        if (result.kind !== "rejected" && current.kind === "pending" && current.items === 1 && current.bytes === 1) { this.#inputOffset++; this.#copied++; this.#written++; this.#sourceKind = null; const length = OwnedUiResidentPayload.pageLength(this.#resident, this, this.#writer); if (length === null || this.#written > length) return step("rejected", "paged-page-length", 1); this.#copyPhase = this.#written === length ? "seal" : "copy"; } return result;
      }
      if (this.#inputOffset === this.#input!.length) { if (!admitted(grant, 128)) return step("blocked", "paged-input-copy-detach"); this.#input = null; this.#copyPhase = "proof"; return step("pending", "paged-input-copy-detach", 128); }
      this.#sourceKind = this.#input!.byteAt(this.#inputOffset, this); this.#copyPhase = "write"; return step("pending", "paged-input-byte", 1);
    } catch (error) { this.#retainFailure(error); return step("rejected", "paged-copy-fault"); }
  }
  beginRead(grant: NumericIndexGrant): OwnedUiOperationPayloadReadAdmission { if (!this.#resident || this.#closing || this.#failure !== NO_FAILURE) return { step: step("rejected", "paged-reader-owner"), reader: null }; return this.#resident.beginReader(this, grant); }
  //#endregion 📥️FixedPageCopy

  //#region ♻️BoundedClose
  beginClose(): void { this.#closing = true; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 128)) return step("blocked", "paged-builder-close"); if (!this.#closing) throw new Error("Paged builder close has not begun");
    try {
      if (this.#phase === "closed") return step("complete", "paged-builder-close");
      if (this.#failure !== NO_FAILURE) return step("rejected", "paged-builder-fault-held");
      if (OwnedUiOperationPayloadBuilder.bodyEmpty(this)) return step("blocked", "paged-builder-binding-retirement");
      if (this.#field && OwnedKernelReturnInputField.matchesBuilder(this.#field, this)) return step("blocked", "paged-evidence-retirement-admission");
      if (this.#reader instanceof OwnedUiResidentPayloadReader) return childStep(this.#resident!.closeReader(this.#reader, grant), grant);
      if (this.#reader) return step("blocked", "paged-reader-binding");
      if (this.#copyPhase === "proof" || this.#copyPhase === "receipt" || this.#copyPhase === "range-observe") return this.#settleCopy(grant);
      if (this.#phase === "open") { this.#bound = OwnedKernelReturnInputField.matchesBuilder(this.#field, this); if (this.#bound) { const fragment = this.#copyFragment ?? this.#field!.fragment; this.#fragment = fragment === this.#lastFragment ? null : fragment; this.#field!.beginClose(); } this.#input = null; this.#copyFragment = null; this.#phase = this.#fragment ? "proof" : "pages"; return step("pending", "paged-input-detach", 128); }
      if (this.#phase === "proof") return step("blocked", "paged-evidence-admission");
      if (this.#phase === "release") { const receipt = this.#fragment!.release(this.#proof); if (!receipt) return step("rejected", "paged-input-release-refused", 128); this.#release = receipt; this.#phase = "receipt"; return step("pending", "paged-input-release", 128); }
      if (this.#phase === "receipt") { if (!OwnedKernelReturnInputRelease.matches(this.#release, this.#fragment!, this.#proof)) return step("rejected", "paged-input-release-authority"); this.#fragment = null; this.#proof = null; this.#release = null; this.#bound = false; this.#phase = "pages"; return step("pending", "paged-input-release-retire", 128); }
      if (this.#phase === "pages" && this.#head) return childStep(this.#resident!.closePage(this.#head, grant), grant);
      this.#lastFragment = null; this.#reader = null; this.#sourceKind = null;
      this.#field = null; this.#resident = null; this.#bound = false; this.#phase = "closed"; return step("complete", "paged-builder-close", 128);
    } catch (error) { this.#retainFailure(error); return step("rejected", "paged-builder-close-fault"); }
  }
  terminalIsEmpty(): boolean { return this.#empty(); }
  #empty(): boolean { return this.#failure === NO_FAILURE && this.#closing && this.#phase === "closed" && !this.#field && !this.#resident && !this.#fragment && !this.#proof && !this.#release && !this.#input && !this.#copyFragment && !this.#lastFragment && !this.#copyProof && !this.#copyRelease && !this.#head && !this.#tail && !this.#writer && !this.#reader && this.#readerPhase !== "held" && this.#readerPhase !== "detached"; }
  //#endregion ♻️BoundedClose
}



/** 📥️ Exact copied-input evidence; no public mint or successful copy path is exposed by this staging boundary. */
export class OwnedUiOperationInputCopied {
  #proof: Evidence | OwnedUiOperationPayloadBuilder;
  private constructor(mint: object, builder: OwnedUiOperationPayloadBuilder, resident: OwnedUiResidentPayload, fragment: OwnedKernelReturnInputFragment) {
    if (mint !== MINT) throw new Error("Invalid copied input authority"); this.#proof = builder; const installed = resident.installEvidence(this, builder, { maxItems: 1, maxBytes: 64 }); if (installed.kind !== "ready") throw new Error("Evidence installation refused");
    this.#proof = { fragment, field: fragment.field, builder, offset: fragment.offset, length: fragment.length, release: null, phase: "constructed", token: this }; Object.seal(this.#proof);
  }
  static { copiedProof = (builder, resident, fragment) => new OwnedUiOperationInputCopied(MINT, builder, resident, fragment); copiedState = token => token !== null && typeof token === "object" && #proof in token ? token.#proof : null; }
  static matches(token: unknown, fragment: object, field: object, builder: object, offset: bigint, length: number): token is OwnedUiOperationInputCopied {
    return token !== null && typeof token === "object" && #proof in token && !builderBrand(token.#proof) && exact(token.#proof, fragment, field, builder, offset, length);
  }
  static matchesRelease(token: unknown, release: unknown): token is OwnedUiOperationInputCopied { return token !== null && typeof token === "object" && #proof in token && !builderBrand(token.#proof) && evidenceRelease(token.#proof, token, release); }
  static matchesSourceDetached(token: unknown, release: unknown): token is OwnedUiOperationInputCopied { return token !== null && typeof token === "object" && #proof in token && !builderBrand(token.#proof) && evidenceRelease(token.#proof, token, release) && token.#proof.phase === "source-detached" && !token.#proof.fragment && !token.#proof.field && !token.#proof.builder; }
}

/** ♻️ Exact detached-input cancellation evidence, distinct from copied bytes and semantic publication. */
export class OwnedUiOperationInputCancelled {
  #proof: Evidence | OwnedUiOperationPayloadBuilder;
  private constructor(mint: object, builder: OwnedUiOperationPayloadBuilder, resident: OwnedUiResidentPayload, fragment: OwnedKernelReturnInputFragment) {
    if (mint !== MINT) throw new Error("Invalid cancelled input authority"); this.#proof = builder; const installed = resident.installEvidence(this, builder, { maxItems: 1, maxBytes: 64 }); if (installed.kind !== "ready") throw new Error("Evidence installation refused");
    this.#proof = { fragment, field: fragment.field, builder, offset: fragment.offset, length: fragment.length, release: null, phase: "constructed", token: this }; Object.seal(this.#proof);
  }
  static { cancelledProof = (builder, resident, fragment) => new OwnedUiOperationInputCancelled(MINT, builder, resident, fragment); cancelledState = token => token !== null && typeof token === "object" && #proof in token ? token.#proof : null; }
  static matches(token: unknown, fragment: object, field: object, builder: object, offset: bigint, length: number): token is OwnedUiOperationInputCancelled {
    return token !== null && typeof token === "object" && #proof in token && !builderBrand(token.#proof) && exact(token.#proof, fragment, field, builder, offset, length);
  }
  static matchesRelease(token: unknown, release: unknown): token is OwnedUiOperationInputCancelled { return token !== null && typeof token === "object" && #proof in token && !builderBrand(token.#proof) && evidenceRelease(token.#proof, token, release); }
  static matchesSourceDetached(token: unknown, release: unknown): token is OwnedUiOperationInputCancelled { return token !== null && typeof token === "object" && #proof in token && !builderBrand(token.#proof) && evidenceRelease(token.#proof, token, release) && token.#proof.phase === "source-detached" && !token.#proof.fragment && !token.#proof.field && !token.#proof.builder; }
}
//#endregion 📄️PrivatePagedInputAuthority
