/** ⚠️ First-party refusal identities declared by the language-neutral schema. */
export type ValueRefusalKind = "invalidValue" | "canceled" | "ownershipLimit" | "allocationFailed" | "workLimit" | "depthLimit" | "unsupportedOwner" | "invariantViolated";
/** 🚨️ An owned typed refusal retains authority through every parent field or ordinal. */
export class ValueError extends Error {
  constructor(public readonly kind: ValueRefusalKind, message: string) { super(message); this.name = "ValueError"; }
  /** 🪆️ Decorates the dotted display path without changing the semantic identity. */
  under(segment: string | number | bigint): ValueError { return new ValueError(this.kind, `${segment}.${this.message}`); }
  /** 🗣️ Projects prose only at a declared terminal text boundary. */
  intoMessage(): string { return this.message; }
  /** 🔤️ Renders the same owned message as the native implementation. */
  override toString(): string { return this.message; }
}
