/** 🗝️ Finds duplicate decoded members in a syntactically valid JSON document. */
export function jsonDocumentDuplicateKeys(source: string): string[] {
  const stack: (Set<string> | null)[] = [], problems: string[] = [];
  for (const token of source.matchAll(/"(?:\\.|[^"\\])*"\s*:?|[{}\[\]]/gu)) {
    const value = token[0];
    if (value === "{") stack.push(new Set());
    else if (value === "[") stack.push(null);
    else if (value === "}" || value === "]") stack.pop();
    else if (value.endsWith(":")) {
      const key = JSON.parse(value.slice(0, -1).trim()) as string, keys = stack.at(-1);
      if (keys?.has(key)) problems.push(`Duplicate JSON member ${JSON.stringify(key)} at offset ${token.index}`);
      keys?.add(key);
    }
  }
  return problems;
}
