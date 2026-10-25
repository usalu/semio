//#region 📄️PrivatePagedInputAuthority
import type { OwnedUiInstance } from "../../../🏘️instance/🟦️.ts";
import type { UiIndexGrant } from "../../../🔌️host/🟦️.ts";
import type { RetainedUiWireStep } from "../../../📦️wire/🟦️.ts";
import type { UiActivationLease, UiInstanceLifetime } from "../../../🔌️host/🟦️.ts";
import { uiInputHost, type UiInputField, type UiInputFragment, type UiInputRelease } from "../../../🔌️host/📥️input/🟦️.ts";

const MINT = Object.freeze({});
const NO_FAILURE = Object.freeze({});
type Evidence = { fragment: object | null; field: object | null; builder: OwnedUiOperationPayloadBuilder | null; readonly offset: bigint; readonly length: number; release: UiInputRelease | null; phase: "constructed" | "available" | "issued" | "source-consuming" | "source-observing" | "source-detached" | "source-settled" | "domain-retired"; token: OwnedUiOperationInputCopied | OwnedUiOperationInputCancelled | null };
const admitted = (grant: UiIndexGrant, bytes: number): boolean => Number.isSafeInteger(grant.maxItems) && grant.maxItems >= 1 && Number.isSafeInteger(grant.maxBytes) && grant.maxBytes >= bytes;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
type InputToken = OwnedUiOperationInputCopied | OwnedUiOperationInputCancelled;
let cancelledProof: (builder: OwnedUiOperationPayloadBuilder, resident: UiResidentPayloadSeam, fragment: UiInputFragment) => OwnedUiOperationInputCancelled;
let copiedProof: (builder: OwnedUiOperationPayloadBuilder, resident: UiResidentPayloadSeam, fragment: UiInputFragment) => OwnedUiOperationInputCopied;
let copiedState: (token: unknown) => Evidence | OwnedUiOperationPayloadBuilder | null;
let cancelledState: (token: unknown) => Evidence | OwnedUiOperationPayloadBuilder | null;
let builderBrand: (value: unknown) => value is OwnedUiOperationPayloadBuilder;
function evidenceState(token: unknown): Evidence | null { const value = copiedState(token) ?? cancelledState(token); return value !== null && !builderBrand(value) ? value : null; }
export type OwnedUiOperationPayloadAdmission = { readonly step: RetainedUiWireStep; readonly builder: OwnedUiOperationPayloadBuilder | null };
export type OwnedUiOperationPayloadReadStep = RetainedUiWireStep | { readonly kind: "byte"; readonly value: number; readonly items: number; readonly bytes: number };
/** 🧷️ The resident augments this map with its concrete classes, so that a program that includes the resident sees them instead of the seams. */
export interface UiResidentTypes {}
type ResidentReader = UiResidentTypes extends { readonly reader: infer Reader } ? Reader : UiResidentPayloadReaderSeam;
export type OwnedUiOperationPayloadReadAdmission = { readonly step: RetainedUiWireStep; readonly reader: ResidentReader | null };
function childStep(current: RetainedUiWireStep, grant: UiIndexGrant): RetainedUiWireStep {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes) return { ...current, kind: "rejected" };
  return current.kind === "ready" || current.kind === "complete" ? { ...current, kind: "pending" } : current;
}
function exact(proof: Evidence, fragment: object, field: object, builder: object, offset: bigint, length: number): boolean {
  return typeof offset === "bigint" && offset >= 0n && offset <= 18446744073709551615n && Number.isSafeInteger(length) && length >= 0 && length <= 4096 && offset + BigInt(length) <= 18446744073709551615n
    && (proof.phase === "available" || proof.phase === "issued" || proof.phase === "source-consuming" || proof.phase === "source-observing") && proof.fragment === fragment && proof.field === field && proof.builder === builder && proof.offset === offset && proof.length === length && OwnedUiOperationPayloadBuilder.matchesField(builder, field);
}
function evidenceRelease(proof: Evidence, token: object, release: unknown): boolean { return proof.token === token && proof.release !== null && proof.release === release && (proof.phase === "issued" || proof.phase === "source-consuming" || proof.phase === "source-observing" || proof.phase === "source-detached" || proof.phase === "source-settled"); }

/** 📄️ Exact native field admission is retained by its resident parent before exposure to a caller. */
/** 🏗️ A resident reader as the builder drives it. */
export interface UiResidentPayloadReaderSeam { advance(grant: UiIndexGrant): OwnedUiOperationPayloadReadStep; terminalIsEmpty(): boolean }
/** 🧾️ The resident's reader retirement witness, opaque to the builder. */
export type UiResidentReaderRetirementSeam = object;
/** 📃️ A resident page window as the builder writes it. */
export interface UiResidentPageSeam { allocate(grant: UiIndexGrant): RetainedUiWireStep; writeByte(value: number, grant: UiIndexGrant): RetainedUiWireStep; seal(grant: UiIndexGrant): RetainedUiWireStep }
/** 💾️ The resident payload as the builder reaches it: the admissions the builder asks for and installs into. */
export interface UiResidentPayloadSeam {
  installBuilder(builder: OwnedUiOperationPayloadBuilder, grant: UiIndexGrant): RetainedUiWireStep;
  beginBuilder(field: UiInputField, grant: UiIndexGrant): OwnedUiOperationPayloadAdmission;
  beginEvidence(builder: OwnedUiOperationPayloadBuilder, grant: UiIndexGrant): { readonly step: RetainedUiWireStep; readonly evidence: InputToken | null };
  advanceEvidence(builder: OwnedUiOperationPayloadBuilder, grant: UiIndexGrant): RetainedUiWireStep;
  installEvidence(token: InputToken, builder: OwnedUiOperationPayloadBuilder, grant: UiIndexGrant): RetainedUiWireStep;
  beginReader(builder: OwnedUiOperationPayloadBuilder, grant: UiIndexGrant): OwnedUiOperationPayloadReadAdmission;
  closeReader(reader: UiResidentPayloadReaderSeam, grant: UiIndexGrant): RetainedUiWireStep;
  beginPage(builder: OwnedUiOperationPayloadBuilder, length: number, grant: UiIndexGrant): { readonly step: RetainedUiWireStep; readonly page: UiResidentPageSeam | null };
  closePage(page: UiResidentPageSeam, grant: UiIndexGrant): RetainedUiWireStep;
}
/** 🪪️ The resident payload's brand and phase checks the builder consults. */
export interface UiResidentPayloadStatics {
  matchesBuilderConstruction(payload: unknown, field: unknown): payload is UiResidentPayloadSeam;
  matchesBuilderPhase(payload: unknown, builder: unknown, phase: "source-installing" | "witness-ready"): payload is UiResidentPayloadSeam;
  matchesBuilderLive(payload: unknown, builder: unknown): payload is UiResidentPayloadSeam;
  matchesBuilderRetirement(payload: unknown, builder: unknown, field: unknown, witness: unknown): boolean;
  matchesBuilderRetirementPhase(payload: unknown, builder: unknown, phase: "binding-detaching" | "binding-settled"): boolean;
  matchesInputCancellation(payload: unknown, builder: unknown): boolean;
  matchesEvidenceRetirement(payload: unknown, builder: unknown, token: unknown): boolean;
  matchesEvidenceCancellation(payload: unknown, builder: unknown, token: unknown): boolean;
  matchesEvidencePhase(payload: unknown, builder: unknown, phase: "constructing" | "witness-ready" | "finalized"): payload is UiResidentPayloadSeam;
  matchesOwner(payload: unknown, owner: OwnedUiInstance, activation: UiActivationLease, lifetime: UiInstanceLifetime): payload is UiResidentPayloadSeam;
  matchesReaderConstruction(payload: unknown, reader: unknown): boolean;
  matchesReaderBinding(payload: unknown, reader: unknown, witness: unknown): boolean;
  matchesPageConstruction(payload: unknown, page: unknown): boolean;
  matchesPageRetirement(payload: unknown, page: unknown, proof: unknown): boolean;
  pageLength(payload: unknown, builder: unknown, page: unknown): number | null;
}
/** 🪧️ The resident reader retirement witness's brand and phase checks the builder consults. */
export interface UiResidentReaderRetirementStatics {
  matchesBody(proof: unknown, reader: unknown, payload: UiResidentPayloadSeam): boolean;
  matchesDetached(proof: unknown, reader: unknown, payload: UiResidentPayloadSeam): boolean;
}
/** 🏦️ The resident the builder is installed into, declared by the builder so the builder never imports the resident. */
export interface UiOperationResidentHost { readonly payload: UiResidentPayloadStatics; readonly readerRetirement: UiResidentReaderRetirementStatics; isReader(value: unknown): value is UiResidentPayloadReaderSeam }
let boundResidentHost: UiOperationResidentHost | null = null;
/** 🔗️ Binds, once, the resident's brand and phase checks. */
export function bindUiOperationResidentHost(host: UiOperationResidentHost): void {
  if (boundResidentHost) throw new Error("UI operation resident host is already bound");
  boundResidentHost = host;
}
function residentHost(): UiOperationResidentHost {
  if (!boundResidentHost) throw new Error("UI operation resident host is unbound");
  return boundResidentHost;
}
export class OwnedUiOperationPayloadBuilder {
  #field: UiInputField | null;
  #resident: UiResidentPayloadSeam | null;
  #bound = false;
  #closing = false;
  #phase: "open" | "proof" | "release" | "receipt" | "pages" | "owner" | "closed" = "open";
  #fragment: UiInputFragment | null = null;
  #proof: OwnedUiOperationInputCancelled | null = null;
  #release: UiInputRelease | null = null;
  #failure: unknown = NO_FAILURE;
  #copyPhase: "idle" | "page" | "page-observe" | "allocate" | "allocate-observe" | "copy" | "write" | "seal" | "seal-observe" | "proof" | "receipt" | "range-observe" | "ready" = "idle";
  #sourceKind: "pending" | "complete" | number | null = null;
  #input: UiInputFragment | null = null;
  #copyFragment: UiInputFragment | null = null;
  #lastFragment: UiInputFragment | null = null;
  #inputOffset = 0;
  #copied = 0n;
  #copyProof: OwnedUiOperationInputCopied | null = null;
  #copyRelease: UiInputRelease | null = null;
  #head: UiResidentPageSeam | null = null;
  #tail: UiResidentPageSeam | null = null;
  #writer: UiResidentPageSeam | null = null;
  #written = 0;
  #reader: UiResidentPayloadReaderSeam | UiResidentReaderRetirementSeam | null = null;
  #readerPhase: "none" | "held" | "detached" | "settled" = "none";
  static { builderBrand = (value): value is OwnedUiOperationPayloadBuilder => value !== null && typeof value === "object" && #field in value; }
  private constructor(mint: object, field: UiInputField, resident: UiResidentPayloadSeam) {
    if (mint !== MINT) throw new Error("Invalid paged builder authority"); this.#field = field; this.#resident = resident;
    const installed = resident.installBuilder(this, { maxItems: 1, maxBytes: 64 }); if (installed.kind !== "ready" || installed.bytes !== 64) throw new Error("Paged builder registration refused");
  }
  static matchesField(builder: unknown, field: object): builder is OwnedUiOperationPayloadBuilder {
    return builder !== null && typeof builder === "object" && #field in builder && builder.#field === field && builder.#resident !== null;
  }
  static matchesResident(builder: unknown, resident: UiResidentPayloadSeam): builder is OwnedUiOperationPayloadBuilder { return builder !== null && typeof builder === "object" && #resident in builder && builder.#resident === resident; }
  static hasBrand(value: unknown): value is OwnedUiOperationPayloadBuilder { return builderBrand(value); }
  static activeInput(builder: OwnedUiOperationPayloadBuilder): boolean { return builder.#phase === "open" && (builder.#input !== null || builder.#copyFragment !== null); }
  static cancellationPrepared(builder: OwnedUiOperationPayloadBuilder): boolean { return builder.#closing && builder.#phase === "proof" && builder.#fragment !== null && !builder.#input && !builder.#copyFragment && builder.#sourceKind === null; }
  static prepareInputCancellation(builder: OwnedUiOperationPayloadBuilder, resident: UiResidentPayloadSeam, grant: UiIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 128)) return step("blocked", "paged-active-input-detach");
    if (!residentHost().payload.matchesInputCancellation(resident, builder) || builder.#resident !== resident || builder.#failure !== NO_FAILURE || !OwnedUiOperationPayloadBuilder.activeInput(builder) || builder.#proof || builder.#release || builder.#copyProof || builder.#copyRelease) return step("rejected", "paged-active-input-owner");
    const fragment = builder.#copyFragment ?? builder.#input; if (!fragment || !builder.#field || !uiInputHost().fragment.matches(fragment, builder.#field) || builder.#input !== null && builder.#input !== fragment) return step("rejected", "paged-active-input-fragment");
    builder.#closing = true; builder.#fragment = fragment; builder.#input = null; builder.#copyFragment = null; builder.#sourceKind = null; builder.#copyPhase = "idle"; builder.#phase = "proof"; return step("pending", "paged-active-input-detach", 128);
  }
  static evidenceEligible(builder: unknown, resident: UiResidentPayloadSeam): builder is OwnedUiOperationPayloadBuilder { return builderBrand(builder) && builder.#resident === resident && builder.#failure === NO_FAILURE && builder.#bound && builder.#field !== null && (builder.#closing && !builder.#input && !builder.#copyProof || builder.#copyPhase === "proof" && builder.#input === null) && (builder.#copyFragment !== null || builder.#fragment !== null || builder.#field.fragment !== null); }
  static matchesEvidenceConstruction(token: unknown, builder: OwnedUiOperationPayloadBuilder): token is InputToken { return copiedState(token) === builder || cancelledState(token) === builder; }
  static matchesEvidence(token: unknown, builder: OwnedUiOperationPayloadBuilder): token is InputToken { const proof = evidenceState(token); return proof !== null && proof.token === token && proof.builder === builder; }
  static constructEvidence(builder: OwnedUiOperationPayloadBuilder, resident: UiResidentPayloadSeam, grant: UiIndexGrant): InputToken {
    if (!admitted(grant, 168) || !residentHost().payload.matchesEvidencePhase(resident, builder, "constructing") || !OwnedUiOperationPayloadBuilder.evidenceEligible(builder, resident)) throw new Error("Invalid original evidence reservation");
    const fragment = builder.#copyFragment ?? builder.#fragment ?? builder.#field!.fragment; if (!fragment || !uiInputHost().fragment.matches(fragment, builder.#field!)) throw new Error("Missing exact evidence fragment");
    return builder.#copyPhase === "proof" ? copiedProof(builder, resident, fragment) : cancelledProof(builder, resident, fragment);
  }
  static finalizeEvidence(token: InputToken, builder: OwnedUiOperationPayloadBuilder, resident: UiResidentPayloadSeam, grant: UiIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 64)) return step("blocked", "paged-evidence-finalization"); if (!residentHost().payload.matchesEvidencePhase(resident, builder, "witness-ready") || !OwnedUiOperationPayloadBuilder.matchesEvidence(token, builder)) return step("rejected", "paged-evidence-finalization"); Object.freeze(token); return step("pending", "paged-evidence-finalization", 64);
  }
  static publishEvidence(token: InputToken, builder: OwnedUiOperationPayloadBuilder, resident: UiResidentPayloadSeam): void { const proof = evidenceState(token); if (!residentHost().payload.matchesEvidencePhase(resident, builder, "finalized") || !proof || proof.builder !== builder || proof.token !== token || proof.phase !== "constructed") throw new Error("Invalid evidence publication"); proof.phase = "available"; }
  static evidenceEmpty(token: unknown): boolean { const proof = evidenceState(token); return proof !== null && proof.phase === "domain-retired" && !proof.fragment && !proof.field && !proof.builder && !proof.release && !proof.token; }
  static advanceEvidence(token: InputToken, builder: OwnedUiOperationPayloadBuilder, resident: UiResidentPayloadSeam, grant: UiIndexGrant): RetainedUiWireStep {
    const proof = evidenceState(token); if (!proof || !residentHost().payload.matchesEvidenceRetirement(resident, builder, token) || builder.#failure !== NO_FAILURE || !builder.#field) return step("rejected", "paged-evidence-owner");
    const cancelling = residentHost().payload.matchesEvidenceCancellation(resident, builder, token);
    if (!admitted(grant, proof.phase === "available" ? 128 : proof.phase === "source-consuming" && !cancelling ? 1 : 64)) return step("blocked", "paged-evidence-retirement"); const field = builder.#field;
    if (proof.phase === "available") { const fragment = proof.fragment; if (!uiInputHost().fragment.matches(fragment, field) || proof.builder !== builder || proof.token !== token) return step("rejected", "paged-evidence-release-owner"); const receipt = fragment.release(token); if (!receipt || !uiInputHost().release.matches(receipt, fragment, token)) return step("rejected", "paged-evidence-release-refused", 128); proof.release = receipt; if (copiedState(token) === proof) { builder.#inputOffset = 0; proof.phase = "source-consuming"; } else proof.phase = "issued"; return step("pending", "paged-evidence-release", 128); }
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
      if (uiInputHost().release.matchesSourceDetached(proof.release, field, token)) { const fragment = proof.fragment; if (builder.#input === fragment && fragment !== null) return step("rejected", "paged-evidence-input-held"); if (builder.#fragment === fragment) builder.#fragment = null; if (builder.#copyFragment === fragment) builder.#copyFragment = null; if (builder.#lastFragment === fragment) builder.#lastFragment = null; if (builder.#proof === token) builder.#proof = null; if (builder.#copyProof === token) builder.#copyProof = null; if (builder.#release === proof.release) builder.#release = null; if (builder.#copyRelease === proof.release) builder.#copyRelease = null; proof.fragment = null; proof.field = null; proof.builder = null; proof.phase = "source-detached"; return step("pending", "paged-evidence-ui-detach", 64); }
      return childStep({ ...field.detachInputRelease(proof.release, token, grant), phase: "paged-evidence-source-detach" }, grant);
    }
    if (proof.phase === "source-detached") {
      if (!proof.release) return step("rejected", "paged-evidence-release-missing");
      if (uiInputHost().release.matchesSettled(proof.release, token)) { proof.phase = "source-settled"; return step("pending", "paged-evidence-ui-settle", 64); }
      return childStep({ ...field.settleInputRelease(proof.release, token, grant), phase: "paged-evidence-source-settle" }, grant);
    }
    if (proof.phase === "source-settled") { if (!proof.release || !uiInputHost().release.matchesSettled(proof.release, token)) return step("rejected", "paged-evidence-settled-owner"); proof.release = null; proof.token = null; proof.phase = "domain-retired"; return step("pending", "paged-evidence-capsule-clear", 64); }
    return step(OwnedUiOperationPayloadBuilder.evidenceEmpty(token) ? "complete" : "rejected", "paged-evidence-body");
  }
  static matchesPage(builder: OwnedUiOperationPayloadBuilder, page: UiResidentPageSeam, resident: UiResidentPayloadSeam): boolean { return builder.#resident === resident && builder.#head === page && builder.#tail === page && (builder.#writer === page || builder.#writer === null); }
  static installPage(builder: OwnedUiOperationPayloadBuilder, page: UiResidentPageSeam, resident: UiResidentPayloadSeam, grant: UiIndexGrant): RetainedUiWireStep { if (!admitted(grant, 64)) return step("blocked", "paged-page-install"); if (builder.#resident !== resident || builder.#head || builder.#tail || builder.#writer || builder.#failure !== NO_FAILURE || builder.#closing || !residentHost().payload.matchesPageConstruction(resident, page)) return step("rejected", "paged-page-install"); builder.#head = page; builder.#tail = page; builder.#writer = page; builder.#written = 0; return step("pending", "paged-page-install", 64); }
  static matchesPageDetached(builder: OwnedUiOperationPayloadBuilder, page: UiResidentPageSeam, proof: unknown, resident: UiResidentPayloadSeam): boolean { return builder.#resident === resident && builder.#head === null && builder.#tail === null && builder.#writer === null && builder.#failure === NO_FAILURE && residentHost().payload.matchesPageRetirement(resident, page, proof); }
  static detachPage(builder: OwnedUiOperationPayloadBuilder, page: UiResidentPageSeam, proof: unknown, resident: UiResidentPayloadSeam, grant: UiIndexGrant): RetainedUiWireStep { if (!admitted(grant, 64)) return step("blocked", "paged-page-detach"); if (builder.#resident !== resident || !OwnedUiOperationPayloadBuilder.matchesPage(builder, page, resident) || builder.#failure !== NO_FAILURE || !residentHost().payload.matchesPageRetirement(resident, page, proof)) return step("rejected", "paged-page-detach"); builder.#head = null; builder.#tail = null; builder.#writer = null; return step("complete", "paged-page-detach", 64); }
  static readerIsEmpty(builder: OwnedUiOperationPayloadBuilder): boolean { return builder.#reader === null; }
  static readerAvailable(builder: OwnedUiOperationPayloadBuilder, resident: UiResidentPayloadSeam): boolean { return builder.#resident === resident && builder.#reader === null && builder.#readerPhase === "none" && builder.#failure === NO_FAILURE && !builder.#closing; }
  static matchesReader(builder: OwnedUiOperationPayloadBuilder, reader: UiResidentPayloadReaderSeam, resident: UiResidentPayloadSeam): boolean { return builder.#resident === resident && builder.#reader === reader && builder.#readerPhase === "held"; }
  static installReader(builder: OwnedUiOperationPayloadBuilder, reader: UiResidentPayloadReaderSeam, resident: UiResidentPayloadSeam, grant: UiIndexGrant): RetainedUiWireStep { if (!admitted(grant, 64)) return step("blocked", "paged-reader-install"); if (!OwnedUiOperationPayloadBuilder.readerAvailable(builder, resident) || !residentHost().payload.matchesReaderConstruction(resident, reader)) return step("rejected", "paged-reader-install"); builder.#reader = reader; builder.#readerPhase = "held"; return step("pending", "paged-reader-install", 64); }
  static readerEof(builder: OwnedUiOperationPayloadBuilder, resident: UiResidentPayloadSeam, consumed: bigint): boolean { return builder.#resident === resident && builder.#failure === NO_FAILURE && builder.#copyPhase === "ready" && builder.#field !== null && builder.#field.complete && consumed === builder.#field.value.byteLength && consumed === builder.#copied; }
  static detachReader(builder: OwnedUiOperationPayloadBuilder, reader: UiResidentPayloadReaderSeam, proof: UiResidentReaderRetirementSeam, resident: UiResidentPayloadSeam, grant: UiIndexGrant): RetainedUiWireStep { if (!admitted(grant, 64)) return step("blocked", "paged-reader-detach"); if (builder.#resident !== resident || builder.#failure !== NO_FAILURE || builder.#reader !== reader && !(builder.#reader === null && builder.#readerPhase === "none") || !residentHost().readerRetirement.matchesBody(proof, reader, resident)) return step("rejected", "paged-reader-detach"); builder.#reader = proof; builder.#readerPhase = "detached"; return step("pending", "paged-reader-detach", 64); }
  static matchesReaderDetached(builder: OwnedUiOperationPayloadBuilder, reader: UiResidentPayloadReaderSeam, proof: UiResidentReaderRetirementSeam, resident: UiResidentPayloadSeam): boolean { return builder.#resident === resident && builder.#reader === proof && builder.#readerPhase === "detached" && residentHost().payload.matchesReaderBinding(resident, reader, proof); }
  static settleReader(builder: OwnedUiOperationPayloadBuilder, reader: UiResidentPayloadReaderSeam, proof: UiResidentReaderRetirementSeam, resident: UiResidentPayloadSeam, grant: UiIndexGrant): RetainedUiWireStep { if (!admitted(grant, 64)) return step("blocked", "paged-reader-settle"); if (!OwnedUiOperationPayloadBuilder.matchesReaderDetached(builder, reader, proof, resident) || !residentHost().readerRetirement.matchesDetached(proof, reader, resident)) return step("rejected", "paged-reader-settle"); builder.#reader = null; builder.#readerPhase = "settled"; return step("complete", "paged-reader-settle", 64); }
  static matchesReaderSettled(builder: OwnedUiOperationPayloadBuilder, reader: UiResidentPayloadReaderSeam, proof: UiResidentReaderRetirementSeam, resident: UiResidentPayloadSeam): boolean { return builder.#resident === resident && builder.#reader === null && builder.#readerPhase === "settled" && residentHost().payload.matchesReaderBinding(resident, reader, proof); }
  static construct(field: UiInputField, resident: UiResidentPayloadSeam, grant: UiIndexGrant): OwnedUiOperationPayloadBuilder {
    if (!admitted(grant, 272) || !residentHost().payload.matchesBuilderConstruction(resident, field)) throw new Error("Invalid original builder reservation"); return new OwnedUiOperationPayloadBuilder(MINT, field, resident);
  }
  static bindSource(builder: OwnedUiOperationPayloadBuilder, resident: UiResidentPayloadSeam, grant: UiIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 64)) return step("blocked", "paged-source-bind"); if (!residentHost().payload.matchesBuilderPhase(resident, builder, "source-installing") || builder.#closing || builder.#failure !== NO_FAILURE || !builder.#field) return step("rejected", "paged-source-bind");
    if (!builder.#field.bind(builder)) return step("rejected", "paged-source-bind", 64); builder.#bound = uiInputHost().field.matchesBuilder(builder.#field, builder); return step(builder.#bound ? "pending" : "rejected", "paged-source-bind", 64);
  }
  static finalize(builder: OwnedUiOperationPayloadBuilder, resident: UiResidentPayloadSeam, grant: UiIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 64)) return step("blocked", "paged-finalization"); if (!residentHost().payload.matchesBuilderPhase(resident, builder, "witness-ready") || !builder.#bound || builder.#closing || builder.#failure !== NO_FAILURE) return step("rejected", "paged-finalization"); Object.freeze(builder); return step("pending", "paged-finalization", 64);
  }
  static healthy(builder: OwnedUiOperationPayloadBuilder): boolean { return !builder.#closing && builder.#failure === NO_FAILURE && builder.#bound; }
  static empty(builder: OwnedUiOperationPayloadBuilder): boolean { return builder.#empty(); }
  static bodyEmpty(builder: OwnedUiOperationPayloadBuilder): boolean { return builder.#failure === NO_FAILURE && builder.#closing && !builder.#fragment && !builder.#proof && !builder.#release && !builder.#input && !builder.#copyFragment && !builder.#lastFragment && !builder.#copyProof && !builder.#copyRelease && !builder.#head && !builder.#tail && !builder.#writer && !builder.#reader && builder.#readerPhase !== "held" && builder.#readerPhase !== "detached"; }
  static sourceDetached(builder: OwnedUiOperationPayloadBuilder): boolean { return builder.#field === null && !builder.#bound && OwnedUiOperationPayloadBuilder.bodyEmpty(builder); }
  static matchesRetirementOwner(builder: OwnedUiOperationPayloadBuilder, field: unknown, witness: unknown): boolean { return builder.#resident !== null && residentHost().payload.matchesBuilderRetirement(builder.#resident, builder, field, witness); }
  static detachRetirementSource(builder: OwnedUiOperationPayloadBuilder, resident: UiResidentPayloadSeam, grant: UiIndexGrant): RetainedUiWireStep { if (!admitted(grant, 64)) return step("blocked", "paged-builder-source-detach"); if (builder.#resident !== resident || !OwnedUiOperationPayloadBuilder.bodyEmpty(builder) || !residentHost().payload.matchesBuilderRetirementPhase(resident, builder, "binding-detaching")) return step("rejected", "paged-builder-source-detach"); builder.#field = null; builder.#bound = false; return step("pending", "paged-builder-source-detach", 64); }
  static finishRetirement(builder: OwnedUiOperationPayloadBuilder, resident: UiResidentPayloadSeam, grant: UiIndexGrant): RetainedUiWireStep { if (!admitted(grant, 64)) return step("blocked", "paged-builder-binding-finish"); if (builder.#resident !== resident || !OwnedUiOperationPayloadBuilder.sourceDetached(builder) || !residentHost().payload.matchesBuilderRetirementPhase(resident, builder, "binding-settled")) return step("rejected", "paged-builder-binding-finish"); builder.#resident = null; builder.#sourceKind = null; builder.#phase = "closed"; return step("complete", "paged-builder-binding-finish", 64); }
  static begin(owner: OwnedUiInstance, activation: UiActivationLease, lifetime: UiInstanceLifetime, field: unknown, resident: unknown, grant: UiIndexGrant): OwnedUiOperationPayloadAdmission {
    if (!admitted(grant, 32)) return { step: step("blocked", "paged-admission"), builder: null };
    if (!uiInputHost().field.matchesOwner(field, owner, activation, lifetime) || !residentHost().payload.matchesOwner(resident, owner, activation, lifetime)) return { step: step("rejected", "paged-admission-owner"), builder: null };
    return resident.beginBuilder(field, grant);
  }
  get failure(): unknown { return this.#failure === NO_FAILURE ? null : this.#failure; }
  #retainFailure(error: unknown): void { if (this.#failure !== NO_FAILURE && !Object.is(this.#failure, error)) throw error; this.#failure = error; }

  //#region 📥️FixedPageCopy
  #copyChild(current: RetainedUiWireStep, grant: UiIndexGrant): RetainedUiWireStep {
    const result = childStep(current, grant); if (result.kind === "rejected" && this.#failure === NO_FAILURE) this.#retainFailure(current); return result;
  }
  #settleCopy(grant: UiIndexGrant): RetainedUiWireStep {
    if (this.#copyPhase === "proof") { const current = this.#resident!.beginEvidence(this, grant); const forwarded = this.#copyChild(current.step, grant); if (forwarded.kind !== "rejected" && current.step.kind === "ready") { const fragment = this.#copyFragment; if (!fragment || !this.#field || !OwnedUiOperationInputCopied.matches(current.evidence, fragment, this.#field, this, fragment.offset, fragment.length)) throw new Error("Missing original copied evidence"); this.#copyProof = current.evidence; this.#copyPhase = "receipt"; } return forwarded; }
    if (this.#copyPhase === "receipt") { const current = this.#resident!.advanceEvidence(this, grant); const forwarded = this.#copyChild(current, grant); if (forwarded.kind !== "rejected" && current.kind === "complete") this.#copyPhase = "range-observe"; return forwarded; }
    if (!admitted(grant, 64)) return step("blocked", "paged-range-observation");
    if (!this.#field || this.#input || this.#copyFragment || this.#copyProof || this.#copyRelease || this.#sourceKind !== null || this.#field.consumed !== this.#copied || this.#field.complete !== (this.#copied === this.#field.value.byteLength)) throw new Error("Copied range differs from original source");
    this.#inputOffset = 0; this.#copyPhase = this.#field.complete ? "ready" : "idle"; return step("pending", "paged-range-observation", 64);
  }
  advance(grant: UiIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 1)) return step("blocked", "paged-copy"); if (this.#closing || !this.#bound || this.#failure !== NO_FAILURE || !residentHost().payload.matchesBuilderLive(this.#resident, this)) return step("rejected", "paged-copy");
    try {
      if (this.#copyPhase === "ready") return step("ready", "paged-copy");
      if (this.#copyPhase === "proof" || this.#copyPhase === "receipt" || this.#copyPhase === "range-observe") return this.#settleCopy(grant);
      if (this.#copyPhase === "idle") {
        if (!admitted(grant, 128)) return step("blocked", "paged-input-admit");
        const fragment = this.#field!.fragment; if (!fragment || fragment === this.#lastFragment) return step("blocked", "paged-source-continuation");
        if (!uiInputHost().fragment.matches(fragment, this.#field!) || fragment.offset !== this.#copied || fragment.offset + BigInt(fragment.length) > this.#field!.value.byteLength) return step("rejected", "paged-source-range");
        this.#input = fragment; this.#copyFragment = fragment; this.#inputOffset = 0; this.#copyPhase = "copy"; return step("pending", "paged-input-admit", 128);
      }
      if (this.#copyPhase === "copy" && this.#inputOffset !== this.#input!.length && !this.#writer) { if (this.#head) return step("blocked", "paged-page-window"); this.#copyPhase = "page"; }
      if (this.#copyPhase === "page") { const remaining = this.#field!.value.byteLength - this.#copied; const length = Number(remaining < 256n ? remaining : 256n); const current = this.#resident!.beginPage(this, length, grant); const result = this.#copyChild(current.step, grant); if (result.kind !== "rejected" && current.step.kind === "ready") this.#copyPhase = "page-observe"; return result; }
      if (this.#copyPhase === "page-observe") { if (!admitted(grant, 64)) return step("blocked", "paged-page-observation"); if (!this.#writer || !OwnedUiOperationPayloadBuilder.matchesPage(this, this.#writer, this.#resident!) || residentHost().payload.pageLength(this.#resident, this, this.#writer) === null) return step("rejected", "paged-page-owner"); this.#copyPhase = "allocate"; return step("pending", "paged-page-observation", 64); }
      if (this.#copyPhase === "allocate") { const current = this.#writer!.allocate(grant); const result = this.#copyChild(current, grant); if (result.kind !== "rejected" && current.kind === "ready") this.#copyPhase = "allocate-observe"; return result; }
      if (this.#copyPhase === "allocate-observe") { if (!admitted(grant, 64)) return step("blocked", "paged-allocation-observation"); this.#copyPhase = "copy"; return step("pending", "paged-allocation-observation", 64); }
      if (this.#copyPhase === "seal") { const current = this.#writer!.seal(grant); const result = this.#copyChild(current, grant); if (result.kind !== "rejected" && current.kind === "ready") this.#copyPhase = "seal-observe"; return result; }
      if (this.#copyPhase === "seal-observe") { if (!admitted(grant, 64)) return step("blocked", "paged-seal-observation"); this.#writer = null; this.#copyPhase = "copy"; return step("pending", "paged-seal-observation", 64); }
      if (this.#copyPhase === "write") {
        if (typeof this.#sourceKind !== "number") return step("rejected", "paged-byte-latch"); const current = this.#writer!.writeByte(this.#sourceKind, grant); const result = this.#copyChild(current, grant);
        if (result.kind !== "rejected" && current.kind === "pending" && current.items === 1 && current.bytes === 1) { this.#inputOffset++; this.#copied++; this.#written++; this.#sourceKind = null; const length = residentHost().payload.pageLength(this.#resident, this, this.#writer); if (length === null || this.#written > length) return step("rejected", "paged-page-length", 1); this.#copyPhase = this.#written === length ? "seal" : "copy"; } return result;
      }
      if (this.#inputOffset === this.#input!.length) { if (!admitted(grant, 128)) return step("blocked", "paged-input-copy-detach"); this.#input = null; this.#copyPhase = "proof"; return step("pending", "paged-input-copy-detach", 128); }
      this.#sourceKind = this.#input!.byteAt(this.#inputOffset, this); this.#copyPhase = "write"; return step("pending", "paged-input-byte", 1);
    } catch (error) { this.#retainFailure(error); return step("rejected", "paged-copy-fault"); }
  }
  beginRead(grant: UiIndexGrant): OwnedUiOperationPayloadReadAdmission { if (!this.#resident || this.#closing || this.#failure !== NO_FAILURE) return { step: step("rejected", "paged-reader-owner"), reader: null }; return this.#resident.beginReader(this, grant); }
  //#endregion 📥️FixedPageCopy

  //#region ♻️BoundedClose
  beginClose(): void { this.#closing = true; }
  closeStep(grant: UiIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 128)) return step("blocked", "paged-builder-close"); if (!this.#closing) throw new Error("Paged builder close has not begun");
    try {
      if (this.#phase === "closed") return step("complete", "paged-builder-close");
      if (this.#failure !== NO_FAILURE) return step("rejected", "paged-builder-fault-held");
      if (OwnedUiOperationPayloadBuilder.bodyEmpty(this)) return step("blocked", "paged-builder-binding-retirement");
      if (this.#field && uiInputHost().field.matchesBuilder(this.#field, this)) return step("blocked", "paged-evidence-retirement-admission");
      if (residentHost().isReader(this.#reader)) return childStep(this.#resident!.closeReader(this.#reader, grant), grant);
      if (this.#reader) return step("blocked", "paged-reader-binding");
      if (this.#copyPhase === "proof" || this.#copyPhase === "receipt" || this.#copyPhase === "range-observe") return this.#settleCopy(grant);
      if (this.#phase === "open") { this.#bound = uiInputHost().field.matchesBuilder(this.#field, this); if (this.#bound) { const fragment = this.#copyFragment ?? this.#field!.fragment; this.#fragment = fragment === this.#lastFragment ? null : fragment; this.#field!.beginClose(); } this.#input = null; this.#copyFragment = null; this.#phase = this.#fragment ? "proof" : "pages"; return step("pending", "paged-input-detach", 128); }
      if (this.#phase === "proof") return step("blocked", "paged-evidence-admission");
      if (this.#phase === "release") { const receipt = this.#fragment!.release(this.#proof); if (!receipt) return step("rejected", "paged-input-release-refused", 128); this.#release = receipt; this.#phase = "receipt"; return step("pending", "paged-input-release", 128); }
      if (this.#phase === "receipt") { if (!uiInputHost().release.matches(this.#release, this.#fragment!, this.#proof)) return step("rejected", "paged-input-release-authority"); this.#fragment = null; this.#proof = null; this.#release = null; this.#bound = false; this.#phase = "pages"; return step("pending", "paged-input-release-retire", 128); }
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
  private constructor(mint: object, builder: OwnedUiOperationPayloadBuilder, resident: UiResidentPayloadSeam, fragment: UiInputFragment) {
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
  private constructor(mint: object, builder: OwnedUiOperationPayloadBuilder, resident: UiResidentPayloadSeam, fragment: UiInputFragment) {
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
