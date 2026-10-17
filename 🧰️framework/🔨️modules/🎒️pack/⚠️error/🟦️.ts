/** ⚠️ Container and schema errors preserve actual owned Value refusal identities. */
import { ValueError, type ValueRefusalKind } from "../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts";
import { TextError } from "@semio-tech/framework-diagnostic-ts/text-error";

export { ValueError, type ValueRefusalKind };

/** 🔌️ The actual provider operation owns the external failure category. */
export type PackTransportCategory = "nativeIo" | "httpRequest" | "httpBody";

/** 🔁️ The transport owner explicitly chooses whether one failure may be retried. */
export type PackRetryDisposition = "never" | "transient";

/** 📋️ The lower borrowed source's refusal kind: the three spellings a paged list can produce. */
export type PagedRefusalKind = Extract<ValueRefusalKind, "ownershipLimit" | "allocationFailed" | "invariantViolated">;

/** 📋️ The lower borrowed source's kind and static reason, without any prose parsing. */
export type PagedRefusalMetadata = Readonly<{kind: PagedRefusalKind; reason: string}>;

/** 🧱️ The borrowed cause plus the actual allocated byte witness. */
export type PagedAllocationMetadata = PagedRefusalMetadata & Readonly<{allocatedBytes: number}>;

/** 🧭️ The semantic cause of a Pack error: its original refusal kind, never derived from display prose. */
export type PackCauseKind = Readonly<{kind: "refusal"; refusalKind: ValueRefusalKind}>;

export type PackErrorData =
  | Readonly<{kind: "BadMagic"}>
  | Readonly<{kind: "UnsupportedVersion"; major: number; minor: number}>
  | Readonly<{kind: "UnknownRequiredFlags"; flags: number}>
  | Readonly<{kind: "Truncated"; offset: bigint}>
  | Readonly<{kind: "ChecksumMismatch"; segment: string; offset: bigint}>
  | Readonly<{kind: "ContentHashMismatch"}>
  | Readonly<{kind: "LimitExceeded"; refusalKind: ValueRefusalKind; limit: string}>
  | Readonly<{kind: "RetainedMalformed" | "Malformed"; refusalKind: ValueRefusalKind; what: string; offset: bigint; detail: string}>
  | Readonly<{kind: "RetainedAllocation"; refusalKind: ValueRefusalKind; allocatedBytes: number; what: string; offset: bigint; detail: string}>
  | Readonly<{kind: "NonCanonical"; detail: string}>
  | Readonly<{kind: "UnsupportedCodec"; codec: number}>
  | Readonly<{kind: "ValueRefusal"; error: ValueError}>
  | Readonly<{kind: "TextRefusal"; error: TextError}>
  | Readonly<{kind: "Io"; error: ValueError; retry: PackRetryDisposition}>;

/** 🗨️ Renders the complete container error taxonomy without classifying message text. */
export function packErrorMessage(data: PackErrorData): string {
  switch (data.kind) {
    case "BadMagic": return "bad magic";
    case "UnsupportedVersion": return `unsupported version ${data.major}.${data.minor}`;
    case "UnknownRequiredFlags": return `unknown required feature bits 0x${data.flags.toString(16)}`;
    case "Truncated": return `truncated at offset ${data.offset}`;
    case "ChecksumMismatch": return `checksum mismatch in ${data.segment} at offset ${data.offset}`;
    case "ContentHashMismatch": return "content hash mismatch";
    case "LimitExceeded": return `limit exceeded: ${data.limit}`;
    case "RetainedMalformed": case "RetainedAllocation": case "Malformed": return `malformed ${data.what} at offset ${data.offset}: ${data.detail}`;
    case "NonCanonical": return `non-canonical encoding: ${data.detail}`;
    case "UnsupportedCodec": return `unsupported codec ${data.codec}`;
    case "ValueRefusal": return `schema error: ${data.error.message}`;
    case "TextRefusal": return `schema error: ${data.error.toString()}`;
    case "Io": return `io error: ${data.error.message}`;
  }
}

/** 🧭️ Reads intrinsic syntax authority or the actual mandatory producer-authored kind. */
export function packErrorRefusalKind(data: PackErrorData): ValueRefusalKind {
  switch (data.kind) {
    case "BadMagic": case "Truncated": case "ChecksumMismatch": case "ContentHashMismatch": case "NonCanonical": return "invalidValue";
    case "UnsupportedVersion": case "UnknownRequiredFlags": case "UnsupportedCodec": return "unsupportedOwner";
    case "LimitExceeded": case "RetainedMalformed": case "RetainedAllocation": case "Malformed": return data.refusalKind;
    case "ValueRefusal": case "TextRefusal": case "Io": return data.error.kind;
  }
}

/** 🪪️ Owns one explicit error payload and retains a typed refusal as its original cause. */
export class PackError extends Error {
  constructor(public readonly data: PackErrorData) {
    super(packErrorMessage(data), {cause: data.kind === "ValueRefusal" || data.kind === "TextRefusal" || data.kind === "Io" ? data.error : undefined});
    this.name = "PackError";
  }
  /** 🔎️ The semantic kind of this refusal. */
  get refusalKind(): ValueRefusalKind { return packErrorRefusalKind(this.data); }
  /** 🧭️ The cause identity: always a refusal here, with the original kind. */
  get causeKind(): PackCauseKind { return {kind: "refusal", refusalKind: this.refusalKind}; }
  /** 🔁️ The retry choice of the transport owner, `null` for every error no transport produced. */
  get retry(): PackRetryDisposition | null { return this.data.kind === "Io" ? this.data.retry : null; }
  static fromValue(error: ValueError): PackError { return new PackError({kind: "ValueRefusal", error}); }
  static fromText(error: TextError): PackError { return new PackError({kind: "TextRefusal", error}); }
  /** 📋️ Preserves the lower borrowed source's kind and static reason. */
  static fromPagedRefusal(error: PagedRefusalMetadata, what: string, offset: bigint): PackError {
    return new PackError({kind: "RetainedMalformed", refusalKind: error.kind, what, offset, detail: error.reason});
  }
  /** 🧱️ Preserves the actual allocated byte witness alongside the borrowed cause. */
  static fromPagedAllocation(error: PagedAllocationMetadata, what: string, offset: bigint): PackError {
    return new PackError({kind: "RetainedAllocation", refusalKind: error.kind, allocatedBytes: error.allocatedBytes, what, offset, detail: error.reason});
  }
}
