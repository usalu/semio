/** 📚️ Canonical IDML package identity and owned event algebra. */
export const ARTIFACT_KIND = 's.stdio.idml';
export const DOCUMENT_SCHEMA = 'stdio.idml';
export const STANDARD = '8.0';
export { parseIdmlPackageValue } from './🧬️schema/🟦️.ts';
export type { IdmlPackage, IdmlPart } from './🧬️schema/🟦️.ts';
export type { IdmlSnapshot } from './🧬️schema/📸️snapshot/🟦️.ts';
export type { IdmlDiff, IdmlPatch, IdmlPartPatch } from './🧬️schema/🔺️diff/🟦️.ts';
export type { IdmlMutation } from './🧬️schema/🧬️mutations/🟦️.ts';
export { applyIdmlDiff, composeIdmlDiffs, diffIdmlSnapshots, inverseIdmlDiff, mutationIdmlDiff, inverseIdmlMutation, IdmlDiffError } from './🧬️schema/🔺️diff/🧮️algebra/🟦️.ts';
export type { IdmlAdmission } from './🧬️schema/🔺️diff/🧮️algebra/🟦️.ts';
export { IdmlAdmissionCursor, IdmlNameLookup, IdmlVersionDecoder, admitIdmlParts, IDML_ADMISSION } from './🚪️io/🛂️admission/🟦️.ts';
export type { IdmlAdmissionSource, IdmlAdmissionSnapshot, IdmlAdmissionGrant, IdmlAdmissionStep, IdmlVersionView, IdmlVersionDecodeStep, IdmlLookupKey, IdmlLookupStep } from './🚪️io/🛂️admission/🟦️.ts';
export { IdmlChangeCursor } from './🚪️io/🛂️admission/🔺️change/🟦️.ts';
export type { IdmlChangeEvent, IdmlChangeStep } from './🚪️io/🛂️admission/🔺️change/🟦️.ts';
