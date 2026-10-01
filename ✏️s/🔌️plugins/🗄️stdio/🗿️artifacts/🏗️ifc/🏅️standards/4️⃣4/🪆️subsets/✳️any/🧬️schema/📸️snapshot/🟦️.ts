import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 🔤️ One typed value in IFC4's Part-21 argument-list syntax. */
export type IfcValue =
  | { kind: "unset" }
  | { kind: "derived" }
  | { kind: "integer"; value: bigint }
  | { kind: "real"; value: Binary64 }
  | { kind: "string"; value: string }
  | { kind: "enum"; value: string }
  | { kind: "reference"; value: bigint }
  | { kind: "aggregate"; value: IfcValue[] }
  | { kind: "typedValue"; value: {name: string; items: IfcValue[]} };

/** 🧩️ One additional `(TYPE(args...) ...)` member of an IFC4 COMPLEX instance. */
export interface IfcComplexType {
  name: string;
  args: IfcValue[];
}

/** 📦️ One `#N = TYPE(args...);` IFC4 instance — id-keyed strong entity. */
export interface IfcEntity {
  id: bigint;
  name: string;
  args: IfcValue[];
  complex: IfcComplexType[];
}

/** 📇️ The three standard `HEADER;` records, typed via `IfcValue`. */
export interface IfcHeader {
  fileDescription: IfcValue[];
  fileName: IfcValue[];
  fileSchema: IfcValue[];
}

/** 🧬️ IfcSnapshot schema — the full, lossless IFC4 Part-21 graph in IFC's own typed model. */
export interface IfcSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ header: IfcHeader;
  /** @state artifact */ entities: IfcEntity[];
}
