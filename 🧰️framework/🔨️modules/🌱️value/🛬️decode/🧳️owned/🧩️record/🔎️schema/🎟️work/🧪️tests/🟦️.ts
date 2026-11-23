import { readFileSync } from 'node:fs';
import Ajv from 'ajv';
import { isEqual } from 'lodash-es';
import { describe, expect, it } from 'vitest';

describe('native matcher byte oracle', () => {
  it('matches UTF-8 bytes independently', () => {
    const fixture = JSON.parse(readFileSync(new URL('../🧫️byte-work.json', import.meta.url), 'utf8'));
    const schema = JSON.parse(readFileSync(new URL('../🧬️byte-work.schema.json', import.meta.url), 'utf8'));
    expect(new Ajv().validate(schema, fixture)).toBe(true);
    for (const row of fixture.cases) {
      expect(isEqual(Array.from(Buffer.from(row.key)), Array.from(Buffer.from(row.name)))).toBe(row.equal);
      expect(row.maximumBytesPerTurn).toBe(1);
      expect(row.deniedWork).toBe(0);
    }
  });
});

describe('native flatten resolution oracle', () => {
  const long = `${'a'.repeat(129)}z`;
  const flattened = ['dup', 'beta', long];
  const byName = (key: string) => {
    if (key === 'alpha') return { kind: 'field', field: 0 };
    if (key === 'dup') return { kind: 'ambiguous', field: null };
    if (key === 'union') return { kind: 'field', field: 3 };
    return flattened.includes(key) ? { kind: 'field', field: 1 } : { kind: 'missing', field: null };
  };
  const byOrdinal = (ordinal: number) => {
    if (ordinal === 7) return { kind: 'field', field: 3 };
    return [0, 1, 2].includes(ordinal) ? { kind: 'field', field: ordinal } : { kind: 'missing', field: null };
  };
  it('addresses a flattened field by name through its parts and by ordinal as itself', () => {
    const fixture = JSON.parse(readFileSync(new URL('../📐️.json', import.meta.url), 'utf8'));
    const schema = JSON.parse(readFileSync(new URL('../🔣️.json', import.meta.url), 'utf8'));
    expect(new Ajv().validate(schema, fixture)).toBe(true);
    for (const row of fixture.cases) {
      if (row.kind === 'refused' || row.kind === 'cut') continue;
      const expected = row.key === undefined ? byOrdinal(row.ordinal) : byName(row.key);
      expect({ kind: row.kind, field: row.field }).toEqual(expected);
    }
  });
});
