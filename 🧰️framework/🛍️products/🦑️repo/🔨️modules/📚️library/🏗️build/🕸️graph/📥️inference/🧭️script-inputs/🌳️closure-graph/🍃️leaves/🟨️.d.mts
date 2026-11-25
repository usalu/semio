import type{ClosureGraphNodeV1}from"../🟨️.mjs";
/** 🏷️ The workspace name of an absolute path. */
export const workspaceNameV1:(workspaceRoot:string,path:string)=>string;
/** 📍️ The absolute path of a workspace name. */
export const workspacePathV1:(workspaceRoot:string,name:string)=>string;
/** 🔑️ The key of the leaf of one file. */
export const fileLeafKeyV1:(name:string)=>string;
/** 🔑️ The key of the node of one glob. */
export const globNodeKeyV1:(base:string,pattern:string)=>string;
/** 🪪️ The stamp of a path, undefined inside the racy window. */
export function pathStampV1(path:string,now?:number,racyMilliseconds?:number):string|undefined;
/** 🍃️ The leaf node of one file. */
export function fileLeafNodeV1(workspaceRoot:string,path:string):ClosureGraphNodeV1;
/** 🗂️ The files a glob matches below a base directory. */
export function globMatchesV1(workspaceRoot:string,base:string,pattern:string):{files:string[];directories:[string,string|null][]};
/** 🌐️ The node of one declared glob and its file leaves. */
export function globNodesV1(workspaceRoot:string,base:string,pattern:string):ClosureGraphNodeV1[];
/** 🪪️ Whether a stored leaf or glob node still holds by its stamps. */
export function leafStampHoldsV1(workspaceRoot:string,record:ClosureGraphNodeV1,memo?:Map<string,string>):boolean;
/** 🛰️ Whether a leaf made elsewhere holds by the local bytes. */
export function leafContentHoldsV1(workspaceRoot:string,record:ClosureGraphNodeV1):boolean;
