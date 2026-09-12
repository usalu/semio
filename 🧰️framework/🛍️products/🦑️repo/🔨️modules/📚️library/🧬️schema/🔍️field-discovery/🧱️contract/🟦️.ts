export type PolicySchemaFieldCardinality = "scalar" | "list" | "fixedList" | "map";

export type PolicySchemaFieldShape = {
  name: string;
  optional: boolean;
  cardinality: PolicySchemaFieldCardinality;
  scalar: string;
  state: string;
};

export type PolicySchemaLeafExtract = {
  typeName: string;
  fields: PolicySchemaFieldShape[];
};

/** 🔤 Normalizes a state-class token to its canonical identifier. */
export function policyCanonicalState(raw: string): string {
  return raw.trim().toLowerCase().replace(/_/g, "-");
}

/** 🔤 Maps exact representation type tokens to canonical scalar identifiers. */
export function policyCanonicalScalar(raw: string): string {
  const token = raw.replace(/\s+/g, "").trim();
  const scalars: Record<string, string> = {
    String: "string",
    string: "string",
    bool: "bool",
    boolean: "bool",
    Boolean: "bool",
    i32: "int32",
    u32: "uint32",
    i64: "int64",
    f32: "float32",
    f64: "float64",
    Int: "int32",
    Float: "float64",
    bytes: "bytes",
    "Vec<u8>": "bytes",
  };
  return scalars[token] ?? token;
}

/** 🔎 Locates a schema declaration only through its declared name. */
export function policyFindSchemaDeclaration(text: string, declarationPattern: RegExp, expected: string | null): { typeName: string; bodyStart: number } | null {
  if (!expected) return null;
  const pattern = new RegExp(declarationPattern.source, declarationPattern.flags.includes("g") ? declarationPattern.flags : `${declarationPattern.flags}g`);
  let match: RegExpExecArray | null;
  while ((match = pattern.exec(text))) if (match[1] === expected) return { typeName: match[1]!, bodyStart: match.index + match[0].length };
  return null;
}

/** 🔤 Converts underscore-delimited field names to canonical camel case. */
export function policySnakeToCamel(name: string): string {
  const parts = name.split("_");
  return parts[0] + parts.slice(1).map((part) => part.charAt(0).toUpperCase() + part.slice(1)).join("");
}
