/** 🔊️ The actual native sparse channel/sample/tag diff with owned binary32 words. */
import type {SemioAudioSnapshot,SemioAudioChannel,SemioAudioTag,SemioAudioFormat} from "../📸️snapshot/🟦️.ts";
import {parseBinary32} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export interface IndexedTripleDiff<D,T>{removed:number[];modified:{index:number;diff:D}[];added:{index:number;item:T}[]}
export interface SemioAudioChannelDiff{samples?:SemioAudioChannel["samples"]}
export type SemioAudioChannelsDiff=IndexedTripleDiff<SemioAudioChannelDiff,SemioAudioChannel>;
export type SemioAudioTagsDiff=IndexedTripleDiff<SemioAudioTag,SemioAudioTag>;
export interface SemioAudioDiff{sampleRate?:SemioAudioSnapshot["sampleRate"];format?:SemioAudioFormat;channels?:SemioAudioChannelsDiff;tags?:SemioAudioTagsDiff}
//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1AudioDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1AudioDiffGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1AudioDiffGuardRefusal(at, why);
};

type stdioSemioV1AudioDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1AudioDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1AudioDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1AudioDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1AudioDiffGuardReject(at, "value is not an object");
export const stdioSemioV1AudioDiffGuardArray = (value: unknown, at: string, bounds: stdioSemioV1AudioDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1AudioDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1AudioDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1AudioDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1AudioDiffGuardString = (value: unknown, at: string, bounds: stdioSemioV1AudioDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1AudioDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1AudioDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1AudioDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1AudioDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1AudioDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1AudioDiffGuardReject(at, "value is not a boolean"));
export const stdioSemioV1AudioDiffGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1AudioDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1AudioDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1AudioDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1AudioDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1AudioDiffGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1AudioDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1AudioDiffGuardNumber(value, at, bounds) : stdioSemioV1AudioDiffGuardReject(at, "value is not an integer");
export const stdioSemioV1AudioDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1AudioDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1AudioDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1AudioDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers


function tag(value:unknown,at:string):SemioAudioTag{const r=stdioSemioV1AudioDiffGuardObject(value,at);return{key:stdioSemioV1AudioDiffGuardString(r.key,at+".key"),value:stdioSemioV1AudioDiffGuardString(r.value,at+".value")};}
function channel(value:unknown,at:string):SemioAudioChannel{const r=stdioSemioV1AudioDiffGuardObject(value,at);return{samples:stdioSemioV1AudioDiffGuardArray(r.samples,at+".samples").map(parseBinary32)};}
function channelDiff(value:unknown,at:string):SemioAudioChannelDiff{const r=stdioSemioV1AudioDiffGuardObject(value,at);return{samples:r.samples===undefined?undefined:stdioSemioV1AudioDiffGuardArray(r.samples,at+".samples").map(parseBinary32)};}
function triple<D,T>(value:unknown,at:string,diff:(v:unknown,a:string)=>D,item:(v:unknown,a:string)=>T):IndexedTripleDiff<D,T>{const r=stdioSemioV1AudioDiffGuardObject(value,at);return{removed:stdioSemioV1AudioDiffGuardArray(r.removed,at+".removed").map((v,i)=>stdioSemioV1AudioDiffGuardInteger(v,at+".removed["+i+"]",{minimum:0})),modified:stdioSemioV1AudioDiffGuardArray(r.modified,at+".modified").map((v,i)=>{const a=at+".modified["+i+"]",m=stdioSemioV1AudioDiffGuardObject(v,a);return{index:stdioSemioV1AudioDiffGuardInteger(m.index,a+".index",{minimum:0}),diff:diff(m.diff,a+".diff")};}),added:stdioSemioV1AudioDiffGuardArray(r.added,at+".added").map((v,i)=>{const a=at+".added["+i+"]",m=stdioSemioV1AudioDiffGuardObject(v,a);return{index:stdioSemioV1AudioDiffGuardInteger(m.index,a+".index",{minimum:0}),item:item(m.item,a+".item")};})};}
export function parseSemioAudioDiff(value:unknown,at="$"):SemioAudioDiff{const r=stdioSemioV1AudioDiffGuardObject(value,at);return{sampleRate:r.sampleRate===undefined?undefined:stdioSemioV1AudioDiffGuardInteger(r.sampleRate,at+".sampleRate",{minimum:0,maximum:0xffffffff}),format:r.format===undefined?undefined:stdioSemioV1AudioDiffGuardMember(r.format,at+".format",["pcm8","pcm16","pcm24","pcm32","f32","f64"] as const),channels:r.channels===undefined?undefined:triple(r.channels,at+".channels",channelDiff,channel),tags:r.tags===undefined?undefined:triple(r.tags,at+".tags",tag,tag)};}
