/** 🧭️ Revision-bound address of one authoritative SpreadsheetML sheetData element. */
export interface XlsxWorksheetAddress {
  readonly partPath: string;
  readonly nodePath: readonly number[];
  readonly namespaceUri: 'http://schemas.openxmlformats.org/spreadsheetml/2006/main' | 'http://purl.oclc.org/ooxml/spreadsheetml/main';
  readonly localName: 'sheetData';
  readonly revision: string;
}

/** 🕳️ One unoccupied coordinate inside an addressed worksheet. */
export interface XlsxCellVacancyAddress {
  readonly worksheet: XlsxWorksheetAddress;
  readonly row: number;
  readonly column: number;
  readonly placement?: { readonly index: number; readonly unanchorNext: boolean };
}

/** 🔎️ Refuses coordinates outside the ECMA-376 worksheet bounds. */
export function parseXlsxCellVacancyAddress(value: unknown, at = '$'): XlsxCellVacancyAddress {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) throw new Error(`${at}: value is not an object`);
  const row = value as Readonly<Record<string, unknown>>;
  for (const key of Object.keys(row)) if (!['worksheet', 'row', 'column', 'placement'].includes(key)) throw new Error(`${at}.${key}: unknown vacancy field`);
  const worksheet = row.worksheet as Readonly<Record<string, unknown>>;
  if (worksheet === null || typeof worksheet !== 'object' || Array.isArray(worksheet)) throw new Error(`${at}.worksheet: value is not an object`);
  const partPath = worksheet.partPath, nodePath = worksheet.nodePath, namespaceUri = worksheet.namespaceUri, localName = worksheet.localName, revision = worksheet.revision;
  if (typeof partPath !== 'string' || !/^[^/\\]+(?:\/[^/\\]+)*$/u.test(partPath) || /(^|\/)\.{1,2}(\/|$)/u.test(partPath)) throw new Error(`${at}.worksheet.partPath: part path is not canonical`);
  if (!Array.isArray(nodePath) || nodePath.some((index) => typeof index !== 'number' || !Number.isSafeInteger(index) || index < 0)) throw new Error(`${at}.worksheet.nodePath: child indexes are invalid`);
  if (namespaceUri !== 'http://schemas.openxmlformats.org/spreadsheetml/2006/main' && namespaceUri !== 'http://purl.oclc.org/ooxml/spreadsheetml/main') throw new Error(`${at}.worksheet.namespaceUri: namespace is unsupported`);
  if (localName !== 'sheetData' || typeof revision !== 'string' || !/^[0-9a-f]{16}$/u.test(revision)) throw new Error(`${at}.worksheet: identity is invalid`);
  if (typeof row.row !== 'number' || !Number.isInteger(row.row) || row.row < 1 || row.row > 1048576) throw new Error(`${at}.row: row is outside SpreadsheetML bounds`);
  if (typeof row.column !== 'number' || !Number.isInteger(row.column) || row.column < 0 || row.column > 16383) throw new Error(`${at}.column: column is outside SpreadsheetML bounds`);
  const address = { worksheet: { partPath, nodePath: nodePath as number[], namespaceUri, localName, revision }, row: row.row, column: row.column } satisfies XlsxCellVacancyAddress;
  if (!Object.prototype.hasOwnProperty.call(row, 'placement')) return address;
  const placement = row.placement as Readonly<Record<string, unknown>>;
  if (placement === null || typeof placement !== 'object' || Array.isArray(placement)) throw new Error(`${at}.placement: value is not an object`);
  for (const key of Object.keys(placement)) if (!['index', 'unanchorNext'].includes(key)) throw new Error(`${at}.placement.${key}: unknown placement field`);
  const { index, unanchorNext } = placement;
  if (typeof index !== 'number' || !Number.isSafeInteger(index) || index < 0 || typeof unanchorNext !== 'boolean') throw new Error(`${at}.placement: index or successor flag is invalid`);
  return { ...address, placement: { index, unanchorNext } };
}
