import { policyCanonicalScalar, policyCanonicalState, policyFindSchemaDeclaration, policySnakeToCamel, type PolicySchemaFieldShape, type PolicySchemaLeafExtract } from "../🧱️contract/🟦️.ts";

/** 🧩 Parses a Rust field type into its representation-neutral shape. */
export function policyParseRustFieldType(typeText: string): Pick<PolicySchemaFieldShape, "optional" | "cardinality" | "scalar"> {
  let token = typeText.replace(/\s+/g, " ").trim();
  let optional = false;
  if (/^Option\s*</.test(token)) {
    optional = true;
    token = token.replace(/^Option\s*<\s*/, "").replace(/\s*>\s*$/, "");
  }
  const map = /^(?:BTreeMap|HashMap)\s*<\s*String\s*,\s*(.+)\s*>$/.exec(token);
  if (map) return { optional, cardinality: "map", scalar: policyCanonicalScalar(map[1]!.trim()) };
  const fixed = /^\[\s*(.+?)\s*;\s*\d+\s*\]$/.exec(token);
  if (fixed) return { optional, cardinality: "fixedList", scalar: policyCanonicalScalar(fixed[1]!.trim()) };
  if (/^Vec\s*<\s*u8\s*>$/.test(token)) return { optional, cardinality: "scalar", scalar: "bytes" };
  const vector = /^Vec\s*<\s*(.+)\s*>$/.exec(token);
  if (vector) return { optional, cardinality: "list", scalar: policyCanonicalScalar(vector[1]!.trim()) };
  return { optional, cardinality: "scalar", scalar: policyCanonicalScalar(token) };
}

/** 🦀 Extracts public fields from the declared top-level Rust structure. */
export function policyExtractRustSchemaFields(text: string, expectedTypeName: string | null = null): PolicySchemaLeafExtract {
  const declaration = policyFindSchemaDeclaration(text, /\bpub\s+struct\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/, expectedTypeName);
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
  const fieldStart = /((?:(?:#\[[^\]]*\]|\/\/\/[^\n]*(?:\n|$)|\/\*[\s\S]*?\*\/)\s*)*)pub\s+([a-z][a-z0-9_]*)\s*:\s*/g;
  let match: RegExpExecArray | null;
  while ((match = fieldStart.exec(body))) {
    const attributes = match[1] ?? "";
    const state = /#\[state\(([^)]*)\)\]/.exec(attributes)?.[1]?.trim() ?? "";
    const renamed = /#\[value\([^\]]*?\brename\s*=\s*"([^"\\]*)"/.exec(attributes)?.[1];
    let angleDepth = 0, squareDepth = 0, end = fieldStart.lastIndex;
    for (; end < body.length; end++) {
      const character = body[end]!;
      if (character === "<") angleDepth++;
      else if (character === ">") angleDepth = Math.max(0, angleDepth - 1);
      else if (character === "[") squareDepth++;
      else if (character === "]") squareDepth = Math.max(0, squareDepth - 1);
      else if ((character === "," || character === "}") && angleDepth === 0 && squareDepth === 0) break;
    }
    const parsed = policyParseRustFieldType(body.slice(fieldStart.lastIndex, end).trim());
    fieldStart.lastIndex = end;
    fields.push({ name: renamed ?? policySnakeToCamel(match[2]!), optional: parsed.optional, cardinality: parsed.cardinality, scalar: parsed.scalar, state: state ? policyCanonicalState(state) : "" });
  }
  return { typeName, fields };
}
