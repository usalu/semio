import { BROWSER_ACTOR_CHILD_LIMITS } from "../🧵️child/🧬️schema/🟦️.ts";

type DescriptorCodec<Value> = Readonly<{ encode(value: Value): Uint8Array; decode(bytes: Uint8Array): Value }>;
const hashKeys = ["coreWasmSha256", "descriptorSha256", "wasmSha256"];

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

/** 🧾️ Verifies canonical guest metadata against the staged descriptor with only emitter hashes blanked. */
export function verifyBrowserActorDescribeV1<Value>(guest: Uint8Array, staged: Uint8Array, codec: DescriptorCodec<Value>): void {
  assertBrowserActorDescribeCapacityV1(staged.byteLength);
  assertBrowserActorDescribeCapacityV1(guest.byteLength);
  const guestValue = codec.decode(guest),
    stagedValue = codec.decode(staged);
  const guestCanonical = codec.encode(guestValue),
    stagedCanonical = codec.encode(stagedValue);
  let expected: Uint8Array | undefined;
  try {
    if (!equal(guest, guestCanonical) || !equal(staged, stagedCanonical)) throw new Error("browser actor describe: noncanonical");
    const guestHashes = record(record(guestValue).hashes),
      stagedHashes = record(record(stagedValue).hashes);
    if (Object.keys(guestHashes).sort().join() !== hashKeys.join() || Object.keys(stagedHashes).sort().join() !== hashKeys.join()) throw new Error("browser actor describe: hash shape");
    for (const key of hashKeys) {
      if (guestHashes[key] !== "" || typeof stagedHashes[key] !== "string" || !/^[0-9a-f]{64}$/u.test(stagedHashes[key])) throw new Error("browser actor describe: hash identity");
      stagedHashes[key] = "";
    }
    expected = codec.encode(stagedValue);
    if (!equal(guest, expected)) throw new Error("browser actor describe: metadata mismatch");
  } finally {
    guestCanonical.fill(0);
    stagedCanonical.fill(0);
    expected?.fill(0);
  }
}
