/** 🖥️ Exact owner-authored native executable identity. */
export interface PlaygroundNativeHostV1 {
  readonly cratePath: string;
  readonly project: string;
  readonly package: string;
  readonly binary: string;
  readonly target: string;
}

/** 📏️ Exact script admission result after decoding and UTF-8 encoding. */
export interface NativeHostScriptAdmissionV1 {readonly withinLimit: boolean;}

/** 📂️ Bounded caller-owned metadata source access. */
export interface NativeHostSourceViewV1 {
  readonly kind: (path: string) => "file" | "directory" | "symlink" | null;
  readonly readText: (path: string) => string;
  readonly admitScript: (path: string) => NativeHostScriptAdmissionV1;
}

/** 🪪️ Producer identities required for executable admission. */
export interface NativeHostOwnerFactsV1 {
  readonly package: string;
  readonly project: string;
  readonly binaries: readonly string[];
  readonly targets: readonly string[];
  readonly modes: Readonly<Record<string, readonly ("prototype" | "release")[]>>;
}

/** 📦️ Actual publication directories read from the owned project target. */
export interface NativeHostSourceFactsV1 extends NativeHostOwnerFactsV1 {
  readonly outputs: Readonly<Record<string, Readonly<Record<"prototype" | "release", readonly string[] | undefined>>>>;
}

/** 🔏️ Admits the exact declaration against the selected owner. */
export function admitPlaygroundNativeHostV1(value: unknown, resolveOwner: (cratePath: string) => NativeHostOwnerFactsV1): PlaygroundNativeHostV1 | undefined;
/** 📜️ Parses an optional complete nativeHost or mcpHost inline table. */
export function parsePlaygroundNativeHostV1(block: string, fieldName?: "nativeHost" | "mcpHost"): PlaygroundNativeHostV1 | undefined;
/** 🧾️ Reads validated Cargo and project source facts. */
export function nativeHostSourceFactsV1(cratePath: string, view: NativeHostSourceViewV1): NativeHostSourceFactsV1;
/** 🚀️ Resolves a declared platform executable from its actual publication directory. */
export function nativeHostArtifactPathV1(host: PlaygroundNativeHostV1, profile: "prototype" | "release", facts: NativeHostSourceFactsV1, platform: string): string;
/** 🧵️ Returns the exact selected producer input closure. */
export function declaredPlaygroundHostInputPathsV1(manifest: string, view: NativeHostSourceViewV1): string[];

/** 📏️ Computes the consumed script byte-bound predicate without exposing implementation bytes. */
export function nativeHostScriptAdmissionV1(source: string): NativeHostScriptAdmissionV1;
