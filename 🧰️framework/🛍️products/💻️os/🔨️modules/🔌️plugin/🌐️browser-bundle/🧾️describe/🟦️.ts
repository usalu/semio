import { BROWSER_ACTOR_CHILD_LIMITS } from "../../../../../../🔨️modules/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts";

type DescriptorCodec<Value> = Readonly<{ encode(value: Value): Uint8Array; decode(bytes: Uint8Array): Value }>;

/** 📏️ Requires a complete descriptor result to fit with its transport framing. */
export function assertBrowserActorDescribeCapacityV1(byteLength: number): void {
  if (!Number.isSafeInteger(byteLength) || byteLength < 1 || byteLength > BROWSER_ACTOR_CHILD_LIMITS.outputBytes - 8) throw new Error("browser actor describe: result capacity");
}

function record(value: unknown): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("browser actor describe: invalid record");
  return value as Record<string, unknown>;
}

function equal(left: Uint8Array, right: Uint8Array): boolean {
  return left.length === right.length && left.every((byte, index) => byte === right[index]);
}

/** 🧾️ Verifies a guest's canonical SEMANTIC `describe()` answer against the staged descriptor the hub served: both are canonical
 * semantic descriptors carrying no byte binding (the package's `ComponentReceiptV1` names the bytes), and they are byte-equal. */
export function verifyBrowserActorDescribeV1<Value>(guest: Uint8Array, staged: Uint8Array, codec: DescriptorCodec<Value>): void {
  assertBrowserActorDescribeCapacityV1(staged.byteLength);
  assertBrowserActorDescribeCapacityV1(guest.byteLength);
  const guestValue = codec.decode(guest),
    stagedValue = codec.decode(staged);
  const guestCanonical = codec.encode(guestValue),
    stagedCanonical = codec.encode(stagedValue);
  try {
    if (!equal(guest, guestCanonical) || !equal(staged, stagedCanonical)) throw new Error("browser actor describe: noncanonical");
    if ("hashes" in record(guestValue) || "hashes" in record(stagedValue)) throw new Error("browser actor describe: byte binding");
    if (!equal(guest, staged)) throw new Error("browser actor describe: metadata mismatch");
  } finally {
    guestCanonical.fill(0);
    stagedCanonical.fill(0);
  }
}
