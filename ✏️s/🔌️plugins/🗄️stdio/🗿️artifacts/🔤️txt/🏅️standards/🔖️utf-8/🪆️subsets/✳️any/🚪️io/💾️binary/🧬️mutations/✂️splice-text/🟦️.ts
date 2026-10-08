import {failTxtProtobufDecode} from "../🔣️protobuf/🟦️.ts";
import {TxtProtobufReader, txtProtobufKey, txtProtobufString} from "../🔣️protobuf/🟦️.ts";
import {SpliceTextPayload, TextSplice} from "../../../../🧬️schema/🧬️mutations/✂️splice-text/🟦️.ts";

const decodeTextSplice = (bytes: Uint8Array): TextSplice => {
  const reader = new TxtProtobufReader(bytes);
  let offset: number | undefined;
  let del: number | undefined;
  let insert: string | undefined;
  const number = (path: string): number => { const raw = reader.varint(path); return raw > 4294967295n ? failTxtProtobufDecode('u32', path) : Number(raw); };
  while (reader.remaining) {
    const [field, wire] = txtProtobufKey(reader, 'protobuf.spliceText.splice');
    if (field === 1 && wire === 0) { if (offset !== undefined) return failTxtProtobufDecode('protobuf-duplicate', 'protobuf.spliceText.splice.offset'); offset = number('protobuf.spliceText.splice.offset'); }
    else if (field === 2 && wire === 0) { if (del !== undefined) return failTxtProtobufDecode('protobuf-duplicate', 'protobuf.spliceText.splice.delete'); del = number('protobuf.spliceText.splice.delete'); }
    else if (field === 3 && wire === 2) { if (insert !== undefined) return failTxtProtobufDecode('protobuf-duplicate', 'protobuf.spliceText.splice.insert'); insert = txtProtobufString(reader.nested('protobuf.spliceText.splice.insert'), 'protobuf.spliceText.splice.insert'); }
    else if (field === 1 || field === 2 || field === 3) return failTxtProtobufDecode('protobuf-wire', 'protobuf.spliceText.splice');
    else return failTxtProtobufDecode('protobuf-unknown', 'protobuf.spliceText.splice');
  }
  return offset === undefined || del === undefined || insert === undefined ? failTxtProtobufDecode('keys', 'protobuf.spliceText.splice') : { offset, delete: del, insert };
};

export const decodeSpliceTextProtobuf = (bytes: Uint8Array): SpliceTextPayload => {
  if (!(bytes instanceof Uint8Array)) return failTxtProtobufDecode('protobuf-wire', 'protobuf.spliceText');
  const reader = new TxtProtobufReader(bytes);
  const splices: TextSplice[] = [];
  while (reader.remaining) {
    const [field, wire] = txtProtobufKey(reader, 'protobuf.spliceText');
    if (field === 1 && wire === 2) splices.push(decodeTextSplice(reader.nested('protobuf.spliceText.splices')));
    else if (field === 1) return failTxtProtobufDecode('protobuf-wire', 'protobuf.spliceText');
    else return failTxtProtobufDecode('protobuf-unknown', 'protobuf.spliceText');
  }
  return { splices };
};
