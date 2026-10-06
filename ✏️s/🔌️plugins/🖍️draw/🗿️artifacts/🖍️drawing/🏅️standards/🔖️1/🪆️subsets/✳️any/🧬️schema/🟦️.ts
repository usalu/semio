import { binary64, type Binary64 } from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import { parseBinary64 } from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {parseFillRule,type FillRule} from "./🎨️fill/🌀️rule/🟦️.ts";
/** 🧬️ Drawing artifact schema — every field with its state class. */

export interface DrawingArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  layers: DrawingLayerNode[];
  /** @state artifact */
  assets: Record<string, DrawingImageAsset>;
  /** @state artifact */
  artboard?: DrawingArtboard;
}

/** 🎨️ Canonical authored blending operations shared by layers, patches and mutations. */
export const DRAWING_BLEND_MODES = ["normal", "multiply", "screen", "overlay", "darken", "lighten", "colorDodge", "colorBurn", "hardLight", "softLight", "difference", "exclusion", "hue", "saturation", "color", "luminosity"] as const;
export type BlendMode = typeof DRAWING_BLEND_MODES[number];
export function parseBlendMode(value:unknown,at="$"):BlendMode {
  return drawingDrawingArtifactGuardMember(value,at,DRAWING_BLEND_MODES);
}

export interface DrawingTransform {x:Binary64;y:Binary64;scaleX:Binary64;scaleY:Binary64;shear:Binary64;rotation:Binary64}
export type DrawingPoint=[Binary64,Binary64];
export type DrawingColor=[Binary64,Binary64,Binary64,Binary64];
export interface DrawingGradientStop {offset:Binary64;color:DrawingColor}
export type DrawingFill =
 | {kind:"solid";color:DrawingColor}
 | {kind:"linearGradient";x1:Binary64;y1:Binary64;x2:Binary64;y2:Binary64;stops:DrawingGradientStop[]}
 | {kind:"radialGradient";cx:Binary64;cy:Binary64;r:Binary64;stops:DrawingGradientStop[]};
export interface DrawingStroke {color:DrawingColor;width:Binary64;cap:"butt"|"round"|"square";join:"miter"|"round"|"bevel";dash?:Binary64[]}
export interface DrawingAttributes {fillRule:FillRule;fill?:DrawingFill;stroke?:DrawingStroke}
export interface DrawingLayerBase {id:string;name:string;visible:boolean;locked:boolean;opacity:Binary64;blendMode:string;transform:DrawingTransform;attributes:DrawingAttributes}
export interface DrawingRect {x:Binary64;y:Binary64;width:Binary64;height:Binary64}
export interface DrawingEllipse {cx:Binary64;cy:Binary64;rx:Binary64;ry:Binary64}
export interface DrawingCircle {cx:Binary64;cy:Binary64;r:Binary64}
export interface DrawingLine {x1:Binary64;y1:Binary64;x2:Binary64;y2:Binary64}
export type DrawingPathSegment =
 | {kind:"move"|"line";to:DrawingPoint}
 | {kind:"quad";ctrl:DrawingPoint;to:DrawingPoint}
 | {kind:"cubic";ctrl1:DrawingPoint;ctrl2:DrawingPoint;to:DrawingPoint}
 | {kind:"arc";rx:Binary64;ry:Binary64;rotation:Binary64;largeArc:boolean;sweep:boolean;to:DrawingPoint}
 | {kind:"close"};
export type DrawingLayerNode =
 | (DrawingLayerBase & {kind:"shape";shapeKind:string;rect?:DrawingRect;ellipse?:DrawingEllipse;circle?:DrawingCircle;line?:DrawingLine;polygon?:{points:DrawingPoint[]}})
 | (DrawingLayerBase & {kind:"path";segments:DrawingPathSegment[]})
 | (DrawingLayerBase & {kind:"text";x:Binary64;y:Binary64;content:string;size:Binary64})
 | (DrawingLayerBase & {kind:"image";imageKey:string;width:Binary64;height:Binary64})
 | (DrawingLayerBase & {kind:"group";isolation:boolean;children:DrawingLayerNode[]})
 | (DrawingLayerBase & {kind:"boolean";operation:string;children:string[]})
 | (DrawingLayerBase & {kind:"trace";sourceKey:string;params:{threshold:Binary64;simplifyEpsilon:Binary64}});
export interface DrawingImageAsset {mime:string;data:string;width?:number;height?:number}
export interface DrawingArtboard {width:Binary64;height:Binary64}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class drawingDrawingArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const drawingDrawingArtifactGuardReject = (at: string, why: string): never => {
  throw new drawingDrawingArtifactGuardRefusal(at, why);
};

type drawingDrawingArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type drawingDrawingArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type drawingDrawingArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const drawingDrawingArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : drawingDrawingArtifactGuardReject(at, "value is not an object");
export const drawingDrawingArtifactGuardArray = (value: unknown, at: string, bounds: drawingDrawingArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return drawingDrawingArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) drawingDrawingArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) drawingDrawingArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const drawingDrawingArtifactGuardString = (value: unknown, at: string, bounds: drawingDrawingArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return drawingDrawingArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) drawingDrawingArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) drawingDrawingArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) drawingDrawingArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const drawingDrawingArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : drawingDrawingArtifactGuardReject(at, "value is not a boolean"));
export const drawingDrawingArtifactGuardNumber = (value: unknown, at: string, bounds: drawingDrawingArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return drawingDrawingArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) drawingDrawingArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) drawingDrawingArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const drawingDrawingArtifactGuardInteger = (value: unknown, at: string, bounds: drawingDrawingArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? drawingDrawingArtifactGuardNumber(value, at, bounds) : drawingDrawingArtifactGuardReject(at, "value is not an integer");
export const drawingDrawingArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : drawingDrawingArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const drawingDrawingArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : drawingDrawingArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDrawingArtifact(value: unknown, at = "$"): DrawingArtifact {
  const row = drawingDrawingArtifactGuardObject(value, at);
  return {
    schema: drawingDrawingArtifactGuardString(row["schema"], `${at}.schema`),
    id: drawingDrawingArtifactGuardString(row["id"], `${at}.id`),
    title: row["title"] === undefined ? undefined : drawingDrawingArtifactGuardString(row["title"], `${at}.title`),
    layers: drawingDrawingArtifactGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseDrawingLayerNode(item, `${at}.layers[${index}]`)),
    assets: Object.fromEntries(
      Object.entries(drawingDrawingArtifactGuardObject(row["assets"], `${at}.assets`))
        .map(([key, item]) => [key, parseDrawingImageAsset(item, `${at}.assets.${key}`)]),
    ),
    artboard: row["artboard"] === undefined ? undefined : parseDrawingArtboard(row["artboard"], `${at}.artboard`),
  };
}

const ownedWord=(value:unknown,at:string):Binary64=>{try{return parseBinary64(value);}catch{return drawingDrawingArtifactGuardReject(at,"expected a Binary64 word or number");}};
const ownedRecord=drawingDrawingArtifactGuardObject,ownedArray=drawingDrawingArtifactGuardArray,ownedText=drawingDrawingArtifactGuardString,ownedBool=drawingDrawingArtifactGuardBoolean;
function ownedPoint(value:unknown,at:string):DrawingPoint{const v=ownedArray(value,at,{minItems:2,maxItems:2});return[ownedWord(v[0],at+'[0]'),ownedWord(v[1],at+'[1]')];}
function ownedColor(value:unknown,at:string):DrawingColor{const v=ownedArray(value,at,{minItems:4,maxItems:4});return[ownedWord(v[0],at+'[0]'),ownedWord(v[1],at+'[1]'),ownedWord(v[2],at+'[2]'),ownedWord(v[3],at+'[3]')];}
function ownedAttributes(value:unknown,at:string):DrawingAttributes{const v=value===undefined?{}:ownedRecord(value,at),out:DrawingAttributes={fillRule:v.fillRule===undefined?"evenodd":parseFillRule(v.fillRule)};
 if(v.fill!==undefined){const f=ownedRecord(v.fill,at+'.fill'),kind=drawingDrawingArtifactGuardMember(f.kind,at+'.fill.kind',["solid","linearGradient","radialGradient"] as const);if(kind==='solid')out.fill={kind,color:ownedColor(f.color,at+'.fill.color')};else{const stops=ownedArray(f.stops,at+'.fill.stops').map((item,i)=>{const p=ownedRecord(item,at+'.fill.stops['+i+']');return{offset:ownedWord(p.offset,at+'.fill.offset'),color:ownedColor(p.color,at+'.fill.color')};});out.fill=kind==='linearGradient'?{kind,x1:ownedWord(f.x1,at+'.fill.x1'),y1:ownedWord(f.y1,at+'.fill.y1'),x2:ownedWord(f.x2,at+'.fill.x2'),y2:ownedWord(f.y2,at+'.fill.y2'),stops}:{kind,cx:ownedWord(f.cx,at+'.fill.cx'),cy:ownedWord(f.cy,at+'.fill.cy'),r:ownedWord(f.r,at+'.fill.r'),stops};}}
 if(v.stroke!==undefined){const p=ownedRecord(v.stroke,at+'.stroke');out.stroke={color:ownedColor(p.color,at+'.stroke.color'),width:ownedWord(p.width,at+'.stroke.width'),cap:drawingDrawingArtifactGuardMember(p.cap,at+'.stroke.cap',["butt","round","square"] as const),join:drawingDrawingArtifactGuardMember(p.join,at+'.stroke.join',["miter","round","bevel"] as const)};if(p.dash!==undefined)out.stroke.dash=ownedArray(p.dash,at+'.stroke.dash').map((v,i)=>ownedWord(v,at+'.stroke.dash['+i+']'));}return out;
}
export function parseDrawingPathSegment(value:unknown,at='$'):DrawingPathSegment{const v=ownedRecord(value,at),kind=drawingDrawingArtifactGuardMember(v.kind,at+'.kind',["move","line","quad","cubic","arc","close"] as const);switch(kind){case'move':case'line':return{kind,to:ownedPoint(v.to,at+'.to')};case'quad':return{kind,ctrl:ownedPoint(v.ctrl,at+'.ctrl'),to:ownedPoint(v.to,at+'.to')};case'cubic':return{kind,ctrl1:ownedPoint(v.ctrl1,at+'.ctrl1'),ctrl2:ownedPoint(v.ctrl2,at+'.ctrl2'),to:ownedPoint(v.to,at+'.to')};case'arc':return{kind,rx:ownedWord(v.rx,at+'.rx'),ry:ownedWord(v.ry,at+'.ry'),rotation:ownedWord(v.rotation,at+'.rotation'),largeArc:ownedBool(v.largeArc,at+'.largeArc'),sweep:ownedBool(v.sweep,at+'.sweep'),to:ownedPoint(v.to,at+'.to')};case'close':return{kind};}}
/** 🖍️ Validate explicit layer fields and reconstruct the forest without recursive cloning. */
export function parseDrawingLayerNode(value:unknown,at='$'):DrawingLayerNode{
 const result:DrawingLayerNode[]=[],pending:[unknown,string,DrawingLayerNode[]][]=[[value,at,result]];
 while(pending.length){const[value,path,out]=pending.pop()!,v=ownedRecord(value,path),kind=drawingDrawingArtifactGuardMember(v.kind,path+'.kind',["shape","path","text","image","group","boolean","trace"] as const),t=ownedRecord(v.transform,path+'.transform');const base:DrawingLayerBase={id:ownedText(v.id,path+'.id'),name:ownedText(v.name,path+'.name'),visible:ownedBool(v.visible,path+'.visible'),locked:ownedBool(v.locked,path+'.locked'),opacity:ownedWord(v.opacity,path+'.opacity'),blendMode:ownedText(v.blendMode,path+'.blendMode'),transform:{x:ownedWord(t.x,path+'.transform.x'),y:ownedWord(t.y,path+'.transform.y'),scaleX:ownedWord(t.scaleX,path+'.transform.scaleX'),scaleY:ownedWord(t.scaleY,path+'.transform.scaleY'),shear:ownedWord(t.shear,path+'.transform.shear'),rotation:ownedWord(t.rotation,path+'.transform.rotation')},attributes:ownedAttributes(v.attributes,path+'.attributes')};
 switch(kind){
 case'shape':{const node:Extract<DrawingLayerNode,{kind:'shape'}>={...base,kind,shapeKind:ownedText(v.shapeKind,path+'.shapeKind')};if(v.rect!==undefined){const p=ownedRecord(v.rect,path+'.rect');node.rect={x:ownedWord(p.x,path+'.rect.x'),y:ownedWord(p.y,path+'.rect.y'),width:ownedWord(p.width,path+'.rect.width'),height:ownedWord(p.height,path+'.rect.height')};}if(v.ellipse!==undefined){const p=ownedRecord(v.ellipse,path+'.ellipse');node.ellipse={cx:ownedWord(p.cx,path+'.ellipse.cx'),cy:ownedWord(p.cy,path+'.ellipse.cy'),rx:ownedWord(p.rx,path+'.ellipse.rx'),ry:ownedWord(p.ry,path+'.ellipse.ry')};}if(v.circle!==undefined){const p=ownedRecord(v.circle,path+'.circle');node.circle={cx:ownedWord(p.cx,path+'.circle.cx'),cy:ownedWord(p.cy,path+'.circle.cy'),r:ownedWord(p.r,path+'.circle.r')};}if(v.line!==undefined){const p=ownedRecord(v.line,path+'.line');node.line={x1:ownedWord(p.x1,path+'.line.x1'),y1:ownedWord(p.y1,path+'.line.y1'),x2:ownedWord(p.x2,path+'.line.x2'),y2:ownedWord(p.y2,path+'.line.y2')};}if(v.polygon!==undefined){const p=ownedRecord(v.polygon,path+'.polygon');node.polygon={points:ownedArray(p.points,path+'.polygon.points').map((p,i)=>ownedPoint(p,path+'.polygon.points['+i+']'))};}out.push(node);break;}
 case'path':out.push({...base,kind,segments:ownedArray(v.segments,path+'.segments').map((s,i)=>parseDrawingPathSegment(s,path+'.segments['+i+']'))});break;
 case'text':out.push({...base,kind,x:ownedWord(v.x,path+'.x'),y:ownedWord(v.y,path+'.y'),content:ownedText(v.content,path+'.content'),size:ownedWord(v.size,path+'.size')});break;
 case'image':out.push({...base,kind,imageKey:ownedText(v.imageKey,path+'.imageKey'),width:ownedWord(v.width,path+'.width'),height:ownedWord(v.height,path+'.height')});break;
 case'group':{const children:DrawingLayerNode[]=[];out.push({...base,kind,isolation:v.isolation===undefined?false:ownedBool(v.isolation,path+'.isolation'),children});const items=ownedArray(v.children,path+'.children');for(let i=items.length-1;i>=0;i--)pending.push([items[i],path+'.children['+i+']',children]);break;}
 case'boolean':out.push({...base,kind,operation:ownedText(v.operation,path+'.operation'),children:ownedArray(v.children,path+'.children').map((s,i)=>ownedText(s,path+'.children['+i+']'))});break;
 case'trace':{const p=ownedRecord(v.params,path+'.params');out.push({...base,kind,sourceKey:ownedText(v.sourceKey,path+'.sourceKey'),params:{threshold:ownedWord(p.threshold,path+'.params.threshold'),simplifyEpsilon:ownedWord(p.simplifyEpsilon,path+'.params.simplifyEpsilon')}});break;}
 }
 }return result[0]!;
}

export function parseDrawingImageAsset(value: unknown, at = "$"): DrawingImageAsset {
  const row = drawingDrawingArtifactGuardObject(value, at);
  return {
    mime: drawingDrawingArtifactGuardString(row["mime"], `${at}.mime`),
    data: drawingDrawingArtifactGuardString(row["data"], `${at}.data`),
    width: row["width"] === undefined ? undefined : drawingDrawingArtifactGuardInteger(row["width"], `${at}.width`, {"minimum": 0,"maximum":4294967295}),
    height: row["height"] === undefined ? undefined : drawingDrawingArtifactGuardInteger(row["height"], `${at}.height`, {"minimum": 0,"maximum":4294967295}),
  };
}

export function parseDrawingArtboard(value: unknown, at = "$"): DrawingArtboard {
  const row = drawingDrawingArtifactGuardObject(value, at);
  return {
    width: ownedWord(row["width"], `${at}.width`),
    height: ownedWord(row["height"], `${at}.height`),
  };
}

/** ✏️ Numeric path geometry for arithmetic operations; persisted fields use DrawingPathSegment. */
export type PathGeometrySegment =
  | { kind: "move"; to: [number,number] }
  | { kind: "line"; to: [number,number] }
  | { kind: "quad"; ctrl: [number,number]; to: [number,number] }
  | { kind: "cubic"; ctrl1: [number,number]; ctrl2: [number,number]; to: [number,number] }
  | { kind: "arc"; rx: number; ry: number; rotation: number; largeArc: boolean; sweep: boolean; to: [number,number] }
  | { kind: "close" };

/** 📐 Lift calculated geometry into the persisted binary64 domain. */
export function drawingPathFromGeometry(value:PathGeometrySegment):DrawingPathSegment {
 const segment=parsePathGeometrySegment(value),point=(p:[number,number]):DrawingPoint=>[binary64(p[0]),binary64(p[1])];
 switch(segment.kind){
 case "move":case "line":return {kind:segment.kind,to:point(segment.to)};
 case "quad":return {kind:segment.kind,ctrl:point(segment.ctrl),to:point(segment.to)};
 case "cubic":return {kind:segment.kind,ctrl1:point(segment.ctrl1),ctrl2:point(segment.ctrl2),to:point(segment.to)};
 case "arc":return {kind:segment.kind,rx:binary64(segment.rx),ry:binary64(segment.ry),rotation:binary64(segment.rotation),largeArc:segment.largeArc,sweep:segment.sweep,to:point(segment.to)};
 case "close":return {kind:segment.kind};
 }
}

/** 📍 Validates owned geometry without accepting malformed or unknown segment fields. */
export function parsePathGeometrySegment(value: unknown, at = "$"): PathGeometrySegment {
  const row = drawingDrawingArtifactGuardObject(value, at);
  const kind = drawingDrawingArtifactGuardMember(row.kind, `${at}.kind`, ["move", "line", "quad", "cubic", "arc", "close"] as const);
  const fields:readonly string[] = { move: ["to"], line: ["to"], quad: ["ctrl", "to"], cubic: ["ctrl1", "ctrl2", "to"], arc: ["rx", "ry", "rotation", "largeArc", "sweep", "to"], close: [] }[kind];
  for (const key of Object.keys(row)) if (key !== "kind" && !fields.includes(key)) drawingDrawingArtifactGuardReject(`${at}.${key}`, "unknown segment field");
  const point = (key: string): [number, number] => {
    const values = drawingDrawingArtifactGuardArray(row[key], `${at}.${key}`, { minItems: 2, maxItems: 2 });
    return [drawingDrawingArtifactGuardNumber(values[0], `${at}.${key}[0]`), drawingDrawingArtifactGuardNumber(values[1], `${at}.${key}[1]`)];
  };
  switch (kind) {
    case "move": case "line": return { kind, to: point("to") };
    case "quad": return { kind, ctrl: point("ctrl"), to: point("to") };
    case "cubic": return { kind, ctrl1: point("ctrl1"), ctrl2: point("ctrl2"), to: point("to") };
    case "arc": return { kind, rx: drawingDrawingArtifactGuardNumber(row.rx, `${at}.rx`, { minimum: 0 }), ry: drawingDrawingArtifactGuardNumber(row.ry, `${at}.ry`, { minimum: 0 }), rotation: drawingDrawingArtifactGuardNumber(row.rotation, `${at}.rotation`), largeArc: drawingDrawingArtifactGuardBoolean(row.largeArc, `${at}.largeArc`), sweep: drawingDrawingArtifactGuardBoolean(row.sweep, `${at}.sweep`), to: point("to") };
    case "close": return { kind };
  }
}

/** 📐 Validate a decoded transform value. */
export function parseDrawingTransform(value:unknown,at="$"):DrawingTransform {const row=ownedRecord(value,at);return {x:ownedWord(row.x,at+".x"),y:ownedWord(row.y,at+".y"),scaleX:ownedWord(row.scaleX,at+".scaleX"),scaleY:ownedWord(row.scaleY,at+".scaleY"),shear:ownedWord(row.shear,at+".shear"),rotation:ownedWord(row.rotation,at+".rotation")};}
/** 🎨 Validate a decoded fill value. */
export function parseDrawingFill(value:unknown,at="$"):DrawingFill {return ownedAttributes({fill:value},at).fill!;}
/** 🖊️ Validate a decoded stroke value. */
export function parseDrawingStroke(value:unknown,at="$"):DrawingStroke {return ownedAttributes({stroke:value},at).stroke!;}
/** 🖼️ Decoded trace settings. */
export interface DrawingTraceParams {threshold:Binary64;simplifyEpsilon:Binary64}
/** 🖼️ Validate decoded trace settings. */
export function parseDrawingTraceParams(value:unknown,at="$"):DrawingTraceParams {const row=ownedRecord(value,at);return {threshold:ownedWord(row.threshold,at+".threshold"),simplifyEpsilon:ownedWord(row.simplifyEpsilon,at+".simplifyEpsilon")};}
