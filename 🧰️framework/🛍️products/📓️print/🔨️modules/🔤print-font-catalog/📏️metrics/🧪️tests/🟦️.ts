/** 🧪️ Verifies source-bound font schema, shaping and advances using independent native Canvas. */
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {createCanvas,GlobalFonts} from "@napi-rs/canvas";
import Ajv2020 from "ajv/dist/2020.js";
import {measurePrintSans,printSansGlyphs,printSansMetrics} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import vectors from "./🔣️.json";
export type FontMetricsCheck={name:string;subject:()=>unknown;oracle:()=>unknown;tolerance?:number};
const fontPath=join(import.meta.dir,"../../../../🖼️assets/🔤️font/🅰️anta/🅰️Anta-Regular.ttf");
let context:ReturnType<ReturnType<typeof createCanvas>["getContext"]>|undefined;
function canvasWidth(text:string,size:number):number{
  if(context===undefined){if(!GlobalFonts.registerFromPath(fontPath,"SemioSansMetricsOracle"))throw new Error("tracked Anta font registration failed");context=createCanvas(1,1).getContext("2d");}
  context.font="2048px SemioSansMetricsOracle";
  return context.measureText(text).width/2048*size*25.4/72.27;
}
/** 📏️ Provides language-neutral advance vectors and schema/digest assertions. */
export function printFontMetricsChecks():FontMetricsCheck[]{
  const metrics=printSansMetrics();
  const checks:FontMetricsCheck[]=[
    {name:"font-metrics/source-digest",subject:()=>metrics.sha256,oracle:()=>createHash("sha256").update(readFileSync(fontPath)).digest("hex")},
    {name:"font-metrics/schema",subject:()=>new Ajv2020({strict:false}).compile(schema)(metrics),oracle:()=>true},
    {name:"font-metrics/ligature",subject:()=>printSansGlyphs("fi").length,oracle:()=>1},
    ...vectors.sizes.flatMap(size=>vectors.cases.map(entry=>({name:"font-metrics/"+entry.language+"/"+entry.text+"/"+size,subject:()=>[measurePrintSans(entry.text,size)],oracle:()=>[canvasWidth(entry.text,size)],tolerance:1e-4})))
  ];
  return checks;
}
