import {failTxtProtobufDecode} from "../🔣️protobuf/🟦️.ts";
import {TxtProtobufReader,txtProtobufKey,txtProtobufString} from "../🔣️protobuf/🟦️.ts";
import {SetLinePayload} from "../../../../🧬️schema/🧬️mutations/✏️set-line/🟦️.ts";

export const decodeSetLineProtobuf = (bytes: Uint8Array): SetLinePayload => {
  if (!(bytes instanceof Uint8Array)) return failTxtProtobufDecode('protobuf-wire', 'protobuf.setLine');
  const reader = new TxtProtobufReader(bytes);
  let index: number | undefined;
  let text: string | undefined;
  while (reader.remaining) {
    const [field, wire] = txtProtobufKey(reader, 'protobuf.setLine');
    if (field === 1 && wire === 0) { if (index !== undefined) return failTxtProtobufDecode('protobuf-duplicate', 'protobuf.setLine.index'); const raw = reader.varint('protobuf.setLine.index'); index = raw <= 0xffff_ffffn ? Number(raw) : failTxtProtobufDecode('u32', 'protobuf.setLine.index'); }
    else if (field === 2 && wire === 2) { if (text !== undefined) return failTxtProtobufDecode('protobuf-duplicate', 'protobuf.setLine.text'); text = txtProtobufString(reader.nested('protobuf.setLine.text'), 'protobuf.setLine.text'); }
    else if (field === 1 || field === 2) return failTxtProtobufDecode('protobuf-wire', 'protobuf.setLine');
    else return failTxtProtobufDecode('protobuf-unknown', 'protobuf.setLine');
  }
  return index === undefined || text === undefined ? failTxtProtobufDecode('keys', 'protobuf.setLine') : { index, text };
};
