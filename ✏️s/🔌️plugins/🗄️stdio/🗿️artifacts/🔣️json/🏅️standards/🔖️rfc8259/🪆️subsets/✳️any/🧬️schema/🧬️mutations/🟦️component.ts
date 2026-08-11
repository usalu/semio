/** 🧬️ JsonMutation union — every variant addresses its target via a JsonPath. Restates
 *  `../📸️snapshot/🟦️component.ts`'s `JsonValue`/`JsonSnapshot` so this leaf is self-contained. */
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
export interface JsonSnapshot {
  schema: string;
  value: JsonValue;
}

export type JsonPathSegment = { kind: "key"; value: string } | { kind: "index"; value: number };
export type JsonPath = JsonPathSegment[];

export type JsonMutation =
  | { mutation: "noMutation" }
  | { mutation: "setSnapshot"; snapshot: JsonSnapshot }
  | { mutation: "setMember"; path: JsonPath; key: string; value: JsonValue }
  | { mutation: "removeMember"; path: JsonPath; key: string }
  | { mutation: "insertArrayElement"; path: JsonPath; index: number; value: JsonValue }
  | { mutation: "removeArrayElement"; path: JsonPath; index: number }
  | { mutation: "setScalar"; path: JsonPath; value: JsonValue };
