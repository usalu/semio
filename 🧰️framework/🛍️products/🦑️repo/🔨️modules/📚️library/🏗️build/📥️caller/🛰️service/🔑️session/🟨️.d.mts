export type CaveatV1={k:"scope";v:string[]}|{k:"projects";v:string[]}|{k:"notAfter";v:number}|{k:"cwd";v:string}|{k:"worktree";v:string}|{k:"agent";v:string};
export function canonicalJsonV1(value:unknown):string;
export function createRootKeyV1():Buffer;
export function keyDigestV1(key:Uint8Array):string;
export function mintCapabilityV1(key:Uint8Array,caveats:readonly CaveatV1[],id?:string):string;
export function attenuateCapabilityV1(token:string,caveat:CaveatV1):string;
export function verifyCapabilityV1(key:Uint8Array,token:string):CaveatV1[]|undefined;
export function capabilityIdV1(token:string):string|undefined;
export function authorizeCapabilityV1(caveats:readonly CaveatV1[],request:{commandClass:string;command?:string;cwd:string;now:number;projects:readonly string[];worktree?:string;agent?:string}):{code:"scope"|"capability";class?:string;reason?:string}|undefined;
export function canonicalRootV1(root:string,platform?:string):string;
export function worktreeIdV1(root:string):string;
export interface ResidentEndpointsV1{id:string;directory:string;endpoint:string;key:string;descriptor:string;state:string;lock:string;audit:string;revoked:string}
export function endpointsV1(root:string,environment?:Readonly<Record<string,string|undefined>>,platform?:string,user?:string):ResidentEndpointsV1;
export function stateDirectoryV1(root:string,environment?:Readonly<Record<string,string|undefined>>,platform?:string,user?:string,home?:string):string;
export function readServiceKeyV1(keyPath:string,platform?:string):Buffer;
export interface RevocationRegistryV1{has(session:string):boolean;hasAgent(agent:string):boolean;revoke(session:string):void;revokeAgent(agent:string):void;list():{sessions:string[];agents:string[]}}
export function createRevocationRegistryV1(path:string,now?:()=>number):RevocationRegistryV1;
export function appendAuditV1(path:string,row:object):void;
