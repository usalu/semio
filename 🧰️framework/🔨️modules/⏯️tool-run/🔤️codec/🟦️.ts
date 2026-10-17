/** 🔤️ Tool run pack wire codec: pack values, the bounded buffer and reader, and the trace page, trace delta and tick encoders and decoders. TypeScript twin of `🔤️codec/🦀️.rs`. */

import { TOOL_RUN_COUNTERS_MAX, TOOL_RUN_STEP_ARGS_MAX, TOOL_RUN_STEP_RING_CAPACITY, TOOL_RUN_TICK_BYTES_MAX, TOOL_RUN_TRACE_PAGE_BYTES_MAX, TOOL_RUN_TRACE_PAGE_OPS_MAX, ToolRunCodecError, type ToolRunIdentity } from "../🟦️.ts";
import { TOOL_RUN_STATES } from "../🚦️lifecycle/🟦️.ts";
import { TOOL_RUN_STEP_KINDS, TOOL_RUN_VERDICTS, type ToolRunCounter, type ToolRunProgress, type ToolRunStep, type ToolRunStepArg } from "../📊️progress/🟦️.ts";
import type { ToolRunTraceDelta, ToolRunTraceOp, ToolRunTracePage, ToolRunTraceSubject } from "../🗄️trace-store/🟦️.ts";
import type { ToolRunTick } from "../🎞️tick/🟦️.ts";

//#region 🔖️Codec
const TAG_ABSENT = 0x00;
const TAG_FALSE = 0x01;
const TAG_TRUE = 0x02;
const TAG_UINT = 0x04;
const TAG_F64 = 0x05;
const TAG_BYTES = 0x08;
const TAG_LIST = 0x0c;
const TAG_RECORD = 0x0d;

type PackValue = { readonly t: "uint"; readonly v: bigint } | { readonly t: "bool"; readonly v: boolean } | { readonly t: "f64"; readonly v: number } | { readonly t: "bytes"; readonly v: Uint8Array } | { readonly t: "record"; readonly v: PackRecord } | { readonly t: "list"; readonly v: readonly PackValue[] };
type PackRecord = Map<number, PackValue>;

class PackBuffer {
  bytes = new Uint8Array(256);
  length = 0;

  reserve(extra: number): void {
    if (this.length + extra <= this.bytes.length) return;
    const next = new Uint8Array(Math.max(this.bytes.length * 2, this.length + extra));
    next.set(this.bytes.subarray(0, this.length));
    this.bytes = next;
  }

  u8(value: number): void {
    this.reserve(1);
    this.bytes[this.length++] = value;
  }

  varint(value: bigint | number): void {
    let rest = BigInt(value);
    this.reserve(10);
    while (rest >= 0x80n) {
      this.bytes[this.length++] = Number(rest & 0x7fn) | 0x80;
      rest >>= 7n;
    }
    this.bytes[this.length++] = Number(rest);
  }

  raw(bytes: Uint8Array): void {
    this.reserve(bytes.length);
    this.bytes.set(bytes, this.length);
    this.length += bytes.length;
  }

  finish(): Uint8Array {
    return this.bytes.slice(0, this.length);
  }
}

function writePackValue(out: PackBuffer, value: PackValue): void {
  switch (value.t) {
    case "uint":
      out.u8(TAG_UINT);
      out.varint(value.v);
      return;
    case "bool":
      out.u8(value.v ? TAG_TRUE : TAG_FALSE);
      return;
    case "f64": {
      out.u8(TAG_F64);
      const cell = new DataView(new ArrayBuffer(8));
      if (Number.isNaN(value.v)) cell.setBigUint64(0, 0x7ff8_0000_0000_0000n, true);
      else cell.setFloat64(0, value.v, true);
      out.raw(new Uint8Array(cell.buffer));
      return;
    }
    case "bytes":
      out.u8(TAG_BYTES);
      out.varint(value.v.length);
      out.raw(value.v);
      return;
    case "record":
      out.u8(TAG_RECORD);
      writePackFields(out, value.v);
      return;
    case "list":
      out.u8(TAG_LIST);
      out.varint(value.v.length);
      for (const item of value.v) writePackValue(out, item);
  }
}

function writePackFields(out: PackBuffer, record: PackRecord): void {
  const ids = [...record.keys()].sort((a, b) => a - b);
  out.varint(ids.length);
  for (const id of ids) {
    out.varint(id);
    writePackValue(out, record.get(id)!);
  }
}

function encodePackBody(record: PackRecord): Uint8Array {
  const out = new PackBuffer();
  out.varint(0);
  writePackFields(out, record);
  return out.finish();
}

class PackReader {
  offset = 0;
  readonly bytes: Uint8Array;
  constructor(bytes: Uint8Array) {
    this.bytes = bytes;
  }

  u8(): number {
    const byte = this.bytes[this.offset];
    if (byte === undefined) throw new ToolRunCodecError("pack", "truncated");
    this.offset += 1;
    return byte;
  }

  varint(): bigint {
    let result = 0n;
    for (let shift = 0n; shift < 70n; shift += 7n) {
      const byte = this.u8();
      result |= BigInt(byte & 0x7f) << shift;
      if ((byte & 0x80) === 0) {
        if (result > 0xffff_ffff_ffff_ffffn) throw new ToolRunCodecError("pack", "varint exceeds u64");
        return result;
      }
    }
    throw new ToolRunCodecError("pack", "varint exceeds u64");
  }

  length(): number {
    const value = this.varint();
    if (value > BigInt(this.bytes.length - this.offset)) throw new ToolRunCodecError("pack", "declared length exceeds remaining bytes");
    return Number(value);
  }

  raw(length: number): Uint8Array {
    const slice = this.bytes.slice(this.offset, this.offset + length);
    this.offset += length;
    return slice;
  }
}

function readPackValue(reader: PackReader, depth: number): PackValue | undefined {
  if (depth > 64) throw new ToolRunCodecError("pack", "max depth exceeded");
  const tag = reader.u8();
  switch (tag) {
    case TAG_ABSENT:
      return undefined;
    case TAG_FALSE:
      return { t: "bool", v: false };
    case TAG_TRUE:
      return { t: "bool", v: true };
    case TAG_UINT:
      return { t: "uint", v: reader.varint() };
    case TAG_F64:
      return { t: "f64", v: new DataView(reader.raw(8).buffer).getFloat64(0, true) };
    case TAG_BYTES:
      return { t: "bytes", v: reader.raw(reader.length()) };
    case TAG_RECORD:
      return { t: "record", v: readPackFields(reader, depth + 1) };
    case TAG_LIST: {
      const count = reader.length();
      const items: PackValue[] = [];
      for (let index = 0; index < count; index += 1) {
        const item = readPackValue(reader, depth + 1);
        if (item === undefined) throw new ToolRunCodecError("malformed", "absent list item");
        items.push(item);
      }
      return { t: "list", v: items };
    }
    default:
      throw new ToolRunCodecError("malformed", `unexpected tag 0x${tag.toString(16)}`);
  }
}

function readPackFields(reader: PackReader, depth: number): PackRecord {
  const count = reader.length();
  const record: PackRecord = new Map();
  for (let index = 0; index < count; index += 1) {
    const id = reader.varint();
    if (id > 0xffffn) throw new ToolRunCodecError("pack", "field id exceeds u16");
    const value = readPackValue(reader, depth + 1);
    if (value !== undefined) record.set(Number(id), value);
  }
  return record;
}

function decodePackBody(bytes: Uint8Array, knownIds: readonly number[]): PackRecord {
  const reader = new PackReader(bytes);
  const symbols = reader.length();
  for (let index = 0; index < symbols; index += 1) reader.raw(reader.length());
  const record = readPackFields(reader, 0);
  if (reader.offset !== bytes.length) throw new ToolRunCodecError("pack", "trailing bytes after the terminal record");
  assertKnown(record, knownIds);
  return record;
}

function assertKnown(record: PackRecord, knownIds: readonly number[]): void {
  for (const id of record.keys()) if (!knownIds.includes(id)) throw new ToolRunCodecError("pack", `unknown field ${id}`);
}

const IDENTITY_IDS = [1, 2, 3, 4];
const STEP_IDS = [1, 2, 3, 4, 5, 6, 7];
const PROGRESS_IDS = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const PAGE_IDS = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
const DELTA_IDS = [1, 2, 3, 4];
const TICK_IDS = [1, 2, 3, 4, 5, 6, 7, 8, 9];

const uintValue = (v: bigint | number): PackValue => ({ t: "uint", v: BigInt(v) });
const bytesValue = (v: Uint8Array): PackValue => ({ t: "bytes", v });

function getUint(record: PackRecord, id: number): bigint {
  const value = getOptionalUint(record, id);
  if (value === undefined) throw new ToolRunCodecError("malformed", "required unsigned field");
  return value;
}

function getOptionalUint(record: PackRecord, id: number): bigint | undefined {
  const value = record.get(id);
  if (value === undefined) return undefined;
  if (value.t !== "uint") throw new ToolRunCodecError("malformed", "unsigned field");
  return value.v;
}

function getBytes(record: PackRecord, id: number): Uint8Array {
  const value = record.get(id);
  if (value === undefined) return new Uint8Array(0);
  if (value.t !== "bytes") throw new ToolRunCodecError("malformed", "bytes field");
  return value.v;
}

function getList(record: PackRecord, id: number): readonly PackValue[] {
  const value = record.get(id);
  if (value === undefined) return [];
  if (value.t !== "list") throw new ToolRunCodecError("malformed", "list field");
  return value.v;
}

function getRecord(value: PackValue | undefined, what: string, knownIds: readonly number[]): PackRecord {
  if (value?.t !== "record") throw new ToolRunCodecError("malformed", what);
  assertKnown(value.v, knownIds);
  return value.v;
}

function itemBytes(item: PackValue): Uint8Array {
  if (item.t !== "bytes") throw new ToolRunCodecError("malformed", "bytes item");
  return item.v;
}

function narrow(value: bigint, max: number, what: string): number {
  if (value > BigInt(max)) throw new ToolRunCodecError("malformed", what);
  return Number(value);
}

function view(bytes: Uint8Array): DataView {
  return new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
}

function identityRecord(identity: ToolRunIdentity): PackValue {
  if (identity.baseRevision.length !== 32) throw new ToolRunCodecError("malformed", "baseRevision");
  return { t: "record", v: new Map([[1, uintValue(identity.id.appInstanceId)], [2, uintValue(identity.id.run)], [3, uintValue(identity.generation)], [4, bytesValue(identity.baseRevision)]]) };
}

function identityFrom(value: PackValue | undefined): ToolRunIdentity {
  const record = getRecord(value, "identity", IDENTITY_IDS);
  const baseRevision = getBytes(record, 4);
  if (baseRevision.length !== 32) throw new ToolRunCodecError("malformed", "baseRevision");
  return { id: { appInstanceId: narrow(getUint(record, 1), 0xffff_ffff, "appInstanceId"), run: getUint(record, 2) }, generation: narrow(getUint(record, 3), 0xffff_ffff, "generation"), baseRevision };
}

function stepRecord(step: ToolRunStep): PackValue {
  if (step.args.length > TOOL_RUN_STEP_ARGS_MAX) throw new ToolRunCodecError("limit", "step args");
  if (step.repeat === 0) throw new ToolRunCodecError("malformed", "repeat");
  const record: PackRecord = new Map([[1, uintValue(step.sequence)], [2, uintValue(TOOL_RUN_STEP_KINDS.indexOf(step.kind))], [3, uintValue(step.stage)], [4, uintValue(step.reason)], [6, uintValue(step.repeat)]]);
  if (step.subject !== undefined) record.set(5, uintValue(step.subject));
  if (step.args.length > 0) {
    const args = new Uint8Array(9 * step.args.length);
    step.args.forEach((arg, index) => {
      if ("unsigned" in arg) view(args).setBigUint64(9 * index + 1, arg.unsigned, true);
      else {
        args[9 * index] = 1;
        view(args).setFloat64(9 * index + 1, arg.float, true);
      }
    });
    record.set(7, bytesValue(args));
  }
  return { t: "record", v: record };
}

function stepFrom(value: PackValue): ToolRunStep {
  const record = getRecord(value, "record item", STEP_IDS);
  const rawArgs = getBytes(record, 7);
  if (rawArgs.length % 9 !== 0 || rawArgs.length / 9 > TOOL_RUN_STEP_ARGS_MAX) throw new ToolRunCodecError("malformed", "step args");
  const args: ToolRunStepArg[] = [];
  for (let offset = 0; offset < rawArgs.length; offset += 9) {
    if (rawArgs[offset] === 0) args.push({ unsigned: view(rawArgs).getBigUint64(offset + 1, true) });
    else if (rawArgs[offset] === 1) args.push({ float: view(rawArgs).getFloat64(offset + 1, true) });
    else throw new ToolRunCodecError("malformed", "step arg kind");
  }
  const repeat = narrow(getUint(record, 6), 0xffff_ffff, "repeat");
  if (repeat === 0) throw new ToolRunCodecError("malformed", "repeat");
  const kind = TOOL_RUN_STEP_KINDS[narrow(getUint(record, 2), 0xff, "step kind")];
  if (kind === undefined) throw new ToolRunCodecError("malformed", "step kind");
  const subject = getOptionalUint(record, 5);
  return { sequence: getUint(record, 1), kind, stage: narrow(getUint(record, 3), 0xffff, "stage"), reason: narrow(getUint(record, 4), 0xffff, "reason"), ...(subject === undefined ? {} : { subject }), repeat, args };
}

function progressRecord(progress: ToolRunProgress): PackValue {
  if (progress.counters.length > TOOL_RUN_COUNTERS_MAX) throw new ToolRunCodecError("limit", "counters");
  const record: PackRecord = new Map([[1, identityRecord(progress.identity)], [2, uintValue(progress.sequence)], [3, uintValue(TOOL_RUN_STATES.indexOf(progress.state))], [4, uintValue(progress.stage)], [5, uintValue(progress.completed)], [8, { t: "f64", v: Math.fround(progress.unitsPerSecond) }], [9, uintValue(progress.conflicts)]]);
  if (progress.total !== undefined) record.set(6, uintValue(progress.total));
  if (progress.counters.length > 0) {
    const counters = new Uint8Array(10 * progress.counters.length);
    progress.counters.forEach((counter, index) => {
      view(counters).setUint16(10 * index, counter.counter, true);
      view(counters).setBigUint64(10 * index + 2, counter.value, true);
    });
    record.set(7, bytesValue(counters));
  }
  if (progress.steps.length > 0) record.set(10, { t: "list", v: progress.steps.map(stepRecord) });
  return { t: "record", v: record };
}

function progressFrom(value: PackValue): ToolRunProgress {
  const record = getRecord(value, "progress", PROGRESS_IDS);
  const rawCounters = getBytes(record, 7);
  if (rawCounters.length % 10 !== 0 || rawCounters.length / 10 > TOOL_RUN_COUNTERS_MAX) throw new ToolRunCodecError("malformed", "counters");
  const counters: ToolRunCounter[] = [];
  for (let offset = 0; offset < rawCounters.length; offset += 10) counters.push({ counter: view(rawCounters).getUint16(offset, true), value: view(rawCounters).getBigUint64(offset + 2, true) });
  const units = record.get(8);
  if (units?.t !== "f64") throw new ToolRunCodecError("malformed", "unitsPerSecond");
  const state = TOOL_RUN_STATES[narrow(getUint(record, 3), 0xff, "state")];
  if (state === undefined) throw new ToolRunCodecError("malformed", "state");
  const steps = getList(record, 10).map(stepFrom);
  if (steps.length > TOOL_RUN_STEP_RING_CAPACITY) throw new ToolRunCodecError("limit", "step ring capacity");
  const total = getOptionalUint(record, 6);
  return { identity: identityFrom(record.get(1)), sequence: getUint(record, 2), state, stage: narrow(getUint(record, 4), 0xffff, "stage"), completed: getUint(record, 5), ...(total === undefined ? {} : { total }), counters, unitsPerSecond: Math.fround(units.v), conflicts: narrow(getUint(record, 9), 0xffff_ffff, "conflicts"), steps };
}

/** 🎒️ Columnar pack record body of a trace page. */
export function encodeToolRunTracePage(page: ToolRunTracePage): Uint8Array {
  if (page.ops.length > TOOL_RUN_TRACE_PAGE_OPS_MAX) throw new ToolRunCodecError("limit", "trace page ops");
  const kinds: number[] = [];
  const keys: bigint[] = [];
  const verdicts: number[] = [];
  const reasons: number[] = [];
  const subjectKinds: number[] = [];
  const meshes: number[] = [];
  const shapes: number[] = [];
  const entities: bigint[] = [];
  const floats: number[] = [];
  for (const op of page.ops) {
    if (op.op === "clear") {
      kinds.push(2);
      continue;
    }
    keys.push(op.key);
    if (op.op === "retire") {
      kinds.push(1);
      continue;
    }
    kinds.push(0);
    verdicts.push(TOOL_RUN_VERDICTS.indexOf(op.verdict));
    reasons.push(op.reason);
    const subject = op.subject;
    if (subject.kind === "instance3d") {
      subjectKinds.push(0);
      meshes.push(subject.mesh);
      floats.push(...subject.position, ...subject.rotation, subject.scale);
    } else if (subject.kind === "placement2d") {
      subjectKinds.push(1);
      shapes.push(subject.shape);
      floats.push(...subject.position, subject.rotation);
    } else {
      subjectKinds.push(2);
      entities.push(subject.entity);
    }
  }
  const column = (count: number, width: number, write: (cell: DataView, index: number) => void): Uint8Array => {
    const bytes = new Uint8Array(count * width);
    const cell = view(bytes);
    for (let index = 0; index < count; index += 1) write(cell, index);
    return bytes;
  };
  const columns: [number, Uint8Array][] = [
    [3, Uint8Array.from(kinds)],
    [4, column(keys.length, 8, (cell, index) => cell.setBigUint64(8 * index, keys[index]!, true))],
    [5, Uint8Array.from(verdicts)],
    [6, column(reasons.length, 2, (cell, index) => cell.setUint16(2 * index, reasons[index]!, true))],
    [7, Uint8Array.from(subjectKinds)],
    [8, column(meshes.length, 4, (cell, index) => cell.setUint32(4 * index, meshes[index]!, true))],
    [9, column(shapes.length, 4, (cell, index) => cell.setUint32(4 * index, shapes[index]!, true))],
    [10, column(entities.length, 8, (cell, index) => cell.setBigUint64(8 * index, entities[index]!, true))],
    [11, column(floats.length, 4, (cell, index) => cell.setFloat32(4 * index, floats[index]!, true))],
  ];
  const record: PackRecord = new Map([[1, identityRecord(page.identity)], [2, uintValue(page.page)]]);
  for (const [id, bytes] of columns) if (bytes.length > 0) record.set(id, bytesValue(bytes));
  const bytes = encodePackBody(record);
  if (bytes.length > TOOL_RUN_TRACE_PAGE_BYTES_MAX) throw new ToolRunCodecError("limit", "trace page bytes");
  return bytes;
}

export function decodeToolRunTracePage(bytes: Uint8Array): ToolRunTracePage {
  if (bytes.length > TOOL_RUN_TRACE_PAGE_BYTES_MAX) throw new ToolRunCodecError("limit", "trace page bytes");
  return tracePageFrom(decodePackBody(bytes, PAGE_IDS));
}

function tracePageFrom(record: PackRecord): ToolRunTracePage {
  const kinds = getBytes(record, 3);
  if (kinds.length > TOOL_RUN_TRACE_PAGE_OPS_MAX) throw new ToolRunCodecError("limit", "trace page ops");
  const keys = getBytes(record, 4);
  const verdicts = getBytes(record, 5);
  const reasons = getBytes(record, 6);
  const subjectKinds = getBytes(record, 7);
  const meshes = getBytes(record, 8);
  const shapes = getBytes(record, 9);
  const entities = getBytes(record, 10);
  const floats = getBytes(record, 11);
  const count = (bytes: Uint8Array, value: number) => bytes.reduce((sum, byte) => sum + (byte === value ? 1 : 0), 0);
  const upserts = count(kinds, 0);
  const keyed = upserts + count(kinds, 1);
  const instances = count(subjectKinds, 0);
  const placements = count(subjectKinds, 1);
  const entitySubjects = count(subjectKinds, 2);
  if (kinds.some((kind) => kind > 2) || keys.length % 8 !== 0 || keys.length / 8 !== keyed || verdicts.length !== upserts || reasons.length !== 2 * upserts || subjectKinds.length !== upserts || instances + placements + entitySubjects !== upserts || meshes.length % 4 !== 0 || meshes.length / 4 !== instances || shapes.length % 4 !== 0 || shapes.length / 4 !== placements || entities.length % 8 !== 0 || entities.length / 8 !== entitySubjects || floats.length % 4 !== 0 || floats.length / 4 !== 8 * instances + 3 * placements) {
    throw new ToolRunCodecError("malformed", "trace page columns");
  }
  const keyCells = view(keys);
  const reasonCells = view(reasons);
  const meshCells = view(meshes);
  const shapeCells = view(shapes);
  const entityCells = view(entities);
  const floatCells = view(floats);
  const float = (index: number) => floatCells.getFloat32(4 * index, true);
  let keyAt = 0;
  let upsertAt = 0;
  let meshAt = 0;
  let shapeAt = 0;
  let entityAt = 0;
  let floatAt = 0;
  const ops: ToolRunTraceOp[] = [];
  for (const kind of kinds) {
    if (kind === 2) {
      ops.push({ op: "clear" });
      continue;
    }
    const key = keyCells.getBigUint64(8 * keyAt++, true);
    if (kind === 1) {
      ops.push({ op: "retire", key });
      continue;
    }
    const verdict = TOOL_RUN_VERDICTS[verdicts[upsertAt]!];
    if (verdict === undefined) throw new ToolRunCodecError("malformed", "verdict");
    let subject: ToolRunTraceSubject;
    const subjectKind = subjectKinds[upsertAt]!;
    if (subjectKind === 0) {
      subject = { kind: "instance3d", mesh: meshCells.getUint32(4 * meshAt++, true), position: [float(floatAt), float(floatAt + 1), float(floatAt + 2)], rotation: [float(floatAt + 3), float(floatAt + 4), float(floatAt + 5), float(floatAt + 6)], scale: float(floatAt + 7) };
      floatAt += 8;
    } else if (subjectKind === 1) {
      subject = { kind: "placement2d", shape: shapeCells.getUint32(4 * shapeAt++, true), position: [float(floatAt), float(floatAt + 1)], rotation: float(floatAt + 2) };
      floatAt += 3;
    } else {
      subject = { kind: "entity", entity: entityCells.getBigUint64(8 * entityAt++, true) };
    }
    ops.push({ op: "upsert", key, verdict, reason: reasonCells.getUint16(2 * upsertAt, true), subject });
    upsertAt += 1;
  }
  return { identity: identityFrom(record.get(1)), page: narrow(getUint(record, 2), 0xffff_ffff, "page"), ops };
}

export function encodeToolRunTraceDelta(delta: ToolRunTraceDelta): Uint8Array {
  const record: PackRecord = new Map([[1, identityRecord(delta.identity)], [2, { t: "bool", v: delta.clear }], [3, uintValue(delta.next)]]);
  if (delta.pages.length > 0) record.set(4, { t: "list", v: delta.pages.map((page) => bytesValue(encodeToolRunTracePage(page))) });
  return encodePackBody(record);
}

export function decodeToolRunTraceDelta(bytes: Uint8Array): ToolRunTraceDelta {
  const record = decodePackBody(bytes, DELTA_IDS);
  const clear = record.get(2);
  if (clear?.t !== "bool") throw new ToolRunCodecError("malformed", "boolean field");
  return { identity: identityFrom(record.get(1)), clear: clear.v, next: narrow(getUint(record, 3), 0xffff_ffff, "next"), pages: getList(record, 4).map((item) => decodeToolRunTracePage(itemBytes(item))) };
}

/** 🧳️ Pack record body of a tick; rejects ticks over `TOOL_RUN_TICK_BYTES_MAX`. */
export function encodeToolRunTick(tick: ToolRunTick): Uint8Array {
  const record: PackRecord = new Map([[1, identityRecord(tick.identity)], [2, uintValue(tick.sequence)]]);
  if (tick.progress !== undefined) record.set(3, progressRecord(tick.progress));
  if (tick.steps.length > 0) record.set(4, { t: "list", v: tick.steps.map(stepRecord) });
  if (tick.trace.length > 0) record.set(5, { t: "list", v: tick.trace.map((page) => bytesValue(encodeToolRunTracePage(page))) });
  if (tick.appendOps.length > 0) record.set(6, { t: "list", v: tick.appendOps.map(bytesValue) });
  if (tick.appendEntities.length > 0) {
    const entities = new Uint8Array(8 * tick.appendEntities.length);
    tick.appendEntities.forEach((entity, index) => view(entities).setBigUint64(8 * index, entity, true));
    record.set(7, bytesValue(entities));
  }
  if (tick.retractTo !== undefined) record.set(8, uintValue(tick.retractTo));
  if (tick.payload !== undefined) record.set(9, bytesValue(tick.payload));
  const bytes = encodePackBody(record);
  if (bytes.length > TOOL_RUN_TICK_BYTES_MAX) throw new ToolRunCodecError("limit", "tick bytes");
  return bytes;
}

export function decodeToolRunTick(bytes: Uint8Array): ToolRunTick {
  if (bytes.length > TOOL_RUN_TICK_BYTES_MAX) throw new ToolRunCodecError("limit", "tick bytes");
  const record = decodePackBody(bytes, TICK_IDS);
  const rawEntities = getBytes(record, 7);
  if (rawEntities.length % 8 !== 0) throw new ToolRunCodecError("malformed", "appendEntities");
  const appendEntities: bigint[] = [];
  for (let offset = 0; offset < rawEntities.length; offset += 8) appendEntities.push(view(rawEntities).getBigUint64(offset, true));
  const progress = record.get(3);
  const retractTo = getOptionalUint(record, 8);
  const payload = record.has(9) ? getBytes(record, 9) : undefined;
  return {
    identity: identityFrom(record.get(1)),
    sequence: getUint(record, 2),
    ...(progress === undefined ? {} : { progress: progressFrom(progress) }),
    steps: getList(record, 4).map(stepFrom),
    trace: getList(record, 5).map((item) => decodeToolRunTracePage(itemBytes(item))),
    appendOps: getList(record, 6).map(itemBytes),
    appendEntities,
    ...(retractTo === undefined ? {} : { retractTo: narrow(retractTo, 0xffff_ffff, "retractTo") }),
    ...(payload === undefined ? {} : { payload }),
  };
}
//#endregion 🔖️Codec
