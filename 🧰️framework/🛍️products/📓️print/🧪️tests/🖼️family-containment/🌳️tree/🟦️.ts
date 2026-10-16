import{PRINT_PDF_OPS_V1 as OPS,printPdfPageV1,printPdfTransformV1,printPdfApplyV1,printPdfInverseV1,printPdfArrayV1,printPdfNumberV1,printPdfVectorV1,printPdfNameV1,printPdfRecordV1,printPdfGlyphsV1,type PrintPdfPageV1}from"../../../📦️artifacts/📖️pdf/🟦️.ts";
import type{PrintExecutionV1}from"../../../📦️artifacts/🎛️operation/🟦️.ts";
/** 🌳️ Measures native hierarchy projection and figure containment against independent D3. */
import {createRequire} from "node:module";
import {withCompiledVizProbeDocumentV1} from "../../../🔨️modules/🧪️viz-probe/🟦️.ts";
import fixture from "./🔣️.json";
type Row={id:string;parent:string|null;value:number;label:string};
type Node={id:string;x:number;y:number;data:Row;sort(compare:(a:Node,b:Node)=>number):Node;descendants():Node[]};
type Stratifier={ (rows:readonly Row[]):Node;id(accessor:(row:Row)=>string):Stratifier;parentId(accessor:(row:Row)=>string|null):Stratifier };
type Layout={ (node:Node):Node;size(size:[number,number]):Layout };
const reference=createRequire(import.meta.url)("d3-hierarchy") as {stratify():Stratifier;tree():Layout;cluster():Layout};
type Vector={id:string;kind:string;options?:string;orientation?:string;inset?:number;node?:string;projection?:boolean;data?:string;layout?:"tree"|"cluster";separateLabels?:string[]};
/** 🧭 Executes the authored table and stock families through the registered native compiler route. */
export async function compileNativeTreeContainment(execution:PrintExecutionV1):Promise<void>{
 for(const theme of ["light","dark"]){
  const cases=(fixture.cases as readonly Vector[]).filter(entry=>process.env.PRINT_NATIVE_TREE_PHASE!=="cladogram"||entry.id==="stock-cladogram"||entry.id.startsWith("equal-")),rows=fixture.rows as readonly Row[],body=cases.map(entry=>{
   const options=entry.orientation===undefined?entry.options??"":`data=${entry.data??"tree-neutral"},orientation=${entry.orientation},inset=${entry.inset},node=${entry.node},nodeRadius=2,sort=name-ascending,separationDepth=false,${entry.options??""}`;
   return {raw:`\\clearpage\\SemioVizProbeBegin{hierarchy-containment}{${entry.id}}\\begin{VizFigure}[title={${entry.id}},width=${fixture.frame[0]},height=${fixture.frame[1]}]\\begin{scope}[local bounding box=tree-result]\\TreeMeasure\\SemioVizChart{${entry.kind}}[${options}]\\end{scope}\\ExplSyntaxOn\\pgfextractx\\l_tmpa_dim{\\pgfpointanchor{tree-result}{south~west}}\\pgfextracty\\l_tmpb_dim{\\pgfpointanchor{tree-result}{south~west}}\\semio_viz_probe_values:nx{tree/min}{\\fp_eval:n{\\dim_to_fp:n{\\l_tmpa_dim}/(\\dim_to_fp:n{100mm}/100)},\\fp_eval:n{\\dim_to_fp:n{\\l_tmpb_dim}/(\\dim_to_fp:n{100mm}/100)}}\\pgfextractx\\l_tmpa_dim{\\pgfpointanchor{tree-result}{north~east}}\\pgfextracty\\l_tmpb_dim{\\pgfpointanchor{tree-result}{north~east}}\\semio_viz_probe_values:nx{tree/max}{\\fp_eval:n{\\dim_to_fp:n{\\l_tmpa_dim}/(\\dim_to_fp:n{100mm}/100)},\\fp_eval:n{\\dim_to_fp:n{\\l_tmpb_dim}/(\\dim_to_fp:n{100mm}/100)}}\\ExplSyntaxOff\\end{VizFigure}`};
  });
  const table="\\SemioVizTable{tree-neutral}{id,parent,value,label}"+rows.map(row=>`\\SemioVizRow{tree-neutral}{${row.id},${row.parent??""},${row.value},${row.label}}`).join("")+"\\SemioVizTable{tree-branch-neutral}{id,parent,value,length,label}"+fixture.branchRows.map(row=>`\\SemioVizRow{tree-branch-neutral}{${row.id},${row.parent??""},${row.value},${row.length},${row.label}}`).join("");
  await withCompiledVizProbeDocumentV1({case:"hierarchy-containment",scenario:"cases",documentClass:"semio",documentClassOptions:`type=paper,language=en,theme=${theme}`,packages:["semio-viz"],geometry:true,preamble:["\\makeatletter\\newcommand\\TreeMeasure{\\pgf@relevantforpicturesizetrue}\\makeatother","\\title{Hierarchy Containment}","\\author{Semio}","\\date{}"],body:[{raw:table},...body]},{execution,scenario:undefined},async({records,probe})=>{
  const failures:string[]=[];let count=0;
  for(const entry of cases){
   const own=records.filter(record=>record.scenario===entry.id),min=own.find(record=>record.key==="tree/min")!.values.map(Number),max=own.find(record=>record.key==="tree/max")!.values.map(Number);
   for(let axis=0;axis<2;axis++)if(min[axis]!< -fixture.tolerance||max[axis]!>fixture.frame[axis]!+fixture.tolerance)failures.push(`${theme}/${entry.id}: actual bounds ${min} .. ${max} exceed ${fixture.frame}`);
   if(entry.projection===true&&entry.orientation!==undefined){
    const inset=entry.inset!,horizontal=["left-right","right-left"].includes(entry.orientation),radial=entry.orientation==="radial",width=fixture.frame[0]!,height=fixture.frame[1]!,extent:[number,number]=radial?[2*Math.PI,Math.min(width,height)/2-inset]:horizontal?[height-2*inset,width-2*inset]:[width-2*inset,height-2*inset];
    const nodes=reference[entry.layout??"tree"]().size(extent)(reference.stratify().id(row=>row.id).parentId(row=>row.parent)(entry.data==="tree-branch-neutral"?fixture.branchRows:rows).sort((a,b)=>a.id<b.id?-1:a.id>b.id?1:0)).descendants();
    for(const node of nodes){
     const expected=entry.orientation==="bottom-up"?[inset+node.x,inset+node.y]:entry.orientation==="left-right"?[inset+node.y,height-inset-node.x]:entry.orientation==="right-left"?[width-inset-node.y,height-inset-node.x]:radial?[width/2+node.y*Math.sin(node.x),height/2+node.y*Math.cos(node.x)]:[inset+node.x,height-inset-node.y];
     const actual=own.find(record=>record.key===`geometry/node/${entry.node}`&&record.values[0]===node.id)?.values.slice(1,3).map(Number);
     if(!actual||actual.some((value,axis)=>Math.abs(value-expected[axis]!)>1e-4))failures.push(`${theme}/${entry.id}/${node.id}: ${actual} differs from D3 ${expected}`);count++;
    }
   }
  }
  const pdf=await probe.inspect();let pages=0;
  let labelPairs=0;
  {for(let page=1;page<=pdf.pages.length;page++){
   const text=await(printPdfPageV1(pdf,page));pages++;
   for(const entry of cases.filter(value=>value.separateLabels)){
    const items=entry.separateLabels!.map(label=>text.items.find(item=>"str" in item&&item.str===label));
    const [a,b]=items;
    if(items.every(item=>item&&"str" in item)&&a&&"str"in a&&b&&"str"in b){
     const gap=Math.max(b!.transform[4]!-a!.transform[4]!-a!.width,a!.transform[4]!-b!.transform[4]!-b!.width,Math.abs(a!.transform[5]!-b!.transform[5]!)-Math.max(a!.height,b!.height))*25.4/72;
     if(gap<fixture.minimumLabelGap)failures.push(`${theme}/${entry.id}: actual PDF label gap ${gap} mm is below ${fixture.minimumLabelGap}`);labelPairs++;
    }
   }
  }}
  if(labelPairs!==cases.filter(entry=>entry.separateLabels).length)failures.push(`${theme}: expected label pairs were not found exactly once`);
  if(failures.length)throw Error(failures.length+" hierarchy containment/projection failures:\n"+failures.join("\n"));
  console.log(`[native-grammar] ${theme}: ${count} D3 hierarchy frame projections, ${cases.length} actual contained families, ${labelPairs} separated PDF label pairs, ${pages} PDF.js pages matched`);
  });
 }
}
