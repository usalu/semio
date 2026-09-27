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
