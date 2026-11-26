import { expect, test } from "bun:test";
import law from "../🧫️fixtures/variadic-signature.json";

test("variadic implementations select homogeneous or exact ordered tuples", () => {
  for (const row of law.cases) {
    const items: Readonly<Record<string, string | null | undefined>> = row.items;
    const signature = Object.keys(items)
      .filter(key => /^\+?\d+$/.test(key))
      .map(BigInt)
      .filter(key => key <= (1n << 64n) - 1n)
      .sort((a, b) => a < b ? -1 : a > b ? 1 : 0)
      .map(index => items[index.toString()])
      .filter((schema): schema is string => typeof schema === "string");
    const arity = signature.length;
    const withinBounds = arity >= row.min && (row.max === null || arity <= row.max);
    const selected = withinBounds && law.implementations.find(({ schemas }) =>
      schemas.length === 1
        ? signature.every(schema => schema === schemas[0])
        : arity === schemas.length && signature.every((schema, index) => schema === schemas[index])
    );
    expect((selected && selected.marker) || (row.fallback ? 4 : null), row.case).toBe(row.expected);
  }
});
