/** 🔤️ Declared Remodeling Text reader binds primitive words, record tables, nested braced cells, and literal references. */

import {
  REMODELING_SNAPSHOT_SPEC,
  kebabOf,
  type FieldSpec,
  type RecordSpec,
  type RemodelingSnapshot,
  type ValueSpec,
} from "../../../../../../../🧬️schema/📸️snapshot/🟦️.ts";

import {binary32,binary64} from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";

import {base64StandardDecode} from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🔤️base64/🟦️.ts";

/** 🚫 A DSL refusal carrying the 1-based line it was raised on. */
export class RemodelingDslError extends Error {
  constructor(
    readonly line: number,
    message: string,
  ) {
    super(`line ${line}: ${message}`);
    this.name = "RemodelingDslError";
  }
}

//#region 🔖️Lexer
type TokenKind = "word" | "string" | "open-brace" | "close-brace" | "open-bracket" | "close-bracket" | "equals" | "colon";

interface Token {
  kind: TokenKind;
  text: string;
  line: number;
}

const PUNCTUATION: Record<string, TokenKind> = { "{": "open-brace", "}": "close-brace", "[": "open-bracket", "]": "close-bracket", "=": "equals", ":": "colon" };

/** 🔍 Splits DSL body text into punctuation, bare lexemes and quoted text. */
export function lexDsl(text: string): Token[] {
  const tokens: Token[] = [];
  let line = 1;
  let index = 0;
  while (index < text.length) {
    const character = text[index];
    if (character === "\n") {
      line += 1;
      index += 1;
      continue;
    }
    if (character === " " || character === "\t" || character === "\r") {
      index += 1;
      continue;
    }
    if (character in PUNCTUATION) {
      tokens.push({ kind: PUNCTUATION[character], text: character, line });
      index += 1;
      continue;
    }
    if (character === '"') {
      let value = "";
      index += 1;
      while (index < text.length && text[index] !== '"') {
        if (text[index] === "\\") {
          const escape = text[index + 1];
          value += escape === "n" ? "\n" : escape === "t" ? "\t" : escape === "r" ? "\r" : escape;
          index += 2;
          continue;
        }
        value += text[index];
        index += 1;
      }
      if (index >= text.length) throw new RemodelingDslError(line, "unterminated quoted text");
      index += 1;
      tokens.push({ kind: "string", text: value, line });
      continue;
    }
    let start = index;
    while (index < text.length && !(text[index] in PUNCTUATION) && !/[\s"]/.test(text[index])) index += 1;
    if (index === start) throw new RemodelingDslError(line, `unexpected character ${JSON.stringify(character)}`);
    tokens.push({ kind: "word", text: text.slice(start, index), line });
  }
  return tokens;
}
//#endregion 🔖️Lexer

//#region 🔖️Parser
type DslCell = Token | DslNode | DslNode[];

interface DslTable {
  columns: { name: string; type: string }[];
  rows: DslCell[][];
  line: number;
}

interface DslNode {
  scalars: Map<string, Token>;
  blocks: Map<string, DslNode>;
  tables: Map<string, DslTable>;
  maps: Map<string, Map<string, DslNode>>;
  arrays: Map<string, Token[]>;
  statements: {name:string,node:DslNode}[];
  line: number;
}

const emptyNode = (line: number): DslNode => ({ scalars: new Map(), blocks: new Map(), tables: new Map(), maps: new Map(), arrays:new Map(), statements:[], line });

class Cursor {
  index = 0;
  constructor(readonly tokens: Token[]) {}
  peek(offset = 0): Token | undefined {
    return this.tokens[this.index + offset];
  }
  next(): Token {
    const token = this.tokens[this.index];
    if (token === undefined) throw new RemodelingDslError(this.tokens[this.tokens.length - 1]?.line ?? 1, "unexpected end of document");
    this.index += 1;
    return token;
  }
  expect(kind: TokenKind): Token {
    const token = this.next();
    if (token.kind !== kind) throw new RemodelingDslError(token.line, `expected ${kind}, got ${token.kind} ${JSON.stringify(token.text)}`);
    return token;
  }
}

const readTable = (cursor: Cursor, line: number): DslTable => {
  const columns: { name: string; type: string }[] = [];
  cursor.expect("open-bracket");
  while (cursor.peek()?.kind !== "close-bracket") {
    const name = cursor.expect("word").text;
    cursor.expect("colon");
    columns.push({ name, type: cursor.expect("word").text });
  }
  cursor.expect("close-bracket");
  cursor.expect("open-brace");
  const rows: DslCell[][] = [];
  while (cursor.peek()?.kind !== "close-brace") {
    const row: DslCell[] = [];
    for (const column of columns) {
      const cell = cursor.next();
      if(cell.kind==="word"&&cell.text==="_"){row.push(cell);continue;}
      if(column.type==="TABLE"){
        if(cell.kind!=="open-bracket")throw new RemodelingDslError(cell.line,"nested table requires a braced record list");
        const values:DslNode[]=[];while(cursor.peek()?.kind!=="close-bracket"){const opening=cursor.expect("open-brace");values.push(readNode(cursor,opening.line,true));}
        cursor.expect("close-bracket");row.push(values);continue;
      }
      if(column.type==="BLOCK"){
        if(cell.kind!=="open-brace")throw new RemodelingDslError(cell.line,"nested block requires an opening brace");
        row.push(readNode(cursor,cell.line,true));continue;
      }
      if (cell.kind !== "word" && cell.kind !== "string") throw new RemodelingDslError(cell.line, `expected a scalar cell for column "${column.name}", got ${cell.kind}`);
      row.push(cell);
    }
    rows.push(row);
  }
  cursor.expect("close-brace");
  return { columns, rows, line };
};

/** 🌳 Parses one `{ … }` body into scalars, blocks, tables and map literals. */
function readNode(cursor: Cursor, line: number, terminated: boolean): DslNode {
  const node = emptyNode(line);
  while (cursor.index < cursor.tokens.length) {
    const head = cursor.peek()!;
    if (head.kind === "close-brace") {
      if (!terminated) throw new RemodelingDslError(head.line, "unbalanced closing brace");
      cursor.next();
      return node;
    }
    const key = cursor.expect("word").text;
    const follow = cursor.peek();
    if((key==="inline"||key==="content")&&follow?.kind!=="equals") {node.statements.push({name:key,node:readNode(cursor,head.line,true)});return node;}
    if (follow?.kind === "equals") {
      cursor.next();
      const value = cursor.next();
      if(value.kind==="open-bracket"){const values:Token[]=[];while(cursor.peek()?.kind!=="close-bracket"){const item=cursor.next();if(item.kind!=="word"&&item.kind!=="string")throw new RemodelingDslError(item.line,"expected a scalar list element");values.push(item)}cursor.expect("close-bracket");node.arrays.set(key,values);continue;}
      if (value.kind === "open-brace") {
        const entries = new Map<string, DslNode>();
        while (cursor.peek()?.kind !== "close-brace") {
          const entryKey = cursor.next();
          if (entryKey.kind !== "word" && entryKey.kind !== "string") throw new RemodelingDslError(entryKey.line, `expected a map key for "${key}", got ${entryKey.kind}`);
          cursor.expect("open-brace");
          entries.set(entryKey.text, readNode(cursor, entryKey.line, true));
        }
        cursor.expect("close-brace");
        node.maps.set(key, entries);
        continue;
      }
      if (value.kind !== "word" && value.kind !== "string") throw new RemodelingDslError(value.line, `expected a value for "${key}", got ${value.kind}`);
      node.scalars.set(key, value);
      continue;
    }
    if (follow?.kind === "open-brace") {
      cursor.next();
      node.blocks.set(key, readNode(cursor, follow.line, true));
      continue;
    }
    if (follow?.kind === "open-bracket") {
      node.tables.set(key, readTable(cursor, follow.line));
      continue;
    }
    throw new RemodelingDslError(head.line, `"${key}" is followed by ${follow?.kind ?? "end of document"}, which starts neither a value, a block nor a table`);
  }
  if (terminated) throw new RemodelingDslError(line, "unterminated block");
  return node;
}
//#endregion 🔖️Parser

//#region 🔖️Binder
const UNIT_SUFFIX = /(mm|m\/s|m|deg|rad|s|ms|px|%)$/;

const scalarFloat = (token:Token,width:32|64):unknown => {
  const raw=token.text.replace(UNIT_SUFFIX,"");
  const widened=/^nan64_([0-9a-f]{16})$/.exec(raw);
  if(width===32&&widened){const bits=BigInt("0x"+widened[1]);if((bits&0x7ff0000000000000n)!==0x7ff0000000000000n||(bits&0xfffffffffffffn)===0n||(bits&0x1fffffffn)!==0n)throw new RemodelingDslError(token.line,"NaN word is not exactly representable at binary32 width");return{bits:Number(((bits>>32n)&0x80000000n)|0x7f800000n|((bits>>29n)&0x7fffffn))};}
  const exact=width===32?/^nan32_([0-9a-f]{8})$/:/^nan64_([0-9a-f]{16})$/;
  const word=exact.exec(raw);
  if(word){const bits=BigInt("0x"+word[1]);if(width===32){if((bits&0x7f800000n)!==0x7f800000n||(bits&0x007fffffn)===0n)throw new RemodelingDslError(token.line,"invalid binary32 NaN word");return{bits:Number(bits)}}if((bits&0x7ff0000000000000n)!==0x7ff0000000000000n||(bits&0x000fffffffffffffn)===0n)throw new RemodelingDslError(token.line,"invalid binary64 NaN word");return{bits}}
  const value=raw==="inf"?Infinity:raw==="-inf"?-Infinity:Number(raw);
  if(raw===""||Number.isNaN(value))throw new RemodelingDslError(token.line,`expected a binary${width} number, got ${JSON.stringify(token.text)}`);
  return width===32?binary32(value):binary64(value);
};
const scalarInteger=(token:Token,signed:boolean):bigint=>{if(!/^-?(0|[1-9][0-9]*)$/.test(token.text)||token.text==="-0")throw new RemodelingDslError(token.line,"expected a canonical integer");const value=BigInt(token.text),minimum=signed?-(1n<<63n):0n,maximum=signed?(1n<<63n)-1n:(1n<<64n)-1n;if(value<minimum||value>maximum)throw new RemodelingDslError(token.line,"integer exceeds native width");return value};

const bindScalar = (token: Token, spec: ValueSpec, key: string): unknown => {
  switch (spec.k) {
    case "text":return token.text;
    case "bytes":if(token.kind!=="string")throw new RemodelingDslError(token.line,"expected quoted Bytes64");return base64StandardDecode(token.text);
    case "bool":
      if (token.text !== "true" && token.text !== "false") throw new RemodelingDslError(token.line, `expected a boolean for "${key}", got ${JSON.stringify(token.text)}`);
      return token.text === "true";
    case "uint": {const value=scalarInteger(token,false);if(value>0xffffffffn)throw new RemodelingDslError(token.line,"integer exceeds unsigned32");return Number(value)}
    case "int":return scalarInteger(token,true);
    case "u64":return scalarInteger(token,false);
    case "f64":return scalarFloat(token,64);
    case "f32":return scalarFloat(token,32);
    case "enum":
      if (!spec.of.includes(token.text)) throw new RemodelingDslError(token.line, `expected one of ${spec.of.join(" | ")} for "${key}", got ${JSON.stringify(token.text)}`);
      return token.text;
    case "tuple": {
      const parts=token.text.split(",");if(parts.length!==spec.len)throw new RemodelingDslError(token.line,`expected ${spec.len} comma-separated numbers for "${key}"`);return parts.map(text=>scalarFloat({...token,text},spec.w));
    }
    case "opt":
      return bindScalar(token, spec.of, key);
    default:
      throw new RemodelingDslError(token.line, `"${key}" is a ${spec.k}, which cannot be written as a DSL scalar`);
  }
};

const inner = (spec: ValueSpec): ValueSpec => (spec.k === "opt" ? spec.of : spec);

const bindArtifactRef=(node:DslNode):Record<string,unknown>=>{
 const required=["artifact-id","artifact-kind","standard","subset"];if(node.blocks.size||node.maps.size||node.tables.size||node.arrays.size||node.statements.length||node.scalars.size!==4)throw new RemodelingDslError(node.line,"artifact reference requires exactly four literal text fields");
 const values=required.map(key=>{const value=node.scalars.get(key);if(!value)throw new RemodelingDslError(node.line,"missing artifact reference "+key);return value.text});return{artifactId:values[0],dialect:{artifactKind:values[1],standard:values[2],subset:values[3]}};
};
const bindFloatBuffer=(node:DslNode):unknown=>{
 if(node.scalars.size||node.blocks.size||node.maps.size||node.tables.size||node.arrays.size||node.statements.length!==1)throw new RemodelingDslError(node.line,"float buffer requires one typed variant");
 const value=node.statements[0]!,body=value.node;
 if(body.blocks.size||body.tables.size||body.maps.size||body.statements.length)throw new RemodelingDslError(body.line,"unexpected float buffer child");
 if(value.name==="inline"){const values=body.arrays.get("values");if(body.scalars.size||body.arrays.size!==1||!values)throw new RemodelingDslError(body.line,"inline buffer requires samples");return{kind:"inline",values:values.map(token=>scalarFloat(token,32))}}
 const id=body.scalars.get("content-id"),count=body.scalars.get("chunk-count");if(body.arrays.size||body.scalars.size!==2||!id||!count)throw new RemodelingDslError(body.line,"content buffer requires literal identity and count");return{kind:"content",contentId:id.text,chunkCount:scalarInteger(count,false)};
};

const dslKeyOf = (field: FieldSpec): string => field.dslKey ?? kebabOf(field.name);

const bindCell=(cell:DslCell,spec:ValueSpec,key:string):unknown=>{
 if(Array.isArray(cell)){const shape=inner(spec);if(shape.k!=="list"||inner(shape.of).k!=="rec")throw new RemodelingDslError(cell[0]?.line??1,"nested table requires a record-list field");const item=inner(shape.of);if(item.k!=="rec")throw new RemodelingDslError(1,"nested table item differs");return cell.map(node=>bindRecord(node,item.of()));}
 if("kind" in cell){if(cell.kind==="word"&&cell.text==="_"){if(spec.k!=="opt")throw new RemodelingDslError(cell.line,"absent table cell requires an optional field");return null;}return bindScalar(cell,spec,key);}
 const shape=inner(spec);if(shape.k!=="rec")throw new RemodelingDslError(cell.line,"nested block requires a record field");return bindRecord(cell,shape.of());
};
const bindRow = (row: DslCell[], table: DslTable, spec: RecordSpec): Record<string, unknown> => {
  const out: Record<string, unknown> = {};
  for (const field of spec.fields) out[camelKey(field.name)] = field.dflt();
  const consumed=new Set<string>();
  table.columns.forEach((column, index) => {
    const field = spec.fields.find((candidate) => dslKeyOf(candidate) === column.name);
    if (field === undefined) throw new RemodelingDslError(table.line, `unknown column "${column.name}" for ${spec.title}`);
    if(consumed.has(column.name))throw new RemodelingDslError(table.line,"duplicate table column "+column.name);
    consumed.add(column.name);out[camelKey(field.name)] = bindCell(row[index]!,field.spec,column.name);
  });
  return out;
};

const camelKey = (snake: string): string => snake.split("_").map((part, index) => (index === 0 ? part : part.charAt(0).toUpperCase() + part.slice(1))).join("");

/** 🧱 Binds one parsed node onto a record spec; an absent key falls to its Rust default. */
export function bindRecord(node: DslNode, spec: RecordSpec): Record<string, unknown> {
  if(spec.title==="ArtifactRef")return bindArtifactRef(node);
  const out: Record<string, unknown> = {};
  const consumed = new Set<string>();
  for (const field of spec.fields) {
    const key = dslKeyOf(field);
    const shape = inner(field.spec);
    consumed.add(key);
    if(shape.k==="floatBuffer"&&node.blocks.has(key)){out[camelKey(field.name)]=bindFloatBuffer(node.blocks.get(key)!);continue;}
    if(shape.k==="list"&&node.arrays.has(key)){out[camelKey(field.name)]=node.arrays.get(key)!.map(token=>bindScalar(token,shape.of,key));continue;}
    if (shape.k === "rec" && node.blocks.has(key)) {
      out[camelKey(field.name)] = bindRecord(node.blocks.get(key)!, shape.of());
      continue;
    }
    if (shape.k === "list" && node.tables.has(key)) {
      const table = node.tables.get(key)!;
      const item = inner(shape.of);
      if (item.k !== "rec") throw new RemodelingDslError(table.line, `"${key}" is a table of ${item.k}, which this reader does not implement`);
      out[camelKey(field.name)] = table.rows.map((row) => bindRow(row, table, item.of()));
      continue;
    }
    if (shape.k === "map" && node.maps.has(key)) {
      const entries = node.maps.get(key)!;
      const value = inner(shape.of);
      if (value.k !== "rec") throw new RemodelingDslError(node.line, `"${key}" is a map of ${value.k}, which this reader does not implement`);
      const bound: Record<string, unknown> = {};
      for (const [entryKey, entryNode] of [...entries.entries()].sort(([left], [right]) => (left < right ? -1 : left > right ? 1 : 0))) Object.defineProperty(bound,entryKey,{value:bindRecord(entryNode,value.of()),enumerable:true,writable:true,configurable:true});
      out[camelKey(field.name)] = bound;
      continue;
    }
    if (node.scalars.has(key)) {
      out[camelKey(field.name)] = bindScalar(node.scalars.get(key)!, field.spec, key);
      continue;
    }
    out[camelKey(field.name)] = field.dflt();
  }
  for (const key of [...node.scalars.keys(), ...node.blocks.keys(), ...node.tables.keys(), ...node.maps.keys(),...node.arrays.keys()])
    if (!consumed.has(key)) throw new RemodelingDslError(node.line, `unknown key "${key}" for ${spec.title}`);
  return out;
}
//#endregion 🔖️Binder

//#region 🔖️Entry
export const REMODELING_DSL_ENVELOPE = "semio remodeling.remodeling.dsl v1";

/** 📄️ Parses a `.dsl.semio` remodeling document into a validated `RemodelingSnapshot`. */
export function remodelingSnapshotFromDslText(text: string): RemodelingSnapshot {
  const newline = text.indexOf("\n");
  const preamble = (newline < 0 ? text : text.slice(0, newline)).trim();
  if (preamble !== REMODELING_DSL_ENVELOPE) throw new RemodelingDslError(1, `expected the preamble ${JSON.stringify(REMODELING_DSL_ENVELOPE)}, got ${JSON.stringify(preamble)}`);
  const body = newline < 0 ? "" : text.slice(newline + 1);
  const cursor = new Cursor(lexDsl(body));
  const node = readNode(cursor, 2, false);
  return bindRecord(node, REMODELING_SNAPSHOT_SPEC) as unknown as RemodelingSnapshot;
}
//#endregion 🔖️Entry
