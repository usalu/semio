/** 🔺️ TiffDiff schema facet — mirrors 🦀️.rs field-for-field. No full-replace slot:
 * `ifds` is an index-keyed removed/modified/added triple; within a modified IFD, `entries` is
 * a TAG-ID-keyed triple (`tag`, not array index — a `TiffTag` is a weak value, so
 * modified/added carry the whole new tag). */

import type { TiffSampleBlock, TiffIfd, TiffValues } from '../📸️snapshot/🟦️.ts';

export interface TiffTagModified { tag: number; values: TiffValues }
export interface TiffTagAdded { tag: number; values: TiffValues }

/** Tag-id-keyed `entries` triple for one IFD. */
export interface TiffTagsDiff {
  removed?: number[];
  modified?: TiffTagModified[];
  added?: TiffTagAdded[];
}

/** One IFD's own delta: the recursive tag triple plus a whole-value slot for its raw strip payload. */
export interface TiffIfdDiff { entries?: TiffTagsDiff; blocks?: TiffSampleBlock[] }
export interface TiffIfdModified { index: number; diff: TiffIfdDiff }
export interface TiffIfdAdded { index: number; ifd: TiffIfd }

/** Index-keyed `ifds` triple (TIFF's IFD chain is positional). */
export interface TiffIfdsDiff {
  removed?: number[];
  modified?: TiffIfdModified[];
  added?: TiffIfdAdded[];
}

/** 🔺️ Sparse diff for `stdio.tiff`. Every field present = changed to a value. No tri-state
 * fields at this level — `byteOrder`/`pixels` are always-present scalars, `ifds` is the only
 * collection. */
export interface TiffDiff {
  ifds?: TiffIfdsDiff;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTiff60DocumentDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTiff60DocumentDiffGuardReject = (at: string, why: string): never => {
  throw new stdioTiff60DocumentDiffGuardRefusal(at, why);
};

type stdioTiff60DocumentDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTiff60DocumentDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTiff60DocumentDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTiff60DocumentDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTiff60DocumentDiffGuardReject(at, "value is not an object");
export const stdioTiff60DocumentDiffGuardArray = (value: unknown, at: string, bounds: stdioTiff60DocumentDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTiff60DocumentDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTiff60DocumentDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTiff60DocumentDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTiff60DocumentDiffGuardString = (value: unknown, at: string, bounds: stdioTiff60DocumentDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTiff60DocumentDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTiff60DocumentDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTiff60DocumentDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTiff60DocumentDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTiff60DocumentDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTiff60DocumentDiffGuardReject(at, "value is not a boolean"));
export const stdioTiff60DocumentDiffGuardNumber = (value: unknown, at: string, bounds: stdioTiff60DocumentDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTiff60DocumentDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTiff60DocumentDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTiff60DocumentDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTiff60DocumentDiffGuardInteger = (value: unknown, at: string, bounds: stdioTiff60DocumentDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTiff60DocumentDiffGuardNumber(value, at, bounds) : stdioTiff60DocumentDiffGuardReject(at, "value is not an integer");
export const stdioTiff60DocumentDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTiff60DocumentDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTiff60DocumentDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTiff60DocumentDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTiffTagsDiff(value: unknown, at = "$"): TiffTagsDiff {
  const row = stdioTiff60DocumentDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioTiff60DocumentDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioTiff60DocumentDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0, "maximum": 65535})),
    modified: row["modified"] === undefined ? undefined : stdioTiff60DocumentDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseTiffTagModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioTiff60DocumentDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseTiffTagAdded(item, `${at}.added[${index}]`)),
  };
}

export function parseTiffIfdDiff(value:unknown,at="$"):TiffIfdDiff{
 const row=stdioTiff60DocumentDiffGuardObject(value,at);if(Object.keys(row).some(key=>!["entries","blocks"].includes(key)))throw Error(at+": foreign owned diff field");
 return {entries:row.entries===undefined?undefined:parseTiffTagsDiff(row.entries,at+".entries"),blocks:row.blocks===undefined?undefined:stdioTiff60DocumentDiffGuardArray(row.blocks,at+".blocks").map((v,i)=>parseTiffSampleBlock(v,at+".blocks["+i+"]"))}
}
export function parseTiffIfdModified(value: unknown, at = "$"): TiffIfdModified {
  const row = stdioTiff60DocumentDiffGuardObject(value, at);
  return {
    index: stdioTiff60DocumentDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    diff: parseTiffIfdDiff(row["diff"], `${at}.diff`),
  };
}

export function parseTiffIfdsDiff(value: unknown, at = "$"): TiffIfdsDiff {
  const row = stdioTiff60DocumentDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioTiff60DocumentDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioTiff60DocumentDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioTiff60DocumentDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseTiffIfdModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioTiff60DocumentDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseTiffIfdAdded(item, `${at}.added[${index}]`)),
  };
}

import {parseTiffIfd,parseTiffValues,parseTiffSampleBlock,parseTiffSnapshot,type TiffSnapshot} from "../📸️snapshot/🟦️.ts";
export function parseTiffTagModified(value:unknown,at="$"):TiffTagModified{const row=stdioTiff60DocumentDiffGuardObject(value,at);if(Object.keys(row).some(v=>!["tag","values"].includes(v)))throw Error(at+": foreign tag diff field");return {tag:stdioTiff60DocumentDiffGuardInteger(row.tag,at+".tag",{minimum:0,maximum:65535}),values:parseTiffValues(row.values,at+".values")}}
export function parseTiffTagAdded(value:unknown,at="$"):TiffTagAdded{return parseTiffTagModified(value,at)}
export function parseTiffIfdAdded(value:unknown,at="$"):TiffIfdAdded{const row=stdioTiff60DocumentDiffGuardObject(value,at);return {index:stdioTiff60DocumentDiffGuardInteger(row.index,at+".index",{minimum:0}),ifd:parseTiffIfd(row.ifd,at+".ifd")}}
export function parseTiffDiff(value:unknown,at="$"):TiffDiff{const row=stdioTiff60DocumentDiffGuardObject(value,at);if(Object.keys(row).some(v=>v!=="ifds"))throw Error(at+": foreign diff field");return {ifds:row.ifds===undefined?undefined:parseTiffIfdsDiff(row.ifds,at+".ifds")}}

function same(left:unknown,right:unknown):boolean{if(left===right)return true;if(Array.isArray(left)&&Array.isArray(right))return left.length===right.length&&left.every((value,index)=>same(value,right[index]));if(left&&right&&typeof left==='object'&&typeof right==='object'){const a=left as Record<string,unknown>,b=right as Record<string,unknown>,keys=Object.keys(a);return keys.length===Object.keys(b).length&&keys.every(key=>same(a[key],b[key]));}return false;}
/** 🔺️ Derives a sparse semantic delta by owned directory and tag identity. */
export function tiffDiffBetween(base:TiffSnapshot,target:TiffSnapshot):TiffDiff{
 const removed:number[]=[],modified:TiffIfdModified[]=[],added:TiffIfdAdded[]=[];
 for(let i=0;i<Math.max(base.ifds.length,target.ifds.length);i++){const a=base.ifds[i],b=target.ifds[i];if(!b){removed.push(i);continue;}if(!a){added.push({index:i,ifd:structuredClone(b)});continue;}const tags:TiffTagsDiff={},byId=new Map(a.entries.map(entry=>[entry.tag,entry]));for(const entry of a.entries)if(!b.entries.some(value=>value.tag===entry.tag))(tags.removed??=[]).push(entry.tag);for(const entry of b.entries){const previous=byId.get(entry.tag);if(!previous)(tags.added??=[]).push(structuredClone(entry));else if(!same(previous,entry))(tags.modified??=[]).push(structuredClone(entry));}const diff:TiffIfdDiff={};if(Object.keys(tags).length)diff.entries=tags;if(!same(a.blocks,b.blocks))diff.blocks=structuredClone(b.blocks);if(Object.keys(diff).length)modified.push({index:i,diff});}
 return removed.length||modified.length||added.length?{ifds:{...(removed.length?{removed}:{}),...(modified.length?{modified}:{}),...(added.length?{added}:{})}}:{};
}
/** 🧬️ Applies one sparse owned delta before validating the complete result. */
export function applyTiffDiff(base:TiffSnapshot,diff:TiffDiff):TiffSnapshot{
 const output=structuredClone(base),change=diff.ifds;if(!change)return output;
 for(const edit of change.modified??[]){const page=output.ifds[edit.index];if(!page)throw Error('tiff: modified directory missing');if(edit.diff.blocks!==undefined)page.blocks=structuredClone(edit.diff.blocks);const tags=edit.diff.entries;if(tags){page.entries=page.entries.filter(entry=>!tags.removed?.includes(entry.tag));for(const entry of tags.modified??[]){const index=page.entries.findIndex(value=>value.tag===entry.tag);if(index<0)throw Error('tiff: modified tag missing');page.entries[index]=structuredClone(entry);}for(const entry of tags.added??[]){if(page.entries.some(value=>value.tag===entry.tag))throw Error('tiff: added tag exists');page.entries.push(structuredClone(entry));}page.entries.sort((a,b)=>a.tag-b.tag);}}
 const removed=new Set(change.removed??[]);if(removed.size!==(change.removed??[]).length||[...removed].some(index=>index<0||index>=output.ifds.length))throw Error('tiff: removed directory identity');output.ifds=output.ifds.filter((_,index)=>!removed.has(index));for(const entry of [...change.added??[]].sort((a,b)=>a.index-b.index)){if(entry.index>output.ifds.length)throw Error('tiff: added directory position');output.ifds.splice(entry.index,0,structuredClone(entry.ifd));}return parseTiffSnapshot(output);
}
/** 🔁️ Composes deltas relative to the same explicit semantic base. */
export function absorbTiffDiff(base:TiffSnapshot,first:TiffDiff,second:TiffDiff):TiffDiff{return tiffDiffBetween(base,applyTiffDiff(applyTiffDiff(base,first),second));}
