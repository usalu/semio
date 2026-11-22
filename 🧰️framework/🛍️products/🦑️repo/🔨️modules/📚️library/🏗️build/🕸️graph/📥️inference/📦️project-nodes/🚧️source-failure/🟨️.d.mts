/** 🧰️ The directory of the Build library whose tooling this process runs from. */
export const IMPLEMENTATION_ROOT_V1:string;
/** 🔎️ The module load failure of an error or of its causes. */
export function moduleLoadFailureV1(error:unknown):{error:Error;importer?:string;specifier?:string;target?:string}|undefined;
/** 🚧️ The withheld-source failure of one owner's own source, or `undefined` when the failure is not the owner's. */
export function ownerSourceFailureV1(error:unknown,scope:{workspaceRoot:string;fallback:string;implementationRoot?:string}):(Error&{code:"BUILD_SOURCE_UNPARSEABLE";source:string;sha256:string})|undefined;
/** ⏳️ The failure marked as a replica that has not settled (`replicaUnsettled`) when its source was written within the window of its kind, else `undefined`. */
export function replicaUnsettledV1<T extends {source?:string;unresolvedImport?:boolean}>(error:T,scope?:{now?:number;modified?:(path:string)=>number}):(T&{replicaUnsettled:true})|undefined;
/** 🛂️ The failure of one owner's source as its boundary classifies it (unsettled replica or withheld source), or `undefined` for a failure that is not the source of the owner. */
export function ownerBoundaryFailureV1(error:unknown,scope:{workspaceRoot:string;fallback:string;implementationRoot?:string}):(Error&{code:"BUILD_SOURCE_UNPARSEABLE";source:string;sha256:string;replicaUnsettled?:true})|undefined;
