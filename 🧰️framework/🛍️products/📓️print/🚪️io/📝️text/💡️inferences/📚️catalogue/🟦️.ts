/** 📚️ Catalogue kinds are inferred into the same figure as custom numerical layers. */
import { vizParseColor } from "../../../../🧬️schema/💡️inferences/🎨theme/🟦️.ts";
import catalog from "../../../../🖼️assets/🔣️viz-catalog.json";
import schema from "../../../../🧬️schema/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import type { VizSchemaDocument, VizOptionSyntaxDescriptor } from "../../../../🧬️schema/🟦️.ts";
import type { VizChartSpecification, VizOptionValue, VizTable, VizRow } from "../../../../🧬️schema/📸️snapshot/📊️chart/🟦️.ts";

const KINDS = new Map(catalog.kinds.map((entry) => [entry.slug, entry]));
const escape = (value: string): string => value.replace(/[\\{}%#$&_^~]/g, (character) => ({ "\\": "\\textbackslash{}", "{": "\\{", "}": "\\}", "%": "\\%", "#": "\\#", "$": "\\$", "&": "\\&", "_": "\\_", "^": "\\textasciicircum{}", "~": "\\textasciitilde{}" })[character]!);
function token(value: string): string { if (!/^[A-Za-z0-9_.:/-]+$/.test(value)) throw new Error(`invalid grammar identifier ${JSON.stringify(value)}`); return value; }
type Paint = { readonly hex: string; readonly alpha: number; readonly name: string };
function color(value: string): Paint | undefined { try { const rgba = vizParseColor(value), hex = rgba.slice(0, 3).map(channel => Math.round(channel).toString(16).padStart(2, "0")).join("").toUpperCase(), alpha = rgba[3] / 255; return { hex, alpha, name: `semio-print-color-${hex}${alpha === 1 ? "" : `-A${String(alpha).replace(".", "p")}`}` }; } catch { return undefined; } }
const METADATA = schema as VizSchemaDocument;
type OptionValue = VizOptionValue | null | readonly (VizOptionValue | null)[];
function scalar(value: VizOptionValue | null | undefined): string { if (value === null || value === undefined) return ""; if (typeof value === "number" && !Number.isFinite(value)) throw new Error("grammar numbers must be finite"); return typeof value === "string" ? escape(value) : String(value); }
function rowScalar(value: VizOptionValue | null | undefined): string { return value === undefined ? "\\SemioVizUndefined{}" : value === null ? "\\SemioVizNull{}" : typeof value === "string" ? `\\SemioVizString{${escape(value)}}` : typeof value === "boolean" ? `\\SemioVizBoolean{${value}}` : scalar(value); }
function paint(value: VizOptionValue | null | undefined): string {
 if(typeof value!=="string")return scalar(value);
 const parsed=color(value);if(parsed)return parsed.name;
 const named=(name:string)=>name==="."||/^[A-Za-z][A-Za-z0-9_-]*$/.test(name),parts=value.split("!");if(named(parts[0]!)&&parts.slice(1).every((part,index)=>index%2===0?part.trim()!==""&&Number.isFinite(Number(part))&&Number(part)>=0&&Number(part)<=100:named(part)))return value;
 throw Error(`Invalid native or CSS paint ${JSON.stringify(value)}`);
}
function recordItems(value:string,separator=","):string[]{
 const items:string[]=[];let start=0,depth=0;
 for(let index=0;index<value.length;index++){const character=value[index];if(character==="\\"){index++;continue;}if(character==="{")depth++;else if(character==="}"&&--depth<0)throw Error("Unbalanced native records");else if(character===separator&&depth===0){items.push(value.slice(start,index).trim());start=index+1;}}
 if(depth)throw Error("Unbalanced native records");items.push(value.slice(start).trim());return items.map(item=>item.startsWith("{")&&item.endsWith("}")?item.slice(1,-1):item);
}
function recordRows(value:OptionValue,descriptor:VizOptionSyntaxDescriptor):string[][]{
 const fields=descriptor.recordFields;if(!fields)return[];const raw=typeof value==="string"?value:Array.isArray(value)?value.join(","):String(value??""),unique=fields.map(()=>new Set<string>());if(!raw.trim())return[];
 return recordItems(raw).map(record=>{const row=recordItems(record,"/");if(row.length!==fields.length)throw Error("Native record requires "+fields.length+" fields");fields.forEach((field,index)=>{const value=row[index]!.trim();if(field.nonempty&&!value)throw Error("Native record field must not be empty");if(field.enum&&!field.enum.includes(value))throw Error("Unknown native record vocabulary "+value);if(field.syntax==="expression"){if(!value||/^[+-]?(?:inf(?:inity)?|nan)$/i.test(value)||(/^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?$/.test(value)&&!Number.isFinite(Number(value))))throw Error("Native record coordinates require finite literals or nonempty expressions");}if(field.unique){if(unique[index]!.has(value))throw Error("Duplicate native record ID "+value);unique[index]!.add(value);}row[index]=value;});return row;});
}
function validateFamilyRecords(owner:string,authored:Readonly<Record<string,OptionValue|undefined>>|undefined,stock:Readonly<Record<string,OptionValue|undefined>>):void{
 const descriptors=METADATA["x-semio-family-options"][owner]?.options;if(!descriptors||!Object.values(descriptors).some(descriptor=>descriptor.recordFields))return;const settings={...stock,...authored},mode=String(settings.mode??descriptors.mode?.default??""),resolved=new Map<string,string[][]>();
 if(descriptors.mode)validateOptionValue(mode,descriptors.mode);
 for(const[key,descriptor]of Object.entries(descriptors)){if(!descriptor.recordFields)continue;const value=settings[key]??descriptor.defaultsByMode?.[mode]??descriptor.default??"";validateOptionValue(value,descriptor);resolved.set(key,recordRows(value,descriptor));}
 for(const[key,rows]of resolved)descriptors[key]!.recordFields!.forEach((field,index)=>{if(!field.reference)return;const reference=field.reference,targets=new Set((resolved.get(reference.option)??[]).map(row=>row[reference.field]));if(rows.some(row=>!targets.has(row[index])))throw Error("Dangling native record endpoint "+owner+"/"+key);});
}
/** 🪢️ Authored neural incidence references ordered table layers and visible unit slots. */
function validateNeuralLinks(settings:Readonly<Record<string,OptionValue|undefined>>,data:string,tables:readonly VizTable[]):void{
 if(settings.links===undefined)return;const descriptor=METADATA["x-semio-family-options"]["neural-network"]!.options.links!,links=recordRows(settings.links,descriptor);if(!links.length)return;
 const authored=tables.find(table=>table.name===data),demo=METADATA["x-semio-demo-tables"].find(table=>table.name===data),rows=authored?.rows??demo?.rows;if(!rows)throw Error("Neural links require a declared layer table");const numeric=(value:unknown)=>typeof value==="number"?value:typeof value==="string"&&/^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?$/.test(value.trim())?Number(value):NaN;const layer=String(settings.layerColumn??"layer"),units=String(settings.unitsColumn??"units"),maximum=numeric(settings.maxUnits??10);if(!Number.isSafeInteger(maximum)||(maximum<1||maximum>2147483647))throw Error("Neural visible unit limit requires a positive integer literal");const ordered=rows.map((row,index)=>({row,index,layer:numeric(row[layer])}));if(ordered.some(value=>!Number.isFinite(value.layer)))throw Error("Neural layer ranks require finite indices");ordered.sort((a,b)=>a.layer-b.layer||a.index-b.index);const counts=ordered.map(({row})=>numeric(row[units]));if(counts.some(count=>!Number.isSafeInteger(count)||count<0||count>2147483647))throw Error("Neural unit counts require positive integers");
 for(const row of links){if(row.some(value=>!/^[1-9]\d*$/.test(value)||!Number.isSafeInteger(Number(value))))throw Error("Neural links require positive integer endpoints");const [from,unit,to,target]=row.map(Number);if(from!>ordered.length||to!>ordered.length||unit!>Math.min(counts[from!-1]!,maximum)||target!>Math.min(counts[to!-1]!,maximum))throw Error("Neural link endpoint exceeds visible layer/unit bounds");}
}
/** 🧯️ Declared primitive contracts adjudicate literal values before source-owned grammar projection. */
function validateOptionValue(value:OptionValue,descriptor:VizOptionSyntaxDescriptor):void{
 const contract=Object.fromEntries(Object.entries(descriptor).filter(([key])=>["type","enum","minimum","exclusiveMinimum","maximum"].includes(key)));if(!Object.keys(contract).length)return;
 const types=Array.isArray(descriptor.type)?descriptor.type:[descriptor.type];let candidate:unknown=value;
 if(descriptor.syntax==="expression"&&typeof value==="string"&&(types.includes("number")||types.includes("integer"))){const literal=value.trim();if(/^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?$/.test(literal))candidate=Number(literal);else if(!types.includes("string")&&literal!==""){contract.type="string";}}
 const failures=validateJsonSchemaSubset(contract,candidate);if(failures.length)throw Error(`Invalid declared native option value: ${failures.join("; ")}`);
}
function nativeOption(value:OptionValue,descriptor:VizOptionSyntaxDescriptor,paintResult=false):string{
 validateOptionValue(value,descriptor);
 const raw=()=>{const item=(value:VizOptionValue|null)=>{if(value===null)throw Error("Native grammar requires strings, finite scalar values or scalar lists");return typeof value==="string"?value:scalar(value);};return Array.isArray(value)?value.map(item).join(","):item(value as VizOptionValue|null);};
 switch(descriptor.syntax){
  case "text":if(typeof value!=="string")throw Error("Literal option text must be a string");return escape(value);
  case "identifier":return raw();
  case "expression":case "key-list":case "style":return raw();
  case "records":{if(descriptor.recordFields)recordRows(value,descriptor);return descriptor.items?recordItems(raw()).map(item=>`{${descriptor.items==="text"?escape(item):token(item)}}`).join(","):raw();}
  case "number-list":{const result=raw(),items=result.trim()===""?[]:recordItems(result);if(descriptor.itemCount!==undefined&&items.length!==descriptor.itemCount)throw Error("Native numeric list requires "+descriptor.itemCount+" items");for(const item of items){const literal=item.trim();if(!literal)throw Error("Native numeric list items must not be empty");if(/^[+-]?(?:inf(?:inity)?|nan)$/i.test(literal)||(/^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?$/.test(literal)&&!Number.isFinite(Number(literal))))throw Error("Native numeric list literals must be finite");}return result;}
  case "scalar":if(typeof value!=="boolean"&&typeof value!=="number")throw Error("Scalar options require a number or boolean");return scalar(value);
  case "value":return Array.isArray(value)?value.map(rowScalar).join(","):paintResult&&typeof value==="string"?paint(value):rowScalar(value as VizOptionValue|null);
  case "paint":if(Array.isArray(value)){if(value.some(item=>typeof item!=="string"))throw Error("Paint lists require strings");return value.map(paint).join(",");}if(typeof value!=="string")throw Error("Paint options require strings or paint lists");return color(value)?.name??recordItems(value).map(paint).join(",");
  default:throw Error("Unknown source-owned option syntax");
 }
}
function options(owner:string,value:Readonly<Record<string,OptionValue>>={},family=false,paintResult=false):string{
 const descriptors=family?METADATA["x-semio-family-options"][owner]?.options:METADATA["x-semio-option-syntax"][owner];
 if(!descriptors)throw Error(`Missing native syntax owner ${owner}`);
 return Object.keys(value).sort().map(key=>{const descriptor=descriptors[key];if(!descriptor)throw Error(`Undeclared native option ${owner}/${key}`);return `${token(key)}={${nativeOption(value[key]!,descriptor,paintResult)}}`;}).join(",");
}


/** 🗾️ Materializes declared spatial views of the authored rows inside canonical inference. */
function spatialTable(table:VizTable,checkpoint:()=>void):string[]{
 const structure=table.structure;if(!structure)return [];
 const name=token(table.name),columns=new Set(table.columns),number=(row:VizRow,column:string):number=>{checkpoint();if(!columns.has(column))throw Error("Unknown spatial column "+column);const value=row[column];if(typeof value!=="number"||!Number.isFinite(value))throw Error("Spatial column "+column+" requires finite numbers");return value;};
 if(!table.rows.length)throw Error("Spatial table requires rows");
 if(structure.kind==="points")return ["\\SemioVizPointSet{"+name+"}{"+table.rows.map(row=>number(row,structure.x)+","+number(row,structure.y)).join(";")+"}"];
 if(structure.kind==="geometry"){
  const groups=new Map<string,{identity:VizOptionValue;points:number[][];value:number}>();
  for(const row of table.rows){checkpoint();const identity=structure.group?row[structure.group]:name;if(identity===undefined||identity===null||typeof identity==="number"&&!Number.isFinite(identity)||structure.group&&!columns.has(structure.group))throw Error("Invalid geometry group");const id=String(identity),value=structure.value?number(row,structure.value):1,point=[number(row,structure.longitude),number(row,structure.latitude)],group=groups.get(id);if(group){if(group.identity!==identity||group.value!==value)throw Error("Geometry group requires one identity and value");group.points.push(point);}else groups.set(id,{identity,points:[point],value});}
  const lines=["\\SemioVizGeoCollection{"+name+"}"],commands={polygon:"SemioVizGeoPolygon",line:"SemioVizGeoLine",point:"SemioVizGeoPointPart"};
  for(const[id,group]of groups){checkpoint();const points=group.points;if(structure.shape==="polygon"&&points.length>1&&points[0]![0]===points.at(-1)![0]&&points[0]![1]===points.at(-1)![1])points.pop();if(structure.shape==="point"?points.length!==1:points.length<(structure.shape==="polygon"?3:2))throw Error("Invalid geometry vertex count");lines.push("\\"+commands[structure.shape]+"{"+name+"}[id={"+escape(id)+"},value="+group.value+"]{"+points.map(point=>point.join(",")).join(";")+"}");}
  return lines;
 }
 if(structure.kind==="grid"){
  const cells=new Map<number,number>(),rows=table.rows.map(row=>{const x=number(row,structure.column),y=number(row,structure.row),value=number(row,structure.value);if(!Number.isInteger(x)||!Number.isInteger(y)||x<0||y<0)throw Error("Grid coordinates require nonnegative integers");return{x,y,value};}),width=rows.reduce((value,row)=>Math.max(value,row.x+1),0),height=rows.reduce((value,row)=>Math.max(value,row.y+1),0);if(width*height!==rows.length)throw Error("Grid must be a complete rectangle");
  for(const row of rows){checkpoint();const index=row.y*width+row.x;if(cells.has(index))throw Error("Duplicate grid cell");cells.set(index,row.value);}
  return ["\\SemioVizValueGrid{"+name+"}{"+width+"}{"+height+"}{"+Array.from({length:rows.length},(_,index)=>cells.get(index)).join(",")+"}"];
 }
 const samples=table.rows.map(row=>({x:number(row,structure.x),y:number(row,structure.y),u:number(row,structure.u),v:number(row,structure.v)})),xs=[...new Set(samples.map(row=>row.x))].sort((a,b)=>a-b),ys=[...new Set(samples.map(row=>row.y))].sort((a,b)=>a-b);
 for(const axis of[xs,ys]){if(axis.length<2)throw Error("Vector field requires two samples per axis");const step=axis[1]!-axis[0]!;if(axis.some((value,index)=>Math.abs(value-axis[0]!-index*step)>1e-10*Math.max(1,Math.abs(step))))throw Error("Vector field requires uniform axes");}
 if(xs.length*ys.length!==samples.length)throw Error("Vector field must be a complete rectangle");
 const xi=new Map(xs.map((value,index)=>[value,index])),yi=new Map(ys.map((value,index)=>[value,index])),cells=new Map<number,{u:number;v:number}>();
 for(const sample of samples){checkpoint();const index=yi.get(sample.y)!*xs.length+xi.get(sample.x)!;if(cells.has(index))throw Error("Duplicate vector field cell");cells.set(index,sample);}
 const component=(key:"u"|"v")=>Array.from({length:samples.length},(_,index)=>cells.get(index)![key]).join(",");
 return ["\\SemioVizVectorField{"+name+"}{"+xs.length+"}{"+ys.length+"}{"+[xs[0],xs.at(-1),ys[0],ys.at(-1)].join(",")+"}{"+component("u")+"}{"+component("v")+"}"];
}

export function renderVizPresetTikz(spec: VizChartSpecification, layeredTikz: string, checkpoint:()=>void=()=>{}): string {
  checkpoint();
  const spatialDeclarations=new Map((spec.tables??[]).map(table=>[table.name,spatialTable(table,checkpoint)]));
  const presets = spec.presets ?? [];
  if (presets.length === 0) return layeredTikz;
  if (spec.language !== "en" && spec.language !== "de") throw new Error("catalogue inference requires explicit language");
  const lines = ["\\begingroup", `\\ExplSyntaxOn\\tl_set:Nn\\l_semio_language_tl{${spec.language}}\\ExplSyntaxOff`, `\\begin{VizFigure}[width=${spec.width},height=${spec.height},title={${escape(spec.title?.[spec.language] ?? "")}}]`];
  const colors = new Map<string, Paint>(), stack: unknown[] = [spec];
  let visited = 0;
  while (stack.length) { if (++visited % 256 === 0) checkpoint(); const value = stack.pop(); if (typeof value === "string") { const paint = color(value); if (paint) colors.set(paint.name, paint); } else if (value && typeof value === "object") stack.push(...Object.values(value)); }
  for (const paint of [...colors.values()].sort((a, b) => a.name.localeCompare(b.name))) { lines.push(`\\definecolor{${paint.name}}{HTML}{${paint.hex}}`); if (paint.alpha !== 1) lines.push(`\\SemioVizPaintAlpha{${paint.name}}{${paint.alpha}}`); }
  const tables = new Set<string>();
  for (const table of spec.tables ?? []) {
    checkpoint();
    const name = token(table.name);
    if (tables.has(name)) throw new Error(`duplicate table ${name}`);
    tables.add(name);
    lines.push(`\\SemioVizTable{${name}}{${table.columns.map(token).join(",")}}`);
    for (const row of table.rows) {checkpoint();lines.push(`\\SemioVizRow{${name}}{${table.columns.map((column) => `{${rowScalar(row[column])}}`).join(",")}}`);}
    for(const declaration of spatialDeclarations.get(name)??[])lines.push(declaration);
  }
  const paintScales=new Set(spec.layers.flatMap(layer=>[layer.encodings?.fill?.scale,layer.encodings?.stroke?.scale]).filter((name):name is string=>name!==undefined));
  for (const scale of spec.scales ?? []) {
    checkpoint();
    const scaleOptions = scale.options ?? {};
    lines.push(`\\SemioVizScale{${token(scale.name)}}{${token(scale.kind)}}{${scale.domain.map(["ordinal","band","point"].includes(scale.kind)?rowScalar:scalar).join(",")}}{${scale.range.map(paintScales.has(scale.name)?paint:scalar).join(",")}}[${options("scale",scaleOptions,false,paintScales.has(scale.name))}]`);
  }
  if (spec.theme) {
    const name = spec.theme.name === "semio" || spec.theme.name === undefined ? "default" : token(spec.theme.name);
    lines.push(`\\SemioVizTheme{${name}}[appearance=${spec.theme.appearance ?? "light"}]`);
    if (spec.theme.palette) lines.push(`\\SemioVizThemeSet[colors={${spec.theme.palette.map(paint).join(",")}}]`);
  }
  for (const preset of presets) {
    checkpoint();
    const kind = token(preset.kind), entry = KINDS.get(kind);
    if (!entry) throw new Error(`unknown catalogue kind ${kind}`);
    const data = preset.data;
    if (data !== undefined && data !== "demo" && data !== entry.data && !tables.has(data)) throw new Error(`unknown preset table ${data}`);
    validateFamilyRecords(entry.family,preset.options,entry.options);
    if(entry.family==="neural-network"){const structural=data!==undefined||["data","layerColumn","unitsColumn","maxUnits","connections"].some(key=>preset.options?.[key]!==undefined),effective={...entry.options,...preset.options,...structural&&preset.options?.links===undefined?{links:undefined}:{}},neuralData=String(data??effective.data??entry.data);validateNeuralLinks(effective,neuralData==="demo"?entry.data:neuralData,spec.tables??[]);}
    const settings = options(entry.family,preset.options,true);
    lines.push(`\\SemioVizChart{${kind}}[${settings}${data===undefined?"":`${settings?",":""}data=${token(data)}`}]`);
  }
  const begin = layeredTikz.indexOf("\n");
  const end = layeredTikz.lastIndexOf("\\end{tikzpicture}");
  if (begin < 0 || end < 0) throw new Error("layer inference did not emit a complete TikZ picture");
  lines.push(layeredTikz.slice(begin + 1, end).trimEnd(), "\\end{VizFigure}", "\\endgroup");
  return `${lines.join("\n")}\n`;
}
