/** 🚨️ Faults and diagnostics: severity with its wire order, fault origin, scope and cause, text spans and `SemioFaultError`.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

// #region 🎠️Kernel
//#region InvocationResponse
/** ⚠️ Canonical severity for faults and diagnostics — TS twin of Rust `os_dsl::Severity`
 * (`🗣️dsl/⚠️diagnostic/🦀️.rs`, `#[serde(rename_all = "camelCase")]`). Declaration order
 * `Info < Warning < Error < Fatal` (0..3, `as_u8`/`from_u8`) mirrors Rust's `derive(Ord)`; `Hint` was
 * removed repo-wide by ticket `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`
 * §C1 and replaced by `Info` everywhere, including `Fault.severity`/`Diagnostic.severity` here. */
export type Severity = "info" | "warning" | "error" | "fatal";

const SEVERITY_ORDER: readonly Severity[] = ["info", "warning", "error", "fatal"];

/** 🔢️ TS twin of Rust `Severity::as_u8` — stable numeric mirror of declaration order, 0..3. */
export function severityAsU8(severity: Severity): number {
  return SEVERITY_ORDER.indexOf(severity);
}

/** 🪞️ TS twin of Rust `Severity::from_u8`; `undefined` for any value outside 0..3. */
export function severityFromU8(value: number): Severity | undefined {
  return SEVERITY_ORDER[value];
}

/** 🧭️ Layer that produced a fault. `"framework"` mirrors Rust `FaultOrigin::Framework`
 * (`💻️os/🔨️modules/🗣️dsl/⚠️diagnostic/🦀️.rs:149`) — the origin for the five ticket
 * 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET `surface.*`/`viewer.*` fault codes. */
export type FaultOrigin = "edge" | "renderer" | "os" | "module" | "plugin" | "app" | "extension" | "framework";

export type FaultScope = {
  readonly pluginId?: string;
  readonly appId?: string;
  readonly instanceId?: string;
  readonly module?: string;
  readonly bodyKey?: string;
};

export type FaultCause = { readonly message: string; readonly code?: string };

export type TextSpan = { readonly line: number; readonly column: number; readonly length: number };

/** 🧯️ Structured abort report shared across Rust, WIT, and TypeScript. */
export type Fault = {
  readonly origin: FaultOrigin;
  readonly code: string;
  readonly severity: Severity;
  readonly message: string;
  readonly scope: FaultScope;
  readonly span?: TextSpan;
  readonly source?: {readonly offset?:bigint;readonly byte?:number;readonly depth?:bigint};
  readonly causes?: readonly FaultCause[];
  /** 🔣️ The named values a localized notice fills its `{name}` placeholders from (Rust `FaultParams`) — never read from `message`. */
  readonly params?: Readonly<Record<string, string>>;
  readonly retryable: boolean;
};

/** 🩺️ A diagnostic emitted alongside an action result. */
export type Diagnostic = {
  readonly code: string;
  readonly severity: Severity;
  readonly message: string;
  readonly scope?: FaultScope;
  readonly span?: TextSpan;
};

/** ❌️ Error subclass carrying a structured {@link Fault}. */
export class SemioFaultError extends Error {
  readonly fault: Fault;
  constructor(fault: Fault) {
    super(fault.message);
    this.name = "SemioFaultError";
    this.fault = fault;
  }
}
//#endregion InvocationResponse
// #endregion 🎠️Kernel
