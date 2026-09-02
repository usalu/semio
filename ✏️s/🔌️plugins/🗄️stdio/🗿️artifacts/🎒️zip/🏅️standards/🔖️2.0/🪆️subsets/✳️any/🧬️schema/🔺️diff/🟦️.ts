import type { ZipEntry } from '../📸️snapshot/🟦️.ts';

export interface ZipEntryDiff { name?: string; data?: number[]; }
export interface ZipEntryModified { name: string; diff: ZipEntryDiff; }
export interface ZipEntriesDiff { removed: string[]; modified: ZipEntryModified[]; added: ZipEntry[]; }
export interface ZipDiff { comment?: string; entries?: ZipEntriesDiff; }
