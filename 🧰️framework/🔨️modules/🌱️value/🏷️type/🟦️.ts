/** 🏷️ Canonical schema vocabulary shared by typed value consumers. */
export type ValueType = Readonly<{ kind: "boolean" | "integer" | "decimal" | "text" | "any" } | { kind: "list"; of: ValueType } | { kind: "schema"; of: string }>;

/** 🔎️ Borrowed classification supplied by the concrete value owner. */
export type ValueKind = Readonly<{ kind: "null" | "boolean" | "integer" | "decimal" | "text" } | { kind: "dictionary"; schema?: string }>;

/** 🎛️ Caller authority for progress, admission and cancellation during a type read. */
export interface ValueTypeReadControl {
  checkpoint(completed: number, phase: "read" | "construct"): void;
}

/** 🛂️ Reads closed canonical wire objects without recursive stack growth. */
export function readValueType(value: unknown, control: ValueTypeReadControl): ValueType {
  const seen = new Set<object>();
  let current = value, lists = 0, result: ValueType;
  for (;;) {
    control.checkpoint(lists, "read");
    if (!current || typeof current !== "object" || Array.isArray(current) || seen.has(current)) throw Error("Expected an acyclic type object");
    seen.add(current);
    const record = current as Record<string, unknown>, keys = Object.keys(record), descriptor = Object.getOwnPropertyDescriptor(record, "kind");
    if (!descriptor?.enumerable || !Object.hasOwn(descriptor, "value") || Reflect.ownKeys(record).length !== keys.length) throw Error("Expected own type fields");
    const kind = descriptor.value;
    if (typeof kind !== "string" || keys.some(key => key !== "kind" && key !== "of")) throw Error("Invalid type fields");
    if (kind === "list" || kind === "schema") {
      const content = Object.getOwnPropertyDescriptor(record, "of");
      if (keys.length !== 2 || !content?.enumerable || !Object.hasOwn(content, "value")) throw Error("Missing type of");
      if (kind === "list") { lists++; current = content.value; continue; }
      if (typeof content.value !== "string") throw Error("Expected a schema string");
      result = { kind, of: content.value };
    } else {
      if (keys.length !== 1 || !["boolean", "integer", "decimal", "text", "any"].includes(kind)) throw Error("Unknown type kind");
      result = { kind: kind as "boolean" | "integer" | "decimal" | "text" | "any" };
    }
    break;
  }
  for (let completed = 0; completed < lists; completed++) { control.checkpoint(completed, "construct"); result = { kind: "list", of: result }; }
  return result;
}

/** 🪪️ Identifies a schema without consulting a specific evaluator. */
export function valueTypeId(type: ValueType): string {
  return type.kind === "schema" ? type.of : type.kind === "decimal" ? "number" : type.kind === "any" ? "value" : type.kind;
}

/** 🎯️ Classifies one owner-supplied value without constructing a carrier value. */
export function valueTypeMatches(type: ValueType, value: ValueKind): boolean {
  if (value.kind === "null") return false;
  switch (type.kind) {
    case "any": return true;
    case "decimal": return value.kind === "boolean" || value.kind === "integer" || value.kind === "decimal";
    case "list": return value.kind === "dictionary" && value.schema === "list";
    case "schema": return value.kind === "dictionary" && value.schema === type.of;
    default: return value.kind === type.kind;
  }
}
