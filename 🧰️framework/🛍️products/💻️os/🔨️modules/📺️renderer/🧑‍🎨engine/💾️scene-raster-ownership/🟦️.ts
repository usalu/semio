import { Blake3Hasher } from "../../../../../../🔨️modules/🔏️hash/🟦️.ts";

export type SceneRasterAlphaMode = "straight";
export type SceneRasterColourProfile = "none" | "srgb";
export type SceneRasterResampling = "linear";

export type SceneRasterIdentity = Readonly<{
  sourceDigest: string;
  decodeProfileVersion: number;
  outputWidth: number;
  outputHeight: number;
  orientation: number;
  alphaMode: SceneRasterAlphaMode;
  colourProfile: SceneRasterColourProfile;
  resampling: SceneRasterResampling;
}>;

export type SceneRasterLease = Readonly<{
  slot: number;
  epoch: bigint;
  leaseId: bigint;
  identity: SceneRasterIdentity;
  contentDigest: string;
}>;

export type SceneRasterWriter = Readonly<{
  slot: number;
  epoch: bigint;
  ownerToken: string;
  consumerGeneration: number;
  cursor: number;
}>;

export type SceneRasterPoolLimits = Readonly<{
  decodedItemBytes: number;
  slotCapacity: number;
  poolBytes: number;
  leaseCapacityPerSlot: number;
  writeChunkBytes: number;
  retireChunkBytes: number;
}>;

type VacantSlot = { state: "vacant"; epoch: bigint };
type BuildingSlot = {
  state: "building";
  epoch: bigint;
  identity: SceneRasterIdentity;
  ownerToken: string;
  consumerGeneration: number;
  totalBytes: number;
  cursor: number;
  pages: Uint8Array[];
  hasher: Blake3Hasher;
  usedAt: number;
};
type ReadySlot = {
  state: "ready";
  epoch: bigint;
  identity: SceneRasterIdentity;
  contentDigest: string;
  totalBytes: number;
  pages: Uint8Array[];
  leases: Set<bigint>;
  usedAt: number;
};
type RetiringSlot = {
  state: "retiring";
  epoch: bigint;
  totalBytes: number;
  remainingBytes: number;
  pages: Uint8Array[];
};
type SceneRasterSlot = VacantSlot | BuildingSlot | ReadySlot | RetiringSlot;

export type SceneRasterBeginResult =
  | Readonly<{ kind: "writer"; writer: SceneRasterWriter }>
  | Readonly<{ kind: "ready"; lease: SceneRasterLease }>
  | Readonly<{ kind: "backpressure"; reason: "identity-building" | "retirement" | "all-live" | "pool-bytes" | "lease-capacity" }>
  | Readonly<{ kind: "refused"; reason: "item-bytes" | "pixel-geometry" | "epoch-exhausted" | "lease-exhausted" }>;

export type SceneRasterPushResult =
  | Readonly<{ kind: "progress"; writer: SceneRasterWriter; writtenBytes: number; totalBytes: number }>
  | Readonly<{ kind: "stale" }>
  | Readonly<{ kind: "refused"; reason: "chunk-bytes" | "cursor" | "item-bytes" }>;

function sameIdentity(left: SceneRasterIdentity, right: SceneRasterIdentity): boolean {
  return left.sourceDigest === right.sourceDigest
    && left.decodeProfileVersion === right.decodeProfileVersion
    && left.outputWidth === right.outputWidth
    && left.outputHeight === right.outputHeight
    && left.orientation === right.orientation
    && left.alphaMode === right.alphaMode
    && left.colourProfile === right.colourProfile
    && left.resampling === right.resampling;
}

const U64_MAX = (1n << 64n) - 1n;

/** 🔢️ Advances one exact ownership epoch without wrapping a live slot identity. */
export function nextSceneRasterEpoch(current: bigint): bigint | null {
  return current >= 0n && current < U64_MAX ? current + 1n : null;
}

function ownedIdentity(identity: SceneRasterIdentity): SceneRasterIdentity {
  return Object.freeze({
    sourceDigest: identity.sourceDigest,
    decodeProfileVersion: identity.decodeProfileVersion,
    outputWidth: identity.outputWidth,
    outputHeight: identity.outputHeight,
    orientation: identity.orientation,
    alphaMode: identity.alphaMode,
    colourProfile: identity.colourProfile,
    resampling: identity.resampling,
  });
}

function hex(bytes: Uint8Array): string {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

function exactByteLength(identity: SceneRasterIdentity): number | null {
  if (!Number.isSafeInteger(identity.outputWidth) || !Number.isSafeInteger(identity.outputHeight) || identity.outputWidth < 1 || identity.outputHeight < 1) return null;
  const pixels = identity.outputWidth * identity.outputHeight;
  const bytes = pixels * 4;
  return Number.isSafeInteger(bytes) ? bytes : null;
}

/** 🖼️ Neutral page-owned raster authority used to prove lease, cancellation, and retirement laws. */
export class SceneRasterPool {
  readonly #limits: SceneRasterPoolLimits;
  readonly #slots: SceneRasterSlot[];
  #clock = 0;
  #leaseSerial = 0n;

  constructor(limits: SceneRasterPoolLimits) {
    if (!(limits.decodedItemBytes > 0 && limits.slotCapacity > 0 && limits.poolBytes >= limits.decodedItemBytes && limits.leaseCapacityPerSlot > 0 && limits.writeChunkBytes > 0 && limits.retireChunkBytes > 0)) throw new Error("scene-raster-limits");
    this.#limits = Object.freeze({ ...limits });
    if (limits.retireChunkBytes < limits.writeChunkBytes) throw new Error("scene-raster-retire-chunk");
    this.#slots = Array.from({ length: limits.slotCapacity }, (): SceneRasterSlot => ({ state: "vacant", epoch: 0n }));
  }

  begin(identity: SceneRasterIdentity, ownerToken: string, consumerGeneration: number, totalBytes: number): SceneRasterBeginResult {
    const expected = exactByteLength(identity);
    if (expected === null || expected !== totalBytes) return { kind: "refused", reason: "pixel-geometry" };
    if (totalBytes > this.#limits.decodedItemBytes) return { kind: "refused", reason: "item-bytes" };
    const readyIndex = this.#slots.findIndex((slot) => slot.state === "ready" && sameIdentity(slot.identity, identity));
    if (readyIndex >= 0) {
      const slot = this.#slots[readyIndex] as ReadySlot;
      if (slot.leases.size >= this.#limits.leaseCapacityPerSlot) return { kind: "backpressure", reason: "lease-capacity" };
      const leaseId = this.#nextLeaseId();
      if (leaseId === null) return { kind: "refused", reason: "lease-exhausted" };
      slot.leases.add(leaseId);
      slot.usedAt = ++this.#clock;
      return { kind: "ready", lease: { slot: readyIndex, epoch: slot.epoch, leaseId, identity: slot.identity, contentDigest: slot.contentDigest } };
    }
    if (this.#slots.some((slot) => slot.state === "building" && sameIdentity(slot.identity, identity))) return { kind: "backpressure", reason: "identity-building" };
    let slotIndex = this.#slots.findIndex((slot) => slot.state === "vacant");
    if (slotIndex < 0) {
      if (this.#slots.some((slot) => slot.state === "retiring")) return { kind: "backpressure", reason: "retirement" };
      const candidate = this.#slots
        .map((slot, index) => ({ slot, index }))
        .filter((entry): entry is { slot: ReadySlot; index: number } => entry.slot.state === "ready" && entry.slot.leases.size === 0)
        .sort((left, right) => left.slot.usedAt - right.slot.usedAt)[0];
      if (!candidate) return { kind: "backpressure", reason: "all-live" };
      this.#slots[candidate.index] = { state: "retiring", epoch: candidate.slot.epoch, totalBytes: candidate.slot.totalBytes, remainingBytes: candidate.slot.totalBytes, pages: candidate.slot.pages };
      return { kind: "backpressure", reason: "retirement" };
    }
    if (this.reservedBytes + totalBytes > this.#limits.poolBytes) return { kind: "backpressure", reason: "pool-bytes" };
    const epoch = nextSceneRasterEpoch(this.#slots[slotIndex]!.epoch);
    if (epoch === null) return { kind: "refused", reason: "epoch-exhausted" };
    const identityOwned = ownedIdentity(identity);
    this.#slots[slotIndex] = {
      state: "building",
      epoch,
      identity: identityOwned,
      ownerToken,
      consumerGeneration,
      totalBytes,
      cursor: 0,
      pages: [],
      hasher: new Blake3Hasher(),
      usedAt: ++this.#clock,
    };
    return { kind: "writer", writer: { slot: slotIndex, epoch, ownerToken, consumerGeneration, cursor: 0 } };
  }

  push(writer: SceneRasterWriter, bytes: Uint8Array): SceneRasterPushResult {
    const slot = this.#building(writer);
    if (!slot) return { kind: "stale" };
    if (bytes.length === 0 || bytes.length > this.#limits.writeChunkBytes) return { kind: "refused", reason: "chunk-bytes" };
    if (writer.cursor !== slot.cursor) return { kind: "refused", reason: "cursor" };
    if (slot.cursor + bytes.length > slot.totalBytes) return { kind: "refused", reason: "item-bytes" };
    const owned = bytes.slice();
    slot.pages.push(owned);
    slot.hasher.update(owned);
    slot.cursor += owned.length;
    const next = { slot: writer.slot, epoch: writer.epoch, ownerToken: writer.ownerToken, consumerGeneration: writer.consumerGeneration, cursor: slot.cursor };
    return { kind: "progress", writer: next, writtenBytes: slot.cursor, totalBytes: slot.totalBytes };
  }

  seal(writer: SceneRasterWriter): SceneRasterLease | null {
    const slot = this.#building(writer);
    if (!slot || writer.cursor !== slot.cursor || slot.cursor !== slot.totalBytes) return null;
    const leaseId = this.#nextLeaseId();
    if (leaseId === null) return null;
    const ready: ReadySlot = {
      state: "ready",
      epoch: slot.epoch,
      identity: slot.identity,
      contentDigest: hex(slot.hasher.digest()),
      totalBytes: slot.totalBytes,
      pages: slot.pages,
      leases: new Set([leaseId]),
      usedAt: ++this.#clock,
    };
    this.#slots[writer.slot] = ready;
    return { slot: writer.slot, epoch: ready.epoch, leaseId, identity: ready.identity, contentDigest: ready.contentDigest };
  }

  acquire(identity: SceneRasterIdentity): SceneRasterLease | null {
    const index = this.#slots.findIndex((slot) => slot.state === "ready" && sameIdentity(slot.identity, identity));
    if (index < 0) return null;
    const slot = this.#slots[index] as ReadySlot;
    if (slot.leases.size >= this.#limits.leaseCapacityPerSlot) return null;
    const leaseId = this.#nextLeaseId();
    if (leaseId === null) return null;
    slot.leases.add(leaseId);
    slot.usedAt = ++this.#clock;
    return { slot: index, epoch: slot.epoch, leaseId, identity: slot.identity, contentDigest: slot.contentDigest };
  }

  release(lease: SceneRasterLease): boolean {
    const slot = this.#slots[lease.slot];
    if (!slot || slot.state !== "ready" || slot.epoch !== lease.epoch || !sameIdentity(slot.identity, lease.identity) || slot.contentDigest !== lease.contentDigest || !slot.leases.delete(lease.leaseId)) return false;
    slot.usedAt = ++this.#clock;
    return true;
  }

  cancel(writer: SceneRasterWriter): boolean {
    const slot = this.#building(writer);
    if (!slot) return false;
    this.#slots[writer.slot] = { state: "retiring", epoch: slot.epoch, totalBytes: slot.totalBytes, remainingBytes: slot.cursor, pages: slot.pages };
    return true;
  }

  retireStep(): number {
    const index = this.#slots.findIndex((slot) => slot.state === "retiring");
    if (index < 0) return 0;
    const slot = this.#slots[index] as RetiringSlot;
    let retired = 0;
    while (slot.pages.length > 0 && retired + slot.pages[0]!.length <= this.#limits.retireChunkBytes) {
      const page = slot.pages.shift()!;
      retired += page.length;
      slot.remainingBytes -= page.length;
    }
    if (retired === 0 && slot.pages.length > 0) {
      const page = slot.pages.shift()!;
      retired = page.length;
      slot.remainingBytes -= page.length;
    }
    if (slot.pages.length === 0) this.#slots[index] = { state: "vacant", epoch: slot.epoch };
    return retired;
  }

  copyPage(lease: SceneRasterLease, pageIndex: number): Uint8Array | null {
    const slot = this.#slots[lease.slot];
    if (!slot || slot.state !== "ready" || slot.epoch !== lease.epoch || slot.contentDigest !== lease.contentDigest || !sameIdentity(slot.identity, lease.identity) || !slot.leases.has(lease.leaseId)) return null;
    slot.usedAt = ++this.#clock;
    return slot.pages[pageIndex]?.slice() ?? null;
  }

  get reservedBytes(): number {
    return this.#slots.reduce((total, slot) => total + (slot.state === "vacant" ? 0 : slot.totalBytes), 0);
  }

  get residentBytes(): number {
    return this.#slots.reduce((total, slot) => total + (slot.state === "building" ? slot.cursor : slot.state === "ready" ? slot.totalBytes : slot.state === "retiring" ? slot.remainingBytes : 0), 0);
  }

  leaseCount(lease: SceneRasterLease): number {
    const slot = this.#slots[lease.slot];
    return slot?.state === "ready" && slot.epoch === lease.epoch && slot.contentDigest === lease.contentDigest && slot.leases.has(lease.leaseId) ? slot.leases.size : 0;
  }

  state(slot: number): SceneRasterSlot["state"] | null {
    return this.#slots[slot]?.state ?? null;
  }

  #building(writer: SceneRasterWriter): BuildingSlot | null {
    const slot = this.#slots[writer.slot];
    return slot?.state === "building"
      && slot.epoch === writer.epoch
      && slot.ownerToken === writer.ownerToken
      && slot.consumerGeneration === writer.consumerGeneration
      ? slot
      : null;
  }

  #nextLeaseId(): bigint | null {
    const next = nextSceneRasterEpoch(this.#leaseSerial);
    if (next === null) return null;
    this.#leaseSerial = next;
    return next;
  }
}

/** 🎨️ Verifies the generation and UV witness required before a mesh-paint writer may seal. */
export function meshPaintSealCurrent(expected: Readonly<{ meshKey: string; meshRevision: number; uvWitness: number }>, current: Readonly<{ meshKey: string; meshRevision: number; uvWitness: number }>): boolean {
  return expected.meshKey === current.meshKey && expected.meshRevision === current.meshRevision && expected.uvWitness > 0 && expected.uvWitness === current.uvWitness;
}

/** 📤️ Derives a row-aligned upload span without exceeding the schema-owned chunk credit. */
export function sceneRasterUploadRows(width: number, remainingRows: number, chunkBytes: number): number {
  if (!Number.isSafeInteger(width) || !Number.isSafeInteger(remainingRows) || !Number.isSafeInteger(chunkBytes) || width < 1 || remainingRows < 1 || chunkBytes < 4) return 0;
  const rowBytes = width * 4;
  if (!Number.isSafeInteger(rowBytes) || rowBytes > chunkBytes) return 1;
  return Math.min(remainingRows, Math.max(1, Math.floor(chunkBytes / rowBytes)));
}

export type SceneRasterUploadToken = Readonly<{
  surfaceId: string;
  generation: number;
  epoch: bigint;
  lease: SceneRasterLease;
  uploadedBytes: number;
  totalBytes: number;
}>;

export type SceneRasterGpuResident = Readonly<{
  surfaceId: string;
  generation: number;
  identity: SceneRasterIdentity;
  contentDigest: string;
}>;

type LiveUpload = {
  state: "uploading" | "awaiting-ack" | "retiring";
  epoch: bigint;
  generation: number;
  lease: SceneRasterLease;
  uploadedBytes: number;
  totalBytes: number;
  retiringBytes: number;
};

/** 📤️ Neutral one-upload-per-surface authority with explicit commit acknowledgement. */
export class SceneRasterUploadAuthority {
  readonly #chunkBytes: number;
  readonly #retireChunkBytes: number;
  readonly #gpuResidentBytes: number;
  readonly #live = new Map<string, LiveUpload>();
  readonly #committed = new Map<string, SceneRasterGpuResident>();
  #committedBytes = 0;
  #epoch = 0n;

  constructor(chunkBytes: number, retireChunkBytes: number, gpuResidentBytes: number) {
    if (!(chunkBytes > 0 && retireChunkBytes > 0 && gpuResidentBytes > 0)) throw new Error("scene-raster-upload-limits");
    this.#chunkBytes = chunkBytes;
    this.#retireChunkBytes = retireChunkBytes;
    this.#gpuResidentBytes = gpuResidentBytes;
  }

  begin(surfaceId: string, generation: number, lease: SceneRasterLease): SceneRasterUploadToken | null {
    if (this.#live.has(surfaceId)) return null;
    const totalBytes = exactByteLength(lease.identity);
    if (totalBytes === null) return null;
    const epoch = nextSceneRasterEpoch(this.#epoch);
    if (epoch === null) return null;
    this.#epoch = epoch;
    this.#live.set(surfaceId, { state: "uploading", epoch, generation, lease, uploadedBytes: 0, totalBytes, retiringBytes: 0 });
    return { surfaceId, generation, epoch, lease, uploadedBytes: 0, totalBytes };
  }

  step(token: SceneRasterUploadToken): Readonly<{ kind: "progress" | "awaiting-ack"; token: SceneRasterUploadToken }> | Readonly<{ kind: "stale" }> {
    const live = this.#matches(token);
    if (!live || live.state !== "uploading") return { kind: "stale" };
    const rowBytes = live.lease.identity.outputWidth * 4;
    const uploadedRows = live.uploadedBytes / rowBytes;
    const remainingRows = live.lease.identity.outputHeight - uploadedRows;
    const rows = sceneRasterUploadRows(live.lease.identity.outputWidth, remainingRows, this.#chunkBytes);
    if (!Number.isInteger(uploadedRows) || rows < 1) return { kind: "stale" };
    live.uploadedBytes += rows * rowBytes;
    live.state = live.uploadedBytes === live.totalBytes ? "awaiting-ack" : "uploading";
    const next = { surfaceId: token.surfaceId, generation: token.generation, epoch: token.epoch, lease: token.lease, uploadedBytes: live.uploadedBytes, totalBytes: live.totalBytes };
    return { kind: live.state === "awaiting-ack" ? "awaiting-ack" : "progress", token: next };
  }

  acknowledge(token: SceneRasterUploadToken): Readonly<{ committed: SceneRasterGpuResident; previous: SceneRasterGpuResident | null; releasedCpuLease: SceneRasterLease }> | null {
    const live = this.#matches(token);
    if (!live || live.state !== "awaiting-ack" || token.uploadedBytes !== live.totalBytes) return null;
    const previous = this.#committed.get(token.surfaceId) ?? null;
    const previousBytes = previous ? exactByteLength(previous.identity) ?? 0 : 0;
    const nextBytes = this.#committedBytes - previousBytes + live.totalBytes;
    if (!Number.isSafeInteger(nextBytes) || nextBytes > this.#gpuResidentBytes) return null;
    const committed = Object.freeze({ surfaceId: token.surfaceId, generation: token.generation, identity: live.lease.identity, contentDigest: live.lease.contentDigest });
    this.#committed.set(token.surfaceId, committed);
    this.#committedBytes = nextBytes;
    this.#live.delete(token.surfaceId);
    return { committed, previous, releasedCpuLease: live.lease };
  }

  cancel(token: SceneRasterUploadToken): boolean {
    const live = this.#matches(token);
    if (!live || live.state === "retiring") return false;
    live.state = "retiring";
    live.retiringBytes = live.uploadedBytes;
    return true;
  }

  retireStep(surfaceId: string): Readonly<{ retiredBytes: number; releasedCpuLease: SceneRasterLease | null }> {
    const live = this.#live.get(surfaceId);
    if (!live || live.state !== "retiring") return { retiredBytes: 0, releasedCpuLease: null };
    const retired = Math.min(live.retiringBytes, this.#retireChunkBytes);
    live.retiringBytes -= retired;
    if (live.retiringBytes !== 0) return { retiredBytes: retired, releasedCpuLease: null };
    this.#live.delete(surfaceId);
    return { retiredBytes: retired, releasedCpuLease: live.lease };
  }

  committed(surfaceId: string): SceneRasterGpuResident | null {
    return this.#committed.get(surfaceId) ?? null;
  }

  get committedBytes(): number {
    return this.#committedBytes;
  }

  liveProgress(surfaceId: string): Readonly<{ uploadedBytes: number; totalBytes: number; state: LiveUpload["state"] }> | null {
    const live = this.#live.get(surfaceId);
    return live ? { uploadedBytes: live.uploadedBytes, totalBytes: live.totalBytes, state: live.state } : null;
  }

  #matches(token: SceneRasterUploadToken): LiveUpload | null {
    const live = this.#live.get(token.surfaceId);
    return live
      && live.epoch === token.epoch
      && live.generation === token.generation
      && live.lease.slot === token.lease.slot
      && live.lease.epoch === token.lease.epoch
      && live.lease.leaseId === token.lease.leaseId
      && live.lease.contentDigest === token.lease.contentDigest
      ? live
      : null;
  }
}
