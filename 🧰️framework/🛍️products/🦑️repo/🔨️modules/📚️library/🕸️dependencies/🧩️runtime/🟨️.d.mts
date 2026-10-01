/** 🧩️ Authored component dependency row consumed by the runtime-closure resolver. */
export interface RuntimeComponentClosureRowV1 {
  readonly pluginId: string;
  readonly dependsOn?: readonly string[];
  readonly consumes?: readonly string[];
  readonly contributes?: readonly string[];
  readonly host?: unknown;
}

/** 🏠️ One closure root. `appScoped` marks a root booted as ONE of its crate's artifact apps, which
 * suppresses the host crate's "pull every component" edge (see `runtimeComponentClosure`). */
export type RuntimeComponentClosureRootV1 = string | { readonly id: string; readonly appScoped?: boolean };

/** 🕸️ Resolves a deterministic transitive runtime component closure. */
export function runtimeComponentClosure(components: readonly RuntimeComponentClosureRowV1[], roots: readonly RuntimeComponentClosureRootV1[]): string[];

/** 📥️ Source admission retains missing input facts separately from admitted closure membership. */
export type RuntimeInputAdmissionV1 = Readonly<{schemaVersion:1;status:"admitted";selected:readonly string[]}> | Readonly<{schemaVersion:1;status:"refused";missing:Readonly<{kind:"component"|"source";value:string}>}>;
export function runtimeInputAdmissionV1(components:readonly RuntimeComponentClosureRowV1[],roots:readonly RuntimeComponentClosureRootV1[],sourcePaths:readonly string[],sourceExists:(path:string)=>boolean):RuntimeInputAdmissionV1;
