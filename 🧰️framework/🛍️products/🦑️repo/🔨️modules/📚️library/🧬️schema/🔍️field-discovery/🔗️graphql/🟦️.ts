import { policyCanonicalScalar, policyCanonicalState, policyFindSchemaDeclaration, type PolicySchemaFieldCardinality, type PolicySchemaFieldShape, type PolicySchemaLeafExtract } from "../🧱️contract/🟦️.ts";

/** 🔗 Extracts fields and state directives from the declared GraphQL type. */
export function policyExtractGraphqlSchemaFields(text: string, expectedTypeName: string | null = null): PolicySchemaLeafExtract {
  text = text.replace(/"""[\s\S]*?"""|"(?:\\.|[^"\\])*"|#[^\n]*/g, (value) => value.replace(/[^\n]/g, " "));
  const declaration = policyFindSchemaDeclaration(text, /\btype\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/, expectedTypeName);
  if (!declaration) return { typeName: "", fields: [] };
  const { typeName, bodyStart } = declaration;
  let depth = 1, index = bodyStart;
  for (; index < text.length; index++) {
    const character = text[index];
    if (character === "{") depth++;
    else if (character === "}" && --depth === 0) break;
  }
  const body = text.slice(bodyStart, index);
  let argumentDepth = 0;
  const declarations = body.replace(/[\s\S]/g, (character) => {
    if (character === "(") argumentDepth++;
    const visible = argumentDepth === 0;
    if (character === ")") argumentDepth--;
    return visible ? character : " ";
  });
  const matches = [...declarations.matchAll(/([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(\[[^\]]+\]|[A-Za-z_][A-Za-z0-9_]*)(!)?/g)];
  const fields: PolicySchemaFieldShape[] = matches.map((match, matchIndex) => {
    const typeToken = match[2]!;
    const directives = body.slice(match.index! + match[0].length, matches[matchIndex + 1]?.index ?? body.length);
    const state = /@state\s*\(\s*class\s*:\s*([A-Z_]+)\s*\)/.exec(directives)?.[1] ?? "";
    let cardinality: PolicySchemaFieldCardinality = "scalar", scalar = typeToken;
    const list = /^\[\s*(.+?)\s*!?\s*\]$/.exec(typeToken);
    if (list) {
      const inner = list[1]!.replace(/!$/, "").trim();
      if (/Entry$/.test(inner)) {
        cardinality = "map";
        scalar = inner.replace(/Entry$/, "");
      } else {
        cardinality = "list";
        scalar = inner;
      }
    }
    return { name: match[1]!, optional: !match[3], cardinality, scalar: policyCanonicalScalar(scalar), state: state ? policyCanonicalState(state) : "" };
  });
  return { typeName, fields };
}
