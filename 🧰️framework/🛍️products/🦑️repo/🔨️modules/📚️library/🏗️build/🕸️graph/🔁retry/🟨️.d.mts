/** 🔁️ Projects only bounded source identities and hashes; raw observed facts and environment values never leave the owner. */
export function buildGraphRetryDiagnosticV1(root:string,reason:string,observed?:Readonly<{before?:string|null;after?:string|null;stale?:readonly Readonly<{kind:unknown;path:string;fact?:unknown}>[]}>,currentDigest?:(path:string)=>string|null):Readonly<{version:1;reason:string;before:string|null;after:string|null;facts:readonly Readonly<{kind:string;path:string|null;before:string;after:string|null}>[]}>;

/** 🧮️ The verdict of a sealed read: per-owner digests of the observed rows, the count of rows that changed after their observation and the owners they belong to. */
export type BuildGraphSealVerdictV1=Readonly<{version:1;mode:"quiet"|"repaired"|"sealed-with-later-changes";rounds:number;sealedWithLaterChanges:number;laterChangedOwners:readonly string[];membershipMoved:boolean;admissionMoved:boolean;owners:Readonly<Record<string,string>>}>;
export function sealVerdictV1(input:Readonly<{rounds:number;staleCount:number;stale:readonly Readonly<{path:string}>[];membershipMoved:boolean;admissionMoved:boolean;projects:Readonly<Record<string,{root:string}>>|undefined;rows:readonly Readonly<{kind:unknown;path:string;fact?:unknown}>[]}>):BuildGraphSealVerdictV1;

/** 📣️ Tells the slice report of the worker what the last sealed read said. */
export function reportSealV1(verdict:BuildGraphSealVerdictV1):void;
