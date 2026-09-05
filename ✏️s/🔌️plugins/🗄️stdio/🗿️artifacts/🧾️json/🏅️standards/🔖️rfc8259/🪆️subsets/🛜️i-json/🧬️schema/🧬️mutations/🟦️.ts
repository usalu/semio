/** 🧬️ JsonIJsonMutation union — the RFC 7493 I-JSON editing vocabulary. Four verbs carry a clause of
 *  the profile (`setTopLevel` §2.1, `setSafeNumber` §2.2, `renameMember` §2.3, `setString` §2.4); the
 *  four object/array verbs are the ✳️any subset's, inherited unchanged. Restates
 *  `../../../✳️any/🧬️schema/🧬️mutations/🟦️.ts`'s `JsonValue`/`JsonSnapshot`/`JsonPath` so
 *  this leaf is self-contained. */
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

/** 🌳️ §2.1 made structural: an I-JSON document root is an object or an array, never a scalar. */
export type JsonIJsonRoot = { kind: "object"; members: JsonMember[] } | { kind: "array"; items: JsonValue[] };

export type JsonIJsonMutation =
  | { mutation: "setSnapshot"; snapshot: JsonSnapshot }
  | { mutation: "setTopLevel"; root: JsonIJsonRoot }
  | { mutation: "upsertMember"; path: JsonPath; key: string; value: JsonValue }
  | { mutation: "removeMember"; path: JsonPath; key: string }
  | { mutation: "renameMember"; path: JsonPath; from: string; to: string }
  | { mutation: "setSafeNumber"; path: JsonPath; lexeme: string }
  | { mutation: "setString"; path: JsonPath; value: string }
  | { mutation: "insertArrayElement"; path: JsonPath; index: number; value: JsonValue }
  | { mutation: "removeArrayElement"; path: JsonPath; index: number };

/** 🔢️ RFC 7493 §2.2 — the largest integer magnitude an IEEE-754 double represents exactly. */
export const MAX_SAFE_INTEGER_MAGNITUDE = 9007199254740991;
