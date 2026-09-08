/** 📄️ The exact route and worker admission owned by one document-opening attempt. */
export interface DocumentOpeningAttemptV1 {
  readonly current: () => boolean;
  readonly socket: () => Promise<unknown>;
  readonly attach: () => Promise<void>;
  readonly commit: () => void;
  readonly close: () => void;
  readonly retire: () => void;
  readonly deadlineMs: number;
}

/** 🧹️ Failed or retired openings release only their admission; every socket deadline is retired. */
export async function runDocumentOpeningAttemptV1(port: DocumentOpeningAttemptV1): Promise<boolean> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  let committed = false;
  try {
    if (!Number.isFinite(port.deadlineMs) || port.deadlineMs <= 0) throw new Error("invalid document opening deadline");
    await Promise.race([
      port.socket(),
      new Promise<never>((_, reject) => { timer = setTimeout(() => reject(new Error("socket actor deadline exceeded")), port.deadlineMs); }),
    ]);
    if (timer !== undefined) { clearTimeout(timer); timer = undefined; }
    if (!port.current()) return false;
    await port.attach();
    if (!port.current()) return false;
    port.commit();
    committed = true;
    return true;
  } finally {
    if (timer !== undefined) clearTimeout(timer);
    try { if (!committed) port.close(); }
    finally { port.retire(); }
  }
}
