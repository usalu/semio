import type {FactIndexV1,GenerationManifestV1,GenerationPlanV1,GenerationSealedV1,MembershipChangeV1} from "../🟨️.mjs";
/** 🚧️ The withheld owners of a generation that do not hold the source that withholds them. */
export function unfoundedWithheldOwnersV1(index:Pick<FactIndexV1,"owners"|"rows"|"classes">,owners:Readonly<Record<string,{kind:string;tag?:string;config?:string}>>,separator?:string):string[];
/** 🧭️ What the plan reads: the fact index view, the fragment names, the clock, the doubt, cutoff settler and derive and hold ports of the generation store. */
export interface GenerationPlanOptionsV1{readIndexView(digest:string):Promise<FactIndexV1|undefined>;fragmentNames():Promise<Iterable<string>>;now():number;doubt?:unknown;settler:unknown;derive:unknown;hold:unknown;workspaceRoot:string|undefined;ledger?:unknown;say?:(text:{readonly en:string;readonly de:string})=>unknown;reasonOf?:(name:string,row:never)=>Promise<unknown>}
/** 📋️ One plan request: the current generation, the identity, the membership changes and the scope. */
export interface GenerationPlanRequestV1{current:GenerationSealedV1|undefined;identity:string;implementation?:string;changes?:MembershipChangeV1[];journalDirty?:unknown;only?:ReadonlySet<string>;repairBase?:GenerationManifestV1;forcedOwners?:readonly string[];verifyAll?:boolean;skipEnvironment?:boolean;signal?:AbortSignal;progress?:unknown}
/** 🧭️ Verifies selected owners' facts and maps membership changes to their roots. */
export function createGenerationPlanV1(options:GenerationPlanOptionsV1):(request:GenerationPlanRequestV1)=>Promise<GenerationPlanV1>;
