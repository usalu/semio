/** ⚖️ Compares canonical field sets without treating declaration order as identity. */
export function policySchemaFieldDifferences(reference: readonly string[], candidate: readonly string[]): { missing: string[]; extra: string[] } {
  const expected = new Set(reference), actual = new Set(candidate);
  return { missing: [...expected].filter((name) => !actual.has(name)).sort(), extra: [...actual].filter((name) => !expected.has(name)).sort() };
}
