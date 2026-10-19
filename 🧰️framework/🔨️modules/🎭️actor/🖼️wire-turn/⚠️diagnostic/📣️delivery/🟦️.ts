/** 📣️ Delivers one original receipt to admitted host observers. */
export class OperationDiagnosticObservers<T> {
  private readonly listeners = new Map<symbol, (receipt: T) => void>();
  constructor(readonly maximumListeners: number, private readonly refused: (error: unknown) => void) {
    if (!Number.isSafeInteger(maximumListeners) || maximumListeners < 1) throw new Error("diagnostic observers require a positive finite capacity");
  }
  subscribe(listener: (receipt: T) => void): () => void {
    if (this.listeners.size === this.maximumListeners) throw new Error("diagnostic observer capacity exhausted");
    const identity = Symbol();
    this.listeners.set(identity, listener);
    return () => { this.listeners.delete(identity); };
  }
  get size(): number { return this.listeners.size; }
  publish(receipt: T): void {
    const original = [...this.listeners.values()];
    for (const listener of original) {
      try { listener(receipt); } catch (error) {
        try { this.refused(error); } catch {}
      }
    }
  }
}
