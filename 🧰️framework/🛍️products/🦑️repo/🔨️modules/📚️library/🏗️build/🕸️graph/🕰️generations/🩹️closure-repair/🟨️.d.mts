import type{GenerationFilePortV1,GenerationManifestV1,GenerationPointerV1}from"../🟨️.mjs";
import type{ClosureNamespaceV1}from"../../🩹️closure-repair/🟨️.mjs";

export type ClosureRepairReferenceV1=Readonly<{namespace:ClosureNamespaceV1;key:string}>;
export type ClosureRepairVerifierV1=(reference:ClosureRepairReferenceV1,signal?:AbortSignal)=>Promise<boolean|undefined>;
export type ClosureRepairProgressV1=(done:number,total:number)=>void|Promise<void>;
export type ClosureRepairV1=Readonly<{
 generation:number;parent:string;receipt:string;owners:readonly string[];missing:readonly ClosureRepairReferenceV1[];
 pointer:GenerationPointerV1;manifest:GenerationManifestV1;
}>;
type StoreV1=Readonly<{root:string;fs:Pick<GenerationFilePortV1,"read">;signal?:AbortSignal;progress?:ClosureRepairProgressV1}>;
type LookupV1=StoreV1&Readonly<{pointer:GenerationPointerV1;identity:string;implementation:string}>;
export const CLOSURE_REPAIR_POLICY_V1:Readonly<{directory:string;reason:string;maximumPointerBytes:number;maximumReceiptBytes:number;maximumManifestBytes:number;maximumFragments:number;maximumFragmentBytes:number;maximumPackedBytes:number;maximumExpandedBytes:number;maximumReferences:number;maximumLinks:number;maximumMissing:number;maximumAffectedOwners:number}>;
export function isClosureRepairReceiptNameV1(name:string):boolean;
export function retireClosureRepairReceiptsV1(options:Readonly<{
 root:string;fs:Pick<GenerationFilePortV1,"read"|"list"|"remove">;pointer:GenerationPointerV1;cutoff:number;signal?:AbortSignal;fence?:(body:()=>Promise<boolean>)=>Promise<boolean>;
}>):Promise<void>;
export function prepareClosureRepairV1(options:StoreV1&Readonly<{
 fs:Pick<GenerationFilePortV1,"read"|"write">;expected:GenerationPointerV1;identity:string;implementation:string;missing:readonly ClosureRepairReferenceV1[];verifyClosure?:ClosureRepairVerifierV1;
}>):Promise<ClosureRepairV1|undefined>;
export function readClosureRepairV1(options:LookupV1):Promise<ClosureRepairV1|undefined>;
export function recoverClosureRepairV1(options:LookupV1&Readonly<{verifyClosure:ClosureRepairVerifierV1}>):Promise<ClosureRepairV1|undefined>;
export function verifyClosureRepairCandidateV1(options:StoreV1&Readonly<{manifest:GenerationManifestV1;verifyClosure:ClosureRepairVerifierV1}>):Promise<boolean>;
