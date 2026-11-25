/** 📄️ Prepares one complete document surface before its caller publishes or retires anything. */
export interface DocumentSurfacePreparationV1<Archive,Surface> {
  readonly current: () => boolean;
  readonly capture: () => Promise<Archive>;
  readonly create: () => Promise<Surface>;
  readonly restore: (surface: Surface,archive: Archive) => Promise<void>;
  readonly attach: (surface: Surface) => Promise<void>;
  /** 🧹️ Rolls back tentative attachments and retires the unpublished successor. */
  readonly release: (surface: Surface) => Promise<void>;
}

/** 🔐️ Captures before creation can revoke predecessor activation; publication follows restoration and attachment. */
export async function prepareDocumentSurfaceV1<Archive,Surface>(ports: DocumentSurfacePreparationV1<Archive,Surface>): Promise<Surface> {
  const requireCurrent = () => { if (!ports.current()) throw new Error("surface-document.predecessor-stale"); };
  requireCurrent();
  const archive = await ports.capture();
  requireCurrent();
  const surface = await ports.create();
  try {
    requireCurrent();
    await ports.restore(surface,archive);
    requireCurrent();
    await ports.attach(surface);
    requireCurrent();
    return surface;
  } catch (error) {
    try { await ports.release(surface); }
    catch (releaseError) { throw new AggregateError([error,releaseError],"surface-document.preparation-and-release-failed"); }
    throw error;
  }
}

/** 🧵️ The Tasks-window task one restore runs as: progress, cancellation and the end of the row. */
export interface DocumentSurfaceTaskV1 {
  readonly signal: AbortSignal;
  readonly progress: (completed: number, total: number | null) => void;
  readonly finish: () => void;
}

/** 📥️ Loads the captured archive into the successor as one cancellable task: every polled status moves the task's progress, its cancel aborts the load (the caller's release then retires the unpublished successor and the predecessor stays), and the row ends however the load ends. */
export async function restoreDocumentSurfaceV1<Archive>(
  task: DocumentSurfaceTaskV1,
  load: (archive: Archive, signal: AbortSignal, progress: (status: { readonly completed: number; readonly total: number }) => void) => Promise<void>,
  archive: Archive,
): Promise<void> {
  try {
    await load(archive, task.signal, (status) => task.progress(status.completed, status.total > 0 ? status.total : null));
  } finally {
    task.finish();
  }
}
