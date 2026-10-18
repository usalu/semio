export type CommitBundleDateSection = { dateLine: string; bullets: string[] };

export type CommitBundleSection = { label: string; dates: CommitBundleDateSection[] };

export const BUNDLE_WIP_SUBJECT_RE = /^(.+🎆️\d{2}🌙️\d{2}☀️\d{2})🔀️$/u;

export const BUNDLE_DATE_SECTION_RE = /^🎆️\d{2}🌙️\d{2}☀️\d{2}$/u;

export type GitDeltaSum = { added: number; removed: number; edited: number };
