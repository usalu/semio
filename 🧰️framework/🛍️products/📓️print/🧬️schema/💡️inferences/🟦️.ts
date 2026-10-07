/** 💡️ Semantic chart inference results, work controls and field dependencies. */
import type { VizChartSnapshot } from "../📸️snapshot/🟦️.ts";
import type { VizRenderPlan, renderVizScenePlan } from "./🖼️render/🟦️.ts";
import type { VizChartDiagnostic } from "./✅️validation/🟦️.ts";
import { admitVizChartSpecification } from "./✅️validation/🟦️.ts";
export { validateVizChartSpecification } from "./✅️validation/🟦️.ts";
export type VizChartInference = { readonly plan?: VizRenderPlan; readonly chart?: VizChartSnapshot; readonly scene?: ReturnType<typeof renderVizScenePlan>; readonly diagnostics: readonly VizChartDiagnostic[]; readonly complete: true } | { readonly plan?: never; readonly chart?: never; readonly scene?: never; readonly diagnostics: readonly VizChartDiagnostic[]; readonly complete: false };
export type VizChartInferenceProgress = { readonly completed: number; readonly total: number };
export type VizChartInferenceControl = { readonly signal?: AbortSignal; readonly onProgress?: (progress: VizChartInferenceProgress) => void };
export const VIZ_CHART_INFERENCE_FIELDS = [{ id: "framework.print.chart.inference.plan", reads: ["chart"] }, { id: "framework.print.chart.inference.chart", reads: ["chart"] }, { id: "framework.print.chart.inference.scene", reads: ["chart"] }] as const;

/** 🌱️ Derives owned semantic values with cancellable copy work and atomic publication. */
export async function inferVizChartSnapshot(snapshot:VizChartSnapshot,control:VizChartInferenceControl={}):Promise<VizChartInference>{
 const refuse=(message:string,code="print.chart.inference"):VizChartInference=>({complete:false,diagnostics:[{code,path:"chart",message}]});
 const checkpoint=(completed:number,total=0):void=>{if(control.signal?.aborted)throw Error("chart inference cancelled");control.onProgress?.({completed,total});if(control.signal?.aborted)throw Error("chart inference cancelled");};
 try{
  checkpoint(0);const admitted=admitVizChartSpecification(snapshot.chart);if(!admitted.chart)return{complete:false,diagnostics:admitted.diagnostics};
  const holder:Record<string,unknown>={},stack:{value:unknown;owner:Record<string,unknown>|unknown[];key:string|number;depth:number}[]=[{value:snapshot,owner:holder,key:"chart",depth:0}];let completed=0;
  while(stack.length){
   const {value,owner,key,depth}=stack.pop()!;if(depth>128||completed>=1_000_000)throw Error("chart inference value admission exceeded");
   let copied:unknown=value;
   if(value!==null&&typeof value==="object"){
    copied=Array.isArray(value)?[]:Object.create(null);const entries=Array.isArray(value)?value.map((entry,index)=>[index,entry] as const):Object.entries(value);
    for(let at=entries.length-1;at>=0;at--){const [key,entry]=entries[at]!;stack.push({value:entry,owner:copied as Record<string,unknown>,key,depth:depth+1});}
   }
   Object.defineProperty(owner,key,{value:copied,writable:true,enumerable:true,configurable:true});completed++;
   if(completed%256===0){checkpoint(completed);await new Promise<void>(resolve=>setTimeout(resolve,0));}
  }
  checkpoint(completed,completed);return{chart:holder.chart as VizChartSnapshot,complete:true,diagnostics:[]};
 }catch(error){return refuse(error instanceof Error?error.message:String(error),control.signal?.aborted?"print.chart.cancelled":"print.chart.inference");}
}
