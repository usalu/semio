/** 🧬️ Canonical PresentationML mutation union. */
import { parsePptxTransform, parseXmlNode } from '../📸️snapshot/🟦️.ts';
import type { PptxTransform, XmlNode } from '../📸️snapshot/🟦️.ts';
import {
  parsePptxShapeAddress,
  parsePptxXmlAddress,
  parsePptxSlideAddress,
  parsePptxXmlVacancyAddress,
} from './🧭️xml-address/🟦️.ts';
import type {
  PptxShapeAddress,
  PptxSlideAddress,
  PptxXmlAddress,
  PptxXmlVacancyAddress,
} from './🧭️xml-address/🟦️.ts';
export type { PptxShapeAddress, PptxSlideAddress, PptxXmlAddress, PptxXmlVacancyAddress } from './🧭️xml-address/🟦️.ts';

export type PptxMutation =
  | { mutation: 'insertSlide'; vacancy: PptxXmlVacancyAddress; entry: XmlNode }
  | { mutation: 'removeSlide'; address: PptxSlideAddress }
  | { mutation: 'moveSlide'; address: PptxSlideAddress; destinationIndex: number }
  | { mutation: 'insertShape'; vacancy: PptxXmlVacancyAddress; shape: XmlNode }
  | { mutation: 'removeShape'; address: PptxShapeAddress }
  | { mutation: 'setShapeText'; address: PptxShapeAddress; text: string }
  | { mutation: 'setShapePosition'; address: PptxShapeAddress; position: PptxTransform }
  | { mutation: 'replaceXmlNode'; address: PptxXmlAddress; node: XmlNode }
  | { mutation: 'setRelationship'; owner: string; id: string; relType: string; target: string; external?: boolean; index?: number }
  | { mutation: 'removeRelationship'; owner: string; id: string }
  | { mutation: 'setContentType'; isOverride?: boolean; name: string; contentType: string; index?: number }
  | { mutation: 'removeContentType'; isOverride?: boolean; name: string };

/** 🚪️ A precise position and reason for refusing a malformed PPTX mutation. */
export class stdioPptxEcma376BaseMutationGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) { super(`${at}: ${why}`); }
}
const reject = (at: string, why: string): never => { throw new stdioPptxEcma376BaseMutationGuardRefusal(at, why); };
const object = (value: unknown, at: string): Readonly<Record<string, unknown>> => value !== null && typeof value === 'object' && !Array.isArray(value) ? value as Record<string, unknown> : reject(at, 'value is not an object');
const text = (value: unknown, at: string): string => typeof value === 'string' ? value : reject(at, 'value is not a string');
const integer = (value: unknown, at: string): number => Number.isSafeInteger(value) && (value as number) >= 0 ? value as number : reject(at, 'value is not a nonnegative integer');

const flag = (value: unknown, at: string): boolean => typeof value === 'boolean' ? value : reject(at, 'value is not a boolean');
const optionalInteger = (value: unknown, at: string): number | undefined => value === undefined ? undefined : integer(value, at);

/** 🚪️ Parses every revision-addressed PPTX mutation. */
export function parsePptxMutation(value: unknown, at = '$'): PptxMutation {
  const row = object(value, at), mutation = text(row.mutation, `${at}.mutation`);
  if (mutation === 'insertSlide') return { mutation, vacancy: parsePptxXmlVacancyAddress(row.vacancy, `${at}.vacancy`), entry: parseXmlNode(row.entry, `${at}.entry`) };
  if (mutation === 'removeSlide') return { mutation, address: parsePptxSlideAddress(row.address, `${at}.address`) };
  if (mutation === 'moveSlide') return { mutation, address: parsePptxSlideAddress(row.address, `${at}.address`), destinationIndex: integer(row.destinationIndex, `${at}.destinationIndex`) };
  if (mutation === 'insertShape') return { mutation, vacancy: parsePptxXmlVacancyAddress(row.vacancy, `${at}.vacancy`), shape: parseXmlNode(row.shape, `${at}.shape`) };
  if (mutation === 'removeShape') return { mutation, address: parsePptxShapeAddress(row.address, `${at}.address`) };
  if (mutation === 'setShapeText') return { mutation, address: parsePptxShapeAddress(row.address, `${at}.address`), text: text(row.text, `${at}.text`) };
  if (mutation === 'setShapePosition') return { mutation, address: parsePptxShapeAddress(row.address, `${at}.address`), position: parsePptxTransform(row.position, `${at}.position`) };
  if (mutation === 'replaceXmlNode') return { mutation, address: parsePptxXmlAddress(row.address, `${at}.address`), node: parseXmlNode(row.node, `${at}.node`) };
  if (mutation === 'setRelationship') return { mutation, owner: text(row.owner, `${at}.owner`), id: text(row.id, `${at}.id`), relType: text(row.relType, `${at}.relType`), target: text(row.target, `${at}.target`), external: row.external === undefined ? undefined : flag(row.external, `${at}.external`), index: optionalInteger(row.index, `${at}.index`) };
  if (mutation === 'removeRelationship') return { mutation, owner: text(row.owner, `${at}.owner`), id: text(row.id, `${at}.id`) };
  if (mutation === 'setContentType') return { mutation, isOverride: row.isOverride === undefined ? undefined : flag(row.isOverride, `${at}.isOverride`), name: text(row.name, `${at}.name`), contentType: text(row.contentType, `${at}.contentType`), index: optionalInteger(row.index, `${at}.index`) };
  if (mutation === 'removeContentType') return { mutation, isOverride: row.isOverride === undefined ? undefined : flag(row.isOverride, `${at}.isOverride`), name: text(row.name, `${at}.name`) };
  return reject(`${at}.mutation`, `unknown PPTX mutation ${mutation}`);
}
