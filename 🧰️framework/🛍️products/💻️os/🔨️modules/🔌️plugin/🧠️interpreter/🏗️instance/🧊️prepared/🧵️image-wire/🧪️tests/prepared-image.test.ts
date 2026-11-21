import { expect, test } from "bun:test";
import Ajv from "ajv";
import fixture from "./prepared-image.json";
import schema from "../🧬️schema/prepared-image.json";

const u64 = (n: bigint | number) => {
  const data = Buffer.alloc(8);
  data.writeBigUInt64LE(BigInt(n));
  return data;
};

test("prepared image bytes bind the component and encode independent Wasm initialization", async () => {
  expect(new Ajv({ strict: true, allowUnionTypes: true }).compile(schema)(fixture)).toBe(true);
  const core = Buffer.from(fixture.coreHex, "hex");
  const component = Buffer.concat([
    Buffer.from("0061736d0d00010001", "hex"),
    Buffer.from([(core.length & 127) | 128, core.length >> 7]),
    core,
  ]);
  const componentSha = new Bun.CryptoHasher("sha256").update(component).digest();
  expect(componentSha.toString("hex")).toBe(fixture.componentSha256);
  const fingerprint = core.reduce((value, byte) => BigInt.asUintN(64, (value ^ BigInt(byte)) * 1099511628211n), 14695981039346656037n);
  const module = await WebAssembly.compile(core);
  expect(WebAssembly.Module.imports(module)).toEqual([]);
  const exports = new WebAssembly.Instance(module).exports;
  const prepare = exports.semio_owned_prepare_actor_v1 as () => number;
  expect(prepare()).toBe(1);
  const memory = Buffer.from((exports.memory as WebAssembly.Memory).buffer);
  expect([...memory.subarray(0, 3)]).toEqual(fixture.memoryPrefix);
  const table = exports.table as WebAssembly.Table;
  const references = Array.from({ length: table.length }, (_, index) => table.get(index) === prepare ? 0 : table.get(index));
  expect(references).toEqual(fixture.table);
  const globals = Buffer.alloc(12);
  globals.writeInt32LE((exports.counter as WebAssembly.Global).value, 1);
  globals[5] = 2;
  globals.writeFloatLE((exports.negativeZero as WebAssembly.Global).value, 6);
  globals[10] = 5;
  expect((exports.reference as WebAssembly.Global).value).toBe(null);
  expect(globals.toString("hex")).toBe(fixture.globalWire);
  const image = Buffer.concat([
    Buffer.from(fixture.prefix, "hex"), componentSha, u64(fingerprint),
    u64(1), u64(memory.length), memory,
    u64(1), u64(table.length), ...references.map(value => u64(value === null ? 2n ** 64n - 1n : value)),
    u64(3), globals,
    u64(fixture.dataLive.length), Buffer.from(fixture.dataLive),
    u64(fixture.elementsLive.length), Buffer.from(fixture.elementsLive),
  ]);
  expect(image.length).toBe(fixture.wireBytes);
  expect(new Bun.CryptoHasher("sha256").update(image).digest("hex")).toBe(fixture.imageSha256);
});
