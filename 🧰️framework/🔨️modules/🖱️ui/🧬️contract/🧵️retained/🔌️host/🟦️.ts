/** 🔌️ Host seam of the retained UI: what an owned UI instance needs from the actor activation that hosts it, declared by the UI so the UI never imports the actor. The actor implements these shapes (the guest lifetime identity and patch receipt mirror the actor wire contract) and binds its brand checks once through {@link bindUiPatchAuthority}. A leaf with no runtime imports, so the binding slot is initialised before either side evaluates. */

/** 📏️ The bounded budget one retained-UI step may spend; the same shape the value and actor modules grant. */
export type UiIndexGrant = { readonly maxItems: number; readonly maxBytes: number };

/** 🪪️ The exact guest lifetime an instance, its surfaces and its patches belong to. */
export type UiInstanceLifetime = { readonly activationGeneration: bigint; readonly instanceId: number; readonly guestLifetime: bigint };

/** 🧾️ The guest-issued identity of one patch within an instance lifetime. */
export type UiPatchReceipt = { readonly lifetime: UiInstanceLifetime; readonly patchSequence: bigint };

/** 🔐️ The live activation an instance is bound to; revocation makes {@link UiActivationLease.assertActive} throw. */
export interface UiActivationLease {
  readonly actorId: string;
  readonly activationGeneration: bigint;
  assertActive(): void;
}

/** 🩹️ The facts of one patch a host hands to the instance. */
export type UiPatchValue = { readonly activation: UiActivationLease; readonly lifetime: UiInstanceLifetime; readonly receipt: UiPatchReceipt; readonly surface: string; readonly baseRevision: number; readonly revision: number; readonly operationCount: number };

/** 🩹️ A private claim on one host patch: its facts, its operations and its input accounting. */
export interface UiPatchSource {
  readonly value: UiPatchValue;
  readonly inputRetired: boolean;
  operation(index: number): unknown;
  acceptInput(claim: object): boolean;
  releaseInput(token: object): boolean;
}

/** 🪪️ The host's brand checks over its patch sources and submission receipts. */
export interface UiPatchAuthority {
  matches(source: unknown, activation: UiActivationLease, lifetime: UiInstanceLifetime): source is UiPatchSource;
  matchesOwner(source: unknown, owner: unknown): source is UiPatchSource;
  matchesReceipt(receipt: unknown, source: object, token: object): boolean;
}

let patchAuthority: UiPatchAuthority | null = null;
/** 🔗️ Binds, once, the host's patch brand checks; an unbound seam refuses every source and receipt. */
export function bindUiPatchAuthority(authority: UiPatchAuthority): void {
  if (patchAuthority) throw new Error("UI patch authority is already bound");
  patchAuthority = authority;
}
/** 🪪️ The bound host patch brand checks, or a refusing stand-in while unbound. */
export function uiPatchAuthority(): UiPatchAuthority { return patchAuthority ?? UNBOUND; }
const UNBOUND: UiPatchAuthority = Object.freeze({ matches: (_source: unknown, _activation: UiActivationLease, _lifetime: UiInstanceLifetime): _source is UiPatchSource => false, matchesOwner: (_source: unknown, _owner: unknown): _source is UiPatchSource => false, matchesReceipt: () => false });
