import { policyCanonicalState, type PolicySchemaFieldShape, type PolicySchemaLeafExtract } from "../🧱️contract/🟦️.ts";

/** 🔤 Resolves a JSON Schema node to its canonical scalar identifier. */
export function policyJsonSchemaScalar(schema: Record<string, unknown>): string {
  if (schema.contentEncoding === "base64") return "bytes";
  if (schema.contentMediaType === "application/json") return "string";
  const type = schema.type, format = schema.format;
  if (type === "string") return "string";
  if (type === "boolean") return "bool";
  if (type === "integer") {
    if (format === "int32") return "int32";
    if (format === "uint32") return "uint32";
    if (format === "int64") return "int64";
    return "int32";
  }
  if (type === "number") {
    if (format === "float") return "float32";
    if (format === "double") return "float64";
    return "float64";
  }
  if (typeof schema.$ref === "string") return schema.$ref.split("/").pop() ?? schema.$ref;
  if (typeof schema.title === "string") return schema.title;
  return typeof type === "string" ? type : "unknown";
}

/** 🧩 Parses a JSON Schema property into its representation-neutral shape. */
export function policyParseJsonSchemaProperty(property: Record<string, unknown>): Pick<PolicySchemaFieldShape, "cardinality" | "scalar"> {
  if (property.type === "array") {
    const items = property.items as Record<string, unknown> | undefined;
    const scalar = items ? policyJsonSchemaScalar(items) : "unknown";
    return typeof property.minItems === "number" && property.minItems === property.maxItems ? { cardinality: "fixedList", scalar } : { cardinality: "list", scalar };
  }
  if (property.type === "object" && property.additionalProperties != null && property.additionalProperties !== false) {
    const additional = property.additionalProperties;
    return { cardinality: "map", scalar: typeof additional === "object" && additional ? policyJsonSchemaScalar(additional as Record<string, unknown>) : "unknown" };
  }
  return { cardinality: "scalar", scalar: policyJsonSchemaScalar(property) };
}

/** 🔣 Extracts declared properties and state metadata from JSON Schema. */
export function policyExtractJsonSchemaFields(text: string): PolicySchemaLeafExtract {
  let document: Record<string, unknown>;
  try {
    document = JSON.parse(text) as Record<string, unknown>;
  } catch {
    return { typeName: "", fields: [] };
  }
  const properties = (document.properties ?? {}) as Record<string, Record<string, unknown>>;
  const required = new Set<string>(Array.isArray(document.required) ? document.required as string[] : []);
  const fields = Object.entries(properties).map(([name, property]): PolicySchemaFieldShape => {
    const parsed = policyParseJsonSchemaProperty(property ?? {});
    const state = typeof property?.["x-semio-state"] === "string" ? property["x-semio-state"] as string : "";
    return { name, optional: !required.has(name), cardinality: parsed.cardinality, scalar: parsed.scalar, state: state ? policyCanonicalState(state) : "" };
  });
  return { typeName: typeof document.title === "string" ? document.title : "", fields };
}
