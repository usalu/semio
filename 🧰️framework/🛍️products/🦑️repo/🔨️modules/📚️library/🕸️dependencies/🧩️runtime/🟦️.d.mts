/** 🧩️ Authored component dependency row consumed by the runtime-closure resolver. */
export interface RuntimeComponentClosureRowV1 {
  readonly pluginId: string;
  readonly dependsOn?: readonly string[];
  readonly consumes?: readonly string[];
  readonly contributes?: readonly string[];
  readonly host?: unknown;
}

/** 🕸️ Resolves a deterministic transitive runtime component closure. */
export function runtimeComponentClosure(components: readonly RuntimeComponentClosureRowV1[], roots: readonly string[]): string[];
