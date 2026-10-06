import {failTxtProtobufDecode} from "../🔣️protobuf/🟦️.ts";
import {TxtProtobufReader,txtProtobufKey,txtProtobufString} from "../🔣️protobuf/🟦️.ts";
import {InsertLinePayload} from "../../../../🧬️schema/🧬️mutations/📥️insert-line/🟦️.ts";

export const decodeInsertLineProtobuf = (bytes: Uint8Array): InsertLinePayload => {
  if (!(bytes instanceof Uint8Array)) return failTxtProtobufDecode('protobuf-wire', 'protobuf.insertLine');
  const reader = new TxtProtobufReader(bytes);
  let index: number | undefined;
  let text: string | undefined;
  while (reader.remaining) {
    const [field, wire] = txtProtobufKey(reader, 'protobuf.insertLine');
    if (field === 1 && wire === 0) { if (index !== undefined) return failTxtProtobufDecode('protobuf-duplicate', 'protobuf.insertLine.index'); const raw = reader.varint('protobuf.insertLine.index'); index = raw <= 0xffff_ffffn ? Number(raw) : failTxtProtobufDecode('u32', 'protobuf.insertLine.index'); }
    else if (field === 2 && wire === 2) { if (text !== undefined) return failTxtProtobufDecode('protobuf-duplicate', 'protobuf.insertLine.text'); text = txtProtobufString(reader.nested('protobuf.insertLine.text'), 'protobuf.insertLine.text'); }
    else if (field === 1 || field === 2) return failTxtProtobufDecode('protobuf-wire', 'protobuf.insertLine');
    else return failTxtProtobufDecode('protobuf-unknown', 'protobuf.insertLine');
  }
  return index === undefined || text === undefined ? failTxtProtobufDecode('keys', 'protobuf.insertLine') : { index, text };
};
