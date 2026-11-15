/** 🧬️ Pins the engine named input to the exact schema bytes this implementation evaluated. */
export const nxEngineNamedInputV1: string;
export const nxEngineNativeNamedInputV1: string;
/** 🔏️ Refuses the plugin root, its private leaves, schemas or contract owners changed after actual module evaluation. */
export function nxBuildDependencySourceIdentityV1(): string;
/** 🧾️ The revisions of the sources this implementation evaluated. */
export function nxBuildDependencySourceRevisionsV1(): readonly Readonly<{path:string;sha256:string}>[];
/** 👁️ Which of the given revisions no longer equal the physical Source DATA. */
export function nxBuildDependencySourceChangesFromV1(revisions: readonly Readonly<{path:string;sha256:string}>[]): readonly string[];
/** 🌱️ A fresh process evaluates the current projection sources and reports identity and revisions. */
export function nxBuildDependencySourceProbeV1(options?: {execPath?:string;timeoutMilliseconds?:number;maximumBytes?:number;run?:(...args:any[])=>any;root?:string}): Readonly<{identity:string;revisions:readonly Readonly<{path:string;sha256:string}>[]}>;
