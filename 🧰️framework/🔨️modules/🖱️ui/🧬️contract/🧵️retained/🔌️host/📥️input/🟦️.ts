import type { UiActivationLease, UiInstanceLifetime, UiPatchReceipt } from "../🟦️.ts";

/** 📥️ Input host seam of the retained UI: what the resident payload and the paged operation builder need from the kernel's captured return input, declared by the UI so the UI never imports the kernel. The kernel implements these shapes and binds its brand checks once through {@link bindUiInputHost}. Arguments that are UI classes are typed `object`, so this leaf imports no UI module. */

/** 📏️ The bounded budget one input step may spend. */
export type UiInputGrant = { readonly maxItems: number; readonly maxBytes: number };
/** 🪜️ One bounded step of input transfer. */
export type UiInputStep = { readonly kind: "pending" | "ready" | "complete" | "blocked" | "rejected"; readonly items: number; readonly bytes: number };
/** 🏷️ The identity of one selected operation field. */
export type UiInputFieldValue = { readonly operation: number; readonly opcode: number; readonly node: bigint | null; readonly name: string; readonly byteLength: bigint; readonly receipt: UiPatchReceipt };

/** 🧾️ The proof that one fragment was copied or cancelled. */
export interface UiInputRelease { readonly kind: "copied" | "cancelled" }
/** 🪢️ The original observation retaining temporary UI aliases until two-way settlement. */
export type UiPayloadDetachment = object;
/** 📄️ One selected raw-page range. */
export interface UiInputFragment {
  readonly field: UiInputField | null;
  readonly offset: bigint;
  readonly length: number;
  byteAt(index: number, builder: unknown): number;
  release(proof: unknown): UiInputRelease | null;
}
/** ☑️ One selected operation field and its admitted consumer. */
export interface UiInputField {
  readonly value: UiInputFieldValue;
  readonly owner: object;
  readonly activation: UiActivationLease;
  readonly lifetime: UiInstanceLifetime;
  readonly fragment: UiInputFragment | null;
  readonly consumed: bigint;
  readonly complete: boolean;
  readonly residentPayloadDetachment: UiPayloadDetachment | null;
  bind(builder: object): boolean;
  beginClose(): void;
  detachBuilder(builder: object, proof: object, grant: UiInputGrant): UiInputStep;
  settleBuilder(proof: object, grant: UiInputGrant): UiInputStep;
  installResidentPayload(payload: object, grant: UiInputGrant): UiInputStep;
  residentPayload(scope: object): object | null;
  detachResidentPayload(payload: object, proof: object, grant: UiInputGrant): UiInputStep;
  settleResidentPayload(detachment: UiPayloadDetachment, proof: object, grant: UiInputGrant): UiInputStep;
  detachInputRelease(release: UiInputRelease, proof: unknown, grant: UiInputGrant): UiInputStep;
  settleInputRelease(release: UiInputRelease, proof: unknown, grant: UiInputGrant): UiInputStep;
  advance(grant: UiInputGrant, builder: unknown): UiInputStep;
}

/** 🪪️ The kernel's brand checks over its input classes. */
export interface UiInputHost {
  readonly field: {
    matchesOwner(field: unknown, owner: object, activation: UiActivationLease, lifetime: UiInstanceLifetime): field is UiInputField;
    matchesBuilder(field: unknown, builder: unknown): boolean;
    matchesBuilderDetached(field: unknown, proof: unknown): boolean;
    matchesBuilderSettled(field: unknown, proof: unknown): boolean;
    matchesResidentPayload(field: unknown, payload: unknown): boolean;
  };
  readonly fragment: { matches(fragment: unknown, field: object): fragment is UiInputFragment };
  readonly release: {
    matches(receipt: unknown, fragment: object, proof: unknown): receipt is UiInputRelease;
    matchesSourceDetached(receipt: unknown, field: unknown, proof: unknown): receipt is UiInputRelease;
    matchesSettled(receipt: unknown, proof: unknown): receipt is UiInputRelease;
  };
  readonly detachment: {
    matchesOwner(observation: unknown, field: unknown): observation is UiPayloadDetachment;
    matches(observation: unknown, field: unknown, payload: unknown): observation is UiPayloadDetachment;
    matchesSettled(observation: unknown, payload: unknown): observation is UiPayloadDetachment;
  };
}

let inputHost: UiInputHost | null = null;
/** 🔗️ Binds, once, the kernel's input brand checks. */
export function bindUiInputHost(host: UiInputHost): void {
  if (inputHost) throw new Error("UI input host is already bound");
  inputHost = host;
}
/** ✅️ The bound kernel input brand checks; an unbound seam is a wiring fault. */
export function uiInputHost(): UiInputHost {
  if (!inputHost) throw new Error("UI input host is unbound");
  return inputHost;
}
