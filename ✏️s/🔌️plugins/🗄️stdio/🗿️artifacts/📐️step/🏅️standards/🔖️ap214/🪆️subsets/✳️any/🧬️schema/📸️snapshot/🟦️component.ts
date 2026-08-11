/** 🧬️ StepSnapshot schema — typed ISO 10303-21 HEADER triple + id-keyed entity graph, mirroring
 * the Rust `StepSnapshot` shape 1:1. Matches serde's default externally-tagged representation for
 * `StepValue` (unit variants serialize as a bare camelCase string; tuple/struct variants as a
 * single-key object). */

/** 🔤️ One typed Part-21 argument value. */
export type StepValue =
  | 'unset'
  | 'derived'
  | { integer: number }
  | { real: number }
  | { string: string }
  | { enum: string }
  | { reference: number }
  | { aggregate: StepValue[] }
  | { typedValue: { typeName: string; value: StepValue } };

/** 📇️ `FILE_DESCRIPTION(description, implementation_level)`. */
export interface StepFileDescription {
  description: string[];
  implementationLevel: string;
}

/** 📇️ `FILE_NAME(name, timestamp, author, organization, preprocessor_version,
 * originating_system, authorization)`. */
export interface StepFileName {
  name: string;
  timestamp: string;
  author: string[];
  organization: string[];
  preprocessorVersion: string;
  originatingSystem: string;
  authorization: string;
}

/** 📇️ `FILE_SCHEMA(schemas)`. */
export interface StepFileSchema {
  schemas: string[];
}

/** 📇️ The full typed `HEADER;` section. */
export interface StepHeader {
  fileDescription: StepFileDescription;
  fileName: StepFileName;
  fileSchema: StepFileSchema;
}

/** 🧩️ An additional type record on a genuinely complex Part-21 instance
 * (`#N=(TYPE1(...)TYPE2(...))`) — rare, spec-legal, never dropped. */
export interface StepComplexType {
  name: string;
  args: StepValue[];
}

/** 🧩️ One `#N = TYPE(args...)` instance — id-keyed identity, positional argument list. */
export interface StepEntity {
  id: number;
  name: string;
  args: StepValue[];
  complex?: StepComplexType[];
}

/** 📸️ Persisted `stdio.step` snapshot. */
export interface StepSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ header: StepHeader;
  /** @state persistent */ entities: StepEntity[];
}
