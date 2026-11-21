import { expect, test } from "bun:test";
import Ajv from "ajv";
import actor from "../../../../🔌️plugin/🧠️interpreter/🏃️runtime/🏗️startup/🧪️tests/🧫️fixtures/prepared.json";
import law from "./🧫️fixtures/cold-image.json";
import schema from "./🧬️schema/cold-image.json";

test("a cold actor reads its channel from the verified prepared bytes", async () => {
  expect(new Ajv({ strict: true }).compile(schema)(law)).toBe(true);
  let hex = actor.coreHex;
  for (const { from, to } of law.patches) {
    expect(hex.split(from)).toHaveLength(2);
    expect(from).toHaveLength(to.length);
    hex = hex.replace(from, to);
  }
  const core = await WebAssembly.compile(Buffer.from(hex, "hex"));
  const open = () => {
    const exports = new WebAssembly.Instance(core).exports;
    const memory = new Uint8Array((exports.memory as WebAssembly.Memory).buffer);
    const packed = (exports.semio_owned_channel_version_v1 as () => bigint)();
    const offset = Number(packed & 0xffffffffn);
    const length = Number(packed >> 32n);
    return { exports, memory, channel: memory.subarray(offset, offset + length) };
  };
  const original = open();
  expect([...original.channel]).toEqual(law.startChannel);
  expect((original.exports.semio_owned_prepare_actor_v1 as () => number)()).toBe(1);
  expect([...original.channel]).toEqual(law.preparedChannel);
  const image = original.memory.slice();
  const digest = (bytes: Uint8Array) => new Bun.CryptoHasher("sha256").update(bytes).digest("hex");
  const expected = digest(image);
  const altered = image.slice();
  altered[actor.marker.offset] ^= 1;
  expect(digest(altered)).not.toBe(expected);
  const cold = open();
  cold.memory.set(image);
  expect(new TextDecoder().decode(cold.channel)).toBe(String(actor.channelVersion));
});
