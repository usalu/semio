/** 🔬️ Canonical fixedListStorageSelfTests fixture and oracle checks. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

export function fixedListStorageSelfTests(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../📋️list/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const ordered = Buffer.alloc(fixture.ordered.count * 8);
  for (let index = 0; index < fixture.ordered.count; index++) ordered.writeBigUInt64LE(BigInt(index), index * 8);
  let sum = 0n;
  for (let index = 0; index < fixture.ordered.count; index++) sum += ordered.readBigUInt64LE(index * 8);
  assert.equal(sum, BigInt(fixture.ordered.sum));
  assert.equal(ordered.readBigUInt64LE(0), BigInt(fixture.ordered.first));
  assert.equal(ordered.readBigUInt64LE(ordered.byteLength - 8), BigInt(fixture.ordered.last));
  assert(Buffer.alloc(fixture.binding.elementBytes).byteLength > fixture.binding.smallGrantBytes);
  assert(Buffer.alloc(fixture.oversized.elementBytes).byteLength > fixture.maximumGrantBytes);
  assert.equal(Buffer.alloc(fixture.edgeCases.zeroCapacity).byteLength, 0);
  assert.equal(Array.from({ length: fixture.edgeCases.zeroSizedCount }, () => null).length, 7);
  const retained = ordered.subarray(0, fixture.edgeCases.retainedPrefix * 8);
  assert.equal(retained.readBigUInt64LE(retained.byteLength - 8), 511n);
  assert.equal(ordered.byteLength - retained.byteLength, (fixture.edgeCases.tailCount - fixture.edgeCases.retainedPrefix) * 8);
  for (const bits of [32, 64]) {
    const maximum = (1n << BigInt(bits - 1)) - 1n;
    const counter = Buffer.alloc(8);
    counter.writeBigUInt64LE(maximum);
    assert(counter.readBigUInt64LE() + 384n > maximum);
  }
  assert.equal(Buffer.alloc(384 * fixture.counter.allocatorMultiplier).byteLength, 768);
  const copy = JSON.parse(readFileSync(new URL("../../🔗️bindings/📋️copy/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const bindingBytes = Buffer.alloc(copy.elementBytes);
  assert(bindingBytes.byteLength <= copy.grantBytes && bindingBytes.byteLength > copy.smallGrantBytes);
  const expected = Array.from({ length: copy.count }, (_, index) => ({ trigger: "activate", action: { scope: copy.scope, name: `action-${index}`, version: 1 } }));
  assert.deepEqual(JSON.parse(JSON.stringify(expected)).slice(0, 3).map((binding: typeof expected[number]) => binding.action.name), copy.names);
  const componentCopy = JSON.parse(readFileSync(new URL("../../🪞️copy/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const components = JSON.parse(readFileSync(new URL("../../♻️retirement/🌳️typed/🧩️components.json", import.meta.url), "utf8"));
  assert.equal(new Set(components.cases.map((row: { component: { type: string } }) => row.component.type)).size, componentCopy.componentCount);
  const textBytes = Buffer.from(componentCopy.text.repeat(componentCopy.textRepeats));
  assert.equal(textBytes.byteLength, 512);
  assert.equal(Buffer.concat(Array(componentCopy.listItems * 2).fill(textBytes)).byteLength, 32768);
  assert.equal(Buffer.alloc(componentCopy.allocationGrant * componentCopy.allocatorMultiplier).byteLength, 65536);
  assert.equal(Buffer.alloc(componentCopy.allocationGrant).byteLength / componentCopy.runtimeWorkGrant, 8);
  const comparison = JSON.parse(readFileSync(new URL("../../⚖️compare/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  for (const row of comparison.cases) assert.equal(Buffer.from(JSON.stringify(row.left)).equals(Buffer.from(JSON.stringify(row.right))), row.equal);
  const frameOracle = Buffer.alloc(comparison.frame.bytes);
  frameOracle.writeUInt16LE(comparison.frame.pageCount - 1, 0);
  frameOracle.writeUInt16LE(comparison.frame.maximumTextBytes * 2, 4);
  assert.equal(frameOracle.toString("hex"), comparison.frame.littleEndian);
  assert.equal(frameOracle.readUInt16LE(4), comparison.frame.maximumPosition);
  const documentComparison = JSON.parse(readFileSync(new URL("../../⚖️compare/📃️document/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const nodeIds = Buffer.alloc(documentComparison.nodeIds.length * 8);
  documentComparison.nodeIds.forEach((id: number, index: number) => nodeIds.writeBigUInt64LE(BigInt(id), index * 8));
  assert.deepEqual(documentComparison.nodeIds.map((_: number, index: number) => Number(nodeIds.readBigUInt64LE(index * 8))), documentComparison.wireOrder);
  assert.equal(Buffer.from(documentComparison.text.repeat(documentComparison.textRepeats)).byteLength, 512);
  const wholePatch = JSON.parse(readFileSync(new URL("../../♻️retirement/🩹️patch/📨️pending/📦️whole/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  assert.equal(Buffer.from(wholePatch.surface).byteLength, wholePatch.surfaceBytes);
  const assembly = JSON.parse(readFileSync(new URL("../../📃️document/🎟️assembly/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const assemblyIds = Buffer.alloc(assembly.nodeIds.length * 8);
  assembly.nodeIds.forEach((id: number, index: number) => assemblyIds.writeBigUInt64LE(BigInt(id), index * 8));
  assert.equal(assemblyIds.toString("hex"), assembly.wireHex);
  assert.equal(Buffer.from(assembly.surface).byteLength, 6);
  assert.equal(Buffer.concat([assemblyIds.subarray(0, 8), assemblyIds.subarray(0, 8)]).byteLength, assembly.comparisonBytesPerIdentity);
  const resident = JSON.parse(readFileSync(new URL("../../🎟️resident/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const residentBytes = Buffer.alloc(8);
  // ⚖️ The aggregate funds every SLOT the ledger admits at one full DOCUMENT each — never a multiple of
  // the per-surface CEILING, which is a maximum one pathological surface may reach and not a price. Read
  // from `4 * surfaceBytes`, the byte ledger refused a twenty-six-surface session at sixty-four free
  // slots (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, lane `react-example-switch-regression`).
  residentBytes.writeBigUInt64LE(BigInt(resident.slots) * BigInt(resident.documentBytes));
  assert.equal(Number(residentBytes.readBigUInt64LE()), resident.aggregateBytes);
  assert(resident.documentBytes < resident.surfaceBytes, "one full document must stay under the per-surface ceiling");
  let mask = resident.rootOwner | resident.outputOwner;
  assert.deepEqual(resident.returnOrder.map((owner: number) => { mask &= ~owner; return mask === 0 ? resident.smallBytes : 0; }), resident.returnedBytes);
  assert(resident.smallReservations * resident.smallBytes < resident.aggregateBytes);
  const residentFixed = JSON.parse(readFileSync(new URL("../../🎟️resident/🗃️fixed/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const fixedArithmetic = residentFixed.arithmetic;
  residentBytes.writeBigUInt64LE(BigInt(fixedArithmetic.contract) + BigInt(fixedArithmetic.runtime) + BigInt(fixedArithmetic.payload));
  assert.equal(Number(residentBytes.readBigUInt64LE()), fixedArithmetic.admitted);
  assert.equal(Number(residentBytes.readBigUInt64LE() - BigInt(fixedArithmetic.payload)), fixedArithmetic.final);
  const residentRoot = JSON.parse(readFileSync(new URL("../../🎟️resident/🌳️root/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  assert.equal(Buffer.from(residentRoot.surface).toString("hex"), residentRoot.surfaceUtf8);
  assert.equal(Buffer.byteLength(residentRoot.payload), residentRoot.payloadUtf8Bytes);
  const rootId = Buffer.alloc(8); rootId.writeBigUInt64LE(BigInt(residentRoot.rootId));
  assert.equal(rootId.toString("hex"), residentRoot.wireHex);
  // 🎟️ The aggregate is no longer a whole multiple of the per-surface ceiling, so filling it takes a
  // last PARTIAL reservation: `pressureRoots` is the count that first covers it, never a clean divisor.
  assert((residentRoot.pressureRoots - 1) * residentRoot.pressureReservationBytes < residentRoot.pressureAggregateBytes);
  assert(residentRoot.pressureRoots * residentRoot.pressureReservationBytes >= residentRoot.pressureAggregateBytes);
  for (const order of residentRoot.outputOrders) {
    let pending = 3;
    assert.deepEqual(order.map((owner: number) => { pending &= ~owner; return pending === 0 ? residentRoot.sealedBytes : 0; }), [0, 32768]);
  }
  return 75;
}
