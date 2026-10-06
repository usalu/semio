import {failTxtProtobufDecode} from "../🔣️protobuf/🟦️.ts";
import {TxtProtobufReader,txtProtobufKey} from "../🔣️protobuf/🟦️.ts";
import {RemoveLinePayload} from "../../../../🧬️schema/🧬️mutations/🗑️remove-line/🟦️.ts";

export const decodeRemoveLineProtobuf = (bytes: Uint8Array): RemoveLinePayload => {
  if (!(bytes instanceof Uint8Array)) return failTxtProtobufDecode('protobuf-wire', 'protobuf.removeLine');
  const reader = new TxtProtobufReader(bytes);
  let index: number | undefined;
  while (reader.remaining) {
    const [field, wire] = txtProtobufKey(reader, 'protobuf.removeLine');
    if (field !== 1) return failTxtProtobufDecode('protobuf-unknown', 'protobuf.removeLine');
    if (wire !== 0) return failTxtProtobufDecode('protobuf-wire', 'protobuf.removeLine.index');
    if (index !== undefined) return failTxtProtobufDecode('protobuf-duplicate', 'protobuf.removeLine.index');
    const raw = reader.varint('protobuf.removeLine.index');
    index = raw <= 0xffff_ffffn ? Number(raw) : failTxtProtobufDecode('u32', 'protobuf.removeLine.index');
  }
  return index === undefined ? failTxtProtobufDecode('keys', 'protobuf.removeLine') : { index };
};
