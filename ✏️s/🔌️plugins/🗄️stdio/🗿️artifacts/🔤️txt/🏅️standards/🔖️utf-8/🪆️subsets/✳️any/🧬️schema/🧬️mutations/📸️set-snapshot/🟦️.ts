/** 📸️ set-snapshot canonical direct payload. */
import { parseTxtSnapshot, type TxtSnapshot } from '../../📸️snapshot/🟦️.ts';
import { TxtProtobufReader, failTxtMutationDecode, txtExact, txtOwn, txtProtobufKey, txtProtobufString } from '../../🔨️modules/🧬️mutation-support/🟦️.ts';

export interface SetSnapshotPayload { readonly snapshot: TxtSnapshot }

const decode = (value: unknown, path: string): SetSnapshotPayload => {
  const record = txtExact(value, path, ['snapshot']);
  if (!txtOwn(record, 'snapshot')) return failTxtMutationDecode('keys', path);
  return { snapshot: parseTxtSnapshot(record.snapshot, `${path}.snapshot`) };
};

export const decodeSetSnapshotJson = (value: unknown): SetSnapshotPayload => decode(value, 'json');
export const decodeSetSnapshotGraphql = (value: unknown): SetSnapshotPayload => decode(value, 'graphql.setSnapshot');
export const decodeSetSnapshotProto = (value: unknown): SetSnapshotPayload => decode(value, 'protobuf.setSnapshot');
export const decodeSetSnapshotProtobuf = (bytes: Uint8Array): SetSnapshotPayload => {
  if (!(bytes instanceof Uint8Array)) return failTxtMutationDecode('protobuf-wire', 'protobuf.setSnapshot');
  const reader = new TxtProtobufReader(bytes);
  const [field, wire] = txtProtobufKey(reader, 'protobuf.setSnapshot');
  if (field !== 1 || wire !== 2) return failTxtMutationDecode('protobuf-wire', 'protobuf.setSnapshot');
  const nested = new TxtProtobufReader(reader.nested('protobuf.setSnapshot.snapshot'));
  reader.finish('protobuf.setSnapshot');
  let schema = '';
  const lines: string[] = [];
  let trailingNewline = false;
  let lineEnding: TxtSnapshot['lineEnding'] = 'lf';
  while (nested.remaining) {
    const [nestedField, nestedWire] = txtProtobufKey(nested, 'protobuf.setSnapshot.snapshot');
    if (nestedField === 1 && nestedWire === 2) schema = txtProtobufString(nested.nested('protobuf.setSnapshot.snapshot.schema'), 'protobuf.setSnapshot.snapshot.schema');
    else if (nestedField === 2 && nestedWire === 2) lines.push(txtProtobufString(nested.nested('protobuf.setSnapshot.snapshot.lines'), 'protobuf.setSnapshot.snapshot.lines'));
    else if (nestedField === 3 && nestedWire === 0) trailingNewline = nested.varint('protobuf.setSnapshot.snapshot.trailingNewline') !== 0n;
    else if (nestedField === 4 && nestedWire === 0) {
      const value = nested.varint('protobuf.setSnapshot.snapshot.lineEnding');
      lineEnding = value === 0n ? 'lf' : value === 1n ? 'crLf' : failTxtMutationDecode('protobuf-wire', 'protobuf.setSnapshot.snapshot.lineEnding');
    } else return failTxtMutationDecode('protobuf-unknown', 'protobuf.setSnapshot.snapshot');
  }
  return { snapshot: { schema, lines, trailingNewline, lineEnding } };
};
