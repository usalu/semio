/** 🔒️ Publishes a compact Cargo lock inventory under original graph writer custody. */
export function cargoLockInput(manifest:string,workspaceRoot:string):{env:string};
/** 🧵️ Matches portable source globs across admitted native backends. */
export function buildInputPathMatchesV1(path:string,pattern:string):boolean;
/** 🧮️ Publishes shared exact-source inventories under original graph writer custody. */
export function compactSourceInputs(inputs:unknown[],workspaceRoot:string,root?:string):unknown[];
/** 🧮️ Interns equivalent source coordinates before native hashing. */
export function deduplicateProjectInputs(inputs:unknown[],root:string):unknown[];
/** 🧬️ Checks whether authored output negations overlap source scopes. */
export function sourceOutputOverlaps(output:string,inputs:unknown[],root:string):boolean;
/** 🗂️ Shares equivalent arrays within one project input declaration. */
export function internProjectInputGroups(groups:Record<string,unknown[]>):Record<string,unknown[]>;
/** 🧾️ Infers owned source groups from admitted first-party project declarations. */
export function inferProjectInputs(json:Record<string,unknown>,root:string,workspaceRoot:string,facts:Record<string,unknown>,scripts:unknown,context:Record<string,unknown>):Promise<Record<string,unknown[]>>;
