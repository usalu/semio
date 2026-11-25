import type{ClosureGraphNodeV1}from"../🟨️.mjs";
/** 🔑️ The key of the node of one crate. */
export const cargoCrateKeyV1:(name:string,tests:boolean)=>string;
/** 🔑️ The key of the node of the Rust sources of one crate. */
export const cargoSourcesKeyV1:(name:string,tests:boolean)=>string;
/** 🦀️ The nodes of one crate and of every crate it depends on by path. */
export function cargoCrateNodesV1(workspaceRoot:string,directory:string,options?:{tests?:boolean;held?:(key:string)=>ClosureGraphNodeV1|undefined;nodes?:Map<string,ClosureGraphNodeV1>;facts?:Map<string,unknown>}):{root:string;nodes:Map<string,ClosureGraphNodeV1>};
/** 🪪️ Whether a stored crate, sources or leaf node still holds by its stamps. */
export function cargoNodeStampHoldsV1(workspaceRoot:string,record:ClosureGraphNodeV1,memo?:Map<string,string>):boolean;
/** 🦀️ The root key of the closure question of an owner that has only a `Cargo.toml`. */
export const cargoOwnerRootV1:(workspaceRoot:string,directory:string)=>string;
