import type {EdgeReadsV1,EnvironmentPartsV1} from "../🧵️edge-reads/🟨️.mjs";
export interface ComponentRowV1{readonly root:string;readonly nativeRoot?:string|null;readonly reads?:EdgeReadsV1;readonly edgesReads?:string}
export interface EnvironmentIndexV1{readonly rows:ReadonlyMap<string,readonly [string,string,string|null]>;digest(key:string):string;readonly tableDigest:string}
/** 🚩️ Whether owner edges are judged by components. */
export declare function componentEdgeValidityOnV1(policy:{readonly componentEdgeValidity?:boolean}|undefined,environment?:Readonly<Record<string,string|undefined>>):boolean;
/** 📇️ The component index of environment parts. */
export declare function indexEnvironmentV1(parts:EnvironmentPartsV1&{readonly lock:string|null;readonly taxonomy:string|null}):EnvironmentIndexV1;
/** 🔑️ The sorted component keys of an owner. */
export declare function recordKeysV1(row:{readonly name:string;readonly root:string;readonly nativeRoot?:string|null},reads:EdgeReadsV1|undefined,index:EnvironmentIndexV1):string[];
/** 🧾️ The digest of the components of `keys`. */
export declare function readsDigestV1(keys:readonly string[],index:EnvironmentIndexV1):string;
/** 🧩️ The judge of owner rows under one environment. */
export declare function createComponentValidityV1(parts:EnvironmentPartsV1&{readonly lock:string|null;readonly taxonomy:string|null}):Readonly<{digestOf(name:string,row:ComponentRowV1):string;stamp(name:string,row:ComponentRowV1):{edgesReads:string};holds(name:string,row:ComponentRowV1):boolean;tableDigest:string}>;
/** 🚦️ The judge under the policy flag, undefined when it is off. */
export declare function componentEdgesV1(policy:{readonly componentEdgeValidity?:boolean}|undefined,environment:Readonly<Record<string,string|undefined>>,parts:EnvironmentPartsV1|undefined):ReturnType<typeof createComponentValidityV1>|undefined;
