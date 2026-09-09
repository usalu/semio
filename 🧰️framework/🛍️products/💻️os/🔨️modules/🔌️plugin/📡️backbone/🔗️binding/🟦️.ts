/** 📡️ Exact document-port controls and bounded ownership shared by every renderer and actor host. */
import { decodeBackboneMessage, encodePackValue, packUInt } from "@semio-tech/framework-os";

export const DOCUMENT_BACKBONE_BINDING_SCHEMA_V1 = "semio.plugin.document-backbone-binding.v1";
export const DOCUMENT_BACKBONE_RECEIPT_SCHEMA_V1 = "semio.plugin.document-backbone-binding-receipt.v1";
export const DOCUMENT_BACKBONE_CONTROL_MAXIMUM_BYTES = 4096;

/** 📬️ Guest Ack terminates at Shell as an ingest receipt, never as Hub command completion. */
export function documentBackboneEffectV1(bytes: Uint8Array): "mutations" | "remote-ingest-receipt" {
  const message = decodeBackboneMessage(bytes);
  if (message.kind === "snapshot") throw new Error("actor-document-port.snapshot-requires-cold-pair");
  return message.kind === "ack" ? "remote-ingest-receipt" : "mutations";
}

export type DocumentBackboneControlV1 = Readonly<{
  schema: string;
  operation: string;
  instanceId: number;
  bindingGeneration: bigint;
  uri: string;
  code?: string;
}>;

function validateDocumentBackboneControlV1(value: DocumentBackboneControlV1): void {
  const command = value.schema === DOCUMENT_BACKBONE_BINDING_SCHEMA_V1;
  if ((!command && value.schema !== DOCUMENT_BACKBONE_RECEIPT_SCHEMA_V1) || !(command ? ["bind", "retire"] : ["bound", "retired", "refused"]).includes(value.operation)) throw new Error("actor-document-control.operation");
  if (Object.keys(value).sort().join(",") !== (value.operation === "refused" ? "bindingGeneration,code,instanceId,operation,schema,uri" : "bindingGeneration,instanceId,operation,schema,uri")) throw new Error("actor-document-control.fields");
  if (!Number.isSafeInteger(value.instanceId) || value.instanceId < 0 || value.instanceId > 0xffffffff || typeof value.bindingGeneration !== "bigint" || value.bindingGeneration < 0n || value.bindingGeneration > 0xffffffffffffffffn) throw new Error("actor-document-control.owner");
  const encoder = new TextEncoder();
  if (typeof value.uri !== "string" || !value.uri.length || value.uri.length > 1280 || encoder.encode(value.uri).length > 1280 || new TextDecoder("utf-8", { fatal: true }).decode(encoder.encode(value.uri)) !== value.uri) throw new Error("actor-document-control.uri");
  if (value.operation === "refused" && (typeof value.code !== "string" || value.code.length > 256 || !/^[a-z][a-z0-9.-]*$/.test(value.code))) throw new Error("actor-document-control.code");
}

/** 🔗️ Encodes the schema-owned flat control map with exact unsigned integer carriers. */
export function encodeDocumentBackboneControlV1(value: DocumentBackboneControlV1): Uint8Array {
  validateDocumentBackboneControlV1(value);
  const bytes = encodePackValue({ ...value, instanceId: packUInt(BigInt(value.instanceId)), bindingGeneration: packUInt(value.bindingGeneration) });
  if (bytes.length > DOCUMENT_BACKBONE_CONTROL_MAXIMUM_BYTES) throw new Error("actor-document-control.bytes");
  return bytes;
}

/** 🛂️ Reads only the closed, bounded flat Pack grammar; no recursive generic value decode. */
export function decodeDocumentBackboneControlV1(bytes: Uint8Array): DocumentBackboneControlV1 {
  if (!(bytes instanceof Uint8Array) || !bytes.length || bytes.length > DOCUMENT_BACKBONE_CONTROL_MAXIMUM_BYTES) throw new Error("actor-document-control.bytes");
  let position = 0;
  const byte = () => {
    if (position === bytes.length) throw new Error("actor-document-control.truncated");
    return bytes[position++]!;
  };
  const uint = () => {
    let value = 0n;
    for (let index = 0; index < 10; index++) {
      const current = byte();
      if (index === 9 && current > 1) throw new Error("actor-document-control.integer");
      value |= BigInt(current & 127) << BigInt(index * 7);
      if (current < 128) {
        if (index > 0 && current === 0) throw new Error("actor-document-control.integer");
        return value;
      }
    }
    throw new Error("actor-document-control.integer");
  };
  const count = (maximum: number) => {
    const value = uint();
    if (value > BigInt(maximum)) throw new Error("actor-document-control.count");
    return Number(value);
  };
  const text = () => {
    const length = count(bytes.length - position);
    if (length > bytes.length - position) throw new Error("actor-document-control.truncated");
    const value = new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(position, position + length));
    position += length;
    return value;
  };
  const symbols: string[] = [];
  const symbolCount = count(4);
  for (let index = 0; index < symbolCount; index++) symbols.push(text());
  if (count(1) !== 1 || count(1) !== 1 || byte() !== 0x11 || byte() !== 0x10) throw new Error("actor-document-control.record");
  const fieldCount = count(6);
  if (fieldCount < 5) throw new Error("actor-document-control.fields");
  const record: Record<string, string | bigint> = Object.create(null);
  for (let index = 0; index < fieldCount; index++) {
    if (byte() !== 0x07) throw new Error("actor-document-control.key");
    const key = text();
    if (Object.hasOwn(record, key)) throw new Error("actor-document-control.duplicate");
    const tag = byte();
    if (tag === 0x04) record[key] = uint();
    else if (tag === 0x07) record[key] = text();
    else if (tag === 0x06) {
      const symbol = symbols[count(symbols.length)];
      if (symbol === undefined) throw new Error("actor-document-control.symbol");
      record[key] = symbol;
    } else throw new Error("actor-document-control.value");
  }
  if (position !== bytes.length || typeof record.instanceId !== "bigint" || record.instanceId > 0xffffffffn) throw new Error("actor-document-control.terminal");
  const value = { ...record, instanceId: Number(record.instanceId) } as DocumentBackboneControlV1;
  validateDocumentBackboneControlV1(value);
  const canonical = encodeDocumentBackboneControlV1(value);
  if (canonical.length !== bytes.length || canonical.some((item, index) => item !== bytes[index])) throw new Error("actor-document-control.noncanonical");
  return Object.freeze(value);
}

/** 🧾️ Requires the sole receipt to acknowledge the retained instance, generation and URI. */
function readDocumentBackboneReceiptV1(bytes: Uint8Array, command: DocumentBackboneControlV1): DocumentBackboneControlV1 {
  validateDocumentBackboneControlV1(command);
  const receipt = decodeDocumentBackboneControlV1(bytes);
  if (command.schema !== DOCUMENT_BACKBONE_BINDING_SCHEMA_V1 || receipt.schema !== DOCUMENT_BACKBONE_RECEIPT_SCHEMA_V1 || receipt.instanceId !== command.instanceId || receipt.bindingGeneration !== command.bindingGeneration || receipt.uri !== command.uri) throw new Error("actor-document-control.receipt-owner");
  if (receipt.operation !== "refused" && receipt.operation !== (command.operation === "bind" ? "bound" : "retired")) throw new Error("actor-document-control.receipt-operation");
  return receipt;
}

/** 🧾️ Accepts an exact successful receipt and surfaces a verified refusal code. */
export function requireDocumentBackboneReceiptV1(bytes: Uint8Array, command: DocumentBackboneControlV1): void {
  const receipt = readDocumentBackboneReceiptV1(bytes, command);
  if (receipt.operation === "refused") throw new Error(receipt.code);
}

export type ActorDocumentSourceV1 = Readonly<{
  runtimeKey: string;
  clientInstanceId: string;
  scope: Readonly<{ spaceId: string; documentId: string }> | null;
}>;

export type ActorDocumentOwnerV1 = ActorDocumentSourceV1 & Readonly<{
  actorId: string;
  activationGeneration: bigint;
  instanceId: number;
}>;

export type ActorDocumentMessageLimitsV1 = Readonly<{ messageBytes: number; pendingBytes: number; pendingMessages: number }>;

export interface ActorDocumentMessagePortsV1 {
  readonly limits: ActorDocumentMessageLimitsV1;
  current(): boolean;
  outboundReady?(): boolean;
  assertActive(): void;
  send(payload: Uint8Array): void;
  deliver(payload: Uint8Array): Promise<void>;
  retire(): Promise<void>;
}

export interface ActorDocumentBindingPortsV1 extends Omit<ActorDocumentMessagePortsV1, "retire"> {
  exchange(command: DocumentBackboneControlV1): Promise<readonly Uint8Array[]>;
}

/** 🎟️ Keeps guest binding receipt authority separate from document-message admission. */
export class ActorDocumentBindingV1 {
  readonly port: ActorDocumentMessagePortV1;
  readonly #ports: ActorDocumentBindingPortsV1;
  readonly #command: DocumentBackboneControlV1;
  #binding: Promise<void> | null = null;
  #bound = false;
  #remote: "unsent" | "refused" | "possibly-bound" | "bound" | "retired" = "unsent";

  constructor(owner: ActorDocumentOwnerV1, bindingGeneration: bigint, ports: ActorDocumentBindingPortsV1) {
    this.#ports = ports;
    this.#command = Object.freeze({ schema: DOCUMENT_BACKBONE_BINDING_SCHEMA_V1, operation: "bind", instanceId: owner.instanceId, bindingGeneration, uri: `actor://${owner.runtimeKey}` });
    validateDocumentBackboneControlV1(this.#command);
    this.port = new ActorDocumentMessagePortV1(owner, {
      ...ports,
      current: () => ports.current(),
      outboundReady: () => this.#bound,
      deliver: async payload => {
        await this.bind();
        if (this.port.closing || !ports.current()) return;
        ports.assertActive();
        await ports.deliver(payload);
      },
      retire: async () => {
        if (this.#binding === null) return;
        await this.#binding.catch(() => {});
        if (this.#remote === "unsent" || this.#remote === "refused") return;
        await this.#exchange({ ...this.#command, operation: "retire" });
        this.#remote = "retired";
        this.#bound = false;
      },
    });
  }

  async #exchange(command: DocumentBackboneControlV1): Promise<void> {
    const receipts = await this.#ports.exchange(command);
    if (receipts.length !== 1) throw new Error("actor-document-control.receipt-count");
    const receipt = readDocumentBackboneReceiptV1(receipts[0]!, command);
    if (command.operation === "bind") this.#remote = receipt.operation === "refused" ? "refused" : "bound";
    if (receipt.operation === "refused") throw new Error(receipt.code);
  }

  bind(): Promise<void> {
    if (this.#binding !== null) return this.#binding;
    if (this.port.closing) return Promise.reject(new Error("actor-document-control.closed"));
    this.#binding = Promise.resolve().then(async () => {
      if (!this.#ports.current()) throw new Error("actor-document-control.stale");
      this.#ports.assertActive();
      this.#remote = "possibly-bound";
      await this.#exchange(this.#command);
      if (!this.#ports.current()) throw new Error("actor-document-control.stale");
      this.#ports.assertActive();
      this.#bound = true;
    });
    return this.#binding;
  }
}

/** 📡️ Owns bounded document messages until one captured actor and session have drained. */
export class ActorDocumentMessagePortV1 {
  readonly owner: ActorDocumentOwnerV1;
  readonly uri: string;
  readonly #ports: ActorDocumentMessagePortsV1;
  readonly #limits: ActorDocumentMessageLimitsV1;
  #tail: Promise<void> = Promise.resolve();
  #retirement: Promise<void> | null = null;
  #closing = false;
  #pendingBytes = 0;
  #pendingMessages = 0;

  constructor(owner: ActorDocumentOwnerV1, ports: ActorDocumentMessagePortsV1) {
    if (!owner.actorId || !owner.runtimeKey || !owner.clientInstanceId || typeof owner.activationGeneration !== "bigint" || owner.activationGeneration < 0n || !Number.isSafeInteger(owner.instanceId) || owner.instanceId < 0 || owner.instanceId > 0xffffffff || (owner.scope !== null && (!owner.scope.spaceId || !owner.scope.documentId))) throw new Error("actor-document-port.owner");
    if (Object.values(ports.limits).some(value => !Number.isSafeInteger(value) || value < 1) || ports.limits.pendingBytes < ports.limits.messageBytes) throw new Error("actor-document-port.limits");
    this.owner = Object.freeze({ ...owner, scope: owner.scope === null ? null : Object.freeze({ ...owner.scope }) });
    this.uri = `actor://${owner.runtimeKey}`;
    this.#limits = Object.freeze({ ...ports.limits });
    this.#ports = ports;
  }

  get pending(): Readonly<{ bytes: number; messages: number }> {
    return { bytes: this.#pendingBytes, messages: this.#pendingMessages };
  }

  get closing(): boolean { return this.#closing; }

  #current(): boolean {
    if (this.#closing || !this.#ports.current()) return false;
    try { this.#ports.assertActive(); return true; } catch { return false; }
  }

  #matches(source: ActorDocumentSourceV1): boolean {
    return source.runtimeKey === this.owner.runtimeKey && source.clientInstanceId === this.owner.clientInstanceId &&
      (source.scope === null ? this.owner.scope === null : this.owner.scope !== null && source.scope.spaceId === this.owner.scope.spaceId && source.scope.documentId === this.owner.scope.documentId);
  }

  #checkPayload(payload: Uint8Array): void {
    if (!(payload instanceof Uint8Array) || payload.byteLength === 0 || payload.byteLength > this.#limits.messageBytes) throw new Error("actor-document-port.message-size");
  }

  send(uri: string, payload: Uint8Array): boolean {
    if (uri !== this.uri || !this.#current() || this.#ports.outboundReady?.() === false) return false;
    this.#checkPayload(payload);
    this.#ports.send(payload);
    return true;
  }

  async receive(source: ActorDocumentSourceV1, payload: Uint8Array): Promise<boolean> {
    if (!this.#matches(source) || !this.#current()) return false;
    this.#checkPayload(payload);
    if (this.#pendingMessages === this.#limits.pendingMessages || payload.byteLength > this.#limits.pendingBytes - this.#pendingBytes) throw new Error("actor-document-port.capacity");
    const bytes = payload.slice();
    this.#pendingBytes += bytes.byteLength;
    this.#pendingMessages++;
    const work = this.#tail.then(async () => {
      if (!this.#current()) return false;
      await this.#ports.deliver(bytes);
      return this.#current();
    }).finally(() => { this.#pendingBytes -= bytes.byteLength; this.#pendingMessages--; });
    this.#tail = work.then(() => {}, () => {});
    return work;
  }

  retire(): Promise<void> {
    if (this.#retirement !== null) return this.#retirement;
    this.#closing = true;
    this.#retirement = this.#tail.then(() => this.#ports.retire());
    return this.#retirement;
  }
}
