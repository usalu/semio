import { readFileSync } from "node:fs";
import { describe, expect, test } from "vitest";
import Ajv from "ajv";
import { meshPaintSealCurrent, nextSceneRasterEpoch, SceneRasterPool, SceneRasterUploadAuthority, sceneRasterUploadRows, type SceneRasterIdentity } from "../../💾️scene-raster-ownership/🟦️.ts";

const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🖼️scene-raster-ownership/🔣️.json", import.meta.url), "utf8"));
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🖼️scene-raster-ownership/🔣️.json", import.meta.url), "utf8"));

function identity(seed: string, width = 1, height = 1): SceneRasterIdentity {
  return {
    sourceDigest: seed.repeat(64),
    decodeProfileVersion: 1,
    outputWidth: width,
    outputHeight: height,
    orientation: 1,
    alphaMode: "straight",
    colourProfile: "none",
    resampling: "linear",
  };
}

function smallPool(): SceneRasterPool {
  return new SceneRasterPool({ decodedItemBytes: 16, slotCapacity: 4, poolBytes: 64, leaseCapacityPerSlot: 4, writeChunkBytes: 4, retireChunkBytes: 4 });
}

function publish(pool: SceneRasterPool, raster: SceneRasterIdentity, token: string, bytes: readonly number[]) {
  const begun = pool.begin(raster, token, 1, bytes.length);
  expect(begun.kind).toBe("writer");
  if (begun.kind !== "writer") throw new Error("writer");
  let writer = begun.writer;
  for (let offset = 0; offset < bytes.length; offset += 4) {
    const pushed = pool.push(writer, new Uint8Array(bytes.slice(offset, offset + 4)));
    expect(pushed.kind).toBe("progress");
    if (pushed.kind !== "progress") throw new Error("progress");
    writer = pushed.writer;
  }
  const lease = pool.seal(writer);
  expect(lease).not.toBeNull();
  return lease!;
}

describe("scene raster ownership", () => {
  test("neutral contract owns natural quality, independent workspaces, profiles, and row chunks", () => {
    expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
    expect(fixture.limits.slotCapacity * fixture.limits.decodedItemBytes).toBe(fixture.limits.poolBytes);
    expect(fixture.naturalReference.width * fixture.naturalReference.height * 4).toBe(fixture.naturalReference.rgbaBytes);
    expect(fixture.naturalReference.rgbaBytes).toBeLessThanOrEqual(fixture.limits.decodedItemBytes);
    expect(fixture.limits.workerBitmapBytes + fixture.limits.workerCanvasBytes).toBeGreaterThan(fixture.limits.decodedItemBytes);
    const rows = sceneRasterUploadRows(fixture.naturalReference.width, fixture.naturalReference.height, fixture.limits.uploadChunkBytes);
    expect(Math.ceil(fixture.naturalReference.height / rows)).toBe(fixture.naturalReference.expectedUploadChunks);
    expect(fixture.profiles.map((profile: { id: string; materialColourSpace: string; rendererReachable: boolean }) => [profile.id, profile.materialColourSpace, profile.rendererReachable])).toEqual([
      ["reference-image-map-v1", "none", true],
      ["reference-canvas-raster-v1", "srgb", false],
      ["mesh-paint-map-v1", "none", true],
    ]);
    expect(fixture.capabilities.fullQualityAcceptance).toEqual(["image/png", "image/jpeg"]);
    expect(fixture.capabilities.unreachableRendererRoutes).toEqual(["image/svg+xml", "application/pdf"]);
    expect(fixture.capabilities.unimplementedWgpu).toEqual(["image/avif", "image/tiff", "application/pdf"]);
    expect(fixture.creditLifetimes).toEqual({
      encodedSource: "decode-input-accepted",
      decodeWorkspace: "pool-seal-or-terminal-refusal",
      decodedPool: "zero-leases-and-retirement-terminal",
      preparedLogical: "gpu-commit-or-partial-retirement-terminal",
      gpuUploadLogical: "committed-ack-or-partial-retirement-terminal",
      gpuResident: "texture-retirement-terminal",
    });
    expect(fixture.upload).toEqual({ progress: ["uploadedBytes", "totalBytes"], committedAckRequired: true, replacementPolicy: "keep-old-until-new-commit", cancelPolicy: "retire-partial-before-release" });
  });

  test("two panes share one immutable identity and release independently", () => {
    const pool = smallPool();
    const raster = identity("1", 2, 1);
    const first = publish(pool, raster, "pane-top", [1, 2, 3, 4, 5, 6, 7, 8]);
    const second = pool.begin(raster, "pane-perspective", 1, 8);
    expect(second.kind).toBe("ready");
    if (second.kind !== "ready") throw new Error("ready");
    expect(second.lease.slot).toBe(first.slot);
    expect(second.lease.epoch).toBe(first.epoch);
    expect(second.lease.contentDigest).toBe(first.contentDigest);
    expect(second.lease.leaseId).not.toBe(first.leaseId);
    expect(pool.leaseCount(first)).toBe(fixture.leaseReuse.expectedLeaseCount);
    expect(pool.release(first)).toBe(true);
    expect(pool.release(first)).toBe(false);
    expect(pool.release({ ...first })).toBe(false);
    const page = pool.copyPage(second.lease, 0)!;
    expect(page).toEqual(new Uint8Array([1, 2, 3, 4]));
    page[0] = 255;
    expect(pool.copyPage(second.lease, 0)).toEqual(new Uint8Array([1, 2, 3, 4]));
    expect(pool.leaseCount(second.lease)).toBe(1);
  });

  test("four live identities produce retryable fifth-owner backpressure", () => {
    const pool = smallPool();
    for (const seed of ["1", "2", "3", "4"]) publish(pool, identity(seed), `owner-${seed}`, [1, 2, 3, 4]);
    expect(pool.begin(identity("5"), "owner-5", 1, 4)).toEqual({ kind: "backpressure", reason: "all-live" });
    expect(pool.reservedBytes).toBe(16);
  });

  test("active lease credits refuse atomically and constructor limits are immutable", () => {
    const limits = { decodedItemBytes: 4, slotCapacity: 1, poolBytes: 4, leaseCapacityPerSlot: 2, writeChunkBytes: 4, retireChunkBytes: 4 };
    const pool = new SceneRasterPool(limits);
    limits.leaseCapacityPerSlot = 99;
    limits.decodedItemBytes = 99;
    const raster = identity("5");
    const first = publish(pool, raster, "first", [1, 2, 3, 4]);
    const second = pool.begin(raster, "second", 1, 4);
    expect(second.kind).toBe("ready");
    expect(pool.begin(raster, "third", 1, 4)).toEqual({ kind: "backpressure", reason: "lease-capacity" });
    expect(pool.leaseCount(first)).toBe(2);
    expect(pool.begin(identity("6", 2, 1), "oversized", 1, 8)).toEqual({ kind: "refused", reason: "item-bytes" });
  });

  test("cancellation invalidates the writer and retires only owned pages", () => {
    const pool = smallPool();
    const begun = pool.begin(identity("6", 2, 1), "cancelled", 7, 8);
    if (begun.kind !== "writer") throw new Error("writer");
    const pushed = pool.push(begun.writer, new Uint8Array([1, 2, 3, 4]));
    if (pushed.kind !== "progress") throw new Error("progress");
    expect(pool.cancel(pushed.writer)).toBe(true);
    expect(pool.push(pushed.writer, new Uint8Array([5, 6, 7, 8]))).toEqual({ kind: "stale" });
    expect(pool.seal(pushed.writer)).toBeNull();
    expect(pool.retireStep()).toBe(4);
    expect(pool.state(pushed.writer.slot)).toBe("vacant");
    expect(pool.residentBytes).toBe(0);
  });

  test("zero-lease LRU retires before replacement and never evicts a live lease", () => {
    const pool = smallPool();
    const lease = publish(pool, identity("7"), "old", [1, 2, 3, 4]);
    expect(pool.release(lease)).toBe(true);
    for (const seed of ["8", "9", "a"]) publish(pool, identity(seed), `owner-${seed}`, [1, 2, 3, 4]);
    expect(pool.begin(identity("b"), "replacement", 1, 4)).toEqual({ kind: "backpressure", reason: "retirement" });
    expect(pool.retireStep()).toBe(4);
    expect(pool.begin(identity("b"), "replacement", 1, 4).kind).toBe("writer");
  });

  test("mesh paint seal requires the exact mesh revision and UV witness", () => {
    const expected = { meshKey: fixture.meshPaint.meshKey, meshRevision: fixture.meshPaint.meshRevision, uvWitness: fixture.meshPaint.uvWitness };
    expect(meshPaintSealCurrent(expected, expected)).toBe(true);
    expect(meshPaintSealCurrent(expected, { ...expected, meshRevision: expected.meshRevision + 1 })).toBe(false);
    expect(meshPaintSealCurrent(expected, { ...expected, uvWitness: 0 })).toBe(false);
  });

  test("admission freezes identity metadata and ownership epochs never wrap", () => {
    const pool = smallPool();
    const raster = { ...identity("f"), sourceDigest: "f".repeat(64) };
    const begun = pool.begin(raster, "owner", 1, 4);
    expect(begun.kind).toBe("writer");
    if (begun.kind !== "writer") throw new Error("writer");
    raster.sourceDigest = "0".repeat(64);
    raster.outputWidth = 9;
    const pushed = pool.push(begun.writer, new Uint8Array([1, 2, 3, 4]));
    if (pushed.kind !== "progress") throw new Error("progress");
    const lease = pool.seal(pushed.writer)!;
    expect(lease.identity.sourceDigest).toBe("f".repeat(64));
    expect(lease.identity.outputWidth).toBe(1);
    expect(Object.isFrozen(lease.identity)).toBe(true);
    expect(nextSceneRasterEpoch((1n << 64n) - 2n)).toBe((1n << 64n) - 1n);
    expect(nextSceneRasterEpoch((1n << 64n) - 1n)).toBeNull();
  });

  test("upload progress is row-aligned and replacement keeps the committed texture until acknowledgement", () => {
    const pool = new SceneRasterPool({ decodedItemBytes: 32, slotCapacity: 4, poolBytes: 128, leaseCapacityPerSlot: 4, writeChunkBytes: 8, retireChunkBytes: 8 });
    const oldLease = publish(pool, identity("c", 2, 1), "old", [1, 2, 3, 4, 5, 6, 7, 8]);
    const nextLease = publish(pool, identity("d", 2, 3), "next", Array.from({ length: 24 }, (_, index) => index));
    const uploads = new SceneRasterUploadAuthority(16, 8, 128);
    let oldUpload = uploads.begin("surface", 1, oldLease)!;
    const oldStep = uploads.step(oldUpload);
    expect(oldStep.kind).toBe("awaiting-ack");
    if (oldStep.kind === "stale") throw new Error("upload");
    oldUpload = oldStep.token;
    const oldAck = uploads.acknowledge(oldUpload)!;
    expect(oldAck.committed).toMatchObject({ surfaceId: "surface", generation: 1, identity: oldLease.identity, contentDigest: oldLease.contentDigest });
    expect(pool.release(oldAck.releasedCpuLease)).toBe(true);
    let nextUpload = uploads.begin("surface", 2, nextLease)!;
    const first = uploads.step(nextUpload);
    expect(first.kind).toBe("progress");
    if (first.kind === "stale") throw new Error("upload");
    nextUpload = first.token;
    expect(uploads.liveProgress("surface")).toEqual({ uploadedBytes: 16, totalBytes: 24, state: "uploading" });
    expect(uploads.committed("surface")).toEqual(oldAck.committed);
    const second = uploads.step(nextUpload);
    expect(second.kind).toBe("awaiting-ack");
    if (second.kind === "stale") throw new Error("upload");
    const nextAck = uploads.acknowledge(second.token)!;
    expect(nextAck.committed).toMatchObject({ surfaceId: "surface", generation: 2, identity: nextLease.identity, contentDigest: nextLease.contentDigest });
    expect(nextAck.previous).toEqual(oldAck.committed);
    expect(pool.release(nextAck.releasedCpuLease)).toBe(true);
    expect(uploads.committed("surface")).toEqual(nextAck.committed);
  });

  test("cancelled partial upload retires before its surface credit is reusable", () => {
    const pool = new SceneRasterPool({ decodedItemBytes: 32, slotCapacity: 4, poolBytes: 128, leaseCapacityPerSlot: 4, writeChunkBytes: 8, retireChunkBytes: 8 });
    const lease = publish(pool, identity("e", 2, 3), "upload", Array.from({ length: 24 }, (_, index) => index));
    const uploads = new SceneRasterUploadAuthority(16, 8, 128);
    const token = uploads.begin("surface", 4, lease)!;
    const first = uploads.step(token);
    if (first.kind === "stale") throw new Error("upload");
    expect(uploads.cancel(first.token)).toBe(true);
    expect(uploads.begin("surface", 5, lease)).toBeNull();
    expect(uploads.retireStep("surface")).toEqual({ retiredBytes: 8, releasedCpuLease: null });
    const retired = uploads.retireStep("surface");
    expect(retired.retiredBytes).toBe(8);
    expect(retired.releasedCpuLease).toEqual(lease);
    expect(pool.release(retired.releasedCpuLease!)).toBe(true);
    const reacquired = pool.begin(lease.identity, "retry", 5, 24);
    expect(reacquired.kind).toBe("ready");
    if (reacquired.kind !== "ready") throw new Error("ready");
    expect(uploads.begin("surface", 5, reacquired.lease)).not.toBeNull();
  });

  test("four CPU slots make progress through six distinct committed GPU residents", () => {
    const pool = smallPool();
    const uploads = new SceneRasterUploadAuthority(4, 4, 24);
    const committed: string[] = [];
    for (let index = 1; index <= 6; index += 1) {
      const raster = identity(String(index));
      let begun = pool.begin(raster, `decode-${index}`, 1, 4);
      if (begun.kind === "backpressure" && begun.reason === "retirement") {
        expect(pool.retireStep()).toBe(4);
        begun = pool.begin(raster, `decode-${index}`, 1, 4);
      }
      expect(begun.kind).toBe("writer");
      if (begun.kind !== "writer") throw new Error("writer");
      const pushed = pool.push(begun.writer, new Uint8Array([index, index, index, 255]));
      if (pushed.kind !== "progress") throw new Error("progress");
      const lease = pool.seal(pushed.writer)!;
      let upload = uploads.begin(`surface-${index}`, index, lease)!;
      const stepped = uploads.step(upload);
      expect(stepped.kind).toBe("awaiting-ack");
      if (stepped.kind === "stale") throw new Error("upload");
      upload = stepped.token;
      const ack = uploads.acknowledge(upload)!;
      committed.push(ack.committed.contentDigest);
      expect(pool.release(ack.releasedCpuLease)).toBe(true);
    }
    expect(committed).toHaveLength(6);
    expect(new Set(committed).size).toBe(6);
    for (let index = 1; index <= 6; index += 1) expect(uploads.committed(`surface-${index}`)?.generation).toBe(index);
    expect(pool.reservedBytes).toBe(16);
    expect(uploads.committedBytes).toBe(24);
  });
});
