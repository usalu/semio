import schema from "./🔣️.json" with {type: "json"};

/** 🗄️ One declared loose-file area (a directory of independent cache files) whose bytes count against the host cap; its own cap and age bound the area itself. */
export type CasAreaPolicyV1=Readonly<{id:string;capBytes:number|null;maxAgeMs:number|null}>;
/** 🗄️ The size cap, watermarks, grace times and cadences of the host-wide store (single source: the sibling schema document, definition `Policy`). */
export type CasPolicyV1=Readonly<{version:1;capBytes:number;highWatermark:number;lowWatermark:number;minAgeMs:number;leaseMaxMs:number;touchGranularityMs:number;temporaryMaxAgeMs:number;ledgerQuietMs:number;lockStaleMs:number;progressMs:number;checkEvery:number;fanOutHexLength:number;areas:readonly CasAreaPolicyV1[]}>;
/** 🗣️ The EN+DE texts of the store; the schema document is their single source. */
export const CAS_MESSAGES_V1=Object.freeze(schema.definitions.Messages.const);
/** 🗄️ The default policy: 120 GiB of the 200 GiB disk budget. */
export const CAS_POLICY_V1:CasPolicyV1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Policy.const)) as CasPolicyV1);

/** 🧮️ Fills a message template of the schema with values. */
export function casTextV1(key:keyof typeof CAS_MESSAGES_V1,language:"en"|"de",values:Readonly<Record<string,string|number>>={}):string{
 return CAS_MESSAGES_V1[key][language].replace(/\{(\w+)\}/gu,(_all,name:string)=>String(values[name]??""));
}

/** ✅️ Rejects a policy whose numbers cannot work together (watermarks out of order, non-finite bounds). */
export function requireCasPolicyV1(policy:CasPolicyV1):CasPolicyV1{
 const finite=(value:number):boolean=>Number.isFinite(value)&&value>=0;
 if(!(policy.capBytes>0)||!(policy.lowWatermark>0)||!(policy.lowWatermark<policy.highWatermark)||!(policy.highWatermark<=1))throw Error("Valid cache store watermarks required / Gültige Cache-Speicher-Marken erforderlich");
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
