/** 🧬️ SourcingCurateConfig */
export type SortDirection = "asc" | "desc";

export interface TableSort {
  columnId: string;
  direction: SortDirection;
}

export interface Filters {
  query: string;
  moduleIds: string[];
  typologyPath: string[];
  minAvailability: number;
  sort?: TableSort | null;
}

export interface SourcingCurateConfig {
  /** @state local-ui */
  filters: Filters;
  /** @state local-ui */
  selectedObjectId?: string;
  /** @state local-ui */
  locale: string;
  /** @state local-ui */
  contributionsJson: string;
}
