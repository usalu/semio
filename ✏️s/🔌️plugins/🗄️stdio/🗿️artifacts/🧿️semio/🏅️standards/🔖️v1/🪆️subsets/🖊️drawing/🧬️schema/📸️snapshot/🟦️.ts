/** 🖊️ Exact owned drawing variants, native geometry words and genuine image octets. */
import {parseSemioPoint2,parseSemioPoint3,parseSemioQuaternion,parseSemioTransform,parseSemioRgba,type SemioPoint2,type SemioPoint3,type SemioQuaternion,type SemioTransform,type SemioRgba} from "../../../✉️base/🧬️schema/🧮️geometry/🟦️.ts";
import {parseBinary64,parseBinary32,type Binary64,type Binary32} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export {parseSemioPoint2,parseSemioPoint3 as parsePoint3,parseSemioQuaternion as parseQuaternion,parseSemioTransform as parseTransform,parseSemioRgba as parseRgba};
export type {SemioPoint2,SemioPoint3 as Point3,SemioQuaternion as Quaternion,SemioTransform as Transform,SemioRgba as Rgba};
export type PathSegment={kind:"moveTo";to:SemioPoint2}|{kind:"lineTo";to:SemioPoint2}|{kind:"cubicTo";c1:SemioPoint2;c2:SemioPoint2;to:SemioPoint2}|{kind:"quadTo";c:SemioPoint2;to:SemioPoint2}|{kind:"arcTo";rx:Binary64;ry:Binary64;xRotation:Binary64;largeArc:boolean;sweep:boolean;to:SemioPoint2}|{kind:"close"};
export type DrawNode={kind:"path";segments:PathSegment[];style?:string}|{kind:"text";value:string;at:SemioPoint2;style?:string}|{kind:"group";transform:SemioTransform;children:DrawNode[]}|{kind:"image";at:SemioPoint2;width:Binary64;height:Binary64;mime:string;bytes:Uint8Array};
export interface DrawStyle{name:string;fill?:SemioRgba;stroke?:SemioRgba;strokeWidth?:Binary64;opacity?:Binary32}
export interface DrawLayer{id:string;name:string;visible:boolean;root:DrawNode}
export interface DrawCanvas{width:Binary64;height:Binary64;background?:SemioRgba}
export interface SemioDrawingSnapshot{schema:string;canvas:DrawCanvas;styles:DrawStyle[];layers:DrawLayer[]}
//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1DrawingSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1DrawingSnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1DrawingSnapshotGuardRefusal(at, why);
};

type stdioSemioV1DrawingSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1DrawingSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1DrawingSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1DrawingSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1DrawingSnapshotGuardReject(at, "value is not an object");
export const stdioSemioV1DrawingSnapshotGuardArray = (value: unknown, at: string, bounds: stdioSemioV1DrawingSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1DrawingSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1DrawingSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1DrawingSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1DrawingSnapshotGuardString = (value: unknown, at: string, bounds: stdioSemioV1DrawingSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1DrawingSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1DrawingSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1DrawingSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1DrawingSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1DrawingSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1DrawingSnapshotGuardReject(at, "value is not a boolean"));
export const stdioSemioV1DrawingSnapshotGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1DrawingSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1DrawingSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1DrawingSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1DrawingSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1DrawingSnapshotGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1DrawingSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1DrawingSnapshotGuardNumber(value, at, bounds) : stdioSemioV1DrawingSnapshotGuardReject(at, "value is not an integer");
export const stdioSemioV1DrawingSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1DrawingSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1DrawingSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1DrawingSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers


/** 🧩️ Parse each path command's required fields through its real discriminated variant. */
export function parsePathSegment(value:unknown,at="$"):PathSegment{const r=stdioSemioV1DrawingSnapshotGuardObject(value,at),kind=stdioSemioV1DrawingSnapshotGuardString(r.kind,at+".kind");switch(kind){case"moveTo":case"lineTo":return{kind,to:parseSemioPoint2(r.to,at+".to")};case"cubicTo":return{kind,c1:parseSemioPoint2(r.c1,at+".c1"),c2:parseSemioPoint2(r.c2,at+".c2"),to:parseSemioPoint2(r.to,at+".to")};case"quadTo":return{kind,c:parseSemioPoint2(r.c,at+".c"),to:parseSemioPoint2(r.to,at+".to")};case"arcTo":return{kind,rx:parseBinary64(r.rx),ry:parseBinary64(r.ry),xRotation:parseBinary64(r.xRotation),largeArc:stdioSemioV1DrawingSnapshotGuardBoolean(r.largeArc,at+".largeArc"),sweep:stdioSemioV1DrawingSnapshotGuardBoolean(r.sweep,at+".sweep"),to:parseSemioPoint2(r.to,at+".to")};case"close":return{kind};default:throw new stdioSemioV1DrawingSnapshotGuardRefusal(at,"unknown path command");}}
/** 🌳️ Parse a deep drawing tree iteratively and reject cycles without flattening variants. */
export function parseDrawNode(value:unknown,at="$"):DrawNode{type Task={value:unknown;at:string;out:DrawNode[];exit?:object};const result:DrawNode[]=[],pending:Task[]=[{value,at,out:result}],active=new Set<object>();while(pending.length){const t=pending.pop()!;if(t.exit){active.delete(t.exit);continue;}const r=stdioSemioV1DrawingSnapshotGuardObject(t.value,t.at);if(active.has(r))throw new stdioSemioV1DrawingSnapshotGuardRefusal(t.at,"cyclic drawing tree");const kind=stdioSemioV1DrawingSnapshotGuardString(r.kind,t.at+".kind");switch(kind){case"path":t.out.push({kind,segments:stdioSemioV1DrawingSnapshotGuardArray(r.segments,t.at+".segments").map((v,i)=>parsePathSegment(v,t.at+".segments["+i+"]")),style:r.style===undefined?undefined:stdioSemioV1DrawingSnapshotGuardString(r.style,t.at+".style")});break;case"text":t.out.push({kind,value:stdioSemioV1DrawingSnapshotGuardString(r.value,t.at+".value"),at:parseSemioPoint2(r.at,t.at+".at"),style:r.style===undefined?undefined:stdioSemioV1DrawingSnapshotGuardString(r.style,t.at+".style")});break;case"group":{const children:DrawNode[]=[],source=stdioSemioV1DrawingSnapshotGuardArray(r.children,t.at+".children");t.out.push({kind,transform:parseSemioTransform(r.transform,t.at+".transform"),children});active.add(r);pending.push({value:null,at:t.at,out:t.out,exit:r});for(let i=source.length-1;i>=0;i--)pending.push({value:source[i],at:t.at+".children["+i+"]",out:children});break;}case"image":if(!(r.bytes instanceof Uint8Array))throw new stdioSemioV1DrawingSnapshotGuardRefusal(t.at+".bytes","owned octets required");t.out.push({kind,at:parseSemioPoint2(r.at,t.at+".at"),width:parseBinary64(r.width),height:parseBinary64(r.height),mime:stdioSemioV1DrawingSnapshotGuardString(r.mime,t.at+".mime"),bytes:r.bytes.slice()});break;default:throw new stdioSemioV1DrawingSnapshotGuardRefusal(t.at,"unknown drawing node");}}return result[0]!;}
export function parseDrawStyle(value:unknown,at="$"):DrawStyle{const r=stdioSemioV1DrawingSnapshotGuardObject(value,at);return{name:stdioSemioV1DrawingSnapshotGuardString(r.name,at+".name"),fill:r.fill===undefined?undefined:parseSemioRgba(r.fill),stroke:r.stroke===undefined?undefined:parseSemioRgba(r.stroke),strokeWidth:r.strokeWidth===undefined?undefined:parseBinary64(r.strokeWidth),opacity:r.opacity===undefined?undefined:parseBinary32(r.opacity)};}
export function parseDrawCanvas(value:unknown,at="$"):DrawCanvas{const r=stdioSemioV1DrawingSnapshotGuardObject(value,at);return{width:parseBinary64(r.width),height:parseBinary64(r.height),background:r.background===undefined?undefined:parseSemioRgba(r.background)};}
export function parseDrawLayer(value:unknown,at="$"):DrawLayer{const r=stdioSemioV1DrawingSnapshotGuardObject(value,at);return{id:stdioSemioV1DrawingSnapshotGuardString(r.id,at+".id"),name:stdioSemioV1DrawingSnapshotGuardString(r.name,at+".name"),visible:stdioSemioV1DrawingSnapshotGuardBoolean(r.visible,at+".visible"),root:parseDrawNode(r.root,at+".root")};}
export function parseSemioDrawingSnapshot(value:unknown,at="$"):SemioDrawingSnapshot{const r=stdioSemioV1DrawingSnapshotGuardObject(value,at);return{schema:stdioSemioV1DrawingSnapshotGuardString(r.schema,at+".schema"),canvas:parseDrawCanvas(r.canvas,at+".canvas"),styles:stdioSemioV1DrawingSnapshotGuardArray(r.styles,at+".styles").map((v,i)=>parseDrawStyle(v,at+".styles["+i+"]")),layers:stdioSemioV1DrawingSnapshotGuardArray(r.layers,at+".layers").map((v,i)=>parseDrawLayer(v,at+".layers["+i+"]"))};}
