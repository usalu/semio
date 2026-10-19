/** 🧪️ Types of the jco probe host shim `🟨️.js`: the delayed echo and the five-chunk byte stream the probe component awaits. */
export function slowEcho<T>(ms: number, value: T): Promise<T>;
export function fetchBody(): Promise<AsyncGenerator<number, void, undefined>>;
