/** 🧬️ JsonArtifact schema. */
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

export interface JsonArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ value: JsonValue;
}
