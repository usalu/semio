import { expect, test } from "bun:test";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/prepared.json";
import schema from "./🧬️schema/prepared.json";

test("a context-free actor image retains the constructor marker and has independent memories", async () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  const module = await WebAssembly.compile(Buffer.from(fixture.coreHex, "hex"));
  expect(WebAssembly.Module.imports(module)).toEqual([]);
  const original = new WebAssembly.Instance(module).exports;
  const originalMemory = new Uint8Array((original.memory as WebAssembly.Memory).buffer);
  expect(originalMemory[fixture.marker.offset]).toBe(fixture.marker.afterStart);
  expect((original.semio_owned_prepare_actor_v1 as () => number)()).toBe(1);
  expect(originalMemory[fixture.marker.offset]).toBe(fixture.marker.prepared);

  const open = () => {
    const exports = new WebAssembly.Instance(module).exports;
    const memory = new Uint8Array((exports.memory as WebAssembly.Memory).buffer);
    memory.set(originalMemory);
    const packed = (exports.semio_owned_channel_version_v1 as () => bigint)();
    const offset = Number(packed & 0xffffffffn);
    const length = Number(packed >> 32n);
    expect(JSON.parse(new TextDecoder().decode(memory.subarray(offset, offset + length)))).toBe(fixture.channelVersion);
    expect(memory[fixture.marker.offset]).toBe(fixture.marker.prepared);
    return memory;
  };
  const first = open();
  first[fixture.marker.offset] = fixture.marker.mutated;
  const second = open();
  expect(first[fixture.marker.offset]).toBe(fixture.marker.mutated);
  expect(second[fixture.marker.offset]).toBe(fixture.marker.prepared);
  expect(originalMemory[fixture.marker.offset]).toBe(fixture.marker.prepared);
});
