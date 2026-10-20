import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
/** 🧾️ Observed physical facts do not grant ownership to quarantined material. */
export type HistoryOutcome="inspection"|"not-applied"|"committed-observed"|"quarantined"|"rollover-required"|"selection-required"|"segment-selected"|"cancelled"|"settled"|"history-full"|"conflict-retained";
/** 🔬️ The physical boundary supplies fresh instance facts; structural settlement grants no content authority. */
export type HistoryObservation=Readonly<{request:"inspect"|"reconcile"|"rollover";namespace:"continuous"|"rebound"|"foreign";intent:"valid"|"missing"|"malformed";source:"before"|"after"|"absent"|"foreign";object:"absent"|"expected"|"foreign"|"unknown";outcome:"complete"|"missing"|"partial";segment:"open"|"sealed"|"publishing";selection:"valid"|"missing"|"fork";names:number;bytes:number;segments:number;cancel:"none"|"before"|"after"}>;
/** 🧭️ Plans a bounded maintenance transition without publishing a synthetic command outcome. */
export function classifyHistoryObservation(value:unknown):HistoryOutcome{
 if(validateJsonSchemaSubset(schema.$defs.Observation,value).length)throw Error("invalid-history-observation");const facts=value as HistoryObservation,policy=schema.properties.policy.const;
 if(facts.cancel==="before")return "cancelled";
 if(facts.namespace==="foreign"||facts.intent!=="valid"||facts.selection==="fork")return "quarantined";
 if(facts.request==="rollover"){if(facts.segments>=policy.maximumSegments)return "history-full";return facts.selection==="missing"?"selection-required":"segment-selected";}
 if(facts.request==="inspect")return facts.names>=policy.maximumSegmentNames-policy.reservedMaintenanceNames||facts.bytes>=policy.maximumSegmentBytes-policy.reservedMaintenanceBytes?"rollover-required":"inspection";
 if(facts.object==="foreign"||facts.object==="unknown")return "quarantined";
 if(facts.outcome==="complete")return "settled";
 if(facts.object==="expected"&&(facts.source==="absent"||facts.source==="after"))return "committed-observed";
 if(facts.object==="expected"&&facts.source==="foreign")return "conflict-retained";
 return facts.object==="absent"&&facts.source==="before"?"not-applied":"quarantined";
}
