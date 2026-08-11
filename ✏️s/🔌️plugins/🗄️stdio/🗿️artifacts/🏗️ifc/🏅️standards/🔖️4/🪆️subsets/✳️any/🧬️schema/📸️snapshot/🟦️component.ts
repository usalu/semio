/** 🔤️ One typed value in IFC4's Part-21 argument-list syntax (own type, mirrors STEP's shape). */
export type IfcValue =
  | { kind: "unset" }
  | { kind: "derived" }
  | { kind: "integer"; value: number }
  | { kind: "real"; value: number }
  | { kind: "string"; value: string }
  | { kind: "enum"; value: string }
  | { kind: "reference"; value: number }
  | { kind: "aggregate"; value: IfcValue[] }
  | { kind: "typedValue"; value: [string, IfcValue[]] };

/** 🧩️ One additional `(TYPE(args...) ...)` member of an IFC4 COMPLEX instance. */
export interface IfcComplexType {
  name: string;
  args: IfcValue[];
}

/** 📦️ One `#N = TYPE(args...);` IFC4 instance — id-keyed strong entity. */
export interface IfcEntity {
  id: number;
  name: string;
  args: IfcValue[];
  complex?: IfcComplexType[];
}

/** 📇️ The three standard `HEADER;` records, typed via `IfcValue`. */
export interface IfcHeader {
  fileDescription: IfcValue[];
  fileName: IfcValue[];
  fileSchema: IfcValue[];
}

/** 🧬️ IfcSnapshot schema — the full, lossless IFC4 Part-21 graph in IFC's own typed model. */
export interface IfcSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ header: IfcHeader;
  /** @state persistent */ entities: IfcEntity[];
}
