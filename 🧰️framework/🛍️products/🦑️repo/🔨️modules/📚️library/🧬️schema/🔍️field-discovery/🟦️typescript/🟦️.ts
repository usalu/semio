import { policyCanonicalScalar, policyCanonicalState, policyFindSchemaDeclaration, type PolicySchemaFieldShape, type PolicySchemaLeafExtract } from "../🧱️contract/🟦️.ts";

/** 🧩 Parses a TypeScript property type into its representation-neutral shape. */
export function policyParseTsFieldType(typeText: string, optionalMark: boolean): Pick<PolicySchemaFieldShape, "optional" | "cardinality" | "scalar"> {
  let token = typeText.replace(/\s+/g, " ").trim().replace(/;$/, "");
  const optional = optionalMark || token.endsWith("| undefined") || token.endsWith("| null");
  token = token.replace(/\s*\|\s*undefined$/, "").replace(/\s*\|\s*null$/, "").trim();
  const record = /^Record\s*<\s*string\s*,\s*(.+)\s*>$/.exec(token);
  if (record) return { optional, cardinality: "map", scalar: policyCanonicalScalar(record[1]!.trim()) };
  const tuple = /^\[\s*(.+?)\s*(?:,\s*\1\s*)+\]$/.exec(token);
  if (tuple && token.includes(",")) return { optional, cardinality: "fixedList", scalar: policyCanonicalScalar(tuple[1]!.trim()) };
  const array = /^(?:Array\s*<\s*(.+)\s*>|(.+)\[\])$/.exec(token);
  if (array) return { optional, cardinality: "list", scalar: policyCanonicalScalar((array[1] ?? array[2]!).trim()) };
  return { optional, cardinality: "scalar", scalar: policyCanonicalScalar(token) };
}

/** 🟦 Extracts a declared TypeScript schema through local and resolved aliases. */
export function policyExtractTypescriptSchemaFields(
  text: string,
  expectedTypeName: string | null = null,
  resolveModule?: (specifier: string, importingModule: string) => { moduleId: string; text: string } | null,
  moduleId = "",
): PolicySchemaLeafExtract {
  const seen = new Set<string>();
  const bodyOf = (source: string, start: number): string => {
    let depth = 1, end = start;
    for (; end < source.length; end++) {
      if (source[end] === "{") depth++;
      else if (source[end] === "}" && --depth === 0) break;
    }
    return source.slice(start, end);
  };
  const find = (source: string, sourceId: string, name: string | null, exported: boolean): { text: string; typeName: string; bodyStart: number } | null => {
    const key = JSON.stringify([sourceId, name, exported]);
    if (seen.has(key)) return null;
    seen.add(key);
    try {
      const declaration = policyFindSchemaDeclaration(source, exported ? /\bexport\s+interface\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/ : /\binterface\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/, name);
      if (declaration) return { text: source, ...declaration };
      if (!name) return null;
      for (const match of source.matchAll(/\b(export\s+)?interface\s+([A-Za-z_][A-Za-z0-9_]*)\s+extends\s+([A-Za-z_][A-Za-z0-9_]*(?:\s*,\s*[A-Za-z_][A-Za-z0-9_]*)*)\s*\{/g)) {
        if (match[2] !== name || (exported && !match[1])) continue;
        const own = bodyOf(source, match.index! + match[0].length);
        const inherited: string[] = [];
        for (const base of match[3]!.split(",")) {
          const inheritedDeclaration = find(source, sourceId, base.trim(), false);
          if (!inheritedDeclaration) return null;
          inherited.push(bodyOf(inheritedDeclaration.text, inheritedDeclaration.bodyStart));
        }
        return { text: [own, ...inherited].join("\n"), typeName: name, bodyStart: 0 };
      }
      const resolveDeclaration = (specifier: string, target: string) => {
        const next = resolveModule?.(specifier, sourceId);
        return next ? find(next.text, next.moduleId, target, true) : null;
      };
      for (const match of source.matchAll(/\bexport\s+(?:type\s+)?\{([^}]+)\}\s*(?:from\s*["']([^"']+)["'])?/g)) {
        for (const item of match[1]!.split(",")) {
          const binding = /^(?:type\s+)?([A-Za-z_][A-Za-z0-9_]*)(?:\s+as\s+([A-Za-z_][A-Za-z0-9_]*))?$/.exec(item.trim());
          if (binding && (binding[2] ?? binding[1]) === name) return match[2] ? resolveDeclaration(match[2], binding[1]!) : find(source, sourceId, binding[1]!, false);
        }
      }
      for (const match of source.matchAll(/\b(export\s+)?type\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*([A-Za-z_][A-Za-z0-9_]*)\s*;/g)) {
        if (match[2] === name && (!exported || match[1])) return find(source, sourceId, match[3]!, false);
      }
      if (!exported) {
        for (const match of source.matchAll(/\bimport\s+(?:type\s+)?\{([^}]+)\}\s*from\s*["']([^"']+)["']/g)) {
          for (const item of match[1]!.split(",")) {
            const binding = /^(?:type\s+)?([A-Za-z_][A-Za-z0-9_]*)(?:\s+as\s+([A-Za-z_][A-Za-z0-9_]*))?$/.exec(item.trim());
            if (binding && (binding[2] ?? binding[1]) === name) return resolveDeclaration(match[2]!, binding[1]!);
          }
        }
      }
      return null;
    } finally {
      seen.delete(key);
    }
  };
  const declaration = find(text, moduleId, expectedTypeName, true);
  if (!declaration) return { typeName: "", fields: [] };
  text = declaration.text;
  const typeName = expectedTypeName ?? declaration.typeName;
  const body = bodyOf(text, declaration.bodyStart);
  const fields: PolicySchemaFieldShape[] = [];
  const fieldPattern = /(?:\/\*\*([\s\S]*?)\*\/\s*)?([A-Za-z_][A-Za-z0-9_]*|"[^"\n]+"|'[^'\n]+')(\?)?\s*:\s*([^;]+);/g;
  let match: RegExpExecArray | null;
  while ((match = fieldPattern.exec(body))) {
    const parsed = policyParseTsFieldType(match[4]!.trim(), Boolean(match[3]));
    const name = match[2]!.replace(/^["']|["']$/g, "");
    if (fields.some((field) => field.name === name)) continue;
    const state = /@state\s+([a-z0-9_-]+)/.exec(match[1] ?? "")?.[1];
    fields.push({ name, optional: parsed.optional, cardinality: parsed.cardinality, scalar: parsed.scalar, state: state ? policyCanonicalState(state) : "" });
  }
  return { typeName, fields };
}
