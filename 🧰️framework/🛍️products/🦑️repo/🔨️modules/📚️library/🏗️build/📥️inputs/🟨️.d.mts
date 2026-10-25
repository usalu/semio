/** 🛰️ Adopts the leaves a farm worker staged into the graph under this process's writer custody. */
export function adoptStagedGraphLeavesV1(workspaceRoot:string,stagingRoot:string,names:readonly string[]):Readonly<{published:number;present:number;declined:boolean}>;
/** 📍️ Projects the project-root sentinel to its exact Cargo manifest coordinate. */
export function cargoManifestInputV1(root:string):string;
/** 🔒️ Publishes a compact Cargo lock inventory under original graph writer custody. */
export function cargoLockInput(manifest:string,workspaceRoot:string):{env:string};
/** 🧵️ Matches portable source globs across admitted native backends. */
export function buildInputPathMatchesV1(path:string,pattern:string):boolean;
/** 🧮️ Publishes shared exact-source inventories under original graph writer custody. */
/** ✂️ Cuts a sorted roster into content-defined chunks (a chunk ends after a path whose digest byte is a multiple of 128 once it holds 16 paths). */
export function splitInputClosureV1(files:readonly string[]):string[][];
/** 🕊️ Forgets the in-process memo of captured inventories and verified closure stamps. */
export function releaseInputMemoryV1():void;
export function compactSourceInputs(inputs:unknown[],workspaceRoot:string,root?:string):unknown[];
/** 🧮️ Interns equivalent source coordinates before native hashing. */
export function deduplicateProjectInputs(inputs:unknown[],root:string):unknown[];
/** 🧬️ Checks whether authored output negations overlap source scopes. */
export function sourceOutputOverlaps(output:string,inputs:unknown[],root:string):boolean;
/** 🗂️ Shares equivalent arrays within one project input declaration. */
export function internProjectInputGroups(groups:Record<string,unknown[]>):Record<string,unknown[]>;
/** 🧭️ Resolves contracts only after normalized task references see the actual inferred own groups. */
export interface ProjectInputInferenceContextV1 extends Readonly<Record<string,unknown>>{declaredSourceInputs(project:Record<string,unknown>,workspaceRoot:string,namedInputs:Record<string,unknown[]>):Promise<Record<string,unknown[]>>}
/** 🧾️ Infers owned source groups from admitted first-party project declarations. */
export function inferProjectInputs(json:Record<string,unknown>,root:string,workspaceRoot:string,facts:Record<string,unknown>,scripts:unknown,context:ProjectInputInferenceContextV1):Promise<Record<string,unknown[]>>;
