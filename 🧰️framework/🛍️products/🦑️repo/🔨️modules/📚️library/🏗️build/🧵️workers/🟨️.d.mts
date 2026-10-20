/** 🧾️ A pure task: an exported function of a file-addressed module. */
export interface WorkerTaskV1 { readonly module: string; readonly export: string }
/** 📈️ What a running task may report. */
export interface WorkerTaskContextV1 { progress(completed: number, total: number): void }
export interface WorkerPoolOptionsV1 {
  readonly workers?: number;
  readonly signal?: AbortSignal;
  readonly onProgress?: (row: { completed: number; total: number }) => void;
  readonly onTaskProgress?: (row: { task: number; completed: number; total: number }) => void;
}
export interface WorkerPoolV1 {
  readonly size: number;
  run<T = unknown>(task: WorkerTaskV1, input: unknown, options?: { signal?: AbortSignal }): Promise<T>;
  map<T = unknown>(task: WorkerTaskV1, inputs: readonly unknown[], options?: { signal?: AbortSignal }): Promise<T[]>;
  close(): Promise<void>;
}
/** 🏊️ Creates a bounded pool of worker threads with progress and cancellation. */
export function createWorkerPoolV1(options?: WorkerPoolOptionsV1): WorkerPoolV1;
