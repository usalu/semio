/** 🖼️ Rendering: a §79 chart specification into two independent outputs — the dependency-free 2D
 * scene graph of `🧰️framework/🔨️modules/◻️2d` and TikZ source text for `semio-viz`. Both emitters
 * read the same resolved geometry; opaque native styles apply only to the TikZ backend.
 * @see ../../../../../🔨️modules/◻️2d/🟦️.ts
 * @see ../../../🖋️latex/semio-viz-plot.sty
 */
import type { DrawingScene, SceneNode, PathSegment } from "../../../../../🔨️modules/◻️2d/🟦️.ts";
import { drawVizSymbol, vizCurve, vizPathRecorder, type VizPathCommand } from "../✒️mark/🟦️.ts";
import { vizArc, vizArea, vizLine, vizLink, vizRibbon, type VizLinkKind } from "../🥧shape/🟦️.ts";
import { formatVizTime, formatVizValue } from "../🔢format/🟦️.ts";
import { inferVizLayerTable } from "../🧮transform/🟦️.ts";
import { vizGeoProjection } from "../🌍geo/🟦️.ts";
import { buildVizCoordinate, coordinateGeographic } from "../🧭coordinate/🟦️.ts";
import { buildVizScale, type VizScale } from "../📐scale/🟦️.ts";
import { vizCategoricalColor, vizParseColor, vizSchemeInterpolator, vizTheme, type VizTheme } from "../🎨theme/🟦️.ts";
import type { VizChartSpecification, VizCurveKind, VizExtent, VizLayerSpec, VizGuideSpec, VizPoint, VizRow, VizSymbolKind, VizProjectionKind } from "../../📸️snapshot/📊️chart/🟦️.ts";

import { VIZ_AXIS_DEFAULTS, VIZ_LEGEND_DEFAULTS } from "../../📸️snapshot/📊️chart/🟦️.ts";

import { measurePrintSans, printFontFamily, printFontTexSelector } from "../../../🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts";

//#region 🔖️Items
/** 🖼️ Resolved geometry uses millimetres; text sizes use TeX points. */
type VizRenderStyle = { readonly tikzStyle?: string; readonly fill?: string; readonly stroke?: string; readonly strokeWidth?: number; readonly opacity?: number; readonly dash?: readonly number[]; readonly rotation?: number; readonly clip?: VizExtent; readonly cap?: "butt" | "round" | "square"; readonly join?: "miter" | "round" | "bevel" };

export type VizRenderItem = VizRenderStyle & (
  | { readonly kind: "rect"; readonly x: number; readonly y: number; readonly width: number; readonly height: number; readonly fill?: string; readonly stroke?: string; readonly strokeWidth?: number; readonly opacity?: number }
  | { readonly kind: "circle"; readonly cx: number; readonly cy: number; readonly r: number; readonly fill?: string; readonly stroke?: string; readonly strokeWidth?: number; readonly opacity?: number }
  | { readonly kind: "line"; readonly x1: number; readonly y1: number; readonly x2: number; readonly y2: number; readonly stroke?: string; readonly strokeWidth?: number; readonly dash?: readonly number[] }
  | { readonly kind: "polygon"; readonly points: readonly VizPoint[]; readonly fill?: string; readonly stroke?: string; readonly strokeWidth?: number; readonly opacity?: number }
  | { readonly kind: "path"; readonly commands: readonly VizPathCommand[]; readonly fill?: string; readonly stroke?: string; readonly strokeWidth?: number; readonly opacity?: number }
  | { readonly kind: "text"; readonly x: number; readonly y: number; readonly content: string; readonly size: number; readonly font?: string; readonly fill?: string; readonly anchor?: "start" | "middle" | "end"; readonly baseline?: "alphabetic" | "middle" | "top" | "bottom" });

/** 🖼️ A chart resolved into primitives: the frame it was laid out in and the items to draw. */
export type VizRenderPlan = { readonly width: number; readonly height: number; readonly frame: VizExtent; readonly theme: VizTheme; readonly items: readonly VizRenderItem[] };
//#endregion 🔖️Items

//#region 🔖️Layout
/** 🔘️ Resolves the native boolean option literals without accepting unrelated primitives. */
function guideBoolean(value:unknown,fallback:boolean):boolean { if(value===undefined)return fallback;if(value===true||value==="true")return true;if(value===false||value==="false")return false;throw new Error("guide boolean requires true or false"); }
/** 🧭️ Resolves native axis controls into owned cartesian, segment, polar and broken geometry. */
function axisGuideItems(guide:VizGuideSpec,orientation:string,scale:VizScale<never,never>,frame:VizExtent,width:number,height:number,values:readonly unknown[],label:(value:unknown)=>string,title:string|undefined,theme:VizTheme):VizRenderItem[] {
  const o=guide.options??{},items:VizRenderItem[]=[],horizontal=orientation==="top"||orientation==="bottom",positive=orientation!=="top"&&orientation!=="left",cartesian=["top","bottom","left","right"].includes(orientation);
  const tick=Number(o.tickSize??guide.tickSize??(guide.kind==="grid"?0:VIZ_AXIS_DEFAULTS.tickSize)),padding=Number(o.tickPadding??VIZ_AXIS_DEFAULTS.tickPadding),offset=Number(o.offset??guide.offset??0),stroke=String(o.stroke??theme.chrome[VIZ_AXIS_DEFAULTS.strokeColorRole]),fill=String(o.fill??theme.chrome[VIZ_AXIS_DEFAULTS.textColorRole]),sw=Number(o.strokeWidth??guideStrokeWidth(theme,VIZ_AXIS_DEFAULTS.strokeRole)),size=Number(o.labelSize??VIZ_AXIS_DEFAULTS.labelSize);
  const pair=(value:unknown,fallback:VizPoint):VizPoint=>{if(value===undefined||value==="")return fallback;const parsed=String(value).split(",").map(Number);if(parsed.length!==2||!parsed.every(Number.isFinite))throw new Error("axis coordinate requires two finite numbers");return [parsed[0]!,height-parsed[1]!];};
  const range=scale.range(),r0=Number(range[0]),r1=Number(range.at(-1)),breakValues=String(o.broken??"").split(",").map(Number),broken=o.broken!==undefined&&breakValues.length===2;
  const mapped=(value:unknown)=>Number(scale(value as never))+(scale.bandwidth?.()??0)/2,breakLo=broken?mapped(breakValues[0]):0,breakHi=broken?mapped(breakValues[1]):0,gap=Number(o.breakGap??2),shift=broken?breakHi-breakLo-gap:0,last=r1-shift;
  const center=pair(o.center,[width/2,height/2]),inner=Number(o.innerRadius??0),outer=Number(o.outerRadius??Math.max(1,Math.min(width,height)/2-Math.max(frame.x0,frame.y0,width-frame.x1,height-frame.y1))),angle=Number(o.angle??90)*Math.PI/180,startAngle=Number(o.startAngle??0),endAngle=Number(o.endAngle??360);
  let from:VizPoint,to:VizPoint,normal:VizPoint,gridLength:number;
  if(cartesian){const at=o.at===undefined||o.at===""?(orientation==="top"?frame.y0:orientation==="bottom"?frame.y1:orientation==="left"?frame.x0:frame.x1):horizontal?height-Number(o.at):Number(o.at),axis=at+(positive?offset:-offset);from=horizontal?[r0,axis]:[axis,r0];to=horizontal?[last,axis]:[axis,last];normal=horizontal?[0,positive?1:-1]:[positive?1:-1,0];gridLength=horizontal?(positive?axis-frame.y0:frame.y1-axis):(positive?axis-frame.x0:frame.x1-axis);}
  else if(orientation==="segment"){from=pair(o.from,[r0,frame.y1]);to=pair(o.to,[last,frame.y1]);const dx=to[0]-from[0],dy=to[1]-from[1],length=Math.max(1e-9,Math.hypot(dx,dy));normal=[-dy/length,dx/length];gridLength=length;}
  else if(orientation==="radial"){const c=Math.cos(angle),s=Math.sin(angle);from=[center[0]+inner*c,center[1]-inner*s];to=[center[0]+outer*c,center[1]-outer*s];normal=[s,c];gridLength=outer;}
  else if(orientation==="angular"){from=center;to=center;normal=[1,0];gridLength=outer;}
  else throw new Error("unknown guide orientation "+orientation);
  if(o.gridLength!==undefined&&o.gridLength!=="")gridLength=Number(o.gridLength);
  const point=(position:number):{p:VizPoint;n:VizPoint}=>{if(orientation==="angular"){const theta=position*Math.PI/180,n:VizPoint=[Math.cos(theta),-Math.sin(theta)];return {p:[center[0]+outer*n[0],center[1]+outer*n[1]],n};}const fraction=last===r0?0.5:(position-r0)/(last-r0);return {p:[from[0]+fraction*(to[0]-from[0]),from[1]+fraction*(to[1]-from[1])],n:normal};};
  const extend=(p:VizPoint,n:VizPoint,distance:number):VizPoint=>[p[0]+distance*n[0],p[1]+distance*n[1]];
  const line=(a:VizPoint,b:VizPoint,extra:VizRenderStyle={}):VizRenderItem=>({kind:"line",x1:a[0],y1:a[1],x2:b[0],y2:b[1],stroke,strokeWidth:sw,...extra});
  const positions:number[]=[],grid=guideBoolean(o.grid??guide.grid,guide.kind==="grid"),drawTicks=guide.kind!=="grid"||guideBoolean(o.labels,false)||(o.tickSize!==undefined&&tick!==0),drawLabels=guideBoolean(o.labels,guide.kind!=="grid");
  for(const value of values){if(broken&&Number(value)>breakValues[0]!&&Number(value)<breakValues[1]!)continue;let position=mapped(value);if(!Number.isFinite(position))continue;if(broken&&position>breakHi)position-=shift;positions.push(position);const {p,n}=point(position);
    if(grid){if(cartesian&&o.gridLength===undefined)items.push(horizontal?line([p[0],frame.y0],[p[0],frame.y1],{stroke:String(o.gridStroke??o.stroke??theme.chrome[VIZ_AXIS_DEFAULTS.gridStrokeColorRole]),opacity:VIZ_AXIS_DEFAULTS.gridOpacity,dash:dashArray(o.gridDash)}):line([frame.x0,p[1]],[frame.x1,p[1]],{stroke:String(o.gridStroke??o.stroke??theme.chrome[VIZ_AXIS_DEFAULTS.gridStrokeColorRole]),opacity:VIZ_AXIS_DEFAULTS.gridOpacity,dash:dashArray(o.gridDash)}));else items.push(line(p,extend(p,n,-gridLength),{stroke:String(o.gridStroke??o.stroke??theme.chrome[VIZ_AXIS_DEFAULTS.gridStrokeColorRole]),opacity:VIZ_AXIS_DEFAULTS.gridOpacity,dash:dashArray(o.gridDash)}));}
    if(!drawTicks)continue;items.push(line(p,extend(p,n,tick)));if(drawLabels){const at=extend(p,n,tick+padding),automatic=Math.abs(n[0])>Math.abs(n[1])?n[0]>0?"start":"end":"middle",anchor=o.labelAlign&&o.labelAlign!=="auto"?String(o.labelAlign) as "start"|"middle"|"end":automatic;items.push({kind:"text",x:at[0],y:at[1],content:label(value),size,font:printFontFamily(VIZ_AXIS_DEFAULTS.labelFont),fill,anchor,baseline:o.labelAlign&&o.labelAlign!=="auto"||Math.abs(n[0])>Math.abs(n[1])?"middle":n[1]>0?"top":"bottom",rotation:-Number(o.labelRotation??o.labelRotate??0)});}
  }
  if(guideBoolean(o.minor??guide.minor,false)){const divisions=Number(o.minorTicks??4),length=Number(o.minorSize??0.7);if(!Number.isInteger(divisions)||!Number.isFinite(length))throw new Error("minor ticks require integer subdivisions and finite size");for(let interval=1;interval<positions.length;interval++)for(let division=1;division<divisions;division++){const {p,n}=point(positions[interval-1]!+division/divisions*(positions[interval]!-positions[interval-1]!));items.push(line(p,extend(p,n,length),{stroke:String(o.stroke??theme.chrome[VIZ_AXIS_DEFAULTS.minorStrokeColorRole]),opacity:VIZ_AXIS_DEFAULTS.minorOpacity}));}}
  if(guideBoolean(o.domainLine??o.domain,guide.kind==="axis")){if(orientation==="angular"){const steps=Math.max(8,Math.ceil(Math.abs(endAngle-startAngle)/3));for(let step=1;step<=steps;step++)items.push(line(point(startAngle+(step-1)/steps*(endAngle-startAngle)).p,point(startAngle+step/steps*(endAngle-startAngle)).p));}else{const cap=Number(o.tickSizeOuter??VIZ_AXIS_DEFAULTS.tickSizeOuter);if(cap!==0)items.push(line(extend(from,normal,cap),from));items.push(line(from,to));if(cap!==0)items.push(line(to,extend(to,normal,cap)));}}
  if(broken){const {p,n}=point(breakLo),count=Number(o.breakMarks??2);if(!Number.isInteger(count)||count<0)throw new Error("break marks require a nonnegative integer");for(let index=1;index<=count;index++){const across=(index-(count+1)/2)*gap*0.6;items.push(line([p[0]+across-n[0]*1.1-n[1]*0.8,p[1]+across-n[1]*1.1+n[0]*0.8],[p[0]+across+n[0]*1.1+n[1]*0.8,p[1]+across+n[1]*1.1-n[0]*0.8]));}}
  if(title!==undefined){const fraction=o.titleAnchor==="start"?0:o.titleAnchor==="end"?1:0.5,distance=tick+Number(o.titleGap??VIZ_AXIS_DEFAULTS.titleGap);let at:VizPoint;if(orientation==="angular"){const {p,n}=point(startAngle+fraction*(endAngle-startAngle));at=extend(p,n,distance);}else{const base:VizPoint=[from[0]+fraction*(to[0]-from[0]),from[1]+fraction*(to[1]-from[1])];at=extend(base,normal,distance);}items.push({kind:"text",x:at[0],y:at[1],content:title,size:Number(o.titleSize??VIZ_AXIS_DEFAULTS.titleSize),font:printFontFamily(VIZ_AXIS_DEFAULTS.titleFont),fill,anchor:"middle",rotation:0});}
  return o.style===undefined?items:items.map(item=>({...item,tikzStyle:String(o.style)}));
}
/** 🖊️ Resolves native theme stroke roles from the owned design-token values. */
function guideStrokeWidth(theme:VizTheme,role:string):number {const token=role==="default"?theme.strokes.chromeBorderDefault:role==="hairline"?theme.strokes.chromeBorderHairline:undefined;if(token===undefined)throw new Error("unknown guide stroke role "+role);return token*.75*25.4/72.27;}
/** 🏷️ Resolves schema-owned native legend geometry with source-bound plain-text font advances. */
function legendGuideItems(guide:VizGuideSpec,orientation:string,scale:VizScale<never,never>,width:number,height:number,authored:readonly unknown[]|undefined,label:(value:unknown)=>string,title:string|undefined,theme:VizTheme,palette:readonly string[],checkpoint:()=>void):VizRenderItem[] {
  const o=guide.options??{},d=VIZ_LEGEND_DEFAULTS,items:VizRenderItem[]=[],channel=String(o.channel??""),kind=channel==="fill"?"swatch":channel==="stroke"||channel==="dash"?"line":channel==="shape"?"symbol":channel==="size"?"size":String(o.kind??d.kind),horizontal=["horizontal","top","bottom"].includes(orientation),swatch=Number(o.swatchSize??d.swatchSize),gap=o.spacing===undefined?Number(o.itemGap??d.itemGap):Math.max(0,Number(o.spacing)-swatch),labelGap=Number(o.labelGap??d.labelGap),length=Number(o.length??d.length),columns=Number(o.columns??d.columns),offset=Number(o.offset??guide.offset??d.offset),size=Number(o.labelSize??d.labelSize),textFill=String(o.fill??theme.chrome[d.textColorRole]);
  if(![swatch,gap,labelGap,length,offset,size].every(Number.isFinite)||!Number.isInteger(columns)||columns<1)throw new Error("legend dimensions require finite values and positive integer columns");
  const pair=o.at===undefined||o.at===""?[width-d.inset-length,height-d.inset]:String(o.at).split(",").map(Number);
  if(pair.length!==2||!pair.every(Number.isFinite))throw new Error("legend coordinate requires two finite numbers");
  let x=pair[0]!,y=pair[1]!;if(orientation==="top")y=height+offset;if(orientation==="bottom")y=-offset;if(orientation==="left")x=-length-offset;if(orientation==="right")x=width+offset;x=Number(o.x??x);y=Number(o.y??y);
  const values=authored?.length?authored:(kind==="gradient"||kind==="size")?scale.ticks?.(Number(o.ticks??guide.ticks??d.ticks))??scale.domain():scale.domain(),map=(value:unknown):unknown=>(scale as unknown as (value:unknown)=>unknown)(value),categorical=(index:number):string=>palette[index%palette.length]??vizCategoricalColor(theme.palette,index),stroke=Number(o.strokeWidth??guideStrokeWidth(theme,d.strokeRole)),outline=Number(o.strokeWidth??guideStrokeWidth(theme,d.outlineStrokeRole));
  if(title!==undefined){items.push({kind:"text",x,y,content:title,size:Number(o.titleSize??d.titleSize),font:printFontFamily(d.titleFont),fill:textFill,anchor:"start",baseline:"top"});y-=Math.max(swatch,Number(o.titleSize??d.titleSize)*25.4/72.27)+gap;}
  let advance=0;
  if(kind==="gradient"){
    const domain=scale.domain(),lo=Number(domain[0]),hi=Number(domain.at(-1)),ramp=vizSchemeInterpolator(theme.palette,"primary","oklab");
    for(let slice=0;slice<48;slice++){checkpoint();const fraction=(slice+.5)/48,mapped=map(lo+fraction*(hi-lo));items.push({kind:"rect",x:x+slice/48*length,y:y-swatch,width:length/48,height:swatch,fill:typeof mapped==="string"?mapped:ramp(fraction)});}
    for(const value of values)items.push({kind:"text",x:x+(Number(value)-lo)/(hi-lo+1e-12)*length,y:y-swatch-.5,content:label(value),size,font:printFontFamily(d.labelFont),fill:textFill,anchor:"middle",baseline:"top"});
  }else for(let index=0;index<values.length;index++){
    checkpoint();const value=values[index],content=label(value),mapped=map(value),col=horizontal?x+advance:x+index%columns*(length/columns),row=y-swatch-(horizontal?0:Math.floor(index/columns)*(Math.max(swatch,size*25.4/72.27)+gap)),paint=typeof mapped==="string"&&channel!=="shape"&&channel!=="dash"?mapped:categorical(index);
    if(kind==="size"){const radius=String(o.sizeMode??d.sizeMode)==="radius"?Number(mapped):Math.sqrt(Math.max(0,Number(mapped))/Math.PI),cx=x+advance+radius,cy=y-swatch;items.push({kind:"circle",cx,cy,r:radius,stroke:String(o.stroke??textFill),strokeWidth:outline},{kind:"text",x:cx,y:cy-radius-.5,content,size,font:printFontFamily(d.labelFont),fill:textFill,anchor:"middle",baseline:"top"});advance+=2*radius+gap;continue;}
    if(kind==="line")items.push({kind:"line",x1:col,y1:row+swatch/2,x2:col+swatch,y2:row+swatch/2,stroke:paint,strokeWidth:stroke,dash:channel==="dash"?dashArray(mapped):undefined});
    else if(kind==="symbol"){const path=vizPathRecorder();drawVizSymbol(String(channel==="shape"?mapped:o.symbol??d.symbol) as VizSymbolKind,path,swatch*swatch);items.push({kind:"path",commands:translateCommands(path.commands,col+swatch/2,height-row-swatch/2),fill:paint});}
    else if(kind==="pattern"){const count=Number(o.hatchLines??d.hatchLines);if(!Number.isInteger(count)||count<0)throw new Error("legend hatch count requires a nonnegative integer");items.push({kind:"rect",x:col,y:row,width:swatch,height:swatch,stroke:paint,strokeWidth:outline});for(let line=1;line<=count;line++)items.push({kind:"line",x1:col+line/(count+1)*swatch,y1:row,x2:col,y2:row+line/(count+1)*swatch,stroke:paint,strokeWidth:outline});}
    else items.push({kind:"rect",x:col,y:row,width:swatch,height:swatch,fill:paint});
    items.push({kind:"text",x:col+swatch+labelGap,y:row+swatch/2,content,size,font:printFontFamily(d.labelFont),fill:textFill,anchor:"start",baseline:"middle"});advance+=swatch+labelGap+measurePrintSans(content,size)+2*gap;
  }
  const projected=items.map(item=>item.kind==="rect"?{...item,y:height-item.y-item.height}:item.kind==="circle"?{...item,cy:height-item.cy}:item.kind==="line"?{...item,y1:height-item.y1,y2:height-item.y2}:item.kind==="text"?{...item,y:height-item.y}:item);
  return o.style===undefined?projected:projected.map(item=>({...item,tikzStyle:String(o.style)}));
}
function scalesOf(spec: VizChartSpecification): Map<string, VizScale<never, never>> {
  return new Map((spec.scales ?? []).map((s) => [s.name, buildVizScale(s, spec.theme?.appearance ?? "light")]));
}
function channel(scales: Map<string, VizScale<never, never>>, layer: VizLayerSpec, name: string, row: VizRow, fallback: unknown): unknown {
  const e = layer.encodings?.[name as keyof NonNullable<VizLayerSpec["encodings"]>];
  const raw = e && Object.hasOwn(e, "value") ? e.value : e ? row[e.column ?? name] : Object.hasOwn(row, name) ? row[name] : fallback;
  if (e?.scale === undefined) return raw;
  const s = scales.get(e.scale);
  if (s === undefined) throw new Error(`unknown scale ${e.scale}`);
  return raw == null && s.kind === "diverging" ? s.unknown?.() : (s as unknown as (value: unknown) => unknown)(raw);
}
function numberChannel(scales: Map<string, VizScale<never, never>>, layer: VizLayerSpec, name: string, row: VizRow, fallback: number): number {
  const v = channel(scales, layer, name, row, fallback);
  return v == null ? NaN : Number(v);
}
function dashArray(value: unknown): number[] | undefined {
  if (value === undefined || value === false || value === "") return undefined;
  const out = String(value).split(/[ ,]+/).map(Number);
  if (out.some((v) => !Number.isFinite(v) || v < 0)) throw new Error("dash lengths must be finite and nonnegative");
  return out.some((v) => v > 0) ? out : undefined;
}
function styleOf(scales: Map<string, VizScale<never, never>>, layer: VizLayerSpec, row: VizRow, color: string, theme: VizTheme, stroked = false): VizRenderStyle {
  const o = layer.options ?? {};
  const fill = channel(scales, layer, "fill", row, o.fill ?? (stroked ? undefined : color));
  const stroke = channel(scales, layer, "stroke", row, o.stroke ?? (stroked ? color : undefined));
  return { fill: fill == null || fill === "none" ? undefined : String(fill), stroke: stroke == null || stroke === "none" ? undefined : String(stroke), strokeWidth: Math.max(0, numberChannel(scales, layer, "strokeWidth", row, Number(o.strokeWidth ?? theme.strokes.gridMajor ?? 0.3))), opacity: Math.max(0, Math.min(1, numberChannel(scales, layer, "opacity", row, Number(o.opacity ?? 1)))), dash: dashArray(channel(scales, layer, "dash", row, o.dash)), rotation: numberChannel(scales, layer, "rotation", row, Number(o.rotation ?? 0)), cap: String(o.cap ?? "butt") as VizRenderStyle["cap"], join: String(o.join ?? "miter") as VizRenderStyle["join"] };
}
function translateCommands(commands: readonly VizPathCommand[], x: number, y: number): VizPathCommand[] {
  return commands.map((c) => ({ ...c, args: c.args.map((v, i) => c.op === "arc" || c.op === "rect" ? (i === 0 ? v + x : i === 1 ? v + y : v) : c.op === "arcTo" && i === 4 ? v : i % 2 === 0 ? v + x : v + y) }) as unknown as VizPathCommand);
}
function seriesOf(rows: readonly VizRow[], layer: VizLayerSpec, scales: Map<string, VizScale<never, never>>): VizRow[][] {
  const groups = new Map<unknown, VizRow[]>();
  for (const row of rows) { const key = channel(scales, layer, "detail", row, ""); const group = groups.get(key) ?? []; group.push(row); groups.set(key, group); }
  return [...groups.values()].map((g) => layer.encodings?.order === undefined ? g : g.sort((a,b) => numberChannel(scales,layer,"order",a,0)-numberChannel(scales,layer,"order",b,0)));
}
/** 🖼️ Resolves inferred data, encodings, coordinates, marks, guides and annotations. */
export type VizRenderControl = { readonly signal?: AbortSignal; readonly onProgress?: (completed: number, total: number) => void };
export function planVizChart(spec: VizChartSpecification, control: VizRenderControl = {}): VizRenderPlan {
  control.signal?.throwIfAborted(); control.onProgress?.(0,spec.layers.length);
  let renderedRows=0; const checkpoint=():void=>{ if(++renderedRows%128===0)control.signal?.throwIfAborted(); };
  if (![spec.width,spec.height].every((v) => Number.isFinite(v) && v > 0)) throw new Error("chart dimensions must be finite and positive");
  const margin = spec.margin ?? { top:8,right:8,bottom:14,left:16 };
  const frame: VizExtent = { x0:margin.left,y0:margin.top,x1:spec.width-margin.right,y1:spec.height-margin.bottom };
  if (frame.x1 < frame.x0 || frame.y1 < frame.y0) throw new Error("chart margins exceed dimensions");
  if ((spec.presets?.length ?? 0) > 0) throw new Error("family presets must be rendered through print family inference");
  const theme = vizTheme(spec.theme?.appearance ?? "light",spec.theme?.name ?? "semio");
  const palette = spec.theme?.palette ?? theme.palette.categorical;
  const color = (i:number):string => palette[((i%palette.length)+palette.length)%palette.length] ?? vizCategoricalColor(theme.palette,i);
  const scales = scalesOf(spec);
  const projection = spec.coordinate?.kind === "geographic" ? vizGeoProjection((spec.coordinate.options?.projection ?? "equirectangular") as VizProjectionKind,{scale:Math.min((frame.x1-frame.x0)/(2*Math.PI),(frame.y1-frame.y0)/Math.PI),translate:[(frame.x0+frame.x1)/2,(frame.y0+frame.y1)/2]}) : undefined;
  const coordinate = projection === undefined ? buildVizCoordinate(spec.coordinate?.kind ?? "cartesian",frame,spec.coordinate?.options ?? {}) : coordinateGeographic(frame,(x,y)=>projection([x,y]),projection.invert===undefined?undefined:(x,y)=>projection.invert!([x,y])??[NaN,NaN]);
  const projected = coordinate.kind !== "cartesian" || spec.coordinate?.options?.normalized === true || spec.coordinate?.options?.transpose === true;
  const items: VizRenderItem[] = [];
  const localized = (text:{en:string;de:string}):string => { if (spec.language === undefined) throw new Error("localized text requires an explicit language"); return text[spec.language]; };
  const normalize = (layer:VizLayerSpec,name:string,v:number):number => {
    const sn = layer.encodings?.[name as keyof NonNullable<VizLayerSpec["encodings"]>]?.scale;
    const range = sn === undefined ? undefined : scales.get(sn)?.range();
    if (range === undefined || range.length<2 || typeof range[0] !== "number" || typeof range[range.length-1] !== "number") return v;
    const lo=Number(range[0]),hi=Number(range[range.length-1]); return hi===lo ? 0.5 : (v-lo)/(hi-lo);
  };
  const point = (layer:VizLayerSpec,row:VizRow,xn="x",yn="y",xf=0,yf=0,center=true):VizPoint => {
    checkpoint();
    let x=numberChannel(scales,layer,xn,row,xf),y=numberChannel(scales,layer,yn,row,yf);
    if(center) { const xs=layer.encodings?.[xn as keyof NonNullable<VizLayerSpec["encodings"]>]?.scale,ys=layer.encodings?.[yn as keyof NonNullable<VizLayerSpec["encodings"]>]?.scale; x+=xs===undefined?0:(scales.get(xs)?.bandwidth?.()??0)/2; y+=ys===undefined?0:(scales.get(ys)?.bandwidth?.()??0)/2; }
    if(!projected || layer.layout!==undefined)return [x,y];
    if(coordinate.kind==="geographic")return coordinate.project(x,y);
    const xScaleName=layer.encodings?.[xn as keyof NonNullable<VizLayerSpec["encodings"]>]?.scale??(xn==="x2"?layer.encodings?.x?.scale:undefined);
    const yScaleName=layer.encodings?.[yn as keyof NonNullable<VizLayerSpec["encodings"]>]?.scale??(yn==="y2"?layer.encodings?.y?.scale:undefined);
    const missing=(name:string):boolean=>layer.encodings?.[name as keyof NonNullable<VizLayerSpec["encodings"]>]===undefined&&!Object.hasOwn(row,name);
    if(missing(xn)&&xScaleName===undefined) x=xf===frame.x0?0:xf===frame.x1?1:x;
    if(missing(yn)&&yf===frame.y1) y=yScaleName===undefined?0:Number((scales.get(yScaleName) as unknown as (v:number)=>number)(0));
    else if(missing(yn)&&yScaleName===undefined&&yf===frame.y0)y=1;
    return coordinate.project(normalize(layer,xScaleName===layer.encodings?.x?.scale?"x":xn,x),normalize(layer,yScaleName===layer.encodings?.y?.scale?"y":yn,y));
  };
  const finite=(p:VizPoint):boolean=>p.every(Number.isFinite);
  for (const guide of spec.guides ?? []) {
    const o=guide.options??{},scaleName=String(o.scale??guide.scale??VIZ_AXIS_DEFAULTS.scale),titleValue=o.title===undefined?(guide.title===undefined?undefined:localized(guide.title)):String(o.title),title=titleValue===""?undefined:titleValue;
    const orientation=String(o.orient??guide.orient??(guide.kind==="legend"?VIZ_LEGEND_DEFAULTS.orient:VIZ_AXIS_DEFAULTS.orient));
    const s=scales.get(scaleName);
    if(s===undefined) throw new Error(`unknown guide scale ${scaleName}`);
    const declaredScale=spec.scales?.find(entry=>entry.name===scaleName),format=o.tickFormat===undefined?guide.tickFormat??declaredScale?.options?.tickFormat:String(o.tickFormat),authoredValues=o.tickValues===undefined?guide.tickValues:String(o.tickValues).split(",").map(value=>value.trim()).filter(value=>value!=="");
    const values:readonly unknown[]=(authoredValues?.length?authoredValues:undefined)??declaredScale?.options?.tickValues??s.ticks?.(Number(o.ticks??guide.ticks??declaredScale?.options?.ticks??5))??s.domain();
    const label=(v:unknown):string=>{ if(format===undefined)return String(v); if(spec.language===undefined)throw new Error("formatted guide requires explicit language"); return spec.scales?.find((a)=>a.name===scaleName)?.kind==="temporal"?formatVizTime(format,typeof v==="string"?Date.parse(v):Number(v),spec.language):formatVizValue(format,Number(v),spec.language); };
    if(guide.kind==="legend") {
      const legendFormat=o.tickFormat??o.format??format,legendLabel=(value:unknown):string=>{if(legendFormat===undefined)return String(value);if(spec.language===undefined)throw new Error("formatted legend requires explicit language");return declaredScale?.kind==="temporal"?formatVizTime(String(legendFormat),typeof value==="string"?Date.parse(value):Number(value),spec.language):formatVizValue(String(legendFormat),Number(value),spec.language);};
      items.push(...legendGuideItems(guide,orientation,s,spec.width,spec.height,authoredValues,legendLabel,title,theme,palette,checkpoint));
    } else items.push(...axisGuideItems(guide,orientation,s,frame,spec.width,spec.height,values,label,title,theme));
  }
  spec.layers.forEach((layer,li)=>{
    control.signal?.throwIfAborted(); control.onProgress?.(li,spec.layers.length); control.signal?.throwIfAborted();
    const rows=inferVizLayerTable(spec,layer,{signal:control.signal,onProgress:(done,total)=>{control.onProgress?.(li+0.5*(total===0?1:done/total),spec.layers.length);control.signal?.throwIfAborted();}}).rows,o=layer.options??{},fill=color(li);
    let styledRows=0;
    const style=(row:VizRow,line=false):VizRenderStyle=>{if(++styledRows%128===0){control.onProgress?.(li+0.5+Math.min(0.49,0.5*styledRows/Math.max(1,rows.length)),spec.layers.length);control.signal?.throwIfAborted();}return {...styleOf(scales,layer,row,fill,theme,line),...(o.clip===true?{clip:frame}:{})};};
    const curve=vizCurve(String(o.curve??"linear") as VizCurveKind,{tension:Number(o.tension??0),alpha:Number(o.alpha??0.5),beta:Number(o.beta??0.85)});
    switch(layer.mark) {
      case "bar":case "rect":case "heatmap": {
        const project = projected && layer.layout === undefined;
        const bandwidth=(name:"x"|"y",fallback:number):number=>{ const sn=layer.encodings?.[name]?.scale;return sn===undefined?fallback:scales.get(sn)?.bandwidth?.()??fallback; };
        const baseline=(name:"x"|"y"):number=>{ const sn=layer.encodings?.[name]?.scale;const s=sn===undefined?undefined:scales.get(sn);return s===undefined?(project?0:name==="x"?frame.x0:frame.y1):Number((s as unknown as (value:number)=>number)(0)); };
        for(const row of rows) {
          const horizontal=o.orientation==="horizontal";
          const x=numberChannel(scales,layer,"x",row,project?0:frame.x0),y=numberChannel(scales,layer,"y",row,project?0:frame.y1);
          const width=numberChannel(scales,layer,"width",row,Number(o.width??bandwidth(horizontal?"y":"x",project?0.1:6)));
          const x2=numberChannel(scales,layer,"x2",row,layer.mark==="bar"&&horizontal?baseline("x"):x+width);
          const y2=numberChannel(scales,layer,"y2",row,layer.mark==="bar"&&!horizontal?baseline("y"):y+bandwidth("y",Number(o.height??width)));
          if(![x,y,x2,y2,width].every(Number.isFinite))continue;
          if(project) {
            const nx=normalize(layer,"x",x),ny=normalize(layer,"y",y),nx2=normalize(layer,layer.encodings?.x2?.scale===undefined?"x":"x2",x2),ny2=normalize(layer,layer.encodings?.y2?.scale===undefined?"y":"y2",y2);
            if(coordinate.kind==="polar"||coordinate.kind==="logpolar") {
              const co=spec.coordinate?.options??{},start=co.startAngle??0,end=co.endAngle??2*Math.PI,inner=co.innerRadius??0,outer=co.outerRadius??Math.min(frame.x1-frame.x0,frame.y1-frame.y0)/2;
              const radius=(v:number):number=>inner+(coordinate.kind==="logpolar"?Math.log(1+v*((co.base??10)-1))/Math.log(co.base??10):v)*(outer-inner);
              items.push({kind:"path",commands:translateCommands(vizArc({startAngle:start+nx*(end-start),endAngle:start+nx2*(end-start),innerRadius:Math.min(radius(ny),radius(ny2)),outerRadius:Math.max(radius(ny),radius(ny2)),padAngle:Number(o.padAngle??0),cornerRadius:Number(o.cornerRadius??0)}),(frame.x0+frame.x1)/2,(frame.y0+frame.y1)/2),...style(row)});
            } else items.push({kind:"polygon",points:[coordinate.project(nx,ny),coordinate.project(nx2,ny),coordinate.project(nx2,ny2),coordinate.project(nx,ny2)],...style(row)});
          } else items.push({kind:"rect",x:layer.mark==="bar"&&!horizontal?x:Math.min(x,x2),y:layer.mark==="bar"&&horizontal?y:Math.min(y,y2),width:layer.mark==="bar"&&!horizontal?width:Math.abs(x2-x),height:layer.mark==="bar"&&horizontal?width:Math.abs(y2-y),...style(row)});
        }
        return;
      }
      case "point":case "circle":case "symbol": {
        for(const row of rows){const symbol=String(channel(scales,layer,"shape",row,o.symbol??(layer.layout?.algorithm==="hexbin"?"hexagon":"circle"))) as VizSymbolKind,size=numberChannel(scales,layer,"size",row,Number(o.size??16)),[x,y]=point(layer,row);if(!finite([x,y])||!Number.isFinite(size)||size<0)continue;
          if(symbol==="circle")items.push({kind:"circle",cx:x,cy:y,r:layer.mark==="circle"||layer.layout?.algorithm==="pack"?numberChannel(scales,layer,"radius",row,Math.sqrt(size/Math.PI)):Math.sqrt(size/Math.PI),...style(row)});
          else{const r=vizPathRecorder();if(symbol as string === "hexagon"){const radius=numberChannel(scales,layer,"radius",row,Number(o.radius??Math.sqrt(size/Math.PI)));for(let i=0;i<6;i++){const a=i*Math.PI/3-Math.PI/2; if(i===0)r.moveTo(radius*Math.cos(a),radius*Math.sin(a));else r.lineTo(radius*Math.cos(a),radius*Math.sin(a));}r.closePath();}else drawVizSymbol(symbol,r,size);items.push({kind:"path",commands:translateCommands(r.commands,x,y),...style(row)});}
        }return;
      }
      case "line":case "path":case "trail":case "polygon":case "area":case "band": {
        for(const group of seriesOf(rows,layer,scales)){if(group.length===0)continue;const points=group.map((r)=>point(layer,r)),area=layer.mark==="area"||layer.mark==="band",bottom=group.map((r)=>point(layer,r,"x2","y2",numberChannel(scales,layer,"x",r,0),frame.y1)),defined=(_r:VizRow,i:number):boolean=>finite(points[i]!)&&(!area||finite(bottom[i]!));
          if(layer.mark==="polygon"){const valid=points.filter(finite);if(valid.length>0)items.push({kind:"polygon",points:valid,...style(group[0]!)});continue;}
          const styles = group.map((row) => style(row,!area));
          const varied = styles.some((s) => JSON.stringify(s) !== JSON.stringify(styles[0]));
          if(varied || layer.mark === "trail") {
            for(let i=1;i<group.length;i++) {
              if(!defined(group[i-1]!,i-1)||!defined(group[i]!,i))continue;
              const pair=[group[i-1]!,group[i]!];
              const commands=area?vizArea(pair,{x0:(_,j)=>bottom[i-1+j]![0],y0:(_,j)=>bottom[i-1+j]![1],x1:(_,j)=>points[i-1+j]![0],y1:(_,j)=>points[i-1+j]![1],curve}):vizLine([points[i-1]!,points[i]!],{curve});
              items.push({kind:"path",commands,...styles[i-1]!,...(layer.mark==="trail"?{strokeWidth:numberChannel(scales,layer,"size",group[i-1]!,Number(o.size??styles[i-1]!.strokeWidth??0.3))}:{})});
            }
            continue;
          }
          const commands=area?vizArea(group,{x0:(_,i)=>bottom[i]![0],y0:(_,i)=>bottom[i]![1],x1:(_,i)=>points[i]![0],y1:(_,i)=>points[i]![1],defined,curve}):vizLine(group,{x:(_,i)=>points[i]![0],y:(_,i)=>points[i]![1],defined,curve});if(commands.length>0)items.push({kind:"path",commands,...style(group[0]!,!area)});
        }return;
      }
      case "arc":case "pie":case "ribbon": {
        for(const [i,row] of rows.entries()){checkpoint();const center:VizPoint=[numberChannel(scales,layer,"x",row,Number(o.cx??(frame.x0+frame.x1)/2)),numberChannel(scales,layer,"y",row,Number(o.cy??(frame.y0+frame.y1)/2))],startAngle=numberChannel(scales,layer,"angle",row,Number(row.startAngle??o.startAngle??0)),endAngle=numberChannel(scales,layer,"endAngle",row,numberChannel(scales,layer,"x2",row,Number(row.endAngle??o.endAngle??startAngle))),radius=numberChannel(scales,layer,"radius",row,Number(row.outerRadius??o.outerRadius??Math.min(frame.x1-frame.x0,frame.y1-frame.y0)/2)),innerRadius=numberChannel(scales,layer,"innerRadius",row,Number(o.innerRadius??0));if(![...center,startAngle,endAngle,radius,innerRadius].every(Number.isFinite))continue;
          const commands=layer.mark==="ribbon"?vizRibbon({startAngle,endAngle,radius},{startAngle:Number(row.targetStartAngle??o.targetStartAngle??startAngle),endAngle:Number(row.targetEndAngle??o.targetEndAngle??endAngle),radius:Number(row.targetRadius??o.targetRadius??radius)},{headRadius:Number(o.headRadius??0)}):vizArc({innerRadius,outerRadius:radius,startAngle,endAngle,padAngle:Number(row.padAngle??o.padAngle??0),cornerRadius:Number(o.cornerRadius??0),padRadius:o.padRadius===undefined?undefined:Number(o.padRadius)});
          items.push({kind:"path",commands:translateCommands(commands,center[0],center[1]),...styleOf(scales,layer,row,color(i),theme),...(o.clip===true?{clip:frame}:{})});
        }return;
      }
      case "text":{for(const row of rows){const [x,y]=point(layer,row),size=numberChannel(scales,layer,"size",row,Number(o.size??theme.typography.textSmPx??10));if(finite([x,y])&&Number.isFinite(size))items.push({kind:"text",x,y,content:String(channel(scales,layer,"text",row,o.text??"")),size,anchor:String(o.anchor??"middle") as "start"|"middle"|"end",...style(row)});}return;}
      case "rule":case "link":{for(const row of rows){const hasY=layer.encodings?.y!==undefined||row.y!==undefined,hasX=layer.encodings?.x!==undefined||row.x!==undefined,vertical=o.orientation==="vertical"||(hasX&&!hasY),[x1,y1]=point(layer,row,"x","y",frame.x0,vertical?frame.y0:frame.y1),[px2,py2]=point(layer,row,"x2","y2",frame.x1,frame.y1),x2=vertical&&layer.encodings?.x2===undefined&&row.x2===undefined?x1:px2,y2=!vertical&&layer.encodings?.y2===undefined&&row.y2===undefined?y1:py2;if(![x1,y1,x2,y2].every(Number.isFinite))continue;if(layer.mark==="link")items.push({kind:"path",commands:vizLink(String(o.orientation??"horizontal") as VizLinkKind,[x1,y1],[x2,y2]),...style(row,true)});else items.push({kind:"line",x1,y1,x2,y2,...style(row,true)});}return;}
      default:throw new Error(`unknown mark ${layer.mark}`);
    }
  });
  control.onProgress?.(spec.layers.length,spec.layers.length); control.signal?.throwIfAborted();
  for(const a of spec.annotations??[]){const o=a.options??{},x=a.x??0,y=a.y??0,s:VizRenderStyle={fill:o.fill===undefined?undefined:String(o.fill),stroke:o.stroke===undefined?undefined:String(o.stroke),strokeWidth:Number(o.strokeWidth??0.3),opacity:Number(o.opacity??1),dash:dashArray(o.dash),rotation:Number(o.rotation??0)};
    if(a.kind==="text"||a.kind==="label")items.push({kind:"text",x,y,content:a.text===undefined?"":localized(a.text),size:Number(o.size??theme.typography.textSmPx??10),anchor:String(o.anchor??"middle") as "start"|"middle"|"end",...s});
    else if(a.kind==="rect")items.push({kind:"rect",x,y,width:a.width??(a.x2??x)-x,height:a.height??(a.y2??y)-y,...s});
    else if(a.kind==="circle"||a.kind==="point")items.push({kind:"circle",cx:x,cy:y,r:a.radius??2,...s});
    else if(a.kind==="line"||a.kind==="rule")items.push({kind:"line",x1:x,y1:y,x2:a.x2??x,y2:a.y2??y,...s,stroke:s.stroke??color(0)});
    else throw new Error(`unknown annotation ${a.kind}`);
  }
  if(spec.title!==undefined)items.push({kind:"text",x:spec.width/2,y:margin.top/2,content:localized(spec.title),size:theme.typography.textSmPx??10,anchor:"middle",fill:theme.appearance==="dark"?"#ffffff":"#000000"});
  return {width:spec.width,height:spec.height,frame,theme,items};
}
//#endregion 🔖️Layout

//#region 🔖️SceneGraph
const IDENTITY: readonly [number, number, number, number, number, number] = [1, 0, 0, 1, 0, 0];

function sceneColor(hex: string, opacity = 1): readonly [number, number, number, number] {
  const [r, g, b, a] = vizParseColor(hex);
  return [r / 255, g / 255, b / 255, (a / 255) * opacity];
}

function itemTransform(item: VizRenderItem): SceneNode["transform"] {
  if (!item.rotation) return IDENTITY;
  const x = item.kind === "circle" ? item.cx : item.kind === "line" ? item.x1 : "x" in item ? item.x : 0;
  const y = item.kind === "circle" ? item.cy : item.kind === "line" ? item.y1 : "y" in item ? item.y : 0;
  const a = item.rotation * Math.PI / 180, c = Math.cos(a), s = Math.sin(a);
  return [c, s, -s, c, x - c * x + s * y, y - s * x - c * y];
}
function sceneNode(item: VizRenderItem): SceneNode {
  const fill = item.fill === undefined ? undefined : { kind: "solid" as const, color: sceneColor(item.fill) };
  const stroke = item.stroke === undefined ? undefined : { color: sceneColor(item.stroke), width: item.strokeWidth ?? 0.2, cap: item.cap ?? "butt", join: item.join ?? "miter", dash: item.dash === undefined ? undefined : [...item.dash] };
  const base = { transform: itemTransform(item), opacity: item.opacity, clip: item.clip === undefined ? undefined : pathSegments([{op:"rect",args:[item.clip.x0,item.clip.y0,item.clip.x1-item.clip.x0,item.clip.y1-item.clip.y0]}]), fill, stroke };
  switch (item.kind) {
    case "rect": return { ...base, node: { kind: "rect", x:item.x,y:item.y,width:item.width,height:item.height } };
    case "circle": return { ...base, node: { kind:"circle",cx:item.cx,cy:item.cy,r:item.r } };
    case "line": return { ...base, node: { kind:"line",x1:item.x1,y1:item.y1,x2:item.x2,y2:item.y2 } };
    case "polygon": return { ...base, node: { kind:"polygon",points:item.points.map((p)=>[p[0],p[1]] as const) } };
    case "text": return { ...base, node: { kind:"text",x:item.x,y:item.y,content:item.content,size:item.size*25.4/72.27,...(item.font===undefined?{}:{font:item.font}),anchor:item.anchor ?? "middle",baseline:item.baseline??"middle" } as SceneNode["node"], fill:fill??{kind:"solid",color:[0,0,0,1]} };
    default: return { ...base, node: { kind:"path",segments:pathSegments(item.commands) } };
  }
}

function expandedCommands(commands: readonly VizPathCommand[]): VizPathCommand[] {
  const out: VizPathCommand[] = [];
  let cursor: VizPoint | undefined, start: VizPoint | undefined;
  const append = (command: VizPathCommand): void => {
    out.push(command);
    if (command.op === "moveTo") { cursor = command.args; start = cursor; }
    else if (command.op === "lineTo") cursor = command.args;
    else if (command.op === "quadraticCurveTo") cursor = [command.args[2],command.args[3]];
    else if (command.op === "bezierCurveTo") cursor = [command.args[4],command.args[5]];
    else if (command.op === "closePath") cursor = start;
  };
  const arc = (cx:number,cy:number,r:number,a0:number,a1:number,ccw:number): void => {
    const p:VizPoint = [cx+r*Math.cos(a0),cy+r*Math.sin(a0)];
    if(cursor === undefined) append({op:"moveTo",args:p});
    else if(Math.hypot(cursor[0]-p[0],cursor[1]-p[1])>1e-9) append({op:"lineTo",args:p});
    const tau=2*Math.PI, raw=ccw?a0-a1:a1-a0;
    const sweep=raw>=tau?tau:((raw%tau)+tau)%tau;
    const n=Math.max(1,Math.ceil(sweep/Math.PI)),delta=(ccw?-1:1)*sweep/n;
    for(let i=0;i<n;i++) { out.push({op:"arc",args:[cx,cy,r,a0+i*delta,a0+(i+1)*delta,ccw]}); }
    cursor=[cx+r*Math.cos(a0+(ccw?-1:1)*sweep),cy+r*Math.sin(a0+(ccw?-1:1)*sweep)];
  };
  for (const c of commands) {
    if(c.op === "arc") { arc(...c.args); continue; }
    if(c.op !== "arcTo") { append(c); continue; }
    const [x1,y1,x2,y2,r] = c.args;
    if(cursor === undefined) { append({op:"moveTo",args:[x1,y1]}); continue; }
    const ax=cursor[0]-x1,ay=cursor[1]-y1,bx=x2-x1,by=y2-y1,al=Math.hypot(ax,ay),bl=Math.hypot(bx,by),cross=ax*by-ay*bx;
    if(r<=0||al<1e-12||bl<1e-12||Math.abs(cross)<1e-12) { append({op:"lineTo",args:[x1,y1]});continue; }
    const ux=ax/al,uy=ay/al,vx=bx/bl,vy=by/bl,angle=Math.acos(Math.max(-1,Math.min(1,ux*vx+uy*vy))),distance=r/Math.tan(angle/2),tx=x1+ux*distance,ty=y1+uy*distance,sign=cross<0?1:-1,cx=tx-sign*uy*r,cy=ty+sign*ux*r;
    arc(cx,cy,r,Math.atan2(ty-cy,tx-cx),Math.atan2(y1+vy*distance-cy,x1+vx*distance-cx),cross>0?1:0);
  }
  return out;
}

function pathSegments(commands: readonly VizPathCommand[]): readonly PathSegment[] {
  const segments: PathSegment[] = [];
  let cursor: VizPoint = [0, 0];
  for (const command of expandedCommands(commands)) {
    switch (command.op) {
      case "moveTo":
        cursor = [command.args[0], command.args[1]];
        segments.push({ kind: "move", to: [cursor[0], cursor[1]] });
        break;
      case "lineTo":
        cursor = [command.args[0], command.args[1]];
        segments.push({ kind: "line", to: [cursor[0], cursor[1]] });
        break;
      case "quadraticCurveTo":
        cursor = [command.args[2], command.args[3]];
        segments.push({ kind: "quad", ctrl: [command.args[0], command.args[1]], to: [cursor[0], cursor[1]] });
        break;
      case "bezierCurveTo":
        cursor = [command.args[4], command.args[5]];
        segments.push({ kind: "cubic", ctrl1: [command.args[0], command.args[1]], ctrl2: [command.args[2], command.args[3]], to: [cursor[0], cursor[1]] });
        break;
      case "arc": {
        const [cx, cy, r, a0, a1, ccw] = command.args;
        const to: VizPoint = [cx + r * Math.cos(a1), cy + r * Math.sin(a1)];
        const sweep = ccw === 0;
        let delta = a1 - a0;
        if (sweep) while (delta < 0) delta += 2 * Math.PI;
        else while (delta > 0) delta -= 2 * Math.PI;
        segments.push({ kind: "arc", rx: r, ry: r, rotation: 0, largeArc: Math.abs(delta) > Math.PI, sweep, to: [to[0], to[1]] });
        cursor = to;
        break;
      }
      case "rect":
        segments.push({ kind: "move", to: [command.args[0], command.args[1]] });
        segments.push({ kind: "line", to: [command.args[0] + command.args[2], command.args[1]] });
        segments.push({ kind: "line", to: [command.args[0] + command.args[2], command.args[1] + command.args[3]] });
        segments.push({ kind: "line", to: [command.args[0], command.args[1] + command.args[3]] });
        segments.push({ kind: "close" });
        break;
      default:
        segments.push({ kind: "close" });
    }
  }
  return segments;
}

/** 🖼️ Renders a chart specification into the dependency-free 2D scene graph. */
export function renderVizScene(spec: VizChartSpecification): DrawingScene {
  const plan = planVizChart(spec);
  return renderVizScenePlan(plan);
}
/** 🖌️ Emits one previously inferred render plan into the shared scene contract. */
export function renderVizScenePlan(plan: VizRenderPlan): DrawingScene {
  return { width: plan.width, height: plan.height, nodes: plan.items.map(sceneNode) };
}
//#endregion 🔖️SceneGraph

//#region 🔖️Tikz
function tikzNumber(value: number): string {
  return (Math.round(value * 1e4) / 1e4).toString();
}

function tikzColor(hex: string): string {
  const [r, g, b] = vizParseColor(hex);
  return `{rgb,255:red,${r};green,${g};blue,${b}}`;
}

function tikzPath(commands: readonly VizPathCommand[]): string {
  const parts: string[] = [];
  let cursor:VizPoint=[0,0],start:VizPoint=[0,0];
  const p=(x:number,y:number):string=>`(${tikzNumber(x)},${tikzNumber(y)})`;
  for(const c of expandedCommands(commands)) {
    const a=c.args;
    switch(c.op) {
      case "moveTo": parts.push(p(a[0]!,a[1]!));cursor=[a[0]!,a[1]!];start=cursor;break;
      case "lineTo": parts.push(`-- ${p(a[0]!,a[1]!)}`);cursor=[a[0]!,a[1]!];break;
      case "quadraticCurveTo": parts.push(`.. controls ${p(cursor[0]+2*(a[0]!-cursor[0])/3,cursor[1]+2*(a[1]!-cursor[1])/3)} and ${p(a[2]!+2*(a[0]!-a[2]!)/3,a[3]!+2*(a[1]!-a[3]!)/3)} .. ${p(a[2]!,a[3]!)}`);cursor=[a[2]!,a[3]!];break;
      case "bezierCurveTo": parts.push(`.. controls ${p(a[0]!,a[1]!)} and ${p(a[2]!,a[3]!)} .. ${p(a[4]!,a[5]!)}`);cursor=[a[4]!,a[5]!];break;
      case "arc": parts.push(`arc[start angle=${tikzNumber(-a[3]!*180/Math.PI)},end angle=${tikzNumber(-a[4]!*180/Math.PI)},radius=${tikzNumber(a[2]!)}mm]`);cursor=[a[0]!+a[2]!*Math.cos(a[4]!),a[1]!+a[2]!*Math.sin(a[4]!)];break;
      case "rect": parts.push(`${p(a[0]!,a[1]!)} rectangle ${p(a[0]!+a[2]!,a[1]!+a[3]!)}`);cursor=[a[0]!,a[1]!];break;
      case "closePath": parts.push("-- cycle");cursor=start;break;
    }
  }
  return parts.join(" ");
}

function tikzText(value:string):string {
  const escapes:Record<string,string>={"\\":"\\textbackslash{}","{":"\\{","}":"\\}","$":"\\$","&":"\\&","#":"\\#","%":"\\%","_":"\\_","^":"\\textasciicircum{}","~":"\\textasciitilde{}"};
  return value.replace(/[\\{}$&#%_^~]/g,(c)=>escapes[c]!).replace(/\r?\n/g,"\\\\");
}

function tikzItem(item: VizRenderItem): string {
  const textAnchor=item.kind!=="text"?"":[item.baseline==="alphabetic"?"base":item.baseline==="top"?"north":item.baseline==="bottom"?"south":item.anchor==="start"||item.anchor==="end"?"":"center",item.anchor==="start"?"west":item.anchor==="end"?"east":""].filter(Boolean).join(" ");
  const textOpacity=item.kind==="text"?(item.opacity??1)*vizParseColor(item.fill??"#000000")[3]/255:1;
  const paint:string[]=[];
  if(item.fill!==undefined&&item.kind!=="text") { paint.push(`fill=${tikzColor(item.fill)}`); const alpha=vizParseColor(item.fill)[3]/255; if(alpha<1)paint.push(`fill opacity=${tikzNumber(alpha*(item.opacity??1))}`); }
  if(item.stroke!==undefined) { paint.push(`draw=${tikzColor(item.stroke)}`,`line width=${tikzNumber(item.strokeWidth??0.2)}mm`,`line cap=${item.cap??"butt"}`,`line join=${item.join??"miter"}`); const alpha=vizParseColor(item.stroke)[3]/255;if(alpha<1)paint.push(`draw opacity=${tikzNumber(alpha*(item.opacity??1))}`); }
  if(item.opacity!==undefined&&item.opacity<1)paint.unshift(`opacity=${tikzNumber(item.opacity)}`);
  if(item.dash!==undefined&&item.dash.length>0)paint.push(`dash pattern=${item.dash.map((v,i)=>`${i%2===0?"on":"off"} ${tikzNumber(v)}mm`).join(" ")}`);
  if(item.rotation&&item.kind!=="text") { const x=item.kind==="circle"?item.cx:item.kind==="line"?item.x1:"x"in item?item.x:0,y=item.kind==="circle"?item.cy:item.kind==="line"?item.y1:"y"in item?item.y:0;paint.push(`rotate around={${tikzNumber(-item.rotation)}:(${tikzNumber(x)},${tikzNumber(y)})}`); }
  if(item.tikzStyle!==undefined&&item.tikzStyle!==""&&item.kind!=="text")paint.push(item.tikzStyle);
  const options=paint.length===0?"":`[${paint.join(",")}]`;
  let line:string;
  switch(item.kind) {
    case "rect":line=`\\path${options} (${tikzNumber(item.x)},${tikzNumber(item.y)}) rectangle ++(${tikzNumber(item.width)},${tikzNumber(item.height)});`;break;
    case "circle":line=`\\path${options} (${tikzNumber(item.cx)},${tikzNumber(item.cy)}) circle[radius=${tikzNumber(item.r)}mm];`;break;
    case "line":line=`\\path${options} (${tikzNumber(item.x1)},${tikzNumber(item.y1)}) -- (${tikzNumber(item.x2)},${tikzNumber(item.y2)});`;break;
    case "polygon":line=`\\path${options} ${item.points.map((p)=>`(${tikzNumber(p[0])},${tikzNumber(p[1])})`).join(" -- ")} -- cycle;`;break;
    case "text":line=`\\node[anchor=${textAnchor},text=${tikzColor(item.fill??"#000000")}${textOpacity===1?"":",text opacity="+tikzNumber(textOpacity)},rotate=${tikzNumber(-(item.rotation??0))},align=left,font=${item.font===undefined?"":printFontTexSelector(item.font)?"\\"+printFontTexSelector(item.font):"\\fontspec{"+tikzText(item.font)+"}"}\\fontsize{${tikzNumber(item.size)}}{${tikzNumber(item.size*1.2)}}\\selectfont${item.tikzStyle?","+item.tikzStyle:""}] at (${tikzNumber(item.x)},${tikzNumber(item.y)}) {${tikzText(item.content)}};`;break;
    default:line=`\\path${options} ${tikzPath(item.commands)};`;
  }
  return item.clip===undefined?line:`\\begin{scope}\n\\clip (${tikzNumber(item.clip.x0)},${tikzNumber(item.clip.y0)}) rectangle (${tikzNumber(item.clip.x1)},${tikzNumber(item.clip.y1)});\n${line}\n\\end{scope}`;
}
/** 🖼️ Renders a chart specification into TikZ source text, in figure millimetres. */
export function renderVizTikz(spec: VizChartSpecification): string {
  const plan = planVizChart(spec);
  return renderVizTikzPlan(plan);
}

/** 🖋️ Emits one previously inferred render plan into TikZ source. */
export function renderVizTikzPlan(plan: VizRenderPlan): string {
  const lines = [`\\begin{tikzpicture}[x=1mm,y=-1mm]`, `% ${plan.width}mm × ${plan.height}mm, ${plan.theme.appearance} appearance`, ...plan.items.map(tikzItem), `\\end{tikzpicture}`];
  return `${lines.join("\n")}\n`;
}
//#endregion 🔖️Tikz











