/** 🔐️ Nominal public capability wire types. */
export type SessionCapabilityV1 = string & { readonly __sessionCapabilityV1: unique symbol };
export type ShareCapabilityV1 = string & { readonly __shareCapabilityV1: unique symbol };
export type InviteCapabilityV1 = string & { readonly __inviteCapabilityV1: unique symbol };
export type SocketGrantCapabilityV1 = string & { readonly __socketGrantCapabilityV1: unique symbol };

const capabilityPattern = /^(session|share|invite|socket)\.v1\.([0-9a-f]{32})\.([0-9a-f]{64})$/;

function parseCapability(value: string, expected: "session" | "share" | "invite" | "socket"): string {
  const match = capabilityPattern.exec(value);
  if (!match || match[1] !== expected) throw new Error(`invalid ${expected}.v1 capability`);
  return value;
}

export const parseSessionCapabilityV1 = (value: string): SessionCapabilityV1 => parseCapability(value, "session") as SessionCapabilityV1;
export const parseShareCapabilityV1 = (value: string): ShareCapabilityV1 => parseCapability(value, "share") as ShareCapabilityV1;
export const parseInviteCapabilityV1 = (value: string): InviteCapabilityV1 => parseCapability(value, "invite") as InviteCapabilityV1;
export const parseSocketGrantCapabilityV1 = (value: string): SocketGrantCapabilityV1 => parseCapability(value, "socket") as SocketGrantCapabilityV1;
