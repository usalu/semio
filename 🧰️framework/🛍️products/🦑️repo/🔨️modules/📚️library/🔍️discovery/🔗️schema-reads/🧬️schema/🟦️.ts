/** 🧭️ Original JSON origin or exact inline source range. */
export interface SchemaSourceOrigin { readonly path: string | null; readonly hash: string; readonly selector: readonly string[]; readonly start: number; readonly end: number; }
/** 🔎️ One statically proven validator invocation, without corpus policy inference. */
export interface SchemaValidationRead { readonly schema: SchemaSourceOrigin; readonly value: SchemaSourceOrigin | null; readonly callStart: number; readonly callEnd: number; readonly resolution: "whole" | "projected" | "unresolved"; readonly inlineSchema?: Readonly<Record<string, unknown>>; }
/** ⚠️ Unresolved source evidence cannot prove dependency absence. */
export interface SchemaReadIssue { readonly start: number; readonly end: number; readonly reason: string; }
/** 📋️ Produced binding evidence for one original reader source. */
export interface SchemaValidationReadReport { readonly readerPath: string; readonly readerHash: string | null; readonly completeSyntax: boolean; readonly reads: readonly SchemaValidationRead[]; readonly unresolved: readonly SchemaReadIssue[]; }

/** 📈️ Progress from one original source inventory and reader inspection. */
export interface SchemaReadProgress { readonly phase: "walk" | "inspect"; readonly completed: number; readonly total: number | null; readonly path: string | null; }
/** 🗂️ Produced evidence for the supplied source inventory; unavailable paths never imply absence. */
export interface SchemaValidationReadIndex { readonly reports: readonly SchemaValidationReadReport[]; readonly unavailablePaths: readonly string[]; }
