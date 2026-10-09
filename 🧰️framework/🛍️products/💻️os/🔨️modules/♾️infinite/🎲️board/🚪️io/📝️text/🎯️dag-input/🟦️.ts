import {decodeJsonSyntax,type JsonSyntaxNode,type JsonSyntaxControl,type JsonSyntaxMember} from "../../../../../../../../🔨️modules/🎒️pack/🔤️json/📥️decode/🟦️.ts";
import {JsonMemberPolicy} from "../../../../../../../../🔨️modules/🎒️pack/🔤️json/🧩️members/🟦️.ts";
import {NativeDecodeControl} from "../../../../../../../../🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {ValueError} from "../../../../../../../../🔨️modules/🌱️value/⚠️refusal/🟦️.ts";
import type {DagSelectionDomains,DagChannelRef,DagNodeEvaluationStatus,DagNodeStatuses,DagHoverFacts,DagWireTypeRefusal} from "../../../🧬️schema/🎯️dag-input/🟦️.ts";

/** 🎛️ Carries finite raw syntax limits and cumulative logical typed-slot ownership admission. */
export interface DagTextDecodeControl {readonly syntax:JsonSyntaxControl;readonly ownership:NativeDecodeControl}
function refuse():never{throw new ValueError("invalidValue","invalid DAG input facts")}
function object(value:JsonSyntaxNode,keys?:readonly string[]):readonly JsonSyntaxMember[]{if(value.kind!=="object"||keys&&(value.members.length!==keys.length||value.members.some(row=>!keys.includes(row.name))))refuse();return value.members}
function member(rows:readonly JsonSyntaxMember[],name:string):JsonSyntaxNode{return rows.find(row=>row.name===name)?.value??refuse()}
async function text(value:JsonSyntaxNode,control:NativeDecodeControl,nonempty=false):Promise<string>{
 if(value.kind!=="string"||nonempty&&value.value.length===0)refuse();
 for(let index=0;index<value.value.length;index++){await control.step();const unit=value.value.charCodeAt(index);if(unit>=0xd800&&unit<=0xdbff){const next=value.value.charCodeAt(++index);if(!(next>=0xdc00&&next<=0xdfff))refuse();}else if(unit>=0xdc00&&unit<=0xdfff)refuse();}
 return value.value;
}
async function identifiers(value:JsonSyntaxNode,control:NativeDecodeControl):Promise<readonly string[]>{if(value.kind!=="array")refuse();await control.admitSlots(value.items.length,8);const output:string[]=[];for(const entry of value.items){await control.step();output.push(await text(entry,control,true));}return output}
async function read(source:string,control:DagTextDecodeControl):Promise<JsonSyntaxNode>{await control.ownership.charge(32);const value=await decodeJsonSyntax(source,JsonMemberPolicy.Reject,control.syntax);await control.ownership.checkpoint();return value}
async function channel(value:JsonSyntaxNode,control:NativeDecodeControl):Promise<DagChannelRef>{const rows=object(value,["widgetId","port","direction"]);await control.admitSlots(3,16);const widgetId=await text(member(rows,"widgetId"),control,true),port=await text(member(rows,"port"),control,true),direction=await text(member(rows,"direction"),control);if(direction!=="in"&&direction!=="out")refuse();return {widgetId,port,direction}}

/** 📥️ Admits the canonical selection object before semantic host publication. */
export async function decodeDagSelectionJson(source:string,control:DagTextDecodeControl):Promise<DagSelectionDomains>{
 const rows=object(await read(source,control),["nodes","edges","handles"]);await control.ownership.admitSlots(3,16);
 const nodes=await identifiers(member(rows,"nodes"),control.ownership),edges=await identifiers(member(rows,"edges"),control.ownership),handles=await identifiers(member(rows,"handles"),control.ownership);
 await control.ownership.checkpoint();return {nodes,edges,handles};
}

/** 🔌️ Admits widget-port directions independently of graph selection mutation. */
export async function decodeDagChannelsJson(source:string,control:DagTextDecodeControl):Promise<readonly DagChannelRef[]>{
 const value=await read(source,control);if(value.kind!=="array")refuse();await control.ownership.admitSlots(value.items.length,8);const output:DagChannelRef[]=[];
 for(const entry of value.items){await control.ownership.step();output.push(await channel(entry,control.ownership));}
 await control.ownership.checkpoint();return output;
}

/** 🖱️ Admits the complete hover/refusal record with finite syntax and typed-slot authority. */
export async function decodeDagHoverJson(source:string,control:DagTextDecodeControl):Promise<DagHoverFacts>{
 const rows=object(await read(source,control),["channel","refusal"]);await control.ownership.admitSlots(2,16);const hovered=member(rows,"channel"),refused=member(rows,"refusal");const selected=hovered.kind==="null"?null:await channel(hovered,control.ownership);let refusal:DagWireTypeRefusal|null=null;
 if(refused.kind!=="null"){const fields=object(refused,["source","sourceTypes","target","targetTypes"]);await control.ownership.admitSlots(4,16);refusal={source:await text(member(fields,"source"),control.ownership,true),sourceTypes:await identifiers(member(fields,"sourceTypes"),control.ownership),target:await text(member(fields,"target"),control.ownership,true),targetTypes:await identifiers(member(fields,"targetTypes"),control.ownership)};}
 await control.ownership.checkpoint();return {channel:selected,refusal};
}

/** 🚦️ Admits every evaluation variant before replacing accepted host status facts. */
export async function decodeDagNodeStatusesJson(source:string,control:DagTextDecodeControl):Promise<DagNodeStatuses>{
 const entries=object(await read(source,control));await control.ownership.admitSlots(entries.length,16);const output:Record<string,DagNodeEvaluationStatus>=Object.create(null);
 for(const entry of entries){await control.ownership.step();await text({kind:"string",value:entry.name},control.ownership,true);const rows=object(entry.value),status=await text(member(rows,"status"),control.ownership);await control.ownership.charge(32);
 if(status==="ok"||status==="queued"||status==="computing"){object(entry.value,["status"]);output[entry.name]={status};}
 else if(status==="error"){object(entry.value,["status","message"]);output[entry.name]={status,message:await text(member(rows,"message"),control.ownership)};}
 else if(status==="blocked"){object(entry.value,["status","ports"]);output[entry.name]={status,ports:await identifiers(member(rows,"ports"),control.ownership)};}
 else refuse();}
 await control.ownership.checkpoint();return output;
}

/** 📊️ Admits complete computing facts through the same closed controlled physical boundary. */
export async function decodeDagComputingProgressJson(source:string,control:DagTextDecodeControl):Promise<import("../../../🧬️schema/🎯️dag-input/📊️progress/🟦️.ts").DagComputingProgress>{
 const rows=object(await read(source,control),["active","stale"]);await control.ownership.admitSlots(2,16);const value=member(rows,"active");const active=value.kind==="null"?null:await text(value,control.ownership,true),stale=await identifiers(member(rows,"stale"),control.ownership);await control.ownership.checkpoint();return {active,stale};
}
