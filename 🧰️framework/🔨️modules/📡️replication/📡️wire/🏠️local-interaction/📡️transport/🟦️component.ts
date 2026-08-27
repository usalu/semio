/** 📡️ Fixed local-interaction query framing with lossless u64 authority and strict complete-message decoding. */
import type { LocalInteractionIdentity, LocalInteractionPage, LocalInteractionQueryToken } from "../🟦️component.ts";

//#region 🧬️Transport
export type LocalInteractionQueryCommand = { readonly kind: "read"; readonly requestId: string } | { readonly kind: "acknowledge" | "cancel"; readonly token: LocalInteractionQueryToken };
export type LocalInteractionQueryRejection = "busy" | "closed" | "generation-exhausted" | "source-failed";
export type LocalInteractionQueryReply =
  | { readonly kind: "started"; readonly token: LocalInteractionQueryToken }
  | { readonly kind: "page"; readonly page: LocalInteractionPage }
  | { readonly kind: "closed"; readonly token: LocalInteractionQueryToken; readonly cancelled: boolean }
  | { readonly kind: "rejected"; readonly requestId: string; readonly code: LocalInteractionQueryRejection };
const rejections = ["busy", "closed", "generation-exhausted", "source-failed"] as const;
const maximumWireBytes = 4256;
//#endregion 🧬️Transport

//#region 🔢️FixedCodec
export function encodeLocalInteractionUnsigned(text: string): number[] {
  if (!/^(0|[1-9][0-9]{0,19})$/.test(text)) throw new Error("local-interaction.invalid-u64");
  let value = BigInt(text);
  if (value > 0xffffffffffffffffn) throw new Error("local-interaction.invalid-u64");
  const result: number[] = [];
  do { const byte = Number(value & 127n); value >>= 7n; result.push(byte | (value === 0n ? 0 : 128)); } while (value !== 0n);
  return result;
}

class Reader {
  offset = 0;
  constructor(readonly bytes: Uint8Array) { if (bytes.length > maximumWireBytes) throw new Error("local-interaction.wire-envelope"); }
  byte(): number { const value = this.bytes[this.offset++]; if (value === undefined) throw new Error("local-interaction.truncated"); return value; }
  unsigned(): string {
    let value = 0n;
    for (let index = 0; index < 10; index++) {
      const byte = this.byte();
      if (index === 9 && byte > 1) throw new Error("local-interaction.invalid-u64");
      value |= BigInt(byte & 127) << BigInt(index * 7);
      if ((byte & 128) === 0) { if (index !== 0 && byte === 0) throw new Error("local-interaction.noncanonical-u64"); return value.toString(); }
    }
    throw new Error("local-interaction.invalid-u64");
  }
  hash(): string { let result = ""; for (let index = 0; index < 32; index++) result += this.byte().toString(16).padStart(2, "0"); return result; }
  bool(): boolean { const value = this.byte(); if (value > 1) throw new Error("local-interaction.invalid-bool"); return value === 1; }
  finish(): void { if (this.offset !== this.bytes.length) throw new Error("local-interaction.trailing-bytes"); }
}

export function decodeLocalInteractionUnsigned(bytes: Uint8Array): string { const reader = new Reader(bytes); const value = reader.unsigned(); reader.finish(); return value; }

function hash(out: number[], value: string): void {
  if (!/^[0-9a-f]{64}$/.test(value)) throw new Error("local-interaction.invalid-revision");
  for (let index = 0; index < 64; index += 2) out.push(Number.parseInt(value.slice(index, index + 2), 16));
}

function token(out: number[], value: LocalInteractionQueryToken): void {
  out.push(...encodeLocalInteractionUnsigned(value.requestId), ...encodeLocalInteractionUnsigned(value.queryGeneration));
  const identity = value.identity;
  if (!Number.isInteger(identity.appInstanceId) || identity.appInstanceId < 0 || identity.appInstanceId > 0xffffffff) throw new Error("local-interaction.invalid-instance");
  out.push(...encodeLocalInteractionUnsigned(String(identity.appInstanceId)), ...encodeLocalInteractionUnsigned(identity.generation));
  hash(out, identity.revision); hash(out, identity.documentRevision); hash(out, identity.topologyRevision);
  out.push(...encodeLocalInteractionUnsigned(value.ordinal));
}

function readToken(reader: Reader): LocalInteractionQueryToken {
  const requestId = reader.unsigned(), queryGeneration = reader.unsigned();
  const instance = BigInt(reader.unsigned());
  if (instance > 0xffffffffn) throw new Error("local-interaction.invalid-instance");
  const identity: LocalInteractionIdentity = { appInstanceId: Number(instance), generation: reader.unsigned(), revision: reader.hash(), documentRevision: reader.hash(), topologyRevision: reader.hash() };
  return { requestId, queryGeneration, identity, ordinal: reader.unsigned() };
}
//#endregion 🔢️FixedCodec

//#region 📡️CommandAndReply
export function encodeLocalInteractionQueryCommand(command: LocalInteractionQueryCommand): Uint8Array {
  const out = [command.kind === "read" ? 0 : command.kind === "acknowledge" ? 1 : 2];
  if (command.kind === "read") out.push(...encodeLocalInteractionUnsigned(command.requestId)); else token(out, command.token);
  return Uint8Array.from(out);
}

export function decodeLocalInteractionQueryCommand(bytes: Uint8Array): LocalInteractionQueryCommand {
  const reader = new Reader(bytes), kind = reader.byte();
  if (kind > 2) throw new Error("local-interaction.command-kind");
  const result: LocalInteractionQueryCommand = kind === 0 ? { kind: "read", requestId: reader.unsigned() } : { kind: kind === 1 ? "acknowledge" : "cancel", token: readToken(reader) };
  reader.finish(); return result;
}

export function encodeLocalInteractionQueryReply(reply: LocalInteractionQueryReply): Uint8Array {
  const out: number[] = [];
  switch (reply.kind) {
    case "started": out.push(0); token(out, reply.token); break;
    case "page": out.push(1); token(out, reply.page); out.push(reply.page.terminal ? 1 : 0); if (reply.page.bytes.length > 4096) throw new Error("local-interaction.page-length"); out.push(...encodeLocalInteractionUnsigned(String(reply.page.bytes.length))); for (const byte of reply.page.bytes) { if (!Number.isInteger(byte) || byte < 0 || byte > 255) throw new Error("local-interaction.page-byte"); out.push(byte); } break;
    case "closed": out.push(2); token(out, reply.token); out.push(reply.cancelled ? 1 : 0); break;
    case "rejected": { const code = rejections.indexOf(reply.code); if (code < 0) throw new Error("local-interaction.rejection-code"); out.push(3, ...encodeLocalInteractionUnsigned(reply.requestId), code); break; }
  }
  return Uint8Array.from(out);
}

export function decodeLocalInteractionQueryReply(bytes: Uint8Array): LocalInteractionQueryReply {
  const reader = new Reader(bytes), kind = reader.byte();
  let result: LocalInteractionQueryReply;
  if (kind === 0) result = { kind: "started", token: readToken(reader) };
  else if (kind === 1) {
    const token = readToken(reader), terminal = reader.bool(), length = Number(reader.unsigned());
    if (length > 4096) throw new Error("local-interaction.page-length");
    const payload: number[] = []; for (let index = 0; index < length; index++) payload.push(reader.byte());
    result = { kind: "page", page: { ...token, terminal, bytes: payload } };
  } else if (kind === 2) result = { kind: "closed", token: readToken(reader), cancelled: reader.bool() };
  else if (kind === 3) { const requestId = reader.unsigned(), code = rejections[reader.byte()]; if (code === undefined) throw new Error("local-interaction.rejection-code"); result = { kind: "rejected", requestId, code }; }
  else throw new Error("local-interaction.reply-kind");
  reader.finish(); return result;
}
//#endregion 📡️CommandAndReply
