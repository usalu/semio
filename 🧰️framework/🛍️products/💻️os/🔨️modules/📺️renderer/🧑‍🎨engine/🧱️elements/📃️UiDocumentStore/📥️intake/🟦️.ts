//#region 🧬️NativeIntakeContract
import { OwnedNativeUiPatchAuthority } from "../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import type { OwnedUiInstance, OwnedUiInstanceSurface, OwnedUiInstancePatch, OwnedUiSurfaceLookup, OwnedUiPatchAcknowledgement } from "../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts";
import type { NumericIndexGrant } from "../../../../../../../../🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
import type { RetainedUiWireStep } from "../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🟦️.ts";

type Phase = "lookup" | "lookup-read" | "lookup-take" | "lookup-close" | "patch" | "offer" | "input" | "input-release" | "seal" | "publication" | "ack" | "patch-close" | "ready";
const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
//#endregion 🧬️NativeIntakeContract

//#region 📥️ExactPatchIntake
/** 📥️ One native patch advances through the exact aggregate without copying operations or issuing raw ACKs. */
export class OwnedUiPatchIntake {
  #owner: OwnedUiInstance | null;
  #source: OwnedNativeUiPatchAuthority | null;
  #lookup: OwnedUiSurfaceLookup | null = null;
  #surface: OwnedUiInstanceSurface | null = null;
  #patch: OwnedUiInstancePatch | null = null;
  #phase: Phase = "lookup";
  #ordinal = 0;
  #closing = false;
  #closed = false;
  #failure: string | null = null;

  constructor(owner: OwnedUiInstance, source: OwnedNativeUiPatchAuthority) {
    if (!OwnedNativeUiPatchAuthority.matchesOwner(source, owner)) throw new Error("Foreign native UI intake owner");
    this.#owner = owner; this.#source = source; Object.freeze(this);
  }
  get failure(): string | null { return this.#failure; }
  peekAcknowledgement(): OwnedUiPatchAcknowledgement | null { return this.#patch && !this.#patch.terminalIsEmpty() ? this.#patch.peekAcknowledgement() : null; }
  acceptAcknowledgement(receipt: unknown): boolean {
    if (!this.#patch || this.#patch.terminalIsEmpty() || !this.#patch.acceptAcknowledgement(receipt)) return false;
    this.#phase = "patch-close"; this.#patch.beginClose(); return true;
  }
  takeSurface(): OwnedUiInstanceSurface | null { if (this.#closing || this.#failure || this.#phase !== "ready") return null; const result = this.#surface; this.#surface = null; return result; }

  //#region ▶️Advance
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "intake"); if (this.#closing || this.#failure) return step("rejected", "intake");
    try {
      switch (this.#phase) {
        case "lookup": {
          const source = this.#source!.value; this.#lookup = this.#owner!.beginSurfaceLookup(source.activation, source.lifetime, source.surface);
          if (!this.#lookup) return step("blocked", "intake-lookup-capacity"); this.#phase = "lookup-read"; return step("pending", "intake-lookup", 256 + source.surface.length * 5);
        }
        case "lookup-read": {
          const current = this.#lookup!.advance(grant); if (current.kind === "rejected") { this.#failure = this.#lookup!.failure ?? "Native surface lookup rejected"; return current; }
          if (current.kind === "ready") this.#phase = "lookup-take"; return { ...current, kind: "pending" };
        }
        case "lookup-take": {
          this.#surface = this.#lookup!.takeResult(); if (!this.#surface) throw new Error("Native surface lookup result is missing"); this.#lookup!.beginClose(); this.#phase = "lookup-close"; return step("pending", "intake-surface", 64);
        }
        case "lookup-close": {
          const current = this.#lookup!.closeStep(grant); const result = this.#closeResult(current, this.#lookup!.failure); if (current.kind === "complete") { this.#lookup = null; this.#phase = "patch"; } return result;
        }
        case "patch": this.#patch = this.#owner!.beginPatch(this.#source!, this.#surface!); this.#phase = "offer"; return step("pending", "intake-patch", 2048);
        case "offer": {
          if (this.#ordinal === this.#source!.value.operationCount) { this.#phase = "seal"; return step("pending", "intake-input-finished", 32); }
          if (!this.#patch!.offer(this.#ordinal)) return step("blocked", "intake-input-capacity"); this.#phase = "input"; return step("pending", "intake-offer", 2048);
        }
        case "input": {
          const current = this.#patch!.advance(grant); if (current.kind === "rejected") { this.#failure = this.#patch!.failure ?? "Native input rejected"; return current; }
          if (current.kind === "ready") this.#phase = "input-release"; return { ...current, kind: "pending" };
        }
        case "input-release": {
          const token = this.#patch!.peekInputReceipt(); if (!token || !this.#patch!.releaseInputReceipt(token)) return step("blocked", "intake-input-retirement"); this.#ordinal++; this.#phase = "offer"; return step("pending", "intake-input-retirement", 256);
        }
        case "seal": this.#patch!.finishInput(); this.#phase = "publication"; return step("pending", "intake-seal", 64);
        case "publication": {
          const current = this.#patch!.advance(grant); if (current.kind === "rejected") { this.#failure = this.#patch!.failure ?? "Native publication rejected"; return current; }
          if (current.kind === "ready") this.#phase = "ack"; return { ...current, kind: "pending" };
        }
        case "ack": return step("blocked", "intake-publication-receipt");
        case "patch-close": {
          const current = this.#patch!.closeStep(grant); const result = this.#closeResult(current, this.#patch!.failure); if (current.kind === "complete") { this.#patch = null; this.#phase = "ready"; } return result;
        }
        case "ready": return step("ready", "intake-ready");
      }
    } catch (error) { this.#failure = error instanceof Error ? error.message : "Native intake failed"; return step("rejected", "intake", 4096); }
  }
  //#endregion ▶️Advance

  //#region ♻️Retirement
  #closeResult(current: RetainedUiWireStep, failure: string | null): RetainedUiWireStep {
    if (current.kind === "rejected") { this.#failure ??= failure ?? `Native intake close rejected: ${current.phase}`; return current; }
    return current.kind === "complete" ? { ...current, kind: "pending" } : current;
  }
  beginClose(): void {
    if (this.#closing) return; this.#closing = true; this.#lookup?.beginClose(); if (this.#patch && !this.#patch.terminalIsEmpty()) this.#patch.beginClose();
  }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "intake-close"); if (!this.#closing) throw new Error("Native intake close has not begun"); if (this.#closed) return step("complete", "intake-close");
    if (this.#lookup) { if (this.#lookup.terminalIsEmpty()) { this.#lookup = null; return step("pending", "intake-lookup-release", 32); } return this.#closeResult(this.#lookup.closeStep(grant), this.#lookup.failure); }
    if (this.#patch) {
      if (this.#patch.terminalIsEmpty()) { this.#patch = null; return step("pending", "intake-patch-release", 32); }
      const input = this.#patch.peekInputReceipt(); if (input) return this.#patch.releaseInputReceipt(input) ? step("pending", "intake-input-retirement", 256) : step("blocked", "intake-input-retirement");
      if (this.#patch.peekAcknowledgement()) return step("blocked", "intake-publication-receipt");
      return this.#closeResult(this.#patch.closeStep(grant), this.#patch.failure);
    }
    if (this.#surface) { this.#surface = null; return step("pending", "intake-surface-release", 32); }
    if (this.#source) { this.#source = null; return step("pending", "intake-source-release", 32); }
    this.#owner = null; this.#closed = true; return step("complete", "intake-close", 32);
  }
  terminalIsEmpty(): boolean { return this.#closed && !this.#owner && !this.#source && !this.#lookup && !this.#surface && !this.#patch; }
  //#endregion ♻️Retirement
}
//#endregion 📥️ExactPatchIntake
