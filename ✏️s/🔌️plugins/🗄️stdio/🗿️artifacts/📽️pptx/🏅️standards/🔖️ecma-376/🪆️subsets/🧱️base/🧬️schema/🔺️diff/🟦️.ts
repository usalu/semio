import type { OpcPackage, PptxXmlPart } from '../📸️snapshot/🟦️.ts';
import { parseOpcPackage, parsePptxXmlPart } from '../📸️snapshot/🟦️.ts';

/** 🔺️ Sparse replacements of the three canonical PPTX fields. */
export interface PptxDiff {
  readonly schema?: string;
  readonly opc?: OpcPackage;
  readonly xmlParts?: readonly PptxXmlPart[];
}

/** 🚪️ A precise position and reason for refusing a malformed PPTX diff. */
export class stdioPptxEcma376BaseDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) { super(`${at}: ${why}`); }
}
const reject = (at: string, why: string): never => { throw new stdioPptxEcma376BaseDiffGuardRefusal(at, why); };
const object = (value: unknown, at: string): Readonly<Record<string, unknown>> => value !== null && typeof value === 'object' && !Array.isArray(value) ? value as Record<string, unknown> : reject(at, 'value is not an object');
const text = (value: unknown, at: string): string => typeof value === 'string' ? value : reject(at, 'value is not a string');

/** 🚪️ Parses a canonical PPTX field-replacement diff. */
export function parsePptxDiff(value: unknown, at = '$'): PptxDiff {
  const row = object(value, at);
  for (const key of Object.keys(row)) if (!['schema', 'opc', 'xmlParts'].includes(key)) reject(`${at}.${key}`, 'unknown PPTX diff field');
  const xmlParts: readonly unknown[] | undefined = row.xmlParts === undefined
    ? undefined
    : Array.isArray(row.xmlParts)
      ? row.xmlParts
      : reject(`${at}.xmlParts`, 'value is not an array');
  return {
    schema: row.schema === undefined ? undefined : text(row.schema, `${at}.schema`),
    opc: row.opc === undefined ? undefined : parseOpcPackage(row.opc, `${at}.opc`),
    xmlParts: xmlParts === undefined ? undefined : xmlParts.map((item, index) => parsePptxXmlPart(item, `${at}.xmlParts[${index}]`)),
  };
}
