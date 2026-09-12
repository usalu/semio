import { policyCanonicalScalar, policyCanonicalState, policyFindSchemaDeclaration, policySnakeToCamel, type PolicySchemaFieldShape, type PolicySchemaLeafExtract } from "../🧱️contract/🟦️.ts";

/** 🧩 Parses a Protobuf field type into its representation-neutral shape. */
export function policyParseProtoFieldType(typeText: string, optionalKeyword: boolean, repeatedKeyword: boolean): Pick<PolicySchemaFieldShape, "optional" | "cardinality" | "scalar"> {
  const map = /^map\s*<\s*string\s*,\s*(.+)\s*>$/.exec(typeText.trim());
  if (map) return { optional: optionalKeyword, cardinality: "map", scalar: policyCanonicalScalar(map[1]!.trim()) };
  if (repeatedKeyword) return { optional: optionalKeyword, cardinality: "list", scalar: policyCanonicalScalar(typeText.trim()) };
  return { optional: optionalKeyword, cardinality: "scalar", scalar: policyCanonicalScalar(typeText.trim()) };
}

/** 🛰 Extracts fields and state comments from the declared Protobuf message. */
export function policyExtractProtobufSchemaFields(text: string, expectedTypeName: string | null = null): PolicySchemaLeafExtract {
  const declaration = policyFindSchemaDeclaration(text, /\bmessage\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/, expectedTypeName);
  if (!declaration) return { typeName: "", fields: [] };
  const { typeName, bodyStart } = declaration;
  let depth = 1, index = bodyStart;
  for (; index < text.length; index++) {
    const character = text[index];
    if (character === "{") depth++;
    else if (character === "}" && --depth === 0) break;
  }
  const body = text.slice(bodyStart, index);
  const fields: PolicySchemaFieldShape[] = [];
  const field = /(?:\/\/\s*@state\s+([a-z0-9_-]+)\s*\n\s*)?(optional\s+)?(repeated\s+)?(map\s*<\s*string\s*,\s*[^>]+>|[\w.]+)\s+([a-z][a-z0-9_]*)\s*=\s*\d+\s*;/g;
  let match: RegExpExecArray | null;
  while ((match = field.exec(body))) {
    const parsed = policyParseProtoFieldType(match[4]!, Boolean(match[2]), Boolean(match[3]));
    fields.push({ name: policySnakeToCamel(match[5]!), optional: parsed.optional, cardinality: parsed.cardinality, scalar: parsed.scalar, state: match[1] ? policyCanonicalState(match[1]) : "" });
  }
  return { typeName, fields };
}
