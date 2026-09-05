/** 🧬️ Typed change-node-name apply and local restore operations. */
import type { GltfSnapshot } from '../../📸️snapshot/🟦️.ts';
import { reject, run, type GltfLeafResult, type GltfMutationRejection } from '../../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️.ts';
import { itemIndex } from '../../../🔨️modules/🧬️mutation-support/🧱️structure-geometry/🟦️.ts';

//#region 🔖️Payload
export const GltfChangeNodeNameDescriptor = { id: 's.stdio.gltf.mutation.change-node-name.v1', version: 1, kind: 'change', touchedPaths: ['document/nodes/*/name'], referencePolicy: 'none' } as const;
export interface GltfChangeNodeNamePayload { node: number; value: string | null }
export interface GltfChangeNodeNameRestore { node: number; before: string | null; after: string | null }
export type GltfChangeNodeNameMutation = { readonly phase: 'apply'; readonly value: GltfChangeNodeNamePayload } | { readonly phase: 'restore'; readonly value: GltfChangeNodeNameRestore };
export type GltfChangeNodeNameResult = GltfLeafResult;
//#endregion 🔖️Payload

//#region 🔖️Facade
export type GltfChangeNodeNameDecodeErrorCode = 'record' | 'keys' | 'phase' | 'node' | 'nullable' | 'unicode' | 'protobuf-wire' | 'protobuf-utf8' | 'protobuf-unknown' | 'protobuf-duplicate' | 'protobuf-truncated' | 'protobuf-varint';
export class GltfChangeNodeNameDecodeError extends Error {
  constructor(readonly code: GltfChangeNodeNameDecodeErrorCode, readonly path: string) { super(`gltf.change-node-name.${code}:${path}`); }
}

type WireRecord = Record<string, unknown>;
const maximumNode = 0xffff_ffff;
const failDecode = (code: GltfChangeNodeNameDecodeErrorCode, path: string): never => { throw new GltfChangeNodeNameDecodeError(code, path); };
const own = (value: object, key: string) => Object.prototype.hasOwnProperty.call(value, key);
const wireRecord = (value: unknown, path: string): WireRecord => {
  if (typeof value !== 'object' || value === null || Array.isArray(value) || (Object.getPrototypeOf(value) !== Object.prototype && Object.getPrototypeOf(value) !== null)) return failDecode('record', path);
  return value as WireRecord;
};
const exact = (value: unknown, path: string, allowed: readonly string[]): WireRecord => {
  const record = wireRecord(value, path);
  if (Reflect.ownKeys(record).some((key) => typeof key !== 'string' || !allowed.includes(key))) return failDecode('keys', path);
  return record;
};
const uint32 = (value: unknown, path: string): number => typeof value === 'number' && Number.isFinite(value) && Number.isInteger(value) && value >= 0 && value <= maximumNode ? value : failDecode('node', path);
const unicode = (value: unknown, path: string): string => {
  if (typeof value !== 'string') return failDecode('nullable', path);
  for (let index = 0; index < value.length; index += 1) {
    const code = value.charCodeAt(index);
    if (code >= 0xd800 && code <= 0xdbff) {
      const following = value.charCodeAt(index + 1);
      if (!Number.isInteger(following) || following < 0xdc00 || following > 0xdfff) return failDecode('unicode', path);
      index += 1;
    } else if (code >= 0xdc00 && code <= 0xdfff) return failDecode('unicode', path);
  }
  return value;
};
export const coerceGltfChangeNodeNameUInt32Variable = (value: unknown): number => uint32(value, 'graphql.uint32.variable');
export const coerceGltfChangeNodeNameUInt32Literal = (kind: string, value: unknown): number => kind === 'IntValue' && typeof value === 'string' && /^(?:0|[1-9][0-9]*)$/.test(value) ? uint32(Number(value), 'graphql.uint32.literal') : failDecode('node', 'graphql.uint32.literal');
//#region ⚙️Validation
const graphqlNullable = (value: unknown, path: string): string | null => {
  const record = exact(value, path, ['present', 'absent']);
  const present = own(record, 'present');
  const absent = own(record, 'absent');
  if (present === absent) return failDecode('nullable', path);
  if (present) return unicode(record.present, `${path}.present`);
  return record.absent === true ? null : failDecode('nullable', `${path}.absent`);
};
const protobufNullable = (value: unknown, path: string): string | null => {
  const record = exact(value, path, ['present', 'absent']);
  const present = own(record, 'present');
  const absent = own(record, 'absent');
  if (present === absent) return failDecode('nullable', path);
  if (present) return unicode(record.present, `${path}.present`);
  if (Reflect.ownKeys(exact(record.absent, `${path}.absent`, [])).length !== 0) return failDecode('nullable', `${path}.absent`);
  return null;
};
const apply = (value: unknown, decodeNullable: (value: unknown, path: string) => string | null, path: string): GltfChangeNodeNamePayload => {
  const record = exact(value, path, ['node', 'value']);
  if (!own(record, 'node') || !own(record, 'value')) return failDecode('keys', path);
  return { node: uint32(record.node, `${path}.node`), value: decodeNullable(record.value, `${path}.value`) };
};
const restore = (value: unknown, decodeNullable: (value: unknown, path: string) => string | null, path: string): GltfChangeNodeNameRestore => {
  const record = exact(value, path, ['node', 'before', 'after']);
  if (!own(record, 'node') || !own(record, 'before') || !own(record, 'after')) return failDecode('keys', path);
  return { node: uint32(record.node, `${path}.node`), before: decodeNullable(record.before, `${path}.before`), after: decodeNullable(record.after, `${path}.after`) };
};
//#endregion ⚙️Validation
//#region 🔗️Graphql
export const decodeGltfChangeNodeNameGraphql = (value: unknown): GltfChangeNodeNameMutation => {
  const record = exact(value, 'graphql', ['apply', 'restore']);
  const hasApply = own(record, 'apply');
  const hasRestore = own(record, 'restore');
  if (hasApply === hasRestore) return failDecode('phase', 'graphql');
  return hasApply ? { phase: 'apply', value: apply(record.apply, graphqlNullable, 'graphql.apply') } : { phase: 'restore', value: restore(record.restore, graphqlNullable, 'graphql.restore') };
};
//#endregion 🔗️Graphql
//#region 🛰️Protobuf
export const decodeGltfChangeNodeNameProto = (value: unknown): GltfChangeNodeNameMutation => {
  const record = exact(value, 'protobuf', ['apply', 'restore']);
  const hasApply = own(record, 'apply');
  const hasRestore = own(record, 'restore');
  if (hasApply === hasRestore) return failDecode('phase', 'protobuf');
  return hasApply ? { phase: 'apply', value: apply(record.apply, protobufNullable, 'protobuf.apply') } : { phase: 'restore', value: restore(record.restore, protobufNullable, 'protobuf.restore') };
};

class ProtobufReader {
  #offset = 0;
  constructor(readonly bytes: Uint8Array) {}
  get remaining() { return this.bytes.length - this.#offset; }
  varint(path: string): bigint {
    const start = this.#offset;
    let value = 0n;
    for (let index = 0; index < 10; index += 1) {
      if (this.#offset >= this.bytes.length) return failDecode('protobuf-truncated', path);
      const byte = this.bytes[this.#offset++]!;
      if (index === 9 && (byte & 0x80) !== 0 || index === 9 && byte > 1) return failDecode('protobuf-varint', path);
      value |= BigInt(byte & 0x7f) << BigInt(index * 7);
      if ((byte & 0x80) === 0) {
        let width = 1;
        for (let remaining = value; remaining >= 0x80n; remaining >>= 7n) width += 1;
        return width === this.#offset - start ? value : failDecode('protobuf-varint', path);
      }
    }
    return failDecode('protobuf-varint', path);
  }
  nested(path: string): Uint8Array {
    const length = this.varint(`${path}.length`);
    if (length > BigInt(Number.MAX_SAFE_INTEGER)) return failDecode('protobuf-wire', path);
    const end = this.#offset + Number(length);
    if (end > this.bytes.length) return failDecode('protobuf-truncated', path);
    const value = this.bytes.slice(this.#offset, end);
    this.#offset = end;
    return value;
  }
  finish(path: string) { if (this.remaining !== 0) failDecode('protobuf-wire', path); }
}
const key = (reader: ProtobufReader, path: string): [number, number] => {
  const value = reader.varint(`${path}.tag`);
  const field = value >> 3n;
  if (field === 0n || field > BigInt(Number.MAX_SAFE_INTEGER)) return failDecode('protobuf-wire', path);
  return [Number(field), Number(value & 7n)];
};
const protobufString = (bytes: Uint8Array, path: string): string => {
  try { return unicode(new TextDecoder('utf-8', { fatal: true, ignoreBOM: true }).decode(bytes), path); } catch { return failDecode('protobuf-utf8', path); }
};
const protobufNullableBytes = (bytes: Uint8Array, path: string): string | null => {
  const reader = new ProtobufReader(bytes);
  let value: string | null | undefined;
  while (reader.remaining) {
    const [field, wire] = key(reader, path);
    if (value !== undefined) return failDecode('protobuf-duplicate', path);
    if (field === 1 && wire === 2) value = protobufString(reader.nested(`${path}.present`), `${path}.present`);
    else if (field === 2 && wire === 2) {
      const absent = new ProtobufReader(reader.nested(`${path}.absent`));
      absent.finish(`${path}.absent`);
      value = null;
    } else if (field === 1 || field === 2) return failDecode('protobuf-wire', path);
    else return failDecode('protobuf-unknown', path);
  }
  return value === undefined ? failDecode('nullable', path) : value;
};
const protobufApply = (bytes: Uint8Array, path: string): GltfChangeNodeNamePayload => {
  const reader = new ProtobufReader(bytes);
  let node: number | undefined;
  let value: string | null | undefined;
  while (reader.remaining) {
    const [field, wire] = key(reader, path);
    if (field === 1 && wire === 0) {
      if (node !== undefined) return failDecode('protobuf-duplicate', `${path}.node`);
      const raw = reader.varint(`${path}.node`);
      node = raw <= BigInt(maximumNode) ? Number(raw) : failDecode('node', `${path}.node`);
    } else if (field === 2 && wire === 2) {
      if (value !== undefined) return failDecode('protobuf-duplicate', `${path}.value`);
      value = protobufNullableBytes(reader.nested(`${path}.value`), `${path}.value`);
    } else if (field === 1 || field === 2) return failDecode('protobuf-wire', path);
    else return failDecode('protobuf-unknown', path);
  }
  if (node === undefined || value === undefined) return failDecode('keys', path);
  return { node, value };
};
const protobufRestore = (bytes: Uint8Array, path: string): GltfChangeNodeNameRestore => {
  const reader = new ProtobufReader(bytes);
  let node: number | undefined;
  let before: string | null | undefined;
  let after: string | null | undefined;
  while (reader.remaining) {
    const [field, wire] = key(reader, path);
    if (field === 1 && wire === 0) {
      if (node !== undefined) return failDecode('protobuf-duplicate', `${path}.node`);
      const raw = reader.varint(`${path}.node`);
      node = raw <= BigInt(maximumNode) ? Number(raw) : failDecode('node', `${path}.node`);
    } else if (field === 2 && wire === 2) {
      if (before !== undefined) return failDecode('protobuf-duplicate', `${path}.before`);
      before = protobufNullableBytes(reader.nested(`${path}.before`), `${path}.before`);
    } else if (field === 3 && wire === 2) {
      if (after !== undefined) return failDecode('protobuf-duplicate', `${path}.after`);
      after = protobufNullableBytes(reader.nested(`${path}.after`), `${path}.after`);
    } else if (field === 1 || field === 2 || field === 3) return failDecode('protobuf-wire', path);
    else return failDecode('protobuf-unknown', path);
  }
  if (node === undefined || before === undefined || after === undefined) return failDecode('keys', path);
  return { node, before, after };
};
export const decodeGltfChangeNodeNameProtobuf = (bytes: Uint8Array): GltfChangeNodeNameMutation => {
  if (!(bytes instanceof Uint8Array)) return failDecode('protobuf-wire', 'protobuf');
  const reader = new ProtobufReader(bytes);
  let mutation: GltfChangeNodeNameMutation | undefined;
  while (reader.remaining) {
    const [field, wire] = key(reader, 'protobuf');
    if (mutation) return failDecode('protobuf-duplicate', 'protobuf.phase');
    if (field === 1 && wire === 2) mutation = { phase: 'apply', value: protobufApply(reader.nested('protobuf.apply'), 'protobuf.apply') };
    else if (field === 2 && wire === 2) mutation = { phase: 'restore', value: protobufRestore(reader.nested('protobuf.restore'), 'protobuf.restore') };
    else if (field === 1 || field === 2) return failDecode('protobuf-wire', 'protobuf');
    else return failDecode('protobuf-unknown', 'protobuf');
  }
  reader.finish('protobuf');
  return mutation ?? failDecode('phase', 'protobuf');
};
//#endregion 🛰️Protobuf
//#endregion 🔖️Facade

//#region 🧬️Operation
const nodePath = (node: number) => `document/nodes/${node}/name`;
const node = (value: number, base: GltfSnapshot): GltfMutationRejection | undefined => Number.isInteger(value) && value <= 0xffff_ffff ? itemIndex(value, base.document.nodes.length, 'document/nodes') : reject('gltf.mutation.index-out-of-range', 'document/nodes', `index ${value} is outside the uint32 public domain`);

export const validateGltfChangeNodeName = (payload: GltfChangeNodeNamePayload, base: GltfSnapshot): GltfMutationRejection | undefined => {
  const invalid = node(payload.node, base);
  if (invalid) return invalid;
  return (base.document.nodes[payload.node]!.name ?? null) === payload.value ? reject('gltf.mutation.no-observable-change', nodePath(payload.node), 'name already has the requested presence and value') : undefined;
};
export const applyGltfChangeNodeName = (base: GltfSnapshot, payload: GltfChangeNodeNamePayload): GltfChangeNodeNameResult => run(base, payload, validateGltfChangeNodeName, (next, value) => { next.document.nodes[value.node]!.name = value.value ?? undefined; });
export const validateGltfChangeNodeNameRestore = (restore: GltfChangeNodeNameRestore, base: GltfSnapshot): GltfMutationRejection | undefined => {
  const invalid = node(restore.node, base);
  if (invalid) return invalid;
  if ((base.document.nodes[restore.node]!.name ?? null) !== restore.after) return reject('gltf.mutation.stale-inverse', nodePath(restore.node), 'current name does not equal the inverse after witness');
  return restore.before === restore.after ? reject('gltf.mutation.no-observable-change', nodePath(restore.node), 'inverse before and after witnesses are equal') : undefined;
};
export const applyGltfChangeNodeNameRestore = (base: GltfSnapshot, restore: GltfChangeNodeNameRestore): GltfChangeNodeNameResult => run(base, restore, validateGltfChangeNodeNameRestore, (next, value) => { next.document.nodes[value.node]!.name = value.before ?? undefined; });
//#endregion 🧬️Operation
