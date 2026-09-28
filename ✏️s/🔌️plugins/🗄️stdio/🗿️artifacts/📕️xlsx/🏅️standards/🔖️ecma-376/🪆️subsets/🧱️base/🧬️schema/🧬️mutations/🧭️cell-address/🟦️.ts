/** 🧭️ Revision-bound address of one authoritative SpreadsheetML cell element. */
export interface XlsxCellAddress {
  readonly partPath: string;
  readonly nodePath: readonly number[];
  readonly namespaceUri: 'http://schemas.openxmlformats.org/spreadsheetml/2006/main' | 'http://purl.oclc.org/ooxml/spreadsheetml/main';
  readonly localName: 'c';
  readonly revision: string;
}

/** 🚪️ Refusal of a malformed canonical cell address before a mutation is constructed. */
export class XlsxCellAddressGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) { super(`${at}: ${why}`); }
}

function reject(at: string, why: string): never { throw new XlsxCellAddressGuardRefusal(at, why); }

/** 🔎️ Parses the schema-defined canonical part path, child indexes, expanded name, and revision. */
export function parseXlsxCellAddress(value: unknown, at = '$'): XlsxCellAddress {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) reject(at, 'value is not an object');
  const row = value as Readonly<Record<string, unknown>>;
  for (const key of Object.keys(row)) if (!['partPath', 'nodePath', 'namespaceUri', 'localName', 'revision'].includes(key)) reject(`${at}.${key}`, 'unknown address field');
  const partPath = row.partPath, nodePath = row.nodePath, namespaceUri = row.namespaceUri, localName = row.localName, revision = row.revision;
  if (typeof partPath !== 'string' || !/^[^/\\]+(?:\/[^/\\]+)*$/u.test(partPath) || /(^|\/)\.{1,2}(\/|$)/u.test(partPath)) reject(`${at}.partPath`, 'part path is not canonical and root-relative');
  if (!Array.isArray(nodePath)) reject(`${at}.nodePath`, 'value is not an array');
  const indexes = nodePath.map((index: unknown, position: number) => {
    if (typeof index !== 'number' || !Number.isSafeInteger(index) || index < 0) reject(`${at}.nodePath[${position}]`, 'child index is not a safe unsigned integer');
    return index;
  });
  if (namespaceUri !== 'http://schemas.openxmlformats.org/spreadsheetml/2006/main' && namespaceUri !== 'http://purl.oclc.org/ooxml/spreadsheetml/main') reject(`${at}.namespaceUri`, 'namespace is not a supported SpreadsheetML namespace');
  if (localName !== 'c') reject(`${at}.localName`, 'address does not identify a cell element');
  if (typeof revision !== 'string' || !/^[0-9a-f]{16}$/u.test(revision)) reject(`${at}.revision`, 'revision is not sixteen lowercase hexadecimal digits');
  return { partPath, nodePath: indexes, namespaceUri, localName, revision };
}
