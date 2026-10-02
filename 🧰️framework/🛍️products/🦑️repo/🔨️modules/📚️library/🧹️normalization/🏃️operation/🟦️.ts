/** 🏗️ Canonical 🏃️operation source service. */


export interface TaxonomyProgress {
  readonly operation: "inventory" | "plan" | "apply" | "verify" | "digest";
  readonly phase: string;
  readonly current: number;
  readonly total: number;
  readonly path?: string;
}

/** 📣️ Publishes an immutable progress observation to the invocation's captured callback. */
export function report(progress: ((progress: TaxonomyProgress) => void) | undefined, operation: TaxonomyProgress["operation"], phase: string, current: number, total: number, path?: string): void {
  progress?.(Object.freeze({ operation, phase, current, total, path }));
}

export class TaxonomyCancellationError extends Error {
  constructor() {
    super("Taxonomy operation cancelled");
  }
}
