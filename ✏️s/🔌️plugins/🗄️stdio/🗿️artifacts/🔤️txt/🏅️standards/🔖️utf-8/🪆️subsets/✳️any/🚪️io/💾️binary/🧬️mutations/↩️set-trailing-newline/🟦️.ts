import {failTxtProtobufDecode} from "../🔣️protobuf/🟦️.ts";
import {TxtProtobufReader,txtProtobufKey} from "../🔣️protobuf/🟦️.ts";
import {SetTrailingNewlinePayload} from "../../../../🧬️schema/🧬️mutations/↩️set-trailing-newline/🟦️.ts";

export const decodeSetTrailingNewlineProtobuf = (bytes: Uint8Array): SetTrailingNewlinePayload => {
  if (!(bytes instanceof Uint8Array)) return failTxtProtobufDecode('protobuf-wire', 'protobuf.setTrailingNewline');
  const reader = new TxtProtobufReader(bytes);
  let value: boolean | undefined;
  while (reader.remaining) {
    const [field, wire] = txtProtobufKey(reader, 'protobuf.setTrailingNewline');
    if (field !== 1) return failTxtProtobufDecode('protobuf-unknown', 'protobuf.setTrailingNewline');
    if (wire !== 0) return failTxtProtobufDecode('protobuf-wire', 'protobuf.setTrailingNewline.value');
    if (value !== undefined) return failTxtProtobufDecode('protobuf-duplicate', 'protobuf.setTrailingNewline.value');
    const raw = reader.varint('protobuf.setTrailingNewline.value');
    if (raw !== 0n && raw !== 1n) return failTxtProtobufDecode('protobuf-wire', 'protobuf.setTrailingNewline.value');
    value = raw === 1n;
  }
  return value === undefined ? failTxtProtobufDecode('keys', 'protobuf.setTrailingNewline') : { value };
};
