/** ⚠️ First-party refusal identities declared by the language-neutral schema. */
export type ValueRefusalKind = "invalidValue" | "canceled" | "ownershipLimit" | "allocationFailed" | "workLimit" | "depthLimit" | "unsupportedOwner" | "invariantViolated";
/** 🔎️ Reads the exact first-party closed spelling without classifying error prose. */
export function valueRefusalKindFromWire(value:unknown):ValueRefusalKind|undefined {switch(value){case"invalidValue":case"canceled":case"ownershipLimit":case"allocationFailed":case"workLimit":case"depthLimit":case"unsupportedOwner":case"invariantViolated":return value;default:return undefined;}}
/** 🧾️ Actual physical effects retained by the original failed turn. */
export interface ValueErrorRetainedProgress { readonly copiedItems: number; readonly copiedBytes: number; readonly retainedCapacityBytes: number; readonly releasedBytes: number; }
/** 🚨️ An owned typed refusal retains authority through every parent field or ordinal. */
export class ValueError extends Error {
  private retainedReceipt: ValueErrorRetainedProgress = { copiedItems: 0, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0 };
  /** 🧾️ Preserve original failure effects without creating a replacement wallet. */
  get retainedProgress(): ValueErrorRetainedProgress { return this.retainedReceipt; }
  /** 📥️ Attach the receipt of this original failed turn. */
  withRetainedProgress(progress: ValueErrorRetainedProgress): this { this.retainedReceipt = progress; return this; }
  constructor(public readonly kind: ValueRefusalKind, message: string) { super(message); this.name = "ValueError"; }
  /** 🪆️ Decorates the dotted display path without changing the semantic identity. */
  under(segment: string | number | bigint): ValueError { return new ValueError(this.kind, `${segment}.${this.message}`).withRetainedProgress(this.retainedReceipt); }
  /** 🗣️ Projects prose only at a declared terminal text boundary. */
  intoMessage(): string { return this.message; }
  /** 🔤️ Renders the same owned message as the native implementation. */
  override toString(): string { return this.message; }
}
