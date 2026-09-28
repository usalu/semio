/** 🧭️ Revision-bound address of one authoritative DOCX XML node. */
export interface DocxXmlAddress {
  readonly partPath: string;
  readonly nodePath: readonly number[];
  readonly expectedName: string;
  readonly revision: string;
}

/** 🚪️ Refusal of a malformed canonical XML address before a mutation is constructed. */
export class DocxXmlAddressGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) { super(`${at}: ${why}`); }
}

function reject(at: string, why: string): never { throw new DocxXmlAddressGuardRefusal(at, why); }

/** 🔎️ Parses the schema-defined canonical part path, child indexes, identity, and revision. */
export function parseDocxXmlAddress(value: unknown, at = '$'): DocxXmlAddress {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) reject(at, 'value is not an object');
  const row = value as Readonly<Record<string, unknown>>;
  for (const key of Object.keys(row)) if (!['partPath', 'nodePath', 'expectedName', 'revision'].includes(key)) reject(`${at}.${key}`, 'unknown address field');
  const partPath = row.partPath, nodePath = row.nodePath, expectedName = row.expectedName, revision = row.revision;
  if (typeof partPath !== 'string' || !/^[^/\\]+(?:\/[^/\\]+)*$/u.test(partPath) || /(^|\/)\.{1,2}(\/|$)/u.test(partPath)) reject(`${at}.partPath`, 'part path is not canonical and root-relative');
  if (!Array.isArray(nodePath)) reject(`${at}.nodePath`, 'value is not an array');
  const indexes = nodePath.map((index: unknown, position: number) => {
    if (typeof index !== 'number' || !Number.isSafeInteger(index) || index < 0) reject(`${at}.nodePath[${position}]`, 'child index is not a safe unsigned integer');
    return index;
  });
  if (typeof expectedName !== 'string' || expectedName.length === 0) reject(`${at}.expectedName`, 'node identity is empty');
  if (typeof revision !== 'string' || !/^[0-9a-f]{16}$/u.test(revision)) reject(`${at}.revision`, 'revision is not sixteen lowercase hexadecimal digits');
  return { partPath, nodePath: indexes, expectedName, revision };
}
