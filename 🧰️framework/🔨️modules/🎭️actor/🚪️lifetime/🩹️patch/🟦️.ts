//#region 🧬️IssuedPatchReceipt
import type { ActorInstanceLifetime } from "../🟦️.ts";

export type ActorUiPatchReceipt = { readonly lifetime: ActorInstanceLifetime; readonly patchSequence: bigint };
export const ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES = 35;

/** 📤️ Encodes the exact guest-issued patch identity without deriving authority from surface revisions. */
export function encodeActorUiPatchReceipt(receipt: ActorUiPatchReceipt): Uint8Array {
  const lifetime = receipt.lifetime;
  const valid = (value: bigint): boolean => typeof value === "bigint" && value > 0n && value <= 0xffffffffffffffffn;
  if (!valid(lifetime.activationGeneration) || !valid(lifetime.guestLifetime) || !valid(receipt.patchSequence) || !Number.isInteger(lifetime.instanceId) || lifetime.instanceId < 0 || lifetime.instanceId > 0xffffffff) throw new Error("actor-ui-patch.invalid-authority");
  const output = new Uint8Array(ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES);
  let length = 0;
  const put = (initial: bigint) => {
    let rest = initial;
    do {
      const byte = Number(rest & 127n);
      rest >>= 7n;
      output[length++] = byte | (rest === 0n ? 0 : 128);
    } while (rest !== 0n);
  };
  put(lifetime.activationGeneration); put(BigInt(lifetime.instanceId)); put(lifetime.guestLifetime); put(receipt.patchSequence);
  return output.slice(0, length);
}

/** 📥️ Rejects noncanonical, truncated, overflowed, zero and trailing receipt authority. */
export function decodeActorUiPatchReceipt(bytes: Uint8Array): ActorUiPatchReceipt {
  if (!(bytes instanceof Uint8Array) || bytes.length < 4 || bytes.length > ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES) throw new Error("actor-ui-patch.envelope");
  let offset = 0;
  const get = (maximum: bigint, nonzero: boolean) => {
    let value = 0n;
    for (let index = 0; index < 10; index += 1) {
      const byte = bytes[offset++];
      if (byte === undefined) throw new Error("actor-ui-patch.truncated");
      value |= BigInt(byte & 127) << BigInt(index * 7);
      if ((byte & 128) === 0) {
        if ((index !== 0 && byte === 0) || value > maximum || nonzero && value === 0n) throw new Error("actor-ui-patch.noncanonical-authority");
        return value;
      }
    }
    throw new Error("actor-ui-patch.overlong");
  };
  const activationGeneration = get(0xffffffffffffffffn, true);
  const instanceId = Number(get(0xffffffffn, false));
  const guestLifetime = get(0xffffffffffffffffn, true);
  const patchSequence = get(0xffffffffffffffffn, true);
  if (offset !== bytes.length) throw new Error("actor-ui-patch.trailing");
  return { lifetime: { activationGeneration, instanceId, guestLifetime }, patchSequence };
}

/** 🪪️ Compares wire identity only; equality does not mint a native retirement capability. */
export function actorUiPatchReceiptEquals(left: ActorUiPatchReceipt, right: ActorUiPatchReceipt): boolean {
  return left.lifetime.activationGeneration === right.lifetime.activationGeneration && left.lifetime.instanceId === right.lifetime.instanceId && left.lifetime.guestLifetime === right.lifetime.guestLifetime && left.patchSequence === right.patchSequence;
}

/** 🩹️ Enforces the canonical single-patch turn cardinality before receipt consumption. */
export function validateActorUiPatchPairing(patchCount: number, receipt: ActorUiPatchReceipt | null | undefined): void {
  if (patchCount !== 0 && patchCount !== 1 || (patchCount === 1) !== (receipt != null)) throw new Error("actor-ui-patch.pairing");
  if (receipt != null) encodeActorUiPatchReceipt(receipt);
}
//#endregion 🧬️IssuedPatchReceipt

//#region 🧪️IssuedPatchReceiptLaws
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️actor-ui-patch-receipt-matches-shared-canonical-vectors-and-the-independ/🟦️.ts");
  await registerTests1(import.meta.vitest, { ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES, actorUiPatchReceiptEquals, decodeActorUiPatchReceipt, encodeActorUiPatchReceipt, validateActorUiPatchPairing }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️IssuedPatchReceiptLaws
