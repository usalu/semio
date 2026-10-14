import type{CacheUnit}from"../🟦️.ts";
import schema from"./🧬️schema/🔣️.json";
import{validateJsonSchemaSubset}from"../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
export type CachePressurePolicyV1=Readonly<{version:1;minimumAvailableBytes:number;targetAvailableBytes:number;guardAgeMs:number;maximumDeletionBytes:number}>;
export type CachePressurePlanV1=Readonly<{pressured:boolean;units:readonly CacheUnit[];plannedBytes:number;targetUnmetBytes:number}>;
/** 📉️ Selects bounded oldest inactive slices; actual free-space recovery is verified by the executor. */
export function planCachePressurePruneV1(units:readonly CacheUnit[],nowMs:number,availableBytes:number,policy:CachePressurePolicyV1,originalAvailableBytes:number):CachePressurePlanV1{
 if(validateJsonSchemaSubset(schema.definitions.Policy,policy).length||Object.values(policy).some(value=>!Number.isSafeInteger(value))||policy.targetAvailableBytes<policy.minimumAvailableBytes||!Number.isFinite(nowMs)||nowMs<0||!Number.isSafeInteger(availableBytes)||availableBytes<0||!Number.isSafeInteger(originalAvailableBytes)||originalAvailableBytes<0)throw Error("Invalid finite cache pressure policy or observation");
 if(units.some(unit=>typeof unit.path!=="string"||!unit.path.length||!Number.isSafeInteger(unit.bytes)||unit.bytes<0||!Number.isFinite(unit.recencyMs)||unit.recencyMs<0||typeof unit.lockHeld!=="boolean")||new Set(units.map(unit=>unit.path)).size!==units.length)throw Error("Invalid cache pressure units");
 const pressured=originalAvailableBytes<policy.minimumAvailableBytes&&availableBytes<policy.targetAvailableBytes,required=pressured?Math.max(0,policy.targetAvailableBytes-availableBytes):0,selected:CacheUnit[]=[];let plannedBytes=0;
 for(const unit of[...units].sort((a,b)=>a.recencyMs-b.recencyMs||(a.path<b.path?-1:a.path>b.path?1:0))){if(plannedBytes>=required)break;if(unit.lockHeld||unit.recencyMs>nowMs-policy.guardAgeMs||unit.bytes>policy.maximumDeletionBytes-plannedBytes)continue;selected.push(unit);plannedBytes+=unit.bytes;}
 return{pressured,units:selected,plannedBytes,targetUnmetBytes:Math.max(0,required-plannedBytes)};
}
