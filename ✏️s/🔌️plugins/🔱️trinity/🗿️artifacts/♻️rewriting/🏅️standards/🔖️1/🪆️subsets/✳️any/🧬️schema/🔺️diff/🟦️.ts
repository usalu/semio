/** 🔺️ Typed document replacement and shared map algebra preserve the complete rule domain. */
import {parseMapDelta,type MapDelta} from "./../../../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🎮️mutation/🗂️map/🧬️schema/🟦️.ts";
import {parseLhs,parseRhs,parseLayoutPoint,parsePropertyValue,type Lhs,type Rhs,type JackSnapshot,type LayoutPoint,type PropertyValue} from "../🟦️.ts";
import {parseJackSnapshot} from "./../../../../../../../🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export type{MapDelta,LayoutPoint,PropertyValue,Lhs,Rhs,JackSnapshot};
export interface RewritingDiff{workingGraph?:JackSnapshot|null;lhs?:Lhs|null;rhs?:Rhs|null;parameterBindings?:MapDelta<PropertyValue>|null;ruleLayout?:MapDelta<LayoutPoint>|null}
/** 🪪️ Each sparse replacement delegates to its actual domain owner. */
export function parseRewritingDiff(value:unknown):RewritingDiff{if(!value||typeof value!=="object"||Array.isArray(value))throw Error("rewriting diff object required");const row=value as Record<string,unknown>;if(Object.keys(row).some(key=>!["workingGraph","lhs","rhs","parameterBindings","ruleLayout"].includes(key)))throw Error("rewriting diff has incorrect fields");return{workingGraph:row.workingGraph==null?null:parseJackSnapshot(row.workingGraph),lhs:row.lhs==null?null:parseLhs(row.lhs),rhs:row.rhs==null?null:parseRhs(row.rhs),parameterBindings:row.parameterBindings==null?null:parseMapDelta(row.parameterBindings,parsePropertyValue),ruleLayout:row.ruleLayout==null?null:parseMapDelta(row.ruleLayout,parseLayoutPoint)};}
