import type { OwnedResidentLedger, ResidentGrant, ResidentStep } from "@semio-tech/value-ts/resident";
import type { UiActivationLease } from "../🟦️.ts";

/** 🏦️ Composition host seam of the shared resident pool: what the pool needs from the shard composition that owns it, declared by the UI so the UI never imports the actor. The composition implements it and binds its brand checks once through {@link bindUiResidentComposition}. */

/** 🪣️ The composition as the resident pool sees it. */
export interface UiResidentComposition {
  prepareUiResidentPool(ledger: OwnedResidentLedger, grant: ResidentGrant): ResidentStep;
  installUiResidentPool(pool: object, grant: ResidentGrant): ResidentStep;
  ownsUiResidentPool(pool: unknown): boolean;
  captureUiResidentPoolFault(error: unknown): void;
}

/** 🪪️ The composition's brand checks. */
export interface UiResidentCompositionAuthority {
  matchesResidentLedger(client: unknown, ledger: unknown): client is UiResidentComposition;
  matchesActivation(client: unknown, activation: unknown): activation is UiActivationLease;
  ownsPool(client: UiResidentComposition, pool: unknown): boolean;
}

let authority: UiResidentCompositionAuthority | null = null;
/** 🔗️ Binds, once, the composition's brand checks. */
export function bindUiResidentComposition(bound: UiResidentCompositionAuthority): void {
  if (authority) throw new Error("UI resident composition is already bound");
  authority = bound;
}
/** ✔️ The bound composition brand checks; an unbound seam is a wiring fault. */
export function uiResidentComposition(): UiResidentCompositionAuthority {
  if (!authority) throw new Error("UI resident composition is unbound");
  return authority;
}
