/** 🔁️ Declares the revision-pinned helper loader: the shared `loaded` record, the revision importer and the closed project-input projection. */
export const component: "bootstrap";
export let libraryBootstrap: Promise<void> | undefined;
export function importRevision(source: string | URL, revision: string): Promise<Record<string, any>>;
export function captureProjectInputsV1(module: Record<string, unknown>): Record<string, (...args: any[]) => any>;
export const loaded: Record<string, any>;
export function wire(registry: Record<string, any>): void;
