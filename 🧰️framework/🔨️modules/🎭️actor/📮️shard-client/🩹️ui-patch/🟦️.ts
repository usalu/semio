import { type ActorInstanceLifetime, actorInstanceLifetimeEquals } from "../../🚪️lifetime/🟦️.ts";
import type { ActorUiPatchReceipt } from "../../🚪️lifetime/🩹️patch/🟦️.ts";
import { OwnedUiPatchInputAcceptance, OwnedUiPatchAcknowledgement, OwnedUiPatchInputRetirement } from "../../../🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts";
import { bindUiPatchAuthority, type UiActivationLease, type UiPatchSource } from "../../../🖱️ui/🧬️contract/🧵️retained/🔌️host/🟦️.ts";

/** 🏠️ The part of the instance owner a patch claim consults: the UI host that must own the patch's surface. */
export type NativeUiPatchOwner = { readonly host: unknown };
export type OwnedNativeUiPatchValue = { readonly activation: UiActivationLease; readonly lifetime: ActorInstanceLifetime; readonly receipt: ActorUiPatchReceipt; readonly surface: string; readonly baseRevision: number; readonly revision: number; readonly operationCount: number };
export type NativeUiPatchState = { readonly owner: NativeUiPatchOwner; readonly turn: object; readonly patch: object; readonly operations: readonly unknown[]; readonly value: OwnedNativeUiPatchValue; ordinal: number; read: boolean; original: unknown; input: OwnedUiPatchInputAcceptance | null; token: OwnedUiPatchAcknowledgement | null; submission: Promise<{ readonly receipt: OwnedNativeUiPatchSubmissionReceipt; readonly result: unknown }> | null };
/** 🩹️ One (patch, acknowledgement) pair of a turn's patch BATCH, as the host hands it back. */
export type OwnedUiPatchAcknowledgementEntry = { readonly source: OwnedNativeUiPatchAuthority; readonly token: OwnedUiPatchAcknowledgement };

const NATIVE_PATCH_MINT = Object.freeze({});
export let mintNativePatch: (state: NativeUiPatchState) => OwnedNativeUiPatchAuthority;
export let nativePatchState: (source: OwnedNativeUiPatchAuthority) => NativeUiPatchState;
export let mintNativeSubmission: (source: OwnedNativeUiPatchAuthority, token: OwnedUiPatchAcknowledgement) => OwnedNativeUiPatchSubmissionReceipt;

/** 💊️ A private claim on one patch returned by the original instance turn. */
export class OwnedNativeUiPatchAuthority implements UiPatchSource {
  readonly #state: NativeUiPatchState;
  private constructor(mint: object, state: NativeUiPatchState) { if (mint !== NATIVE_PATCH_MINT) throw new Error("actor-lifecycle.patch-mint"); this.#state = state; Object.freeze(this); }
  static { mintNativePatch = state => new OwnedNativeUiPatchAuthority(NATIVE_PATCH_MINT, state); nativePatchState = source => source.#state; }
  static matches(source: unknown, activation: UiActivationLease, lifetime: ActorInstanceLifetime): source is OwnedNativeUiPatchAuthority {
    return source !== null && typeof source === "object" && #state in source && source.#state.value.activation === activation && actorInstanceLifetimeEquals(source.#state.value.lifetime, lifetime);
  }
  static matchesOwner(source: unknown, owner: unknown): source is OwnedNativeUiPatchAuthority {
    return source !== null && typeof source === "object" && #state in source && source.#state.owner.host !== null && source.#state.owner.host === owner;
  }
  get value(): OwnedNativeUiPatchValue { return this.#state.value; }
  operation(index: number): unknown {
    const state = this.#state;
    if (!Number.isSafeInteger(index) || index !== state.ordinal || index >= state.value.operationCount) throw new Error("actor-lifecycle.patch-operation-index");
    if (!state.read) { state.original = state.operations[index]; state.read = true; }
    return state.original;
  }
  acceptInput(claim: OwnedUiPatchInputAcceptance): boolean {
    const state = this.#state;
    if (!state.read || state.input !== null && state.input !== claim || state.operations[state.ordinal] !== state.original || !OwnedUiPatchInputAcceptance.matches(claim, this, state.ordinal, state.original)) return false;
    state.input = claim;
    return true;
  }
  releaseInput(token: OwnedUiPatchInputRetirement): boolean {
    const state = this.#state;
    if (!state.read || !state.input || state.operations[state.ordinal] !== state.original || !OwnedUiPatchInputRetirement.matches(token, this, state.ordinal, state.original)) return false;
    state.read = false; state.original = undefined; state.input = null; state.ordinal++;
    return true;
  }
  /** 📥️ Confirms transferred UI inputs only; raw operation and turn roots remain retained separately. */
  get inputRetired(): boolean { return this.#state.ordinal === this.#state.value.operationCount && !this.#state.read; }
}

/** 📨️ A successful native submission is inseparable from both original acknowledgement authorities. */
export class OwnedNativeUiPatchSubmissionReceipt {
  readonly #source: OwnedNativeUiPatchAuthority;
  readonly #token: OwnedUiPatchAcknowledgement;
  private constructor(mint: object, source: OwnedNativeUiPatchAuthority, token: OwnedUiPatchAcknowledgement) { if (mint !== NATIVE_PATCH_MINT) throw new Error("actor-lifecycle.submission-mint"); this.#source = source; this.#token = token; Object.freeze(this); }
  static { mintNativeSubmission = (source, token) => new OwnedNativeUiPatchSubmissionReceipt(NATIVE_PATCH_MINT, source, token); }
  static matches(receipt: unknown, source: object, token: object): receipt is OwnedNativeUiPatchSubmissionReceipt {
    return receipt !== null && typeof receipt === "object" && #source in receipt && receipt.#source === source && receipt.#token === token;
  }
}

bindUiPatchAuthority({ matches: OwnedNativeUiPatchAuthority.matches, matchesOwner: OwnedNativeUiPatchAuthority.matchesOwner, matchesReceipt: OwnedNativeUiPatchSubmissionReceipt.matches });
