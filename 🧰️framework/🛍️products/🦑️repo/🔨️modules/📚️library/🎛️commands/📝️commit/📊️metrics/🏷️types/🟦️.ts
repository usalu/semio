export type MetricKindId = "uloc" | "size";

export type MetricKind = {
  id: MetricKindId;
  token: string;
  formatCount: (n: number) => string;
};

export type CommitMetricRow = {
  kind: MetricKindId;
  lang: string;
  emoji: string;
  code: number;
  edited: number;
  added: number;
  removed: number;
};

export type MicroCommitLangMetrics = {
  lang: string;
  emoji: string;
  code: number;
  edited: number;
  added: number;
  removed: number;
};

export type UlocByLanguage = Record<string, number>;

/** 📊️Repo-wide uloc (tracked git files; JSON uses key count) and byte size per language for commit metrics. */
export type MetricsRunner = {
  countRepoUlocByLanguage(root: string): UlocByLanguage;
  countRepoSizeByLanguage(root: string): UlocByLanguage;
};
