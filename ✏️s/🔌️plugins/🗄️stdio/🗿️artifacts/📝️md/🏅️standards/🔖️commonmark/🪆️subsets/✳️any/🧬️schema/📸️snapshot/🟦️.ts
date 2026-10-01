/** 🧩 A real CommonMark inline node. Weak entity (recipe): whole-value replaced in diffs. */
export type MdInline =
  | { kind: 'text'; text: string }
  | { kind: 'emphasis'; inlines: MdInline[] }
  | { kind: 'strong'; inlines: MdInline[] }
  | { kind: 'code'; literal: string }
  | { kind: 'link'; text: MdInline[]; url: string; title?: string }
  | { kind: 'image'; alt: string; url: string; title?: string }
  | { kind: 'softBreak' }
  | { kind: 'hardBreak' }
  | { kind: 'htmlInline'; raw: string };

/** 🧱 A real CommonMark block. Strong-like entity: block collections are index-keyed and
 * per-field diffed (see `../🔺️diff/🟦️.ts`). */
export type MdBlock =
  | { kind: 'heading'; level: number; inlines: MdInline[] }
  | { kind: 'paragraph'; inlines: MdInline[] }
  | { kind: 'list'; ordered: boolean; start?: number; tight: boolean; items: MdBlock[][] }
  | { kind: 'codeBlock'; info?: string; literal: string }
  | { kind: 'blockQuote'; blocks: MdBlock[] }
  | { kind: 'thematicBreak' }
  | { kind: 'htmlBlock'; raw: string };

/** 📸️ Persisted `stdio.md` snapshot: the complete top-level block sequence. */
export interface MdSnapshot {
  schema: string;
  blocks: MdBlock[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioMdCommonmarkAnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioMdCommonmarkAnySnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioMdCommonmarkAnySnapshotGuardRefusal(at, why);
};

type stdioMdCommonmarkAnySnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioMdCommonmarkAnySnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioMdCommonmarkAnySnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioMdCommonmarkAnySnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioMdCommonmarkAnySnapshotGuardReject(at, "value is not an object");
export const stdioMdCommonmarkAnySnapshotGuardArray = (value: unknown, at: string, bounds: stdioMdCommonmarkAnySnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioMdCommonmarkAnySnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioMdCommonmarkAnySnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioMdCommonmarkAnySnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioMdCommonmarkAnySnapshotGuardString = (value: unknown, at: string, bounds: stdioMdCommonmarkAnySnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioMdCommonmarkAnySnapshotGuardReject(at, "value is not a string");
  const length = bounds.minLength === undefined && bounds.maxLength === undefined ? 0 : [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioMdCommonmarkAnySnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioMdCommonmarkAnySnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioMdCommonmarkAnySnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioMdCommonmarkAnySnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioMdCommonmarkAnySnapshotGuardReject(at, "value is not a boolean"));
export const stdioMdCommonmarkAnySnapshotGuardNumber = (value: unknown, at: string, bounds: stdioMdCommonmarkAnySnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioMdCommonmarkAnySnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioMdCommonmarkAnySnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioMdCommonmarkAnySnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioMdCommonmarkAnySnapshotGuardInteger = (value: unknown, at: string, bounds: stdioMdCommonmarkAnySnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioMdCommonmarkAnySnapshotGuardNumber(value, at, bounds) : stdioMdCommonmarkAnySnapshotGuardReject(at, "value is not an integer");
export const stdioMdCommonmarkAnySnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioMdCommonmarkAnySnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioMdCommonmarkAnySnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioMdCommonmarkAnySnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseMdSnapshot(value: unknown, at = "$"): MdSnapshot {
  const row = stdioMdCommonmarkAnySnapshotGuardObject(value, at);
  return {
    schema: stdioMdCommonmarkAnySnapshotGuardString(row["schema"], `${at}.schema`),
    blocks: stdioMdCommonmarkAnySnapshotGuardArray(row["blocks"], `${at}.blocks`).map((item, index) => parseMdBlock(item, `${at}.blocks[${index}]`)),
  };
}

type MdParseFrame={kind:"blocks";values:readonly unknown[];index:number;out:MdBlock[];at:string;single?:boolean}|{kind:"inlines";values:readonly unknown[];index:number;out:MdInline[];at:string;single?:boolean}|{kind:"items";values:readonly unknown[];index:number;out:MdBlock[][];at:string}|{kind:"exit";value:object};
function parseMdTree(value:unknown,mode:"block"|"inline",at:string):MdBlock|MdInline{
 const blocks:MdBlock[]=[],inlines:MdInline[]=[],frames:MdParseFrame[]=[mode==="block"?{kind:"blocks",values:[value],index:0,out:blocks,at,single:true}:{kind:"inlines",values:[value],index:0,out:inlines,at,single:true}],ancestors=new WeakSet<object>();
 while(frames.length){const frame=frames.pop()!;if(frame.kind==="exit"){ancestors.delete(frame.value);continue;}if(frame.index>=frame.values.length)continue;frames.push({...frame,index:frame.index+1});if(frame.kind==="items"){const blocks:MdBlock[]=[];const path=`${frame.at}[${frame.index}]`;frame.out.push(blocks);frames.push({kind:"blocks",values:stdioMdCommonmarkAnySnapshotGuardArray(frame.values[frame.index],path),index:0,out:blocks,at:path});continue;}
  const path=frame.single?frame.at:`${frame.at}[${frame.index}]`,row=stdioMdCommonmarkAnySnapshotGuardObject(frame.values[frame.index],path);if(ancestors.has(row))stdioMdCommonmarkAnySnapshotGuardReject(path,"tree has a cyclic node");ancestors.add(row);frames.push({kind:"exit",value:row});const text=(name:string):string=>stdioMdCommonmarkAnySnapshotGuardString(row[name],`${path}.${name}`),optional=(name:"title"|"info"):Partial<Record<typeof name,string>>=>row[name]===undefined?{}:{[name]:text(name)};
  if(frame.kind==="inlines"){const kind=stdioMdCommonmarkAnySnapshotGuardMember(row.kind,`${path}.kind`,["text","emphasis","strong","code","link","image","softBreak","hardBreak","htmlInline"] as const);switch(kind){
   case "text":frame.out.push({kind,text:text("text")});break;
   case "emphasis":case "strong":{const inlines:MdInline[]=[];frame.out.push({kind,inlines});frames.push({kind:"inlines",values:stdioMdCommonmarkAnySnapshotGuardArray(row.inlines,`${path}.inlines`),index:0,out:inlines,at:`${path}.inlines`});break;}
   case "code":frame.out.push({kind,literal:text("literal")});break;
   case "link":{const children:MdInline[]=[];frame.out.push({kind,text:children,url:text("url"),...optional("title")});frames.push({kind:"inlines",values:stdioMdCommonmarkAnySnapshotGuardArray(row.text,`${path}.text`),index:0,out:children,at:`${path}.text`});break;}
   case "image":frame.out.push({kind,alt:text("alt"),url:text("url"),...optional("title")});break;
   case "softBreak":case "hardBreak":frame.out.push({kind});break;
   case "htmlInline":frame.out.push({kind,raw:text("raw")});break;
  }continue;}
  const kind=stdioMdCommonmarkAnySnapshotGuardMember(row.kind,`${path}.kind`,["heading","paragraph","list","codeBlock","blockQuote","thematicBreak","htmlBlock"] as const);switch(kind){
   case "heading":case "paragraph":{const inlines:MdInline[]=[];frame.out.push(kind==="heading"?{kind,level:stdioMdCommonmarkAnySnapshotGuardInteger(row.level,`${path}.level`,{minimum:0,maximum:255}),inlines}:{kind,inlines});frames.push({kind:"inlines",values:stdioMdCommonmarkAnySnapshotGuardArray(row.inlines,`${path}.inlines`),index:0,out:inlines,at:`${path}.inlines`});break;}
   case "list":{const items:MdBlock[][]=[];frame.out.push({kind,ordered:stdioMdCommonmarkAnySnapshotGuardBoolean(row.ordered,`${path}.ordered`),tight:stdioMdCommonmarkAnySnapshotGuardBoolean(row.tight,`${path}.tight`),...(row.start===undefined?{}:{start:stdioMdCommonmarkAnySnapshotGuardInteger(row.start,`${path}.start`,{minimum:0,maximum:4294967295})}),items});frames.push({kind:"items",values:stdioMdCommonmarkAnySnapshotGuardArray(row.items,`${path}.items`),index:0,out:items,at:`${path}.items`});break;}
   case "codeBlock":frame.out.push({kind,literal:text("literal"),...optional("info")});break;
   case "blockQuote":{const blocks:MdBlock[]=[];frame.out.push({kind,blocks});frames.push({kind:"blocks",values:stdioMdCommonmarkAnySnapshotGuardArray(row.blocks,`${path}.blocks`),index:0,out:blocks,at:`${path}.blocks`});break;}
   case "thematicBreak":frame.out.push({kind});break;
   case "htmlBlock":frame.out.push({kind,raw:text("raw")});break;
  }
 }return mode==="block"?blocks[0]!:inlines[0]!;
}
/** 🌲️ Admits the full native-width block model without recursive stack growth. */
export function parseMdBlock(value:unknown,at="$"):MdBlock{return parseMdTree(value,"block",at) as MdBlock;}
/** 🍃️ Admits every inline subtype while retaining absent and empty optionals. */
export function parseMdInline(value:unknown,at="$"):MdInline{return parseMdTree(value,"inline",at) as MdInline;}
