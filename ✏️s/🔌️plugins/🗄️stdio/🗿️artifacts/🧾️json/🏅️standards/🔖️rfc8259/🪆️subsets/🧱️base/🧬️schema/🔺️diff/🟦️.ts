/** 🔺️ JsonDiff — recursive diff mirroring JsonValue's shape. No full-replace slot: `value` is a
 *  sparse `JsonValueDiff | undefined`, `undefined` meaning no change. Mirrors
 *  `../📸️snapshot/🟦️.ts`'s `JsonValue`/`JsonMember` (restated here, not imported, to
 *  keep each facet leaf self-contained). */
export interface JsonMember {
  key: string;
  value: JsonValue;
}
export type JsonValue =
  | { kind: "null" }
  | { kind: "bool"; value: boolean }
  | { kind: "number"; lexeme: string }
  | { kind: "string"; value: string }
  | { kind: "array"; items: JsonValue[] }
  | { kind: "object"; members: JsonMember[] };

export type JsonValueDiff =
  | { kind: "replace"; value: JsonValue }
  | { kind: "bool"; value: boolean }
  | { kind: "number"; lexeme: string }
  | { kind: "string"; value: string }
  | { kind: "array"; diff: JsonArrayDiff }
  | { kind: "object"; diff: JsonObjectDiff };

export interface JsonArrayModified {
  index: number;
  diff: JsonValueDiff;
}
export interface JsonArrayAdded {
  index: number;
  item: JsonValue;
}
export interface JsonArrayDiff {
  removed?: number[];
  modified?: JsonArrayModified[];
  added?: JsonArrayAdded[];
}

export interface JsonObjectModified {
  key: string;
  diff: JsonValueDiff;
}
export interface JsonObjectAdded {
  index: number;
  key: string;
  item: JsonValue;
}
export interface JsonObjectDiff {
  removed?: string[];
  modified?: JsonObjectModified[];
  added?: JsonObjectAdded[];
}

export interface JsonDiff {
  /** @state artifact */ value?: JsonValueDiff;
}
