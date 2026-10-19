/** 📦️ The artifact chunk CAS boundary derivation test. */
import { webcrypto } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { expect, it } from "vitest";
import { getWorkspaceRoot } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { scopeExport } from "../🧰️hub-integration-fixtures/🟦️.ts";

/** 🧱 Registers the artifact chunk CAS boundary test with AJV and WebCrypto. */
export function registerArtifactChunkCasBoundaryTests(): void {
  it("validates and independently derives every artifact chunk-CAS boundary with AJV and WebCrypto", async () => {
    const root = getWorkspaceRoot();
    const fixtureRoot = join(root, "🌎️hub", "🗿️artifact-authority", "🧪️tests/🧫️fixtures", "🧱️artifact-chunk-cas");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const authorityExport = scopeExport(root, "hub.artifact-authority");
    const validateManifestPlan = authorityExport("ArtifactCasManifestPlanV1");
    const validateLedgerEvent = authorityExport("ArtifactCasRetentionLedgerEventV1");
    const validateObjectKey = authorityExport("ArtifactCasObjectKeyV1");
    const validateSweepCursor = authorityExport("ArtifactCasSweepCursorV1");
    expect(fixture.version).toBe(1);
    expect(fixture.chunkBytes).toBe(262_144);
    expect(fixture.maximumRawBytes).toBe(67_108_864);
    expect(fixture.maximumChunkCount).toBe(256);
    expect(fixture.maximumManifestBytes).toBe(65_536);
    expect(fixture.largePair.totalLength).toBe(fixture.maximumRawBytes);
    for (const plan of [...fixture.vectors, fixture.largePair.pack, fixture.largePair.spr]) expect(validateManifestPlan(plan), "validateManifestPlan rejected its own fixture").toBe(true);
    for (const event of fixture.retentionLedger.events) expect(validateLedgerEvent(event), "validateLedgerEvent rejected its own fixture").toBe(true);
    for (const snapshot of fixture.retentionLedger.snapshots) for (const object of [...snapshot.protected, ...snapshot.eligible]) expect(validateObjectKey(object)).toBe(true);
    expect(validateSweepCursor(fixture.sweepContinuation.expectedFirstCursor), "validateSweepCursor rejected its own fixture").toBe(true);
    expect(fixture.deleteBarrier.orders.map((order: { id: string }) => order.id)).toEqual(["delete-first", "successor-first"]);
    const u64 = (value: number): Buffer => {
      const bytes = Buffer.alloc(8);
      bytes.writeBigUInt64BE(BigInt(value));
      return bytes;
    };
    const u32 = (value: number): Buffer => {
      const bytes = Buffer.alloc(4);
      bytes.writeUInt32BE(value);
      return bytes;
    };
    const field = (bytes: Uint8Array): Buffer => Buffer.concat([u64(bytes.byteLength), Buffer.from(bytes)]);
    const sha256 = async (bytes: Uint8Array): Promise<Buffer> => Buffer.from(await webcrypto.subtle.digest("SHA-256", bytes));
    const pattern = (length: number): Buffer => {
      const bytes = Buffer.alloc(length);
      for (let index = 0; index < length; index += 1) bytes[index] = (index * 31 + Math.floor(index / 251)) % 256;
      return bytes;
    };
    const derive = async (length: number) => {
      const raw = pattern(length);
      const rawSha256 = await sha256(raw);
      const chunks = Array.from({ length: Math.ceil(length / fixture.chunkBytes) }, (_, ordinal) => raw.subarray(ordinal * fixture.chunkBytes, Math.min(length, (ordinal + 1) * fixture.chunkBytes)));
      const chunkIds = await Promise.all(chunks.map((chunk) => sha256(Buffer.concat([
        Buffer.from("semio.hub.artifact-cas.chunk.v1\0"),
        field(Buffer.from(fixture.spaceId)),
        field(u64(chunk.length)),
        field(chunk),
      ]))));
      const manifest = Buffer.concat([
        Buffer.from("semio.hub.artifact-cas.manifest.v1\0"),
        field(Buffer.from(fixture.spaceId)),
        field(rawSha256),
        field(u64(length)),
        field(u32(fixture.chunkBytes)),
        field(u32(chunks.length)),
        ...chunks.flatMap((chunk, ordinal) => [field(u32(ordinal)), field(u32(chunk.length)), field(chunkIds[ordinal])]),
      ]);
      return {
        length,
        rawSha256: rawSha256.toString("hex"),
        chunkCount: chunks.length,
        manifestBytes: manifest.length,
        manifestId: (await sha256(manifest)).toString("hex"),
        firstChunkId: chunkIds[0]?.toString("hex") ?? null,
        lastChunkId: chunkIds.at(-1)?.toString("hex") ?? null,
      };
    };
    for (const vector of fixture.vectors) expect(await derive(vector.length)).toEqual(vector);
    const large = await derive(fixture.largePair.pack.length);
    expect(large).toEqual(fixture.largePair.pack);
    expect(large).toEqual(fixture.largePair.spr);
    expect(fixture.largePair.pack.length + fixture.largePair.spr.length).toBe(fixture.maximumRawBytes);
    expect(fixture.vectors.map((vector: { length: number }) => vector.length)).toEqual([0, 1, 262_143, 262_144, 262_145]);

    type Reservation = { expiresAtMs: number; objects: string[] };
    const events = fixture.retentionLedger.events as Array<{ generation: number; operation: "reserve" | "publish" | "retention" | "space-delete"; checkpointId?: string; expiresAtMs?: number; objects?: string[] }>;
    expect(events.map((event) => event.generation)).toEqual([1, 2, 3, 4, 5, 6, 7]);
    const reachabilityAt = (throughGeneration: number, nowMs: number) => {
      const universe = new Set<string>();
      const reservations = new Map<string, Reservation>();
      const references = new Map<string, string[]>();
      for (const event of events.filter((candidate) => candidate.generation <= throughGeneration)) {
        if (event.operation === "reserve") {
          if (!event.checkpointId || event.expiresAtMs === undefined || !event.objects) throw new Error("invalid reserve fixture");
          reservations.set(event.checkpointId, { expiresAtMs: event.expiresAtMs, objects: [...event.objects] });
          event.objects.forEach((object) => universe.add(object));
        } else if (event.operation === "publish") {
          if (!event.checkpointId) throw new Error("invalid publish fixture");
          const reservation = reservations.get(event.checkpointId);
          if (!reservation) throw new Error("publish without reservation");
          references.set(event.checkpointId, reservation.objects);
          reservations.delete(event.checkpointId);
        } else if (event.operation === "retention") {
          if (!event.checkpointId) throw new Error("invalid retention fixture");
          for (const checkpointId of references.keys()) if (checkpointId !== event.checkpointId) references.delete(checkpointId);
        } else {
          reservations.clear();
          references.clear();
        }
      }
      const protectedObjects = new Set<string>();
      for (const objects of references.values()) objects.forEach((object) => protectedObjects.add(object));
      for (const reservation of reservations.values()) if (reservation.expiresAtMs > nowMs) reservation.objects.forEach((object) => protectedObjects.add(object));
      return {
        protected: [...protectedObjects].sort(),
        eligible: [...universe].filter((object) => !protectedObjects.has(object)).sort(),
      };
    };
    for (const snapshot of fixture.retentionLedger.snapshots) {
      expect(reachabilityAt(snapshot.throughGeneration, snapshot.nowMs)).toEqual({ protected: snapshot.protected, eligible: snapshot.eligible });
    }
    expect(fixture.retentionLedger.reservationMaximumTtlMs).toBe(300_000);
    expect(fixture.retentionLedger.reservationGraceMs).toBe(60_000);
    expect(fixture.retentionLedger.sweepPageMaximum).toBe(16);
    expect(fixture.retentionLedger.sweepObjectMaximum).toBe(4_096);

    type SweepCursor = { observedGeneration: number; afterGeneration: number; objectOffset: number };
    const sweep = fixture.sweepContinuation as {
      tokenPayloadBytes: number;
      tokenAuthenticationBytes: number;
      cursorExposesObjectIdentity: boolean;
      invalidAfterGenerationChange: boolean;
      invalidAfterRestart: boolean;
      ledgerGeneration: number;
      pageLedgerEvents: number;
      planObjectCounts: number[];
      totalObjects: number;
      requestMaximumObjects: number;
      expectedExaminedPerRequest: number[];
      expectedFirstCursor: SweepCursor;
    };
    const advanceSweep = (continuation: SweepCursor | undefined, maximum: number) => {
      let afterGeneration = continuation?.afterGeneration ?? 0;
      let objectOffset = continuation?.objectOffset ?? 0;
      let examined = 0;
      while (examined < maximum && afterGeneration < sweep.ledgerGeneration) {
        const page = sweep.planObjectCounts.slice(afterGeneration, afterGeneration + sweep.pageLedgerEvents);
        const pageObjects = page.reduce((total, count) => total + count, 0);
        const taken = Math.min(maximum - examined, pageObjects - objectOffset);
        examined += taken;
        objectOffset += taken;
        if (objectOffset < pageObjects) return { examined, continuation: { observedGeneration: sweep.ledgerGeneration, afterGeneration, objectOffset } };
        afterGeneration += page.length;
        objectOffset = 0;
      }
      return { examined, continuation: afterGeneration < sweep.ledgerGeneration ? { observedGeneration: sweep.ledgerGeneration, afterGeneration, objectOffset } : undefined };
    };
    const firstSweep = advanceSweep(undefined, sweep.requestMaximumObjects);
    const secondSweep = advanceSweep(firstSweep.continuation, sweep.requestMaximumObjects);
    expect(firstSweep).toEqual({ examined: sweep.expectedExaminedPerRequest[0], continuation: sweep.expectedFirstCursor });
    expect(secondSweep).toEqual({ examined: sweep.expectedExaminedPerRequest[1], continuation: undefined });
    expect(firstSweep.examined + secondSweep.examined).toBe(sweep.totalObjects);

    const encodeSweepContinuation = async (secret: Buffer, execute: boolean, cursor: SweepCursor): Promise<Buffer> => {
      const payload = Buffer.alloc(sweep.tokenPayloadBytes);
      payload[0] = Number(execute);
      payload.writeBigUInt64BE(BigInt(cursor.observedGeneration), 1);
      payload.writeBigUInt64BE(BigInt(cursor.afterGeneration), 9);
      payload.writeUInt32BE(cursor.objectOffset, 17);
      const authentication = await sha256(Buffer.concat([Buffer.from("semio.hub.artifact-cas.sweep-continuation.v1\0"), secret, payload]));
      return Buffer.concat([payload, authentication]);
    };
    const acceptsSweepContinuation = async (token: Buffer, secret: Buffer, execute: boolean, generation: number): Promise<boolean> => {
      if (token.length !== sweep.tokenPayloadBytes + sweep.tokenAuthenticationBytes || token[0] !== Number(execute) || Number(token.readBigUInt64BE(1)) !== generation) return false;
      const expected = await encodeSweepContinuation(secret, execute, {
        observedGeneration: Number(token.readBigUInt64BE(1)),
        afterGeneration: Number(token.readBigUInt64BE(9)),
        objectOffset: token.readUInt32BE(17),
      });
      return expected.equals(token);
    };
    const firstInstanceSecret = await sha256(Buffer.from("first-instance"));
    const restartedInstanceSecret = await sha256(Buffer.from("restarted-instance"));
    const token = await encodeSweepContinuation(firstInstanceSecret, true, sweep.expectedFirstCursor);
    expect(token.length).toBe(sweep.tokenPayloadBytes + sweep.tokenAuthenticationBytes);
    expect(await acceptsSweepContinuation(token, firstInstanceSecret, true, sweep.ledgerGeneration)).toBe(true);
    expect(await acceptsSweepContinuation(token, firstInstanceSecret, true, sweep.ledgerGeneration + 1)).toBe(!sweep.invalidAfterGenerationChange);
    expect(await acceptsSweepContinuation(token, restartedInstanceSecret, true, sweep.ledgerGeneration)).toBe(!sweep.invalidAfterRestart);
    expect(await acceptsSweepContinuation(token, firstInstanceSecret, false, sweep.ledgerGeneration)).toBe(false);
    expect(sweep.cursorExposesObjectIdentity).toBe(false);

    type BarrierOrder = {
      id: "delete-first" | "successor-first";
      actions: string[];
      oldDeleteOutcome: "deleted-before-successor-stage" | "stale-fence-rejected";
      publishedReadOutcome: "exact";
    };
    const barrier = fixture.deleteBarrier as {
      coordinatorId: string;
      spaceId: string;
      objectBytesHex: string;
      initialPhysicalEpoch: number;
      deleteLeaseEpoch: number;
      successorReservationEpoch: number;
      leaseMaximumMs: number;
      dryRunAdvancesEpoch: boolean;
      orders: BarrierOrder[];
    };
    expect(Buffer.from(barrier.coordinatorId, "hex").length).toBe(32);
    expect(barrier.leaseMaximumMs).toBe(5_000);
    expect(barrier.dryRunAdvancesEpoch).toBe(false);
    const objectBytes = Buffer.from(barrier.objectBytesHex, "hex");
    const executeBarrierOrder = (order: BarrierOrder) => {
      let physicalEpoch = barrier.initialPhysicalEpoch;
      let leaseEpoch: number | undefined;
      let leaseLive = false;
      let reservationEpoch: number | undefined;
      let bytes: Buffer | undefined = Buffer.from(objectBytes);
      let referenced = false;
      let oldDeleteOutcome: BarrierOrder["oldDeleteOutcome"] | undefined;
      for (const action of order.actions) {
        const [kind, encodedEpoch] = action.split(":");
        const epoch = encodedEpoch === undefined ? undefined : Number(encodedEpoch);
        if (kind === "lease") {
          expect(epoch).toBeGreaterThan(physicalEpoch);
          leaseEpoch = epoch;
          leaseLive = true;
        } else if (kind === "lease-expire") {
          leaseLive = false;
        } else if (kind === "reserve") {
          expect(leaseLive).toBe(false);
          expect(epoch).toBeGreaterThan(leaseEpoch ?? 0);
          reservationEpoch = epoch;
        } else if (kind === "cas-advance") {
          if (epoch !== leaseEpoch && epoch !== reservationEpoch) throw new Error("advance without directory epoch");
          physicalEpoch = Math.max(physicalEpoch, epoch ?? 0);
        } else if (kind === "validate") {
          expect(leaseLive && leaseEpoch === epoch && physicalEpoch === epoch && !referenced).toBe(true);
        } else if (kind === "delete") {
          if (physicalEpoch === epoch) {
            bytes = undefined;
            leaseLive = false;
            oldDeleteOutcome = "deleted-before-successor-stage";
          } else {
            oldDeleteOutcome = "stale-fence-rejected";
          }
        } else if (kind === "stage") {
          expect(physicalEpoch).toBe(reservationEpoch);
          bytes = Buffer.from(objectBytes);
        } else if (kind === "publish") {
          if (!bytes?.equals(objectBytes)) throw new Error("publication without exact staged bytes");
          referenced = true;
        } else if (kind === "read") {
          expect(referenced && bytes?.equals(objectBytes)).toBe(true);
        } else {
          throw new Error(`unknown barrier action: ${action}`);
        }
      }
      return { oldDeleteOutcome, publishedReadOutcome: referenced && bytes?.equals(objectBytes) ? "exact" : "missing" };
    };
    for (const order of barrier.orders) {
      expect(executeBarrierOrder(order)).toEqual({ oldDeleteOutcome: order.oldDeleteOutcome, publishedReadOutcome: order.publishedReadOutcome });
    }
  });
}
