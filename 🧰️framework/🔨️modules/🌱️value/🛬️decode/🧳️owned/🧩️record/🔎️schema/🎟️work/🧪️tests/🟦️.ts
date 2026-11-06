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
