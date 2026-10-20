import type {GenerationComputeResultV1,GenerationComputeSpecV1,GenerationGraphPolicyV1,GenerationMembershipV1,GenerationRowV1,GenerationStoreOptionsV1,FrontierV1,GenerationManifestV1} from "../🟨️.mjs";
export declare const GENERATION_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<"current"|"verified"|"waiting"|"building"|"published"|"closure"|"cold",string>>>>;
export function selectGenerationLanguageV1(environment:Readonly<Record<string,string|undefined>>):"en"|"de";
export function renderGenerationMessageV1(key:keyof typeof GENERATION_MESSAGES_V1["en"],fields:Readonly<Record<string,string|number>>,environment:Readonly<Record<string,string|undefined>>):string;
export function extractGraphPolicyV1(args:readonly string[],environment:Readonly<Record<string,string|undefined>>):{policy:GenerationGraphPolicyV1;explicit:boolean;args:string[]};
/** 🧩️ What one partial inference produced, in repository terms. */
export interface GenerationPartV1{owners:{name:string;root:string;nativeRoot?:string;config?:string;tag?:string;node?:unknown;unavailable?:unknown;nodeRows?:GenerationRowV1[]}[];edgeOnly:{name:string}[];edges:{source:string;target:string;[key:string]:unknown}[];externalNodes?:Record<string,unknown>;parseUnavailable?:unknown[];rows:GenerationRowV1[];env:{digest:string;rows:GenerationRowV1[]};replaceTags?:string[]}
export function shapeComputeResultV1(input:{workspaceRoot:string;part:GenerationPartV1;stubs:GenerationComputeSpecV1["stubs"];separator?:string}):GenerationComputeResultV1;
export function createGenerationProgressV1(clock:{begin(phase:string,total:number):unknown;advance(phase:string,done:number):unknown;end(phase:string):unknown}):(event:{phase:string;done:number;total:number})=>Promise<void>;
export interface GenerationHostV1{
 workspaceRoot:string;separator?:string;namedInputs:unknown;registryDigest:string;signal?:AbortSignal;verbose?:boolean;
 identity():Promise<string>;
 withShared<T>(body:(store:unknown,control:unknown)=>Promise<T>):Promise<T>;
 withBuilder<T>(body:(store:unknown,control:unknown)=>Promise<T>):Promise<T>;
 storeOptions(store:unknown,control:unknown):GenerationStoreOptionsV1;
 membership(previous:{manifest:GenerationManifestV1;frontier:FrontierV1}|undefined,signal?:AbortSignal):Promise<GenerationMembershipV1>;
 discoverPart(spec:GenerationComputeSpecV1,signal:AbortSignal|undefined,control:unknown):Promise<GenerationPartV1>;
 expand(projects:Record<string,unknown>,control:unknown):Promise<Record<string,unknown>>;
 readGraph(value:unknown):unknown;
 clock(control:unknown):Parameters<typeof createGenerationProgressV1>[0];
 say(key:keyof typeof GENERATION_MESSAGES_V1["en"],fields:Readonly<Record<string,string|number>>):void;
}
export function loadGenerationGraphV1(host:GenerationHostV1,options?:{policy?:GenerationGraphPolicyV1;closure?:readonly string[]}):Promise<unknown>;
