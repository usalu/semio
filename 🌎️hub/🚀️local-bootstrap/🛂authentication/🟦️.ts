import { createHmac } from "node:crypto";
import { LOCAL_BOOTSTRAP_FRAME_MAX } from "../📡️framing/🟦️.ts";

export const LOCAL_BOOTSTRAP_SCHEMA = "semio.hub.local-bootstrap/v1";
const LOCAL_BOOTSTRAP_DOMAIN = "semio/hub/local-bootstrap/v1\0";
export type LocalClientClass = "native" | "mcp" | "react-relay" | "admin-relay";
export type LocalProfile = { readonly profileId: string; readonly subject: string; readonly displayName: string; readonly allowedClientClasses: readonly LocalClientClass[] };

/** 🔏 Computes the canonical proof over the same bounded frame bytes transmitted to the Hub. */
export function hmacProof(channelKey: Buffer, unsigned: object): string {
  const canonical = Buffer.from(JSON.stringify(unsigned));
  if (canonical.length + 4 > LOCAL_BOOTSTRAP_FRAME_MAX) throw new Error("local bootstrap canonical frame exceeded fixed bound");
  const length = Buffer.alloc(4);
  length.writeUInt32BE(canonical.length);
  const proof = createHmac("sha256", channelKey).update(LOCAL_BOOTSTRAP_DOMAIN).update(length).update(canonical).digest("hex");
  canonical.fill(0);
  return proof;
}

/** 📨 Appends one proof without changing the caller's unsigned value. */
export function authenticatedFrame(channelKey: Buffer, unsigned: Record<string, unknown>): Record<string, unknown> {
  return { ...unsigned, proof: hmacProof(channelKey, unsigned) };
}

/** 🛂 Verifies a response proof with fixed-work byte comparison. */
export function verifyAuthenticatedFrame(channelKey: Buffer, frame: Record<string, unknown>): void {
  const proof = frame.proof;
  if (typeof proof !== "string" || !/^[0-9a-f]{64}$/.test(proof)) throw new Error("local bootstrap response proof invalid");
  const unsigned = { ...frame };
  delete unsigned.proof;
  const expected = hmacProof(channelKey, unsigned);
  const left = Buffer.from(expected, "hex");
  const right = Buffer.from(proof, "hex");
  let difference = 0;
  for (let index = 0; index < left.length; index++) difference |= left[index]! ^ right[index]!;
  left.fill(0);
  right.fill(0);
  if (difference !== 0) throw new Error("local bootstrap response proof invalid");
}
