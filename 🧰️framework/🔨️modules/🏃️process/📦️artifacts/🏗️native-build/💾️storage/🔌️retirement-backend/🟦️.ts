/** 🔌️ Platform strategy of the native storage retirement checks: the persistent backend binds for darwin and the fixed backend for every other platform. The storage core and the creator claim dispatch through it, so neither depends on a backend and each backend depends on both. */
export interface NativeRetirementBackendV1 {
  requireSuccessfulChildRetirement(custody: unknown, cleanup: unknown, enrollment: unknown, request: unknown): void;
  requireFailedBirthRetirement(owner: unknown, enrollment: unknown, request: unknown): void;
  requireRecoveredClaim(custody: unknown, cleanup: unknown, enrollment: unknown, request: unknown, claim?: unknown, removed?: boolean): void;
  requireRemovedBirthClaim(owner: unknown, enrollment: unknown, claim: unknown, request: unknown): void;
}
export type NativeRetirementBackendKindV1 = "persistent" | "fixed";

const bound = new Map<NativeRetirementBackendKindV1, NativeRetirementBackendV1>();

/** 🔗️ Binds one backend's checks when its module loads. */
export function bindNativeRetirementBackendV1(kind: NativeRetirementBackendKindV1, backend: NativeRetirementBackendV1): void {
  bound.set(kind, backend);
}

/** 🪪️ The backend of this platform; an unbound backend is a wiring fault. */
export function nativeRetirementBackendV1(): NativeRetirementBackendV1 {
  const kind: NativeRetirementBackendKindV1 = process.platform === "darwin" ? "persistent" : "fixed";
  const backend = bound.get(kind);
  if (!backend) throw new Error("Native storage retirement backend is unbound: " + kind);
  return backend;
}
