/** 🎯️ Inference WAL, catalog, reconcile, approval and command fixtures and the architecture and ledger oracles.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { buildBudgetMs } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { blake3Hex } from "@semio-tech/framework/hash";
import { runRepositoryExactCargoLaws, runCmd, configuredRepositoryCargoPreparationPolicyV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { assertObservationExpectation } from "../../🧪️testing/🛂️expectation/🟦️.ts";
import { proveGisNativeProviderSelectionFixture } from "../🌍️gis-map-fixtures/🟦️.ts";
import { webSha256Hex } from "../../🚀️local-relay/🛰️servers/🟦️.ts";
import { hubLawGroups, requireHubDatum } from "../../🧪️testing/📖️source-readers/🟦️.ts";
import { proveTrustedCatalogIdentityRolesFixture } from "../🏛️trusted-catalog-fixtures/🟦️.ts";

/** 💡️ Independent schema, SHA-256, bounds and lifecycle reference over the neutral ledger corpus. */
async function proveInferenceWalProofFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "🧾️inference-wal-proof-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validateCommand = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceCommandV1");
  const validateTarget = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceWalTargetV1");
  if (!validateCommand(fixture.command)) throw new Error("WAL proof fixture command is not a hub.inference/InferenceCommandV1");
  if (!validateTarget({ scope: fixture.scope, documentKey: fixture.documentKey, generation: fixture.generation, jobId: fixture.jobId, proposalHash: fixture.proposalHash, maximumRecords: fixture.maximumRecords }))
    throw new Error("WAL proof fixture target is not a hub.inference/InferenceWalTargetV1");
  if (fixture.schema !== "semio.hub.inference-wal-proof-fixture/v1" || fixture.payloadAuthority !== "protocol-envelope-only-not-an-executable-gis-mutation") throw new Error("WAL proof fixture envelope drifted");
  if (fixture.bindingMismatches.length !== 2 || fixture.ownership.length !== 3 || fixture.traces.length !== 17 || new Set(fixture.traces.map((trace: { name: string }) => trace.name)).size !== 17) throw new Error("WAL proof fixture inventory drifted");
  const key = `v1:${Buffer.byteLength(fixture.scope.spaceId)}:${Buffer.byteLength(fixture.scope.documentId)}:${fixture.scope.spaceId}${fixture.scope.documentId}`;
  if (key !== fixture.documentKey || key !== fixture.command.documentId) throw new Error("WAL fixture full document key mismatch");
  const encoded: number[] = [];
  const integer = (value: number): void => {
    let remaining = BigInt(value);
    do {
      const byte = Number(remaining & 127n);
      remaining >>= 7n;
      encoded.push(byte | (remaining ? 128 : 0));
    } while (remaining);
  };
  const bytes = (value: Buffer): void => {
    integer(value.byteLength);
    encoded.push(...value);
  };
  const text = (value: string): void => bytes(Buffer.from(value, "utf8"));
  const command = fixture.command;
  text(command.mutationId);
  text(command.documentId);
  text(command.actor);
  integer(command.dependencies.length);
  command.dependencies.forEach(text);
  integer(0);
  integer(0);
  text(command.diff.schema);
  bytes(Buffer.from(command.diff.payloadHex, "hex"));
  text(command.inverse.schema);
  bytes(Buffer.from(command.inverse.payloadHex, "hex"));
  integer(command.timestamp.actor);
  integer(command.timestamp.physicalMs);
  integer(command.timestamp.logical);
  const canonical = Buffer.from(encoded);
  if (canonical.toString("hex") !== fixture.encodedHex || createHash("sha256").update(canonical).digest("hex") !== fixture.commandHash) throw new Error("independent protocol envelope/hash mismatch");
  const ledger = JSON.parse(readFileSync(join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "🗺️gis-inference-job-v1", "🔣️.json"), "utf8"));
  if (
    ledger.outbox.commandHex !== fixture.encodedHex ||
    ledger.outbox.commandHash !== fixture.commandHash ||
    ledger.outbox.mutationId !== command.mutationId ||
    ledger.outbox.jobId !== fixture.jobId ||
    ledger.outbox.proposalHash !== fixture.proposalHash ||
    ledger.identity.spaceId !== fixture.scope.spaceId ||
    ledger.identity.documentId !== fixture.scope.documentId
  )
    throw new Error("ledger and committed-WAL fixtures disagree on exact command authority");
  if (createHash("sha256").update(`semio.hub.inference-approval-mutation/v1\0${fixture.jobId}\0${fixture.proposalHash}`).digest("hex").slice(0, 32) !== command.mutationId)
    throw new Error("committed-WAL mutation does not bind the exact job and proposal");
  for (const mismatch of fixture.bindingMismatches) if (mismatch.jobId === fixture.jobId && mismatch.proposalHash === fixture.proposalHash) throw new Error("hostile witness binding was not distinct");
  const outcome = (trace: any): string => {
    if (trace.cancelAfterRecords === 0) return "cancelled";
    if (trace.tornTail) return "invalid";
    const records = trace.flushed === false ? [] : trace.records;
    let active: { id: number; count: number; matches: number } | undefined;
    let matches = 0;
    let targetRecords = 0;
    const receiptTransactionId = trace.receiptTransactionId ?? 1;
    for (let index = 0; index < records.length; index++) {
      if (trace.cancelAfterRecords === index) return "cancelled";
      const record = records[index];
      if (record.kind === "begin") {
        if (active) return "invalid";
        active = { id: record.txId, count: 0, matches: 0 };
        if (active.id === receiptTransactionId && ++targetRecords > (trace.maximumRecords ?? fixture.maximumRecords)) return "bounds";
      } else if (record.kind === "command") {
        if (!active) return "invalid";
        active.count++;
        if (active.id === receiptTransactionId && ++targetRecords > (trace.maximumRecords ?? fixture.maximumRecords)) return "bounds";
        const bytes = Buffer.from(canonical);
        if (record.bytes === "different") bytes[1] = requireHubDatum(bytes[1],"bytes[1]") ^ 1;
        if (record.bytes === "altered-target") bytes[bytes.length - 1] = requireHubDatum(bytes[bytes.length - 1],"bytes[bytes.length - 1]") ^ 1;
        if (createHash("sha256").update(bytes).digest("hex") === fixture.commandHash) active.matches++;
      } else {
        if (!active || active.id !== record.txId) return "invalid";
        if (active.id === receiptTransactionId && ++targetRecords > (trace.maximumRecords ?? fixture.maximumRecords)) return "bounds";
        if (record.kind === "commit") {
          if (active.count !== record.recordCount) return "invalid";
          if (active.id === receiptTransactionId) matches = active.matches;
        }
        const receiptFinished = active.id === receiptTransactionId;
        active = undefined;
        if (receiptFinished) break;
      }
    }
    if (trace.cancelAfterRecords === records.length) return "cancelled";
    if (active) return "invalid";
    if ((trace.observedGeneration ?? fixture.generation) !== fixture.generation || (trace.spaceId ?? fixture.scope.spaceId) !== fixture.scope.spaceId) return "stale";
    return matches === 1 ? "verified" : "absent";
  };
  for (const trace of fixture.traces) {
    const actual = outcome(trace);
    if (actual !== trace.expected) throw new Error(`WAL proof trace mismatch: ${trace.name}`);
    if (trace.reusableAfterInvalidation !== undefined && (actual !== "verified" || trace.reusableAfterInvalidation !== (fixture.generation === 0))) throw new Error("invalidated WAL witness remained reusable");
  }
  for (const owner of fixture.ownership) {
    const expected = owner.interrupt === "deadline" ? "expired" : "cancelled";
    if (owner.expected !== expected || owner.heldActive !== 1 || owner.releasedActive !== 0 || owner.stoppedProgress !== 1) throw new Error("WAL interruption must retain the owner until explicit close");
  }
  console.log(`inference-wal-proof-oracle: traces=${fixture.traces.length} ownership=${fixture.ownership.length} binding-hostile=${fixture.bindingMismatches.length} protocol-envelope=1 node-sha256=1; committed-WAL runtime still required`);
}

async function proveInferenceWalChainFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "⛓️inference-wal-chain-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const proof = JSON.parse(readFileSync(join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "🧾️inference-wal-proof-v1", "🔣️.json"), "utf8"));
  const validatePolicy = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceWalChainPolicyV1");
  if (!validatePolicy({ hashAlgorithm: fixture.hashAlgorithm, requiredFlags: fixture.requiredFlags, recordDigest: fixture.recordDigest, commitDigest: fixture.commitDigest }))
    throw new Error("WAL chain fixture policy is not a hub.inference/InferenceWalChainPolicyV1");
  if (fixture.schema !== "semio.hub.inference-wal-chain-fixture/v1") throw new Error("WAL chain fixture envelope drifted");
  if (fixture.cases.length !== 14 || new Set(fixture.cases.map((test: { name: string }) => test.name)).size !== 14 || fixture.hashingOwnership.length !== 3 || fixture.retainedBoundaries.length !== 2)
    throw new Error("WAL chain fixture inventory drifted");
  const digest = (bytes: Buffer): Buffer => Buffer.from(blake3Hex(bytes), "hex");
  const crc = (bytes: Buffer): number => {
    let value = 0xffffffff;
    for (const byte of bytes) {
      value ^= byte;
      for (let bit = 0; bit < 8; bit++) value = (value >>> 1) ^ (value & 1 ? 0x82f63b78 : 0);
    }
    return (value ^ 0xffffffff) >>> 0;
  };
  const integer = (value: number, width: 4 | 8): Buffer => {
    const bytes = Buffer.alloc(width);
    if (width === 4) bytes.writeUInt32LE(value);
    else bytes.writeBigUInt64LE(BigInt(value));
    return bytes;
  };
  const varint = (value: number): Buffer => {
    const bytes: number[] = [];
    do {
      const byte = value % 128;
      value = Math.floor(value / 128);
      bytes.push(byte | (value ? 128 : 0));
    } while (value);
    return Buffer.from(bytes);
  };
  type Frame = { start: number; body: number; end: number; next: number; kind: number };
  const frames = (bytes: Buffer): Frame[] => {
    const output: Frame[] = [];
    let at = 32;
    while (at < bytes.length) {
      const start = at;
      let length = 0,
        scale = 1,
        byte = 128;
      for (let count = 0; byte & 128; count++) {
        if (count === 10 || at >= bytes.length) throw new Error("invalid oracle frame length");
        byte = requireHubDatum(bytes[at++],"frame length byte");
        length += (byte & 127) * scale;
        scale *= 128;
      }
      const end = at + length,
        next = end + 8;
      if (!Number.isSafeInteger(length) || length < 2 || next > bytes.length || bytes.readUInt32LE(end + 4) !== next - start) throw new Error("invalid oracle frame bounds");
      output.push({ start, body: at, end, next, kind: requireHubDatum(bytes[at],"frame kind") });
      at = next;
    }
    return output;
  };
  const frame = (kind: number, payload: Buffer): Buffer => {
    const body = Buffer.concat([Buffer.from([kind, 2]), payload]),
      prefix = varint(body.length);
    return Buffer.concat([prefix, body, integer(crc(body), 4), integer(prefix.length + body.length + 8, 4)]);
  };
  const build = (test: any): Buffer[] => {
    const segments: Buffer[] = [];
    for (let index = 0; index < test.segments; index++) {
      const header = Buffer.alloc(32);
      Buffer.from([137, 83, 80, 82, 13, 10, 26, 10]).copy(header);
      header.writeUInt16LE(1, 8);
      header.writeUInt32LE(test.mutation === "missing-required-chain" ? 0 : 1, 12);
      header.writeUInt32LE(crc(header.subarray(0, 20)), 20);
      let bytes = header,
        chain = digest(header),
        sequence = 1,
        previousOffset = 0;
      let pending: Buffer[] = [];
      const append = (kind: number, payload: Buffer): void => {
        pending.push(frame(kind, payload));
      };
      const commit = (): void => {
        chain = digest(Buffer.concat([chain, ...pending.map(digest)]));
        const length = pending.reduce((sum, item) => sum + item.length, 0);
        const payload = Buffer.concat([integer(sequence++, 8), integer(previousOffset, 8), integer(length, 8), integer(pending.length, 4), Buffer.alloc(4), chain]);
        previousOffset = bytes.length + length;
        bytes = Buffer.concat([bytes, ...pending, frame(12, payload)]);
        pending = [];
      };
      const document = Buffer.from(index === 1 && test.mutation === "wrong-segment-document" ? "other-document" : proof.documentKey);
      const previous = index ? Buffer.from(requireHubDatum(segments[index - 1],"segments[index - 1]").subarray(-40, -8)) : Buffer.alloc(0);
      if (index && test.mutation === "wrong-prior-tip") previous[0] = requireHubDatum(previous[0],"previous[0]") ^ 1;
      append(64, Buffer.concat([varint(document.length), document, integer(index === 1 && test.mutation === "skipped-segment-index" ? 2 : index, 8), Buffer.from([index ? 1 : 0]), previous]));
      commit();
      for (const tx of test.segments === 1 ? [1, 2] : [index + 1]) {
        const command = Buffer.from(proof.encodedHex, "hex");
        if (tx === 1) command[command.length - 1] = requireHubDatum(command[command.length - 1],"command[command.length - 1]") ^ 1;
        append(65, integer(tx, 8));
        append(68, command);
        append(66, Buffer.concat([integer(tx, 8), integer(1, 4)]));
        commit();
      }
      segments.push(bytes);
    }
    if (test.mutation.endsWith("crc-repaired")) {
      const bytes = requireHubDatum(segments[0],"segments[0]"),
        all = frames(bytes);
      const selected = requireHubDatum(test.mutation === "record-crc-repaired" ? all.find((item) => item.kind === 68)! : all.filter((item) => item.kind === 12)[1],"test.mutation === \"record-crc-repaired\" ? all.find((item) => item.kind === 68)! : all.filter((it");
      const offsets: Record<string, number> = {
        "commit-hash-crc-repaired": 32,
        "commit-count-crc-repaired": 24,
        "commit-length-crc-repaired": 16,
        "commit-sequence-crc-repaired": 0,
        "commit-offset-crc-repaired": 8,
        "commit-reserved-crc-repaired": 28,
      };
      const offset = test.mutation === "record-crc-repaired" ? selected.end - 1 : test.mutation === "noncritical-commit-crc-repaired" ? selected.body + 1 : selected.body + 2 + requireHubDatum(offsets[test.mutation],"offsets[test.mutation]");
      if (!Number.isSafeInteger(offset)) throw new Error("unknown oracle mutation");
      bytes[offset] = requireHubDatum(bytes[offset],"bytes[offset]") ^ (["record-crc-repaired", "noncritical-commit-crc-repaired"].includes(test.mutation) ? 2 : 1);
      bytes.writeUInt32LE(crc(bytes.subarray(selected.body, selected.end)), selected.end);
    }
    return segments;
  };
  const accepted = (segments: Buffer[], first = 0, requireGenesis = true): boolean => {
    if (requireGenesis && first !== 0) return false;
    let previous: Buffer | undefined;
    for (const [relative, bytes] of segments.slice(first).entries()) {
      const index = relative + first;
      if (bytes.readUInt32LE(12) !== 1 || bytes.readUInt32LE(20) !== crc(bytes.subarray(0, 20))) return false;
      let chain = digest(bytes.subarray(0, 32)),
        count = 0,
        length = 0,
        sequence = 1,
        previousOffset = 0;
      let pending: Buffer[] = [chain];
      const all = frames(bytes);
      for (const [position, item] of all.entries()) {
        const payload = bytes.subarray(item.body + 2, item.end);
        if (bytes[item.body + 1] !== 2 || bytes.readUInt32LE(item.end) !== crc(bytes.subarray(item.body, item.end))) return false;
        if (item.kind === 12) {
          if (
            payload.length !== 64 ||
            item.next - item.start !== 75 ||
            payload.readBigUInt64LE(0) !== BigInt(sequence++) ||
            payload.readBigUInt64LE(8) !== BigInt(previousOffset) ||
            payload.readBigUInt64LE(16) !== BigInt(length) ||
            payload.readUInt32LE(24) !== count ||
            payload.readUInt32LE(28) !== 0 ||
            !payload.subarray(32).equals(digest(Buffer.concat(pending)))
          )
            return false;
          chain = Buffer.from(payload.subarray(32));
          pending = [chain];
          count = 0;
          length = 0;
          previousOffset = item.start;
        } else {
          if (position === 0) {
            const document = Buffer.from(proof.documentKey);
            const tip = previous ?? (index ? payload.subarray(-32) : Buffer.alloc(0));
            const expected = Buffer.concat([varint(document.length), document, integer(index, 8), Buffer.from([index ? 1 : 0]), tip]);
            if (item.kind !== 64 || !payload.equals(expected)) return false;
          } else if (item.kind === 64 || ![65, 66, 68].includes(item.kind)) return false;
          pending.push(digest(bytes.subarray(item.start, item.next)));
          count++;
          length += item.next - item.start;
        }
      }
      if (count !== 0 || sequence === 1 || all.at(-1)?.kind !== 12) return false;
      previous = chain;
    }
    return true;
  };
  if (blake3Hex(Buffer.from("abc")) !== "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85") throw new Error("BLAKE3 independent known-answer mismatch");
  for (const test of fixture.cases) {
    const segments = build(test);
    for (const bytes of segments) for (const item of frames(bytes)) if (crc(bytes.subarray(item.body, item.end)) !== bytes.readUInt32LE(item.end)) throw new Error(`oracle hostile CRC was not repaired: ${test.name}`);
    if (accepted(segments) !== test.accepted) throw new Error(`WAL chain oracle mismatch: ${test.name}`);
  }
  for (const owner of fixture.hashingOwnership) {
    if (owner.expected !== (owner.interrupt === "deadline" ? "expired" : "cancelled") || owner.hashingSteps !== 1 || owner.stoppedProgress !== 0 || owner.heldActive !== 1 || owner.releasedActive !== 0)
      throw new Error("hashing interrupt ownership fixture differs");
  }
  for (const boundary of fixture.retainedBoundaries) {
    const segments = build({ segments: 2, mutation: boundary.mutation });
    if (accepted(segments, 1, false) !== boundary.replayAccepted || accepted(segments, 1, true) !== boundary.genesisProofAccepted) throw new Error("unanchored retained suffix became a genesis proof");
  }
  console.log(
    `inference-wal-chain-oracle: exact=${fixture.cases.length} hashing-ownership=${fixture.hashingOwnership.length} retained-boundaries=${fixture.retainedBoundaries.length} ajv=1 crc-valid=14 blake3-known-answer=1; Rust replay and third-party blake3 parity pending`,
  );
}

async function proveInferenceCatalogSelectionFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "🎯️inference-catalog-selection-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validateSelection = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceCatalogSelectionV1");
  if (!validateSelection({ scope: fixture.scope, descriptor: fixture.descriptor, package: fixture.package, services: fixture.services }))
    throw new Error("catalog selection fixture is not a hub.inference/InferenceCatalogSelectionV1");
  if (fixture.schema !== "semio.hub.inference-catalog-selection-fixture/v1") throw new Error("catalog selection fixture envelope drifted");
  if (fixture.cases.length !== 12 || new Set(fixture.cases.map((test: { name: string }) => test.name)).size !== 12) throw new Error("catalog selection fixture inventory drifted");
  for (const test of fixture.cases) {
    const row = structuredClone(fixture);
    if (test.path.length) {
      let at = row;
      for (const key of test.path.slice(0, -1)) at = at[key];
      at[test.path.at(-1)] = test.value;
    }
    const descriptor = row.descriptor,
      owner = descriptor.owner,
      selected = row.package;
    const services = row.services.filter((service: any) => service.inferenceSchema === "s.gis.gismap.inference");
    const service = services[0];
    const accepted =
      row.scope.spaceId === descriptor.spaceId &&
      row.scope.documentId === descriptor.documentId &&
      descriptor.artifactKind === "s.gis.gismap" &&
      descriptor.artifactSchema === "gis.map" &&
      owner.pluginId === "gis" &&
      owner.packageId === "semio:gis" &&
      selected.pluginId === owner.pluginId &&
      selected.packageId === owner.packageId &&
      selected.version === owner.version &&
      selected.componentSha256 === owner.packageHash &&
      row.services.length <= 64 &&
      services.length === 1 &&
      service.owner === "gis" &&
      service.contributor === "gis" &&
      service.artifactKind === descriptor.artifactKind &&
      service.artifactSchema === "s.gis.gismap" &&
      service.dependsOn.length === 0 &&
      [service.artifactSchemaVersion, service.inferenceSchemaVersion, service.algorithmVersion, service.policyVersion].every((version) => version === 1);
    if (accepted !== test.accepted) throw new Error(`catalog selection mismatch: ${test.name}`);
  }
  console.log(`inference-catalog-projection-oracle: exact=${fixture.cases.length}; no native provider or route authority`);
}

/** 🧭️ Proves request-id reconciliation is closed, reader-bound and expiry-independent. */
async function proveInferenceJobReconcileFixture(repoRoot: string): Promise<void> {
  const { proveJobReconcileContract } = await import(join(repoRoot, "🧰️framework/🔨️modules/🧵️job/🔎️reconcile/🧪️tests/🔬️contract/🟦️.ts"));
  proveJobReconcileContract();
  const root = join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "🧭️inference-job-reconcile-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const neutralSchema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🧵️job/🔎️reconcile/🧬️schema/🔣️.json"), "utf8"));
  const validateRequest = new Ajv({ strict: true }).addKeyword("x-semio-formats").compile({ ...neutralSchema, $ref: "#/$defs/JobReconcileRequestV1" });
  const validateResult = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceJobReconcileResultV1");
  const { parseJobReconcileRequestV1 } = await import(join(repoRoot, "🧰️framework/🔨️modules/🧵️job/🔎️reconcile/🧬️schema/🟦️.ts"));
  const { parseInferenceJobReconcileResultV1 } = await import(join(repoRoot, "🌎️hub", "💡️inference", "🧬️schema", "🟦️.ts"));
  const { parseJobReconcileResultV1 } = await import(join(repoRoot, "🧰️framework/🔨️modules/🧵️job/🔎️reconcile/🧬️schema/🟦️.ts"));
  const { parseInferenceReconcilePayloadV1 } = await import(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🔎️reconcile/🟦️.ts"));
  if (fixture.schema !== "semio.hub.inference-job-reconcile-fixture/v1" || fixture.maximumRequestBytes !== 256 || fixture.requestCases.length !== 8 || fixture.results.length !== 11 || fixture.readerCases.length !== 6) {
    throw new Error("inference reconcile fixture envelope drifted");
  }
  const mutate = (value: unknown, path: string[], replacement: unknown): unknown => {
    const candidate = structuredClone(value);
    if (path.length === 0) return candidate;
    let at = candidate as Record<string, unknown>;
    for (const segment of path.slice(0, -1)) at = at[segment] as Record<string, unknown>;
    at[path.at(-1)!] = replacement;
    return candidate;
  };
  for (const row of fixture.requestCases) {
    const candidate = mutate(fixture.request, row.path, row.value);
    let parsed = false;
    try {
      parsed = JSON.stringify(parseJobReconcileRequestV1(candidate)) === JSON.stringify(candidate);
    } catch {}
    if (validateRequest(candidate) !== row.accepted || parsed !== row.accepted) throw new Error(`inference reconcile request parity ${row.name}`);
  }
  const requestBytes = Buffer.from(JSON.stringify(fixture.request));
  const decodeRequest = (bytes: Buffer): boolean => {
    if (bytes.length === 0 || bytes.length > fixture.maximumRequestBytes) return false;
    try {
      return validateRequest(JSON.parse(bytes.toString("utf8"))) as boolean;
    } catch {
      return false;
    }
  };
  const boundary = Buffer.alloc(fixture.maximumRequestBytes, 0x20);
  requestBytes.copy(boundary);
  if (!decodeRequest(requestBytes) || !decodeRequest(boundary) || decodeRequest(Buffer.concat([boundary, Buffer.from(" ")]))) throw new Error("inference reconcile request byte boundary differs");
  const named = new Map(fixture.results.filter((row: { value?: unknown }) => row.value !== undefined).map((row: { name: string; value: unknown }) => [row.name, row.value]));
  for (const row of fixture.results) {
    const candidate = row.copy === undefined ? row.value : mutate(named.get(row.copy), row.path, row.value);
    let parsed = false;
    try {
      parsed = JSON.stringify(parseInferenceJobReconcileResultV1(candidate)) === JSON.stringify(candidate);
    } catch {}
    let portAccepted = false;
    try { parseJobReconcileResultV1(candidate, parseInferenceReconcilePayloadV1); portAccepted = true; } catch {}
    if (validateResult(candidate) !== row.accepted || parsed !== row.accepted || portAccepted !== row.accepted) throw new Error(`inference reconcile result parity ${row.name}`);
  }
  const identity = { userId: "a".repeat(32), sessionId: "b".repeat(32), authorizationGeneration: 1, spaceId: "c".repeat(32), documentId: "d".repeat(32) };
  for (const row of fixture.readerCases) {
    const reader = { ...identity, ...(row.field === "none" ? {} : { [row.field]: row.value }) };
    const matches = Object.keys(identity).every((key) => reader[key as keyof typeof reader] === identity[key as keyof typeof identity]);
    if (matches !== row.accepted) throw new Error(`inference reconcile reader predicate ${row.name}`);
  }
  console.log(`inference-job-reconcile-oracle: requests=${fixture.requestCases.length} results=${fixture.results.length} readers=${fixture.readerCases.length} byte-boundaries=3 ajv+typescript=1; no lookup, route, cancel, or recovery claim`);
}

export class CanonicalArchitectureScript extends BundleScript {
  async run(): Promise<void> {
    await proveInferenceJobReconcileFixture(this.repoRoot);
    const { proveLocalSessionBrokerContract } = await import("../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🎫️local-session/🗄️broker/🧪️tests/🔬️contract/🟦️.ts");
    await proveLocalSessionBrokerContract(this.invocation,exactCargoCallerControlV1(this.invocation,"hub:session-contract",30000));
    await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 3_600_000, listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [{
      package: "semio-hub-trusted-catalog-contract-laws",
      target: { kind: "lib" },
      laws: ["every_manifest_case_is_accepted_exactly_when_the_fixture_says_so", "only_the_canonical_manifest_bytes_decode_and_both_digests_agree_with_the_fixture", "every_listed_file_verifies_against_its_own_bytes_and_tampering_is_refused", "every_index_case_is_accepted_exactly_when_the_fixture_says_so", "every_file_is_served_with_the_fixture_media_type", "an_assembled_module_is_the_fixture_manifest_and_writes_content_addressed_files"],
    },
    {
      package: "semio-hub-trusted-catalog-laws",
      target: { kind: "lib" },
      laws: ["a_loaded_catalog_indexes_and_serves_every_verified_plugin_module_file_and_refuses_a_tampered_one"],
    }]});
  }
}

export class GisInferenceLedgerOracleScript extends BundleScript {
  async run(): Promise<void> {
    const fixture = JSON.parse(readFileSync(join(this.repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "🗺️gis-inference-job-v1", "🔣️.json"), "utf8"));
    const schemaRoot = join(this.repoRoot, "🌎️hub", "💡️inference", "🧬️schema");
    const validate = hubSchemaExport(this.repoRoot, "schema://hub.inference/InferenceRequestV1");
    const validateIdentity = hubSchemaExport(this.repoRoot, "schema://hub.inference/InferenceIdentityV1");
    const validateOutbox = hubSchemaExport(this.repoRoot, "schema://hub.inference/InferenceApprovalOutboxV1");
    const validateSummary = hubSchemaExport(this.repoRoot, "schema://hub.inference/InferenceMapSummaryV1");
    const validateServerId = hubSchemaExport(this.repoRoot, "schema://hub.inference/InferenceServerIdV1");
    if (fixture.schema !== "semio.hub.gis-inference-ledger-fixture/v1" || fixture.version !== 1) throw new Error("GIS ledger fixture envelope drifted");
    if (fixture.traces.length !== 9 || fixture.hostileRequests.length !== 13 || fixture.hostileIdentities.length !== 17 || fixture.sqliteIntegers.length !== 6) throw new Error("GIS ledger fixture inventory drifted");
    if (!validateSummary(fixture.expectedInference)) throw new Error("GIS ledger expected inference is not a hub.inference/InferenceMapSummaryV1");
    if (!validateOutbox(fixture.outbox)) throw new Error("GIS ledger outbox is not a hub.inference/InferenceApprovalOutboxV1");
    const { parseInferenceRequestV1, parseInferenceIdentityV1, parseInferenceServerIdV1 } = await import(join(schemaRoot, "🟦️.ts"));
    if (!validate(fixture.identity.request)) throw new Error("invalid neutral inference intent");
    if (!validateIdentity(fixture.identity)) throw new Error("GIS ledger identity is not a hub.inference/InferenceIdentityV1");
    parseInferenceIdentityV1(fixture.identity);
    const identityRoot = join(this.repoRoot, "🌎️hub/🧪️tests/🧫️fixtures/🖥️inference-server-identity-v1");
    const identityFixture = JSON.parse(readFileSync(join(identityRoot, "🔣️.json"), "utf8"));
    if (identityFixture.schema !== "semio.hub.inference-server-identity-fixture/v1" || identityFixture.maximumBytes !== 96 || identityFixture.cases.length !== 11) throw new Error("server identity corpus envelope drifted");
    if (JSON.stringify(identityFixture.fields) !== JSON.stringify(["userId", "sessionId", "spaceId", "documentId", "headEditId"])) throw new Error("server identity corpus field set drifted");
    for (const row of identityFixture.cases) {
      let parsed = true;
      try {
        parseInferenceServerIdV1(row.value);
      } catch {
        parsed = false;
      }
      if (validateServerId(row.value) !== row.accepted || parsed !== row.accepted) throw new Error(`server identity contract parity: ${row.name}`);
      assertObservationExpectation(`server identity ${row.name}`, { stage: "contract", result: row.accepted ? "accepted" : "rejected", code: row.name }, row.accepted);
    }
    for (const row of identityFixture.cases)
      for (const field of identityFixture.fields) {
        const candidate = { ...fixture.identity, headOrdinal: 1, headEditId: "0".repeat(32), [field]: row.value };
        if (validateIdentity(candidate) !== row.accepted) throw new Error(`server identity parity: ${row.name}/${field}`);
      }
    const maximumId = "a".repeat(identityFixture.maximumBytes);
    const documentKey = `v1:${maximumId.length}:${maximumId.length}:${maximumId}${maximumId}`;
    const actor = `user:${maximumId}#session:${maximumId}`;
    if (Buffer.byteLength(documentKey) > 256 || Buffer.byteLength(actor) > 256) throw new Error("server identifiers exceed the exact command text bound");
    console.log(`inference-server-identity-oracle: cases=${identityFixture.cases.length} fields=${identityFixture.fields.length} composite-bounds=2 ajv+node=1`);
    for (const hostile of fixture.hostileIdentities) {
      const candidate = JSON.parse(JSON.stringify(fixture.identity));
      let at = candidate;
      for (const segment of hostile.path.slice(0, -1)) at = at[segment] ??= {};
      at[hostile.path[hostile.path.length - 1]] = hostile.value;
      if (validateIdentity(candidate)) throw new Error(`accepted hostile inference identity: ${hostile.name}`);
    }
    parseInferenceRequestV1(fixture.identity.request);
    for (const row of fixture.sqliteIntegers) {
      const integer = BigInt(row.decimal);
      if ((integer >= 0n && integer <= BigInt(Number.MAX_SAFE_INTEGER)) !== row.accepted) throw new Error(`neutral SQLite integer mismatch ${row.decimal}`);
    }
    for (const hostile of fixture.hostileRequests) {
      const candidate = { ...fixture.identity.request, [hostile.field]: hostile.value };
      if (validate(candidate)) throw new Error(`AJV admitted ${hostile.name}`);
      let rejected = false;
      try {
        parseInferenceRequestV1(candidate);
      } catch {
        rejected = true;
      }
      if (!rejected) throw new Error(`TypeScript admitted ${hostile.name}`);
    }
    const hash = (value: string) => createHash("sha256").update(value).digest("hex");
    if (hash(fixture.input) !== fixture.identity.inputHash || hash(`semio.hub.inference-identity/v1\0${JSON.stringify(fixture.identity)}`) !== fixture.identityDigest) throw new Error("neutral input/identity hash mismatch");
    const outbox = fixture.outbox;
    const jobId = hash(`semio.hub.inference-job-id/v1\0${fixture.identityDigest}`).slice(0, 32);
    if (
      jobId !== outbox.jobId ||
      hash(outbox.proposal) !== outbox.proposalHash ||
      createHash("sha256").update(Buffer.from(outbox.commandHex, "hex")).digest("hex") !== outbox.commandHash ||
      hash(`semio.hub.inference-approval-mutation/v1\0${jobId}\0${outbox.proposalHash}`).slice(0, 32) !== outbox.mutationId
    )
      throw new Error("neutral durable outbox identity mismatch");
    const snapshot = JSON.parse(fixture.input);
    const points: number[][] = [];
    const scan = (value: any): void => {
      if (Array.isArray(value)) {
        if (value.length === 2 && value.every((part) => typeof part === "number" && Number.isFinite(part))) points.push(value);
        else value.forEach(scan);
      } else if (typeof value === "object" && value !== null) {
        if (typeof value.lon === "number" && Number.isFinite(value.lon) && typeof value.lat === "number" && Number.isFinite(value.lat)) points.push([value.lon, value.lat]);
        Object.values(value).forEach(scan);
      }
    };
    for (const item of [...snapshot.positions, ...snapshot.routes, ...snapshot.regions]) scan(item.data);
    const bounds = points.length ? { lonMin: Math.min(...points.map((p) => p[0]!)), lonMax: Math.max(...points.map((p) => p[0]!)), latMin: Math.min(...points.map((p) => p[1]!)), latMax: Math.max(...points.map((p) => p[1]!)) } : null;
    const result = { positionCount: snapshot.positions.length, routeCount: snapshot.routes.length, regionCount: snapshot.regions.length, bounds };
    if (JSON.stringify(result) !== JSON.stringify(fixture.expectedInference)) throw new Error("independent deterministic GIS bounds/counts differ");
    for (const trace of fixture.traces) {
      let state = "accepted",
        proposal = "none",
        events = 1,
        hasResult = false;
      for (const operation of trace.operations) {
        if (operation === "start" && state === "accepted") {
          state = "running";
          events++;
        } else if (operation === "succeed" && state === "running") {
          state = "succeeded";
          proposal = "offered";
          hasResult = true;
          events++;
        } else if (operation === "cancel" && (state === "accepted" || state === "running")) {
          state = "cancelled";
          events++;
        } else if (operation === "cancel" && state === "succeeded" && proposal === "offered") {
          proposal = "cancelled";
          hasResult = false;
          events++;
        }
      }
      if (state !== trace.state || proposal !== trace.proposalState || events !== trace.eventCount || hasResult !== trace.hasResult) throw new Error(`neutral lifecycle mismatch ${trace.name}`);
    }
    console.log(
      `gis-inference-ledger-oracle: traces=${fixture.traces.length} hostile=${fixture.hostileRequests.length} identity-hostile=${fixture.hostileIdentities.length} sqlite-integers=${fixture.sqliteIntegers.length} ajv+typescript=1 hashes=6 independent-bounds=1; no executor/route/approval claim`,
    );
    await proveInferenceJobReconcileFixture(this.repoRoot);
    await proveInferenceWalProofFixture(this.repoRoot);
    await proveInferenceCommandFixture(this.repoRoot);
    await proveInferenceApprovalRequestFixture(this.repoRoot);
    await proveInferenceAuthorFixture(this.repoRoot);
    await proveInferenceWalChainFixture(this.repoRoot);
    await proveInferenceCatalogSelectionFixture(this.repoRoot);
    await proveTrustedCatalogIdentityRolesFixture(this.repoRoot);
    await (await import("../../🧩️compositions/🌍️gis/🧪️tests/📇️native-codecs/🟦️.ts")).proveGisNativeCodecReceipts(this.repoRoot);
    await (await import("../../🧩️compositions/🌍️gis/🧪️tests/💡️inference-control/🟦️.ts")).proveGisControlledProposal(this.repoRoot);
    await proveGisNativeProviderSelectionFixture(this.repoRoot);
    await (await import(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🧮️memory-backing/🟦️.ts"))).proveMemoryBackendBackingFixture(this.repoRoot);
    await proveNativeDeficitFixture(this.repoRoot);
  }
}

/** ✅ Validates the closed approval intent independently of Rust and its transport authority. */
async function proveInferenceApprovalRequestFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub/🧪️tests/🧫️fixtures/✅️inference-approval-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validate = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceApprovalRequestV1");
  const { parseInferenceApprovalRequestV1 } = await import(join(repoRoot, "🌎️hub", "💡️inference", "🧬️schema", "🟦️.ts"));
  if (fixture.schema !== "semio.hub.inference-approval-fixture/v1" || fixture.maximumBytes !== 1024 || fixture.hostile.length !== 14) throw new Error("inference approval fixture envelope drifted");
  const decode = (bytes: Buffer): boolean => {
    if (bytes.length > fixture.maximumBytes) return false;
    try {
      return validate(JSON.parse(bytes.toString("utf8"))) as boolean;
    } catch {
      return false;
    }
  };
  const request = Buffer.from(JSON.stringify(fixture.request));
  if (!decode(request)) throw new Error("valid inference approval intent denied");
  parseInferenceApprovalRequestV1(fixture.request);
  for (const hostile of fixture.hostile) {
    const candidate = { ...fixture.request, [hostile.field]: hostile.value };
    const admitted = decode(Buffer.from(JSON.stringify(candidate)));
    let parsed = true;
    try {
      parseInferenceApprovalRequestV1(candidate);
    } catch {
      parsed = false;
    }
    if (parsed) throw new Error(`TypeScript inference approval admitted client authority ${hostile.field}`);
    assertObservationExpectation(`inference approval ${hostile.code}`, hostile, admitted);
  }
  const boundary = Buffer.alloc(fixture.maximumBytes, 0x20);
  request.copy(boundary);
  if (!decode(boundary) || decode(Buffer.concat([boundary, Buffer.from(" ")]))) throw new Error("inference approval exact byte boundary differs");
  console.log(`inference-approval-request-oracle: valid=1 hostile=${fixture.hostile.length} byte-boundaries=2 ajv+node=1; no approval authority`);
}

/** 🛂 Evaluates durable membership and immutable identity predicates through independent SQLite. */
async function proveInferenceAuthorFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub/🧪️tests/🧫️fixtures/🛂️inference-author-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  if (fixture.schema !== "semio.hub.inference-author-fixture/v1" || fixture.cases.length !== 16 || new Set(fixture.cases.map((row: { operation: string }) => row.operation)).size !== 16) throw new Error("inference Author corpus envelope drifted");
  const { Database } = await import("bun:sqlite");
  const database = new Database(":memory:");
  try {
    database.exec(
      "CREATE TABLE session(user TEXT, session TEXT, generation INTEGER, expires INTEGER, revoked INTEGER); INSERT INTO session VALUES('author','session',1,2000,0); CREATE TABLE membership(space TEXT,user TEXT,role TEXT); INSERT INTO membership VALUES('space','author','author')",
    );
    const predicate = database.query(
      "SELECT EXISTS(SELECT 1 FROM session s JOIN membership m ON m.user=s.user WHERE s.user=?1 AND s.session=?2 AND s.generation=?3 AND s.revoked=0 AND s.expires>?4 AND s.expires>?9 AND ?9>=?4 AND m.space=?5 AND m.role='author' AND ?5='space' AND ?6='document' AND ?4>=0 AND ?7=0 AND ?8=0) AS accepted",
    );
    let accepted = 0;
    const seen = new Set<string>();
    for (const row of fixture.cases) {
      if (seen.has(row.operation)) throw new Error("duplicate inference Author operation");
      seen.add(row.operation);
      database.exec("UPDATE session SET revoked=0; DELETE FROM membership; INSERT INTO membership VALUES('space','author','author')");
      let user = "author",
        session = "session",
        generation = 1,
        now = 1000,
        returnedAt = 1000,
        space = "space",
        document = "document",
        cancelled = 0,
        deadline = 0;
      switch (row.operation) {
        case "author":
          break;
        case "cross-space":
          space = "other";
          break;
        case "cross-document":
          document = "other";
          break;
        case "wrong-user":
          user = "other";
          break;
        case "wrong-session":
          session = "other";
          break;
        case "rotated-generation":
          generation = 2;
          break;
        case "spectator":
          database.exec("UPDATE membership SET role='spectator'");
          break;
        case "removed-member":
          database.exec("DELETE FROM membership");
          break;
        case "revoked":
          database.exec("UPDATE session SET revoked=1");
          break;
        case "expiry-exact":
          now = returnedAt = 2000;
          break;
        case "expiry-past":
          now = returnedAt = 2001;
          break;
        case "expiry-after-read":
          returnedAt = 2000;
          break;
        case "clock-regressed":
          returnedAt = 999;
          break;
        case "cancelled":
          cancelled = 1;
          break;
        case "deadline":
          deadline = 1;
          break;
        case "negative-clock":
          now = -1;
          break;
        default:
          throw new Error("unknown inference Author operation");
      }
      const result = predicate.get(user, session, generation, now, space, document, cancelled, deadline, returnedAt) as { accepted: number };
      if ((result.accepted === 1) !== row.accepted) throw new Error(`inference Author predicate mismatch: ${row.operation}`);
      accepted += result.accepted;
    }
    if (accepted !== 1) throw new Error("inference Author corpus must admit exactly one case");
    console.log(`inference-author-oracle: cases=${fixture.cases.length} accepted=${accepted} ajv+sqlite=1; no retained grant or submit authority`);
  } finally {
    database.close();
  }
}

/** ✉️ Independent bounded canonical envelope oracle; it does not interpret GIS mutations. */
async function proveInferenceCommandFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub", "🧪️tests/🧫️fixtures", "✉️inference-command-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validateLimits = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceCommandLimitsV1");
  const validateCommand = hubSchemaExport(repoRoot, "schema://hub.inference/InferenceCommandV1");
  if (fixture.schema !== "semio.hub.inference-command-fixture/v1" || fixture.authority !== "canonical-envelope-only-not-gis-execution" || fixture.source !== "../🧾️inference-wal-proof-v1/🔣️.json")
    throw new Error("inference command fixture envelope drifted");
  if (!validateLimits(fixture.limits)) throw new Error("inference command limits are not a hub.inference/InferenceCommandLimitsV1");
  if (fixture.vectors.length !== 20 || new Set(fixture.vectors.map((row: { name: string }) => row.name)).size !== 20 || new Set(fixture.vectors.map((row: { change: string }) => row.change)).size !== 20)
    throw new Error("inference command fixture inventory drifted");
  const source = JSON.parse(readFileSync(join(root, fixture.source), "utf8"));
  if (!validateCommand(source.command)) throw new Error("inference command source is not a hub.inference/InferenceCommandV1");
  const limits = fixture.limits;
  const variable = (value: number | bigint): Buffer => {
    let remaining = BigInt(value);
    const bytes: number[] = [];
    do {
      const byte = Number(remaining & 127n);
      remaining >>= 7n;
      bytes.push(byte | (remaining ? 128 : 0));
    } while (remaining);
    return Buffer.from(bytes);
  };
  const encode = (command: any): Buffer => {
    const parts: Buffer[] = [];
    const field = (value: Buffer): void => {
      parts.push(variable(value.length), value);
    };
    const text = (value: string): void => field(Buffer.from(value, "utf8"));
    text(command.mutationId);
    text(command.documentId);
    text(command.actor);
    parts.push(variable(command.dependencies.length));
    command.dependencies.forEach(text);
    parts.push(variable(0), variable(0));
    text(command.diff.schema);
    field(Buffer.from(command.diff.payloadHex, "hex"));
    text(command.inverse.schema);
    field(Buffer.from(command.inverse.payloadHex, "hex"));
    parts.push(variable(command.timestamp.actor), variable(command.timestamp.physicalMs), variable(command.timestamp.logical));
    return Buffer.concat(parts);
  };
  const decode = (bytes: Buffer): any => {
    if (!bytes.length || bytes.length > limits.commandBytes) throw new Error("bounds");
    let position = 0;
    const integer = (): number => {
      const start = position;
      let value = 0n;
      for (let shift = 0n; shift <= 63n; shift += 7n) {
        if (position === bytes.length) throw new Error("truncated");
        const byte = requireHubDatum(bytes[position++],"bytes[position++]");
        if (shift === 63n && byte > 1) throw new Error("overflow");
        value |= BigInt(byte & 127) << shift;
        if (!(byte & 128)) {
          if (value > BigInt(limits.integerMaximum) || !variable(value).equals(bytes.subarray(start, position))) throw new Error("noncanonical");
          return Number(value);
        }
      }
      throw new Error("overflow");
    };
    const field = (maximum: number): Buffer => {
      const length = integer();
      if (length > maximum || length > bytes.length - position) throw new Error("bounds");
      const value = bytes.subarray(position, position + length);
      position += length;
      return value;
    };
    const text = (): string => {
      const value = new TextDecoder("utf-8", { fatal: true }).decode(field(limits.textBytes));
      if (!value || /\p{Cc}/u.test(value)) throw new Error("text");
      return value;
    };
    const command: any = { mutationId: text(), documentId: text(), actor: text(), dependencies: [] };
    const count = integer();
    if (count > limits.dependencyCount) throw new Error("bounds");
    for (let index = 0; index < count; index++) {
      const value = text();
      if (command.dependencies.includes(value)) throw new Error("duplicate");
      command.dependencies.push(value);
    }
    if (integer() !== 0 || integer() !== 0) throw new Error("server-stamped command carries an observation or a target");
    command.diff = { schema: text(), payloadHex: field(limits.payloadBytes).toString("hex") };
    command.inverse = { schema: text(), payloadHex: field(limits.payloadBytes).toString("hex") };
    command.timestamp = { actor: integer(), physicalMs: integer(), logical: integer() };
    if (position !== bytes.length || !encode(command).equals(bytes)) throw new Error("trailing");
    return command;
  };
  const canonical = encode(source.command);
  if (canonical.toString("hex") !== source.encodedHex || createHash("sha256").update(canonical).digest("hex") !== source.commandHash) throw new Error("canonical source mismatch");
  const webDigest = await webSha256Hex(canonical);
  if (webDigest !== source.commandHash) throw new Error("independent WebCrypto command hash mismatch");
  for (const vector of fixture.vectors) {
    const command = structuredClone(source.command);
    switch (vector.change) {
      case "text-max-plus-one":
        command.actor = "a".repeat(limits.textBytes + 1);
        break;
      case "control-text":
        command.actor += "\u0001";
        break;
      case "empty-schema":
        command.diff.schema = "";
        break;
      case "dependency-max-plus-one":
        command.dependencies = Array.from({ length: limits.dependencyCount + 1 }, (_, index) => index.toString(16).padStart(32, "0"));
        break;
      case "duplicate-dependency":
        command.dependencies = ["e".repeat(32), "e".repeat(32)];
        break;
      case "diff-max-plus-one":
        command.diff.payloadHex = "00".repeat(limits.payloadBytes + 1);
        break;
      case "inverse-max-plus-one":
        command.inverse.payloadHex = "00".repeat(limits.payloadBytes + 1);
        break;
      case "hlc-actor-max-plus-one":
        command.timestamp.actor = BigInt(limits.integerMaximum) + 1n;
        break;
      case "hlc-time-max-plus-one":
        command.timestamp.physicalMs = BigInt(limits.integerMaximum) + 1n;
        break;
      case "hlc-logical-max-plus-one":
        command.timestamp.logical = BigInt(limits.integerMaximum) + 1n;
        break;
      case "different-actor":
        command.actor = command.actor.replace("a", "e");
        break;
      case "different-scope":
        command.documentId = command.documentId.replace("c", "e");
        break;
      case "different-mutation":
        command.mutationId = "e".repeat(32);
        break;
    }
    let bytes = encode(command);
    switch (vector.change) {
      case "trailing":
        bytes = Buffer.concat([bytes, Buffer.from([0])]);
        break;
      case "truncated":
        bytes = bytes.subarray(0, -1);
        break;
      case "oversize":
        bytes = Buffer.alloc(limits.commandBytes + 1);
        break;
      case "overlong-varint":
        bytes = Buffer.concat([Buffer.from([requireHubDatum(bytes[0],"bytes[0]") | 128, 0]), bytes.subarray(1)]);
        break;
      case "overflow-varint":
        bytes = Buffer.concat([Buffer.alloc(9, 255), Buffer.from([2]), bytes.subarray(1)]);
        break;
      case "invalid-utf8":
        bytes[1] = 255;
        break;
    }
    let outcome = "rejected";
    try {
      const value = decode(bytes);
      outcome = ["mutationId", "documentId", "actor"].every((key) => value[key] === source.command[key]) ? "canonical" : "identity-denied";
    } catch {}
    if (outcome !== vector.expected) throw new Error(`inference command boundary mismatch: ${vector.name}`);
  }
  console.log(`inference-command-oracle: vectors=${fixture.vectors.length} ajv=1 node+webcrypto-hash=2; no GIS execution authority`);
}

async function proveNativeDeficitFixture(repoRoot: string): Promise<void> {
  const { toolJobCooperativeMaintenanceSelfTests } = await import(join(repoRoot, "🧰️framework/🔨️modules/⏳️async/🤝️cooperative/🧪️tests/🔬️tool-job-cooperative-maintenance/🟦️.ts"));
  const checks = toolJobCooperativeMaintenanceSelfTests();
  if (checks !== 8) throw new Error("deficit oracle must execute all six lanes and two hostile bounds");
  console.log(`native-deficit-oracle: lanes=6 maximum-rounds=8 checks=${checks}; shared independent oracle, cooperative host turns remain distinct`);
}

/** 🪶️ Verifies the SQLite foundation without representing it as the later two-user route gate. */
export class GisInferenceLedgerCheckScript extends BundleScript {
  async run(): Promise<void> {
    runCmd("bun", ["./📜️script.ts", "gis-inference-ledger-oracle"], { cwd: this.root, budgetMs: 60_000 });
    const suffixes: [string, string][] = [["semio-hub-inference-ledger", "gis_inference_sqlite_ledger_executes_neutral_traces_with_private_first_terminal_wins"], ["semio-hub-inference-ledger", "gis_inference_sqlite_request_identity_capacity_expiry_and_progress_are_bounded"], ["semio-hub-inference-ledger", "gis_inference_sqlite_concurrent_connections_have_one_durable_request_winner"], ["semio-hub-inference-ledger", "gis_inference_sqlite_prepared_approval_survives_restart_and_reconciles_exactly_once"], ["semio-hub-inference-contract-laws", "inference_wal_proof_executes_literal_committed_transaction_scope_and_cancellation_traces"], ["semio-hub-inference-contract-laws", "inference_wal_proof_dropped_caller_cancels_and_finishes_retained_replay_before_release"], ["semio-hub-inference-contract-laws", "inference_wal_chain_rejects_crc_valid_tampering_and_exact_cross_segment_tip_mismatch"], ["semio-hub-inference-contract-laws", "inference_wal_chain_cancellation_retires_hashing_and_compacted_suffix_is_not_a_genesis_proof"], ["semio-hub-inference-catalog", "inference_catalog_projection_requires_exact_scope_package_and_declared_service"], ["semio-hub-inference-contract-laws", "inference_command_exact_decoder_executes_neutral_bounds_canonical_eof_and_actor_vectors"], ["semio-hub-inference-contract-laws", "inference_wal_proof_rejects_hash_matched_noncanonical_or_wrong_actor_commands"], ["semio-hub-trusted-catalog-laws", "loader_retains_exact_bytes_and_independent_identities_before_atomic_codec_activation"], ["semio-hub-trusted-catalog-laws", "verified_trusted_catalog_document_open_generation_and_resolution_are_exact"], ["semio-hub-inference-contract-laws", "inference_approval_request_accepts_only_job_digest_and_exact_body_bound"], ["semio-hub-inference-catalog", "inference_live_author_rechecks_real_sqlite_session_scope_role_revocation_and_cancellation"], ["semio-hub-trusted-catalog-laws", "gis_native_provider_selection_binds_literal_owner_version_and_cancellation_without_publication"]];
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 60_000 },cwd: this.root,groups: [
        { package: "semio-framework-async", target: { kind: "lib", name: "semio_framework_async" }, laws: ["native_drr_finishes_eligible_deficit_frontier_before_idle", "cooperative_maintenance_retains_deficit_until_later_host_turn"] },
        {
          package: "semio-framework-os-kernel-db",
          target: { kind: "lib", name: "db" },
          laws: ["db_io_memory_backend_heap_tables_have_exact_preflight_credit_and_terminal_return", "db_io_saturated_task_retry_wakes_parked_caller_without_unrelated_ingress"],
        },
        ...hubLawGroups(suffixes),
      ],progress(event) {
        console.log(`gis-inference-ledger ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`);
      }});
    for (const receipt of receipts) console.log(`gis-inference-ledger-receipt: ${JSON.stringify(receipt)}`);
    runCmd("cargo", ["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], { cwd: this.root, budgetMs: 3_600_000 });
    console.log(
      `gis-inference-ledger-check: scheduler=2 memory-backing=1 parked-retry=1 sqlite=4 wal=5 catalog-projection=1 canonical-command=1 trusted-catalog=2 approval-request=1 live-author=1 gis-provider-selection=1 exact=${suffixes.length + 4}; no route/GIS-approval acceptance`,
    );
  }
}
