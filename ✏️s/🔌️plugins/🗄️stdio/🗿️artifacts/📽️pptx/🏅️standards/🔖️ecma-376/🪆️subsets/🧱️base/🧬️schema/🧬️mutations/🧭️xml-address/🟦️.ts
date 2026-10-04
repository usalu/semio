/** 🧭️ Revision-bound address of one authoritative PresentationML XML element. */
export interface PptxXmlAddress {
  readonly partPath: string;
  readonly nodePath: readonly number[];
  readonly namespaceUri: string;
  readonly localName: string;
  readonly revision: string;
}

/** 🎞️ Identity of one slide-list entry and its relationship-resolved slide part. */
export interface PptxSlideAddress {
  readonly entry: PptxXmlAddress;
  readonly slidePartPath: string;
  readonly relationshipId: string;
  readonly slideId: string;
}

/** 🖼️ Identity of one direct shape-tree child, including its stable nonvisual shape id. */
export interface PptxShapeAddress {
  readonly node: PptxXmlAddress;
  readonly shapeId: string;
}

/** 🧭️ Revision-bound insertion point inside a slide list or shape tree. */
export interface PptxXmlVacancyAddress {
  readonly container: PptxXmlAddress;
  readonly index: number;
}

const object = (value: unknown, at: string): Readonly<Record<string, unknown>> => {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) throw new Error(`${at}: value is not an object`);
  return value as Readonly<Record<string, unknown>>;
};
const exact = (row: Readonly<Record<string, unknown>>, keys: readonly string[], at: string): void => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) throw new Error(`${at}.${key}: unknown address field`);
};
const text = (value: unknown, at: string): string => {
  if (typeof value !== 'string' || value.length === 0) throw new Error(`${at}: value is not nonempty text`);
  return value;
};
const path = (value: unknown, at: string): string => {
  const result = text(value, at);
  if (!/^[^/\\]+(?:\/[^/\\]+)*$/u.test(result) || /(^|\/)\.{1,2}(\/|$)/u.test(result)) throw new Error(`${at}: path is not canonical`);
  return result;
};

/** 🔎️ Parses an exact revision-bound XML address. */
export function parsePptxXmlAddress(value: unknown, at = '$'): PptxXmlAddress {
  const row = object(value, at);
  exact(row, ['partPath', 'nodePath', 'namespaceUri', 'localName', 'revision'], at);
  if (!Array.isArray(row.nodePath) || row.nodePath.some((index) => !Number.isSafeInteger(index) || (index as number) < 0)) throw new Error(`${at}.nodePath: child indexes are invalid`);
  const revision = text(row.revision, `${at}.revision`);
  if (!/^[0-9a-f]{16}$/u.test(revision)) throw new Error(`${at}.revision: revision is not canonical`);
  return { partPath: path(row.partPath, `${at}.partPath`), nodePath: row.nodePath as number[], namespaceUri: text(row.namespaceUri, `${at}.namespaceUri`), localName: text(row.localName, `${at}.localName`), revision };
}

/** 🔎️ Parses an addressed slide-list entry. */
export function parsePptxSlideAddress(value: unknown, at = '$'): PptxSlideAddress {
  const row = object(value, at);
  exact(row, ['entry', 'slidePartPath', 'relationshipId', 'slideId'], at);
  const slideId = text(row.slideId, `${at}.slideId`);
  if (!/^[0-9]+$/u.test(slideId)) throw new Error(`${at}.slideId: slide id is not decimal`);
  return { entry: parsePptxXmlAddress(row.entry, `${at}.entry`), slidePartPath: path(row.slidePartPath, `${at}.slidePartPath`), relationshipId: text(row.relationshipId, `${at}.relationshipId`), slideId };
}

/** 🔎️ Parses an addressed shape-tree child. */
export function parsePptxShapeAddress(value: unknown, at = '$'): PptxShapeAddress {
  const row = object(value, at);
  exact(row, ['node', 'shapeId'], at);
  const shapeId = text(row.shapeId, `${at}.shapeId`);
  if (!/^[0-9]+$/u.test(shapeId)) throw new Error(`${at}.shapeId: shape id is not decimal`);
  return { node: parsePptxXmlAddress(row.node, `${at}.node`), shapeId };
}

/** 🔎️ Parses a revision-bound XML insertion point. */
export function parsePptxXmlVacancyAddress(value: unknown, at = '$'): PptxXmlVacancyAddress {
  const row = object(value, at);
  exact(row, ['container', 'index'], at);
  if (!Number.isSafeInteger(row.index) || (row.index as number) < 0) throw new Error(`${at}.index: insertion index is invalid`);
  return { container: parsePptxXmlAddress(row.container, `${at}.container`), index: row.index as number };
}
