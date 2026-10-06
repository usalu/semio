import {failTxtProtobufDecode} from "../🔣️protobuf/🟦️.ts";
import {TxtProtobufReader,txtProtobufKey} from "../🔣️protobuf/🟦️.ts";
import {SetLineEndingPayload} from "../../../../🧬️schema/🧬️mutations/🔚️set-line-ending/🟦️.ts";

export const decodeSetLineEndingProtobuf = (bytes: Uint8Array): SetLineEndingPayload => {
  if (!(bytes instanceof Uint8Array)) return failTxtProtobufDecode('protobuf-wire', 'protobuf.setLineEnding');
  const reader = new TxtProtobufReader(bytes);
  let value: 'lf' | 'crLf' | undefined;
  while (reader.remaining) {
    const [field, wire] = txtProtobufKey(reader, 'protobuf.setLineEnding');
    if (field !== 1) return failTxtProtobufDecode('protobuf-unknown', 'protobuf.setLineEnding');
    if (wire !== 0) return failTxtProtobufDecode('protobuf-wire', 'protobuf.setLineEnding.value');
    if (value !== undefined) return failTxtProtobufDecode('protobuf-duplicate', 'protobuf.setLineEnding.value');
    const raw = reader.varint('protobuf.setLineEnding.value');
    value = raw === 0n ? 'lf' : raw === 1n ? 'crLf' : failTxtProtobufDecode('protobuf-wire', 'protobuf.setLineEnding.value');
  }
  return value === undefined ? failTxtProtobufDecode('keys', 'protobuf.setLineEnding') : { value };
};
