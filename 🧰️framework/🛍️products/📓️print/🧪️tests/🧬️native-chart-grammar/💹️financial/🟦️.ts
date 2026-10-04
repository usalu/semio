/** 💹️ Compiles financial overlays and checks their trailing population bands with independent D3. */
import { join } from "node:path";
import { scaleLinear } from "d3-scale";
import { mean, variance } from "d3-array";
import { compileVizProbeDocument } from "../../../🔨️modules/🧪️viz-probe/🟦️.ts";
import fixture from "./🔣️.json";

/** 📈 Checks stock and authored OHLC inputs through the actual registered native compiler. */
export async function compileNativeFinancialGrammar(workDir: string): Promise<void> {
  let checks = 0;
  for (const theme of ["light", "dark"]) {
    const bodies: {raw:string}[] = [{raw:"\\SemioVizTable{financial-custom}{"+fixture.columns.join(",")+"}"+fixture.rows.map(row=>"\\SemioVizRow{financial-custom}{"+row.join(",")+"}").join("")}];
    for (const entry of fixture.cases) {
      const options = "window="+entry.window+(entry.stock?"":",data=financial-custom,deviation="+entry.deviation);
      bodies.push({raw:"\\clearpage\\SemioVizProbeBegin{native-financial}{"+entry.id+"}\\begin{VizFigure}[width="+fixture.frame[0]+",height="+fixture.frame[1]+"]\\SemioVizChart{"+entry.kind+"}["+options+"]\\end{VizFigure}"});
    }
    const records = await compileVizProbeDocument({case:"native-financial",scenario:"financial",documentClass:"semio",documentClassOptions:"type=paper,language=en,theme="+theme,packages:["semio-viz"],geometry:true,preamble:["\\title{Financial Grammar}","\\author{Semio}","\\date{}","\\errorcontextlines=200"],body:bodies},{workDir:join(workDir,theme),scenario:undefined,keepWorkDir:true});
    for (const entry of fixture.cases) {
      const values=entry.stock?fixture.stockClose:fixture.rows.map(row=>row[4]!);
      const means=values.map((_,index)=>mean(values.slice(Math.max(0,index-entry.window+1),index+1))!);
      const deviations=values.map((_,index)=>{const window=values.slice(Math.max(0,index-entry.window+1),index+1);return window.length===1?0:Math.sqrt(variance(window)!*(window.length-1)/window.length);});
      const own=records.filter(record=>record.scenario===entry.id),keys: [string,number[]][]=[["window-mean",means]];
      const lows=entry.stock?[9,10,8,9,11,10,11,13]:fixture.rows.map(row=>row[3]!),highs=entry.stock?[12,13,11,12,14,13,15,16]:fixture.rows.map(row=>row[2]!);
      const upper=means.map((value,index)=>value+entry.deviation*deviations[index]!),lower=means.map((value,index)=>value-entry.deviation*deviations[index]!);
      const x=scaleLinear([1,values.length],[10,fixture.frame[0]-4]),y=scaleLinear([Math.min(...lows,...(entry.band?lower:[])),Math.max(...highs,...(entry.band?upper:[]))],[8,fixture.frame[1]-4]);
      keys.push(["financial/mean-points",means.flatMap((value,index)=>[x(index+1),y(value)])]);
      if(entry.band)keys.push(["financial/upper-points",upper.flatMap((value,index)=>[x(index+1),y(value)])],["financial/lower-points",lower.flatMap((value,index)=>[x(index+1),y(value)])],["financial/deviation",deviations],["financial/upper",means.map((value,index)=>value+entry.deviation*deviations[index]!)],["financial/lower",means.map((value,index)=>value-entry.deviation*deviations[index]!)]);
      for(const [key,expected] of keys){const actual=own.find(record=>record.key==="geometry/"+key||record.key===key)?.values.map(Number);if(!actual||actual.length!==expected.length||actual.some((value,index)=>!Number.isFinite(value)||Math.abs(value-expected[index]!)>1e-4))throw Error(theme+" "+entry.id+" "+key+": "+JSON.stringify(actual)+" expected "+JSON.stringify(expected));checks++;}
    }
  }
  console.log("[native-grammar] "+checks+" actual financial mean/deviation/band records across "+fixture.cases.length+" neutral cases and 2 themes matched independent D3");
}
/** 🏛️ Compiles every authored stock kind of taxonomy 17 after numerical overlay proof. */
export async function compileNativeFinancialStock(workDir:string):Promise<void>{
  for(const theme of ["light","dark"]){
    const body=fixture.stockKinds.map(kind=>({raw:"\\clearpage\\SemioVizProbeBegin{native-financial-stock}{"+kind+"}\\begin{VizFigure}[width=80,height=40]\\SemioVizChart{"+kind+"}\\end{VizFigure}"}));
    await compileVizProbeDocument({case:"native-financial-stock",scenario:"financial-stock",documentClass:"semio",documentClassOptions:"type=paper,language=en,theme="+theme,packages:["semio-viz"],geometry:true,preamble:["\\title{Stock Financial Grammar}","\\author{Semio}","\\date{}","\\errorcontextlines=200"],body},{workDir:join(workDir,theme),scenario:undefined,keepWorkDir:true});
  }
  console.log("[native-grammar] "+fixture.stockKinds.length+" authored financial/economic stock kinds compiled in 2 themes");
}
