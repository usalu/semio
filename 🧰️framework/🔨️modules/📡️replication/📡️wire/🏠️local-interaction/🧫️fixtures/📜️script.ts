/** 🏠️ Strict local-interaction contracts and independent immutable restore semantics. */
import Ajv from "ajv";
import { produce } from "immer";
import { applyEdits, modify, parse } from "jsonc-parser";
import { sumBy } from "lodash";
import { strict as assert } from "node:assert";
import { createHash } from "node:crypto";
import leb from "@webassemblyjs/leb128/lib/leb.js";

//#region 🧬️Contract
const schema = await Bun.file(new URL("../🧬️schema/🏠️local-interaction/🔣️.schema.json", import.meta.url)).json();
const fixtureSchema = await Bun.file(new URL("./🏠️local-interaction/🧬️.schema.json", import.meta.url)).json();
const fixture = await Bun.file(new URL("./🏠️local-interaction/🔣️.json", import.meta.url)).json();
const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(schema);
const validate = ajv.compile(fixtureSchema);
assert(validate(fixture), JSON.stringify(validate.errors));
assert.equal(new Set(fixture.cases.map((row: any) => row.id)).size, fixture.cases.length);
const { applyLocalInteractionRestoreCold, localInteractionIdentityEquals } = await import("../🟦️.ts");
for (const row of fixture.cases) {
  const untouched = structuredClone(row.before);
  if (row.error) {
    assert.throws(() => applyLocalInteractionRestoreCold(row.before, row.current, row.restore), /stale-authority/);
    assert(!localInteractionIdentityEquals(row.current, row.restore.base));
  } else {
    const result = applyLocalInteractionRestoreCold(row.before, row.current, row.restore);
    const oracle = produce(row.before, (draft: any) => {
      if (row.restore.kind === "full") return row.restore.state;
      for (const [domain, patch] of Object.entries<any>(row.restore.domains)) {
        for (const field of ["selection", "activeMode", "activeGranularity"]) {
          if (patch[field] === null) delete draft[field][domain];
          else draft[field][domain] = patch[field];
        }
      }
    });
    assert.deepEqual(result, row.expected);
    assert.deepEqual(result, oracle);
    let edited = JSON.stringify(row.before);
    if (row.restore.kind === "full") edited = applyEdits(edited, modify(edited, [], row.restore.state, {}));
    else for (const [domain, patch] of Object.entries<any>(row.restore.domains)) for (const field of ["selection", "activeMode", "activeGranularity"]) edited = applyEdits(edited, modify(edited, [field, domain], patch[field] === null ? undefined : patch[field], {}));
    assert.deepEqual(result, parse(edited), `${row.id}: independent JSON edit oracle`);
    assert.deepEqual(row.before, untouched);
  }
}
const privateReplacement = fixture.cases.find((row: any) => row.id === "sparse-explicit-private-replacement");
assert(privateReplacement && privateReplacement.restore.kind === "domains");
for (const [domain, patch] of Object.entries<any>(privateReplacement.restore.domains)) {
  assert(fixture.nonbroadcastDomains.includes(domain));
  assert(patch.selection.anchorId.includes(",") && patch.selection.ids.includes(patch.selection.anchorId));
  for (const field of ["selection", "activeMode", "activeGranularity"]) {
    assert.deepEqual(privateReplacement.expected[field][domain], patch[field]);
    for (const unrelated of Object.keys(privateReplacement.before[field]).filter(key => key !== domain)) assert.deepEqual(privateReplacement.expected[field][unrelated], privateReplacement.before[field][unrelated]);
  }
}
const emptySparse = fixture.cases.find((row: any) => row.id === "sparse-empty-preserves-three-maps");
assert(emptySparse && emptySparse.restore.kind === "domains"); assert.deepEqual(emptySparse.restore.domains, {}); assert.deepEqual(emptySparse.expected, emptySparse.before);
const large = fixture.cases.find((row: any) => row.id === "semantic-unicode-over-page");
assert(Buffer.byteLength(Object.keys(large.expected.selection)[0]) > 4096);
for (const mutate of [
  (value: any) => { value.extra = true; },
  (value: any) => { value.cases[0].restore.selectionJson = "{}"; },
  (value: any) => { value.cases[0].current.generation = 9007199254740993; },
  (value: any) => { value.cases[0].current.revision = "1".repeat(16); },
  (value: any) => { value.cases[1].restore.domains.graph.selection.anchor = "b"; },
  (value: any) => { delete value.cases[1].restore.domains.graph.activeMode; },
  (value: any) => { value.cases[1].restore.domains.graph.selection.ids = ["a", "a"]; },
  (value: any) => { value.cases[0].current.topologyRevision = "ABC"; },
  (value: any) => { value.cases[0].current.generation = "18446744073709551616"; },
  (value: any) => { delete value.nonbroadcastDomains; },
  (value: any) => { value.nonbroadcastDomains = ["private", "private"]; },
  (value: any) => { value.cases = value.cases.filter((row: any) => row.id !== "sparse-explicit-private-replacement"); },
  (value: any) => { value.cases = value.cases.filter((row: any) => row.id !== "sparse-empty-preserves-three-maps"); },
]) {
  const mutant = structuredClone(fixture); mutate(mutant); assert(!validate(mutant));
}
console.log(`[DEBUG] Local-interaction source cases=${fixture.cases.length} hostileRejections=13 oracle=immer+jsonc-parser privateReplacement=1 emptySparse=1 semanticKeyBytes=${Buffer.byteLength(Object.keys(large.expected.selection)[0])} nativeRuntimeClaims=0`);
//#endregion 🧬️Contract

//#region 🌳️RetainedRootContract
const rootFixture = await Bun.file(new URL("../🌳️root/🧫️fixture/🔣️.json", import.meta.url)).json();
const rootSchema = await Bun.file(new URL("../🌳️root/📐️schema/🔣️.json", import.meta.url)).json();
const validateRoot = ajv.compile(rootSchema);
assert(validateRoot(rootFixture), JSON.stringify(validateRoot.errors));
for (const invalid of [
  { ...rootFixture, privateDomainIsCaptured: false },
  { ...rootFixture, anchorIsCaptured: false },
  { ...rootFixture, commaIdIsSplit: true },
  { ...rootFixture, zeroGrantChangesOwner: true },
  { ...rootFixture, coldDecodeEarnsInteractiveCredit: true },
]) assert(!validateRoot(invalid));
function rootStringBytes(state: typeof fixture.cases[number]["before"]): number {
  const selectionBytes = sumBy(Object.entries(state.selection), ([domain, value]: [string, any]) => Buffer.byteLength(domain) + Buffer.byteLength(value.granularity) + sumBy(value.ids, (id: string) => Buffer.byteLength(id)) + (value.anchorId === undefined ? 0 : Buffer.byteLength(value.anchorId)));
  return selectionBytes + sumBy(Object.keys(state.activeMode), (domain: string) => Buffer.byteLength(domain)) + sumBy(Object.entries(state.activeGranularity), ([domain, value]: [string, any]) => Buffer.byteLength(domain) + Buffer.byteLength(value));
}
const rootSource = fixture.cases.find((row: any) => row.id === rootFixture.sourceCase).before;
const largeRootSource = fixture.cases.find((row: any) => row.id === rootFixture.largeSourceCase).expected;
assert.equal(rootStringBytes(rootSource), rootFixture.finalOwnerRetiredBytes);
assert.equal(rootStringBytes(largeRootSource), rootFixture.largeFinalOwnerRetiredBytes);
assert(rootFixture.largeFinalOwnerRetiredBytes > 4096);
assert.deepEqual(produce(rootSource, () => {}), rootSource);
assert(rootSource.selection.private && rootSource.selection.graph.anchorId === "b" && rootSource.selection.graph.ids[0] === "a,comma");
console.log(`[DEBUG] Local-interaction retained-root oracle=lodash+immer bytes=${rootFixture.finalOwnerRetiredBytes}/${rootFixture.largeFinalOwnerRetiredBytes} hostileRejections=5 nativeRuntimeClaims=0`);
//#endregion 🌳️RetainedRootContract

//#region 🩹️RetainedUpdateContract
const updateFixture = await Bun.file(new URL("../🌳️root/🩹️update/🧫️fixture/🔣️.json", import.meta.url)).json();
const updateSchema = await Bun.file(new URL("../🌳️root/🩹️update/📐️schema/🔣️.json", import.meta.url)).json();
const validateUpdate = ajv.compile(updateSchema);
assert(validateUpdate(updateFixture), JSON.stringify(validateUpdate.errors));
for (const field of ["partialCandidateReadable", "cancelPublishesCandidate", "comparisonBytesAreRetiredBytes", "zeroGrantMutates"]) assert(!validateUpdate({ ...updateFixture, [field]: true }));
assert(!validateUpdate({ ...updateFixture, domainKeyCopies: 1 }));
const cancelSource = fixture.cases.find((row: any) => row.id === updateFixture.cancelSourceCase);
const [cancelDomain, cancelPatch] = Object.entries<any>(cancelSource.restore.domains)[0];
const patchBytes = Buffer.byteLength(cancelDomain) + Buffer.byteLength(cancelPatch.activeGranularity) + Buffer.byteLength(cancelPatch.selection.granularity) + sumBy(cancelPatch.selection.ids, (id: string) => Buffer.byteLength(id)) + Buffer.byteLength(cancelPatch.selection.anchorId);
assert.equal(rootStringBytes(cancelSource.before) + patchBytes, updateFixture.cancelOwnedStringBytes);
for (const name of updateFixture.cases) {
  const row = fixture.cases.find((row: any) => row.id === name);
  const oracle = produce(row.before, (draft: any) => {
    for (const [domain, patch] of Object.entries<any>(row.restore.domains)) for (const field of ["selection", "activeMode", "activeGranularity"]) {
      if (patch[field] === null) delete draft[field][domain];
      else draft[field][domain] = patch[field];
    }
  });
  assert.deepEqual(oracle, row.expected);
}
console.log(`[DEBUG] Local-interaction retained-update cases=${updateFixture.cases.length} cancelBytes=${updateFixture.cancelOwnedStringBytes} hostileRejections=5 oracle=immer+lodash nativeRuntimeClaims=0`);
//#endregion 🩹️RetainedUpdateContract

//#region 🔁️InteractionMutationLeaf
const mutationLeaf = new URL("../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/", import.meta.url);
const mutationDescriptor = await Bun.file(new URL("🔣️.json", mutationLeaf)).json();
const mutationSchema = await Bun.file(new URL("🧬️schema/🔣️.json", mutationLeaf)).json();
const mutationFixture = await Bun.file(new URL("🧪️fixture/🔣️.json", mutationLeaf)).json();
const validateMutation = ajv.compile(mutationSchema);
assert(validateMutation(mutationFixture), JSON.stringify(validateMutation.errors));
assert.equal(Object.keys(mutationDescriptor).length, 14);
assert.equal(mutationDescriptor.textOpcode, "set-interaction-state");
assert.equal(mutationDescriptor.binaryTag, null);
assert(mutationDescriptor.owner.endsWith("/🕹️interaction/🧬️mutations/🔁️set-state"));
assert.deepEqual(JSON.parse(Buffer.from(JSON.stringify(mutationFixture), "utf8").toString("utf8")), produce(mutationFixture, () => {}));
for (const invalid of [{ ...mutationFixture, localInteraction: {} }, { ...mutationFixture, activeMode: { graph: "invalid" } }, { ...mutationFixture, selection: { graph: { granularity: "node", ids: [1] } } }]) assert(!validateMutation(invalid));
console.log("[DEBUG] Interaction mutation leaf schema=actual-four-field-stored-state descriptorFields=14 hostileRejections=3 oracle=ajv+immer nativeCodecClaims=0");
//#endregion 🔁️InteractionMutationLeaf

//#region ♻️RetirementContract
const retirement = await Bun.file(new URL("./♻️retirement/🔣️.json", import.meta.url)).json();
const retirementSchema = await Bun.file(new URL("./♻️retirement/🧬️.schema.json", import.meta.url)).json();
const validateRetirement = ajv.compile(retirementSchema);
assert(validateRetirement(retirement), JSON.stringify(validateRetirement.errors));
for (const row of retirement.cases) {
  const source = fixture.cases.find((value: any) => value.id === row.sourceCase)[row.sourceField];
  let bytes = 0;
  for (const [domain, selection] of Object.entries<any>(source.selection)) bytes += Buffer.byteLength(domain) + Buffer.byteLength(selection.granularity) + selection.ids.reduce((sum: number, id: string) => sum + Buffer.byteLength(id), 0) + Buffer.byteLength(selection.anchorId ?? "");
  for (const domain of Object.keys(source.activeMode)) bytes += Buffer.byteLength(domain);
  for (const [domain, granularity] of Object.entries<string>(source.activeGranularity)) bytes += Buffer.byteLength(domain) + Buffer.byteLength(granularity);
  assert.equal(bytes, row.expectedReleasedBytes);
  const strings = [
    ...Object.entries<any>(source.selection).flatMap(([key, value]) => [key, value.granularity, ...value.ids, ...(value.anchorId === undefined ? [] : [value.anchorId])]),
    ...Object.keys(source.activeMode),
    ...Object.entries<string>(source.activeGranularity).flat(),
  ];
  assert.equal(sumBy(strings, (value: string) => new TextEncoder().encode(value).length), row.expectedReleasedBytes);
}
for (const mutant of [{ ...retirement, terminalOwners: 1 }, { ...retirement, zeroItemMutates: true }]) assert(!validateRetirement(mutant));
console.log(`[DEBUG] Local-interaction retirement source cases=${retirement.cases.length} hostileRejections=2 grants=1,64,4096 oracle=lodash runtimeClaims=0`);
//#endregion ♻️RetirementContract

//#region 📃️QueryContract
const query = await Bun.file(new URL("./📃️query/🔣️.json", import.meta.url)).json();
const querySchema = await Bun.file(new URL("./📃️query/🧬️.schema.json", import.meta.url)).json();
const validateQuery = ajv.compile(querySchema);
assert(validateQuery(query), JSON.stringify(validateQuery.errors));
assert.equal(`{"first":${JSON.stringify(query.partialError.first)},"second":`, query.partialError.expectedPrefix);
assert.equal(Buffer.byteLength(query.partialError.expectedPrefix), new TextEncoder().encode(query.partialError.expectedPrefix).length);
function canonical(value: any): any {
  if (Array.isArray(value)) return value.map(canonical);
  if (value !== null && typeof value === "object") return Object.fromEntries(Object.keys(value).sort().map(key => [key, canonical(value[key])]));
  return value;
}
for (const sourceCase of query.sourceCases) {
  const source = fixture.cases.find((row: any) => row.id === sourceCase);
  assert(source, `query source case ${sourceCase}`);
  const expected = Buffer.from(JSON.stringify(canonical({ identity: source.current, state: source.expected })));
  const full = createHash("sha256").update(expected).digest("hex");
  for (const grant of query.grants) {
    const streamed = createHash("sha256");
    let bytes = 0;
    for (let offset = 0; offset < expected.length; offset += Math.min(grant, query.pageBytes)) {
      const page = expected.subarray(offset, offset + Math.min(grant, query.pageBytes));
      streamed.update(page); bytes += page.length;
    }
    assert.equal(bytes, expected.length);
    assert.equal(streamed.digest("hex"), full);
  }
}
for (const mutant of [{ ...query, unacknowledgedPageAdvances: true }, { ...query, cancelledPageReadable: true }, { ...query, terminalRequiresReadReturn: false }]) assert(!validateQuery(mutant));
console.log(`[DEBUG] Local-interaction query source cases=${query.sourceCases.length} partitions=${query.grants.length} hostileRejections=3 oracle=node-crypto nativeRuntimeClaims=0`);
//#endregion 📃️QueryContract

//#region 🔐️TopologyInputAuthority
const topologyAuthority = await Bun.file(new URL("./🔐️topology-authority/🔣️.json", import.meta.url)).json();
const topologyAuthoritySchema = await Bun.file(new URL("./🔐️topology-authority/🧬️.schema.json", import.meta.url)).json();
const validateTopologyAuthority = ajv.compile(topologyAuthoritySchema);
assert(validateTopologyAuthority(topologyAuthority), JSON.stringify(validateTopologyAuthority.errors));
for (const row of topologyAuthority.cases) {
  const generation = Buffer.alloc(8); generation.writeBigUInt64LE(BigInt(row.uiGeneration));
  const actual = createHash("sha256").update(topologyAuthority.domain).update(Buffer.alloc(32, row.documentByte)).update(Buffer.alloc(32, row.configByte)).update(generation).digest("hex");
  assert.equal(actual, row.expected);
}
assert.equal(new Set(topologyAuthority.cases.map((row: any) => row.expected)).size, topologyAuthority.cases.length);
for (const mutant of [{ ...topologyAuthority, overflowMutatesCache: true }, { ...topologyAuthority, canonicalTopologyHash: true }, { ...topologyAuthority, closedAuthorityReadable: true }]) assert(!validateTopologyAuthority(mutant));
console.log(`[DEBUG] Local-interaction topology input-authority source cases=${topologyAuthority.cases.length} hostileRejections=3 oracle=node-crypto nativeRuntimeClaims=0`);
//#endregion 🔐️TopologyInputAuthority

//#region 📡️TransportCodec
const transport = await Bun.file(new URL("../📡️transport/🧫️fixtures/🔣️.json", import.meta.url)).json();
const transportFixtureSchema = await Bun.file(new URL("../📡️transport/📐️schema/🔣️.json", import.meta.url)).json();
const transportSchema = await Bun.file(new URL("../📡️transport/🧬️schema/🔣️.json", import.meta.url)).json();
const validateTransportFixture = ajv.compile(transportFixtureSchema);
const validateTransport = ajv.compile(transportSchema);
assert(validateTransportFixture(transport), JSON.stringify(validateTransportFixture.errors));
const wire = await import("../📡️transport/🟦️.ts");
function oracleUnsigned(value: string): Buffer { const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(BigInt(value)); return Buffer.from(leb.encodeUIntBuffer(bytes)); }
for (const row of transport.unsigned) {
  const encoded = Buffer.from(wire.encodeLocalInteractionUnsigned(row.decimal));
  assert.equal(encoded.toString("hex"), row.hex);
  assert.deepEqual(encoded, oracleUnsigned(row.decimal));
  assert.equal(wire.decodeLocalInteractionUnsigned(encoded), row.decimal);
  assert.equal(leb.decodeUInt64(encoded, 0).value.toString(), row.decimal);
}
for (const hex of transport.malformedUnsigned) assert.throws(() => wire.decodeLocalInteractionUnsigned(Buffer.from(hex, "hex")));
const queryToken = { requestId: "13", queryGeneration: "41", identity: fixture.cases[0].current, ordinal: "2" };
const identity = queryToken.identity;
const oracleToken = Buffer.concat([oracleUnsigned(queryToken.requestId), oracleUnsigned(queryToken.queryGeneration), oracleUnsigned(String(identity.appInstanceId)), oracleUnsigned(identity.generation), Buffer.from(identity.revision, "hex"), Buffer.from(identity.documentRevision, "hex"), Buffer.from(identity.topologyRevision, "hex"), oracleUnsigned(queryToken.ordinal)]);
const commands = [{ kind: "read", requestId: "13" }, { kind: "acknowledge", token: queryToken }, { kind: "cancel", token: queryToken }] as const;
for (const [index, command] of commands.entries()) {
  assert(validateTransport(command), JSON.stringify(validateTransport.errors));
  const encoded = wire.encodeLocalInteractionQueryCommand(command);
  assert.deepEqual(Buffer.from(encoded), Buffer.concat([Buffer.from([index]), index === 0 ? oracleUnsigned("13") : oracleToken]));
  assert.deepEqual(wire.decodeLocalInteractionQueryCommand(encoded), command);
  assert.throws(() => wire.decodeLocalInteractionQueryCommand(Uint8Array.from([...encoded, 0])));
}
const replies = [{ kind: "started", token: queryToken }, { kind: "page", page: { ...queryToken, terminal: true, bytes: [123, 125] } }, { kind: "closed", token: queryToken, cancelled: false }, { kind: "rejected", requestId: "13", code: "busy" }] as const;
const expectedReplies = [Buffer.concat([Buffer.from([0]), oracleToken]), Buffer.concat([Buffer.from([1]), oracleToken, Buffer.from([1, 2, 123, 125])]), Buffer.concat([Buffer.from([2]), oracleToken, Buffer.from([0])]), Buffer.from([3, 13, 0])];
for (const [index, reply] of replies.entries()) {
  assert(validateTransport(reply), JSON.stringify(validateTransport.errors));
  const encoded = wire.encodeLocalInteractionQueryReply(reply);
  assert.deepEqual(Buffer.from(encoded), expectedReplies[index]);
  assert.deepEqual(wire.decodeLocalInteractionQueryReply(encoded), reply);
  assert.throws(() => wire.decodeLocalInteractionQueryReply(Uint8Array.from([...encoded, 0])));
}
for (const invalid of [{ ...commands[0], extra: true }, { kind: "acknowledge", token: { ...queryToken, queryGeneration: 41 } }, { kind: "page", page: { ...queryToken, terminal: true, bytes: Array(4097).fill(0) } }]) assert(!validateTransport(invalid));
console.log(`[DEBUG] Local-interaction transport source unsigned=5 commands=3 replies=4 malformed=3 trailing=7 hostile=3 oracle=@webassemblyjs/leb128 nativeRuntimeClaims=0`);
//#endregion 📡️TransportCodec
