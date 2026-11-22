import schema from "./🔣️.json" with {type: "json"};
import family from "../../📏️workspace-scale/🧬️schema/🔣️.json" with {type: "json"};

/** 🗄️ One declared loose-file area (a directory of independent cache files) whose bytes count against the host cap; its own cap and age bound the area itself. */
export type CasAreaPolicyV1=Readonly<{id:string;capBytes:number|null;maxAgeMs:number|null}>;
/** 🌡️ The used shares of the partition that holds the store: a collection starts at `highUsedFraction` and frees least-recently-accessed objects down to `lowUsedFraction`; at `criticalUsedFraction` a write or a build that would add to it first waits for such a collection and is refused when that frees too little. */
export type CasPartitionPolicyV1=Readonly<{highUsedFraction:number;lowUsedFraction:number;criticalUsedFraction:number}>;
/** 🗄️ The size cap (the upper bound; the cap in force is `casEffectiveCapBytesV1`: at most `capHostSharePercent` of the partition and at most what the host can give while it keeps its free floor, never below `minimumCapBytes`), watermarks, grace times and cadences of the host-wide store (single source: the sibling schema document, definition `Policy`). */
export type CasPolicyV1=Readonly<{version:1;capBytes:number;capHostSharePercent:number;minimumCapBytes:number;enforceTickMilliseconds:number;highWatermark:number;lowWatermark:number;partition:CasPartitionPolicyV1;minAgeMs:number;leaseMaxMs:number;touchGranularityMs:number;temporaryMaxAgeMs:number;ledgerQuietMs:number;lockStaleMs:number;progressMs:number;checkEvery:number;fanOutHexLength:number;areas:readonly CasAreaPolicyV1[]}>;
/** 🗣️ The EN+DE texts of the store; the schema document is their single source. */
export const CAS_MESSAGES_V1=Object.freeze(schema.definitions.Messages.const);
/** 🗄️ The default policy: a 32 GiB object cap, 90 % high and 75 % low watermark. */
export const CAS_POLICY_V1:CasPolicyV1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Policy.const)) as CasPolicyV1);
/** 🧾️ The accounting area whose source inventories are retired by graph custody. */
export const CAS_GRAPH_CLOSURE_AREA_V1=schema.definitions.GraphClosureArea.const;

/** 🧮️ Fills a message template of the schema with values. */
export function casTextV1(key:keyof typeof CAS_MESSAGES_V1,language:"en"|"de",values:Readonly<Record<string,string|number>>={}):string{
 return CAS_MESSAGES_V1[key][language].replace(/\{(\w+)\}/gu,(_all,name:string)=>String(values[name]??""));
}

/** ✅️ Rejects a policy whose numbers cannot work together (watermarks out of order, non-finite bounds). */
export function requireCasPolicyV1(policy:CasPolicyV1):CasPolicyV1{
 const finite=(value:number):boolean=>Number.isFinite(value)&&value>=0;
 if(!(policy.capBytes>0)||!(policy.capHostSharePercent>0&&policy.capHostSharePercent<=100)||!(policy.minimumCapBytes>=0)||!(policy.enforceTickMilliseconds>0)||!(policy.lowWatermark>0)||!(policy.lowWatermark<policy.highWatermark)||!(policy.highWatermark<=1))throw Error("Valid cache store watermarks required / Gültige Cache-Speicher-Marken erforderlich");
 const {lowUsedFraction:low,highUsedFraction:high,criticalUsedFraction:critical}=policy.partition;
 if(!(low>0)||!(low<high)||!(high<critical)||!(critical<=1))throw Error("Valid partition watermarks required / Gültige Partitions-Marken erforderlich");
 for(const value of [policy.minAgeMs,policy.leaseMaxMs,policy.touchGranularityMs,policy.temporaryMaxAgeMs,policy.ledgerQuietMs,policy.lockStaleMs,policy.progressMs])if(!finite(value))throw Error("Finite cache store bounds required / Endliche Cache-Speicher-Grenzen erforderlich");
 if(!Number.isInteger(policy.checkEvery)||policy.checkEvery<1||!Number.isInteger(policy.fanOutHexLength)||policy.fanOutHexLength<1||policy.fanOutHexLength>3)throw Error("Valid cache store cadence required / Gültiger Cache-Speicher-Takt erforderlich");
 return policy;
}

/** 🧲️ One place whose finished files may join the shared store: a directory below the workspace root. */
export type CasPlaceV1=Readonly<{id:string;path:string;mode:"hardlink"|"reflink";reason:string}>;
/** 🧲️ The adoption rules: files from `minBytes` on that nobody wrote for `quietMs`, not matching `skipPattern`, in the declared places share one host object. A place that is immutable by construction (`hardlink`) gets hard links with write permission cleared, so an in-place writer fails instead of corrupting every sharer; every other place (`reflink`) only gets copy-on-write clones, which no write can leak through, and is skipped on a volume that cannot clone. */
export type CasAdoptionPolicyV1=Readonly<{minBytes:number;quietMs:number;clearWriteBits:boolean;skipPattern:string;places:readonly CasPlaceV1[]}>;
/** 🧲️ The default adoption rules of the schema. */
export const CAS_ADOPTION_V1:CasAdoptionPolicyV1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Adoption.const)) as CasAdoptionPolicyV1);

/** 📐️ The declared disk budget of the partition that holds the store: the largest need of any stage in bytes, the divisor that makes the cushion of it and the factor of the partition size from which the budget is derived (single source: the sibling schema document, definition `Budget`). */
export type CasBudgetV1=Readonly<{version:1;largestNeedBytes:number;cushionDivisor:number;minimumTotalFactor:number}>;
/** 📐️ The default disk budget of the schema. */
export const CAS_BUDGET_V1:CasBudgetV1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Budget.const)) as CasBudgetV1);
/** 🛡️ The safety factor of the workspace scale family, the factor of the reserve (one number for every ceiling that must hold the workspace). @see ../../📏️workspace-scale/🧬️schema/🔣️.json */
export const CAS_RESERVE_FACTOR_V1:number=family.definitions.Family.const.safetyFactor;
