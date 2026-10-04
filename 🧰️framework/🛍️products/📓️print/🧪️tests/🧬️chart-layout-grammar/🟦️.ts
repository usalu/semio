/** 📐️ Authored layouts compile in place and publish columns adjudicated by independent D3. */
import {mkdirSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {compileVizProbe} from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
import {defineTestAdapter} from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import fixtures from "./🔣️.json";
import customization from "./⚙️.json";

type Check = {readonly module:string;readonly name:string;readonly subject:()=>unknown;readonly oracle:()=>unknown;readonly tolerance:number};
type Row = Record<string,string|number|null>;
const columns:Record<string,string[]> = {pie:["startAngle","endAngle","padAngle","x","y","radius"],stack:["y0","y1"],bin:["x0","x1","count"],hexbin:["x","y","count"],beeswarm:["x","y"],jitter:["x","y"],arc:["x","y"],chord:["startAngle","endAngle","targetStartAngle","targetEndAngle","value"],sankey:["x","y","x2","y2","width"],alluvial:["x","y","x2","y2","width"],treemap:["x0","y0","x1","y1"],partition:["x0","y0","x1","y1"],pack:["x","y","radius"],force:["x","y"],tree:["x","y"],cluster:["x","y"],dag:["x","y"],bundling:["x","y","x2","y2"],voronoi:["x","y"],delaunay:["x","y"],hull:["x","y"],contour:["x","y","value"],density:["x","y","value"],projection:["x","y"]};
const polygonKinds = new Set(["voronoi","delaunay","hull","contour","density"]);

function source():string {
  const cases=[...fixtures,...customization].map(entry=>{
    const algorithm=entry.layer.layout.algorithm;
    const rows=entry.rows as Row[],cols=Object.keys(rows[0]!),options=Object.entries(entry.layer.layout.options??{}).map(([key,value])=>`${key}={${value}}`).join(",");
    return `\\SemioVizTable{${entry.name}}{${cols.join(",")}}\n`+rows.map(row=>`\\SemioVizRow{${entry.name}}{${cols.map(col=>`{${row[col]??""}}`).join(",")}}`).join("\n")+`\n\\SemioVizLayout{${algorithm}}{${entry.name}}{${entry.name}}[${options}]\n`+columns[algorithm]!.map(col=>`\\ProbeLayoutColumn{${entry.name}/${col}}{${entry.name}}{${col}}`).join("\n");
  });
  return String.raw`\documentclass{article}
\usepackage{semio-viz-transform}
\usepackage{semio-viz-shape}
\usepackage{semio-viz-hierarchy}
\usepackage{semio-viz-network}
\usepackage{semio-viz-flow}
\usepackage{semio-viz-geo}
\usepackage{semio-viz-spatial}
\usepackage{semio-viz-probe}
\ExplSyntaxOn
\seq_new:N \l_layout_probe_seq
\tl_new:N \l_layout_probe_cell_tl
\NewDocumentCommand \ProbeLayoutColumn {m m m} {
  \semio_viz_tr_load:n {#2}
  \seq_clear:N \l_layout_probe_seq
  \seq_map_inline:Nn \l_semio_viz_tr_rows_seq {
    \semio_viz_tr_get:nnN {##1} {#3} \l_layout_probe_cell_tl
    \seq_put_right:NV \l_layout_probe_seq \l_layout_probe_cell_tl
  }
  \semio_viz_probe_values:nx {#1} {\seq_use:Nn \l_layout_probe_seq {,}}
}
\ExplSyntaxOff
\begin{document}
\SemioVizProbeBegin{chart-layout-grammar}{all-layout-kinds}
`+cases.join("\n")+"\n\\SemioVizProbeEnd\n\\end{document}\n";
}

async function reference():Promise<Record<string,readonly number[]>> {
  const [array,shape,hierarchy,network,geo,spatial,hex,flow] = await Promise.all([import("d3-array"),import("d3-shape"),import("d3-hierarchy"),import("d3-force"),import("d3-geo"),import("d3-delaunay"),import("d3-hexbin"),import("d3-sankey")]);
  const result:Record<string,readonly number[]>={};
  let scenario="";
  const put=(name:string,rows:Record<string,unknown>[])=>{for(const col of columns[name]!)result[`${scenario}/${col}`]=rows.map(row=>Number(row[col]));};
  for(const entry of [...fixtures,...customization]){
    scenario=entry.name;
    const name=entry.layer.layout.algorithm,rows=entry.rows as Row[],opt=(entry.layer.layout.options??{}) as Record<string,string|number|boolean>,width=Number(opt.width??80),height=Number(opt.height??40),points=rows.map(row=>[Number(row[String(opt.x??"x")]),Number(row[String(opt.y??"y")])] as [number,number]);
    if(name==="pie")put(name,shape.pie<Row>().value(row=>Number(row[String(opt.value??"value")])).sort(null).sortValues(opt.sort==="descending"?array.descending:opt.sort==="ascending"?array.ascending:null).startAngle(Number(opt.startAngle??0)).endAngle(Number(opt.endAngle??2*Math.PI)).padAngle(Number(opt.padAngle??0))(rows).map(span=>({...span,x:Number(opt.cx??width/2),y:Number(opt.cy??height/2),radius:Number(opt.outerRadius??Math.min(width,height)/2)})));
    else if(name==="bin")put(name,array.bin().thresholds(Number(opt.thresholds))(rows.map(row=>Number(row.value))).map(bin=>({x0:bin.x0,x1:bin.x1,count:bin.length})));
    else if(name==="stack")put(name,shape.stack<Row>().keys(String(opt.keys).split(",")).offset(shape.stackOffsetExpand)(rows).flatMap(series=>series.map(point=>({y0:point[0],y1:point[1]}))));
    else if(name==="hexbin")put(name,hex.hexbin<[number,number]>().radius(Number(opt.radius))(points).map(bin=>({x:bin.x,y:bin.y,count:bin.length})));
    else if(name==="jitter"){const random=(await import("d3-random")).randomLcg(Number(opt.seed));put(name,points.map(([x,y])=>({x:x+(random()-.5)*Number(opt.amount),y:y+(random()-.5)*Number(opt.amount)})));}
    else if(name==="beeswarm"){
      const placed:[number,number][]=[];const radius=Number(opt.radius),vertical=opt.axis==="y";
      put(name,points.map(point=>{const value=point[vertical?1:0],baseline=point[vertical?0:1],candidates=[baseline];for(const prior of placed){const delta=Math.abs(value-prior[0]);if(delta<2*radius){const offset=Math.sqrt(4*radius*radius-delta*delta);candidates.push(prior[1]-offset,prior[1]+offset);}}const offset=array.sort(candidates,(a,b)=>array.ascending(Math.abs(a-baseline),Math.abs(b-baseline))||array.ascending(a,b)).find(candidate=>placed.every(prior=>(value-prior[0])**2+(candidate-prior[1])**2>=4*radius*radius-1e-9))!;placed.push([value,offset]);return vertical?{x:offset,y:value}:{x:value,y:offset};}));
    }
    else if(["tree","cluster","treemap","partition","pack","bundling"].includes(name)){
      const root=hierarchy.stratify<Row>().id(row=>String(row.id)).parentId(row=>row.parent===null?null:String(row.parent))(rows).sum(row=>Number(row.value));
      if(name==="treemap")hierarchy.treemap<Row>().size([width,height]).tile(opt.tile==="slice"?hierarchy.treemapSlice:hierarchy.treemapBinary).round(opt.round===true).paddingInner(Number(opt.paddingInner??0)).paddingOuter(Number(opt.paddingOuter??0)).paddingTop(Number(opt.paddingTop??opt.paddingOuter??0)).paddingRight(Number(opt.paddingRight??opt.paddingOuter??0)).paddingBottom(Number(opt.paddingBottom??opt.paddingOuter??0)).paddingLeft(Number(opt.paddingLeft??opt.paddingOuter??0))(root);
      else if(name==="partition")hierarchy.partition<Row>().size([width,height]).padding(Number(opt.padding??0)).round(opt.round===true)(root);
      else if(name==="pack")hierarchy.pack<Row>().size([width,height]).padding(Number(opt.padding??0))(root);
      else if(name==="tree")hierarchy.tree<Row>().size([width,height]).separation((a,b)=>a.parent===b.parent?Number(opt.separation??1):Number(opt.cousinSeparation??2))(root);
      else hierarchy.cluster<Row>().size([width,height]).separation((a,b)=>a.parent===b.parent?Number(opt.separation??1):Number(opt.cousinSeparation??2))(root);
      const nodes=(opt.leaves===true?root.leaves():root.descendants()) as unknown as Record<string,unknown>[];
      if(name==="bundling")put(name,root.links().map(link=>({x:(link.source as any).x,y:(link.source as any).y,x2:(link.target as any).x,y2:(link.target as any).y})));
      else put(name,nodes.map(node=>({...node,radius:node.r})));
    }
    else if(["force","arc","dag"].includes(name)){
      if(name==="arc")put(name,rows.map((_,index)=>opt.vertical===true?{x:0,y:Number(opt.origin??0)+index*width/(rows.length-1)}:{x:Number(opt.origin??0)+index*width/(rows.length-1),y:0}));
      else if(name==="dag"){const dagre=(await import("dagre")).default;const graph=new dagre.graphlib.Graph().setGraph({rankdir:"LR",ranksep:Number(opt.layerGap),nodesep:12}).setDefaultEdgeLabel(()=>({}));for(const row of rows){graph.setNode(String(row.source),{width:0,height:0});graph.setNode(String(row.target),{width:0,height:0});graph.setEdge(String(row.source),String(row.target));}dagre.layout(graph);put(name,graph.nodes().map((id:string)=>graph.node(id)));}
      else{const hasLinks="source"in rows[0]!,nodes:any[]=hasLinks?Array.from(new Set(rows.flatMap(row=>[String(row.source),String(row.target)]))).map(id=>({id})):rows.map(row=>({...row})),links=hasLinks?rows.map(row=>({source:String(row.source),target:String(row.target)})):[];const simulation=network.forceSimulation(nodes).stop().alpha(Number(opt.alpha??1)).velocityDecay(Number(opt.velocityDecay??.4)).randomSource((await import("d3-random")).randomLcg(Number(opt.seed))).force("charge",network.forceManyBody().strength(Number(opt.charge??-30))).force("center",network.forceCenter(width/2,height/2));if(links.length)simulation.force("link",network.forceLink(links).id((node:any)=>node.id).distance(Number(opt.distance??30)));if(opt.radius!==undefined)simulation.force("collide",network.forceCollide(Number(opt.radius)));simulation.tick(Number(opt.iterations));put(name,nodes);}
    }
    else if(name==="chord"){const names=Array.from(new Set(rows.flatMap(row=>[String(row.source),String(row.target)]))),matrix=names.map(()=>names.map(()=>0));for(const row of rows)matrix[names.indexOf(String(row.source))]![names.indexOf(String(row.target))]!+=Number(row.value);const generator=(await import("d3-chord"))[opt.transpose===true?"chordTranspose":"chord"]().padAngle(Number(opt.padAngle));put(name,generator(matrix).map(chord=>({startAngle:chord.source.startAngle,endAngle:chord.source.endAngle,targetStartAngle:chord.target.startAngle,targetEndAngle:chord.target.endAngle,value:chord.source.value})));}
    else if(name==="sankey"||name==="alluvial"){
      const stages=String(opt.stages??"").split(","),pairs=name==="sankey"?rows.map(row=>({source:String(row.source),target:String(row.target),value:Number(row[String(opt.value??"value")])})):rows.flatMap(row=>stages.slice(0,-1).map((stage,index)=>({source:`${stage}:${row[stage]}`,target:`${stages[index+1]}:${row[stages[index+1]!]}`,value:Number(row[String(opt.value??"value")])}))),links=name==="alluvial"?Array.from(array.rollup(pairs,bucket=>({source:bucket[0]!.source,target:bucket[0]!.target,value:array.sum(bucket,pair=>pair.value)}),pair=>JSON.stringify([pair.source,pair.target])).values()):pairs;
      const ids=Array.from(new Set(links.flatMap(link=>[link.source,link.target]))),layout=flow.sankey<any,any>().nodeId(node=>node.id).nodeWidth(Number(opt.nodeWidth)).nodePadding(Number(opt.nodePadding??8)).iterations(Number(opt.iterations??6)).extent([[0,0],[width,height]]);
      if(opt.align==="left")layout.nodeAlign(flow.sankeyLeft);
      if(name==="alluvial")layout.nodeAlign(flow.sankeyLeft).nodeSort(null);
      const output=layout({nodes:ids.map(id=>({id})),links});put(name,output.links.map(link=>({x:link.source.x1,y:link.y0,x2:link.target.x0,y2:link.y1,width:link.width})));
    }
    else if(name==="projection"){const projection=geo.geoMercator().scale(Number(opt.scale)).translate([Number(opt.translateX??width/2),Number(opt.translateY??height/2)]).rotate([Number(opt.rotateLongitude??0),Number(opt.rotateLatitude??0),Number(opt.rotateGamma??0)]).center([Number(opt.centerLongitude??0),Number(opt.centerLatitude??0)]).reflectX(opt.reflectX===true).reflectY(opt.reflectY===true);put(name,points.map(point=>{const [x,y]=projection(point)!;return {x,y};}));}
    else if(name==="hull")put(name,(await import("d3-polygon")).polygonHull(points)!.map(([x,y])=>({x,y})));
    else if(name==="voronoi"||name==="delaunay"){
      const triangulation=spatial.Delaunay.from(points),polygons=name==="voronoi"?Array.from(triangulation.voronoi([Number(opt.x0??0),Number(opt.y0??0),Number(opt.x1??width),Number(opt.y1??height)]).cellPolygons()).map(polygon=>polygon.slice(0,-1)):Array.from(triangulation.trianglePolygons()).map(polygon=>polygon.slice(0,-1));put(name,polygons.flat().map(([x,y])=>({x,y})));
    }
    else{
      const contour=(await import("d3-contour")).contours();let values=rows.map(row=>Number(row[String(opt.value??"value")])),nx=Number(opt.columns),ny=Number(opt.rows),cell=1,x0=0,y0=0;
      if(name==="density"){cell=Number(opt.cellSize);x0=Number(opt.x0??0);y0=Number(opt.y0??0);nx=Math.ceil((Number(opt.x1??width)-x0)/cell)+1;ny=Math.ceil((Number(opt.y1??height)-y0)/cell)+1;const bandwidth=Number(opt.bandwidth);values=Array.from({length:nx*ny},(_,i)=>array.mean(points,([x,y])=>opt.kernel==="uniform"?Math.abs(x0+i%nx*cell-x)<=bandwidth&&Math.abs(y0+Math.floor(i/nx)*cell-y)<=bandwidth?.25/bandwidth**2:0:Math.exp(-((x0+i%nx*cell-x)**2+(y0+Math.floor(i/nx)*cell-y)**2)/(2*bandwidth**2))/(2*Math.PI*bandwidth**2))!);}
      put(name,contour.size([nx,ny]).thresholds(Number(opt.thresholds))(values).flatMap(entry=>entry.coordinates.flatMap(polygon=>polygon.flatMap(ring=>ring.map(([x,y])=>({x:x0+x*cell,y:y0+y*cell,value:entry.value}))))));
    }
  }
  return result;
}

/** ⚖️ Compiles all authored algorithms in place, then checks numeric geometry columns. */
export async function nativeLayoutGrammarChecks(workDir:string):Promise<readonly Check[]> {
  workDir=resolve(workDir);mkdirSync(workDir,{recursive:true});const input=join(workDir,"chart-layout-grammar.tex");writeFileSync(input,source());
  const [records,oracle]=await Promise.all([compileVizProbe(input,{workDir:join(workDir,"compiled"),keepWorkDir:true,caseName:"chart-layout-grammar",scenario:"all-layout-kinds"}),reference()]);
  const expected=projection(oracle),actual=projection(Object.fromEntries(Object.keys(oracle).map(key=>[key,records.filter(record=>record.key===key).flatMap(record=>record.values.map(Number))])));
  return Object.entries(expected).map(([key,values])=>({module:"native-layout",name:key,subject:()=>actual[key]!,oracle:()=>values,tolerance:2e-5}));
}
function arraySorted(values:readonly number[]):number[]{return [...values].sort((a,b)=>a-b);}
function projection(raw:Record<string,readonly number[]>):Record<string,readonly number[]> {
  const result=Object.fromEntries(Object.entries(raw).map(([key,values])=>[key,polygonKinds.has(key.split("/")[0]!.replace(/-controls$/,""))?arraySorted(values):values]));
  for(const entry of [...fixtures,...customization]){
    if(!polygonKinds.has(entry.layer.layout.algorithm))continue;
    const fields=entry.layer.layout.algorithm==="contour"||entry.layer.layout.algorithm==="density"?["x","y","value"]:["x","y"],xs=raw[`${entry.name}/x`]!;
    const tuples=xs.map((_,index)=>fields.map(field=>raw[`${entry.name}/${field}`]![index]!)).sort((a,b)=>{for(let i=0;i<a.length;i++){const delta=Math.round(a[i]!*1e6)-Math.round(b[i]!*1e6);if(delta)return delta;}return 0;});
    result[`${entry.name}/geometry`]=tuples.flat();
  }
  return result;
}
export default defineTestAdapter({implementation:"typescript",scenarios:{"all-layout-kinds":{subject:async context=>({projection:Object.fromEntries((await nativeLayoutGrammarChecks(context.workDir)).map(check=>[check.name,check.subject() as number[]]))}),oracle:async()=>({projection:projection(await reference())})}}});
