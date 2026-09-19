"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES = void 0;
exports.encodeActorUiPatchReceipt = encodeActorUiPatchReceipt;
exports.decodeActorUiPatchReceipt = decodeActorUiPatchReceipt;
exports.actorUiPatchReceiptEquals = actorUiPatchReceiptEquals;
exports.validateActorUiPatchPairing = validateActorUiPatchPairing;
exports.ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES = 35;
/** 📤️ Encodes the exact guest-issued patch identity without deriving authority from surface revisions. */
function encodeActorUiPatchReceipt(receipt) {
    var lifetime = receipt.lifetime;
    var valid = function (value) { return typeof value === "bigint" && value > 0n && value <= 0xffffffffffffffffn; };
    if (!valid(lifetime.activationGeneration) || !valid(lifetime.guestLifetime) || !valid(receipt.patchSequence) || !Number.isInteger(lifetime.instanceId) || lifetime.instanceId < 0 || lifetime.instanceId > 0xffffffff)
        throw new Error("actor-ui-patch.invalid-authority");
    var output = new Uint8Array(exports.ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES);
    var length = 0;
    var put = function (initial) {
        var rest = initial;
        do {
            var byte = Number(rest & 127n);
            rest >>= 7n;
            output[length++] = byte | (rest === 0n ? 0 : 128);
        } while (rest !== 0n);
    };
    put(lifetime.activationGeneration);
    put(BigInt(lifetime.instanceId));
    put(lifetime.guestLifetime);
    put(receipt.patchSequence);
    return output.slice(0, length);
}
/** 📥️ Rejects noncanonical, truncated, overflowed, zero and trailing receipt authority. */
function decodeActorUiPatchReceipt(bytes) {
    if (!(bytes instanceof Uint8Array) || bytes.length < 4 || bytes.length > exports.ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES)
        throw new Error("actor-ui-patch.envelope");
    var offset = 0;
    var get = function (maximum, nonzero) {
        var value = 0n;
        for (var index = 0; index < 10; index += 1) {
            var byte = bytes[offset++];
            if (byte === undefined)
                throw new Error("actor-ui-patch.truncated");
            value |= BigInt(byte & 127) << BigInt(index * 7);
            if ((byte & 128) === 0) {
                if ((index !== 0 && byte === 0) || value > maximum || nonzero && value === 0n)
                    throw new Error("actor-ui-patch.noncanonical-authority");
                return value;
            }
        }
        throw new Error("actor-ui-patch.overlong");
    };
    var activationGeneration = get(0xffffffffffffffffn, true);
    var instanceId = Number(get(0xffffffffn, false));
    var guestLifetime = get(0xffffffffffffffffn, true);
    var patchSequence = get(0xffffffffffffffffn, true);
    if (offset !== bytes.length)
        throw new Error("actor-ui-patch.trailing");
    return { lifetime: { activationGeneration: activationGeneration, instanceId: instanceId, guestLifetime: guestLifetime }, patchSequence: patchSequence };
}
/** 🪪️ Compares wire identity only; equality does not mint a native retirement capability. */
function actorUiPatchReceiptEquals(left, right) {
    return left.lifetime.activationGeneration === right.lifetime.activationGeneration && left.lifetime.instanceId === right.lifetime.instanceId && left.lifetime.guestLifetime === right.lifetime.guestLifetime && left.patchSequence === right.patchSequence;
}
/** 🩹️ Enforces the turn's patch-batch cardinality before receipt consumption: ONE receipt authorizes
 * every patch a turn carries, and each patch inside the batch is addressed by its own
 * `(receipt, surface, revision)` when the host acknowledges it. The twin of `validate_pairing` in
 * `🩹️patch/🦀️.rs`; before 2026-09-15 both capped the count at one, which is what made N published
 * surfaces cost N + 1 host round trips. */
function validateActorUiPatchPairing(patchCount, receipt) {
    if (!Number.isSafeInteger(patchCount) || patchCount < 0 || (patchCount > 0) !== (receipt != null))
        throw new Error("actor-ui-patch.pairing");
    if (receipt != null)
        encodeActorUiPatchReceipt(receipt);
}
//#endregion 🧬️IssuedPatchReceipt
//#region 🧪️IssuedPatchReceiptLaws
if (import.meta.vitest) {
    var registerTests1 = (await Promise.resolve().then(function () { return require("./🧪️tests/🧪️actor-ui-patch-receipt-matches-shared-canonical-vectors-and-the-independ/🟦️.ts"); })).registerTests1;
    await registerTests1(import.meta.vitest, { ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES: exports.ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES, actorUiPatchReceiptEquals: actorUiPatchReceiptEquals, decodeActorUiPatchReceipt: decodeActorUiPatchReceipt, encodeActorUiPatchReceipt: encodeActorUiPatchReceipt, validateActorUiPatchPairing: validateActorUiPatchPairing }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️IssuedPatchReceiptLaws
