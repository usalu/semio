import type {VizChartInference} from "@semio-tech/print-viz-inference";
import type {VizChartDiagnostic} from "@semio-tech/print-viz-inference/validation";
import document from "./🔣️.json" with { type: "json" };
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";

export type VizChartTextOutput = (Extract<VizChartInference,{complete:true}> & {readonly tikz:string}) | (Extract<VizChartInference,{complete:false}> & {readonly tikz:""});

/** 🧾️ Projects one owned inference result into its physical JSON value. */
export function vizChartTextOutputToJsonValue(value:VizChartTextOutput):VizChartTextOutput{return JSON.parse(JSON.stringify(value)) as VizChartTextOutput;}

/** 🪪️ Admits the physical text output contract alongside its typed semantic result. */
export function validateVizChartTextOutput(value:unknown):readonly VizChartDiagnostic[]{
 try{return validateJsonSchemaSubset(document,value).map(message=>({code:"print.chart.inference-schema",path:"inference",message}));}
 catch(error){return[{code:"print.chart.inference-schema",path:"inference",message:error instanceof Error?error.message:String(error)}];}
}
