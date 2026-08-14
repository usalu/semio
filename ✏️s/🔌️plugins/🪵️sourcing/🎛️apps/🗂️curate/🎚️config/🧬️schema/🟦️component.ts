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
  /** @state config */
  filters: Filters;
  /** @state config */
  locale: string;
  /** @state config */
  contributionsJson: string;
}
