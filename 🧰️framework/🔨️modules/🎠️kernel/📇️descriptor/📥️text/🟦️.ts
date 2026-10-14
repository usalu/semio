import schema from "./🧬️schema/🔣️.json";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 📏️ The independently bounded wire bytes and decoded UTF-16 units of one response. */
export type DescriptorTextPolicy = Readonly<{ version: 1; maximumBytes: number; maximumCodeUnits: number }>;

/** 📨️ Publishes one received header or admitted chunk and may await its consumer. */
export type DescriptorProgress = () => void | Promise<void>;

/** 📇️ The schema-owned descriptor document limits. */
export const PLUGIN_DESCRIPTOR_TEXT_POLICY = Object.freeze(schema.$defs.PluginDescriptorTextPolicy.const) as DescriptorTextPolicy;

/** 🛑️ Refuses a response at its first overflowing byte or decoded code unit. */
export class DescriptorTextLimitError extends RangeError {
  constructor(unit: "byte" | "code-unit") {
    super(`descriptor text exceeds its ${unit} bound`);
    this.name = "DescriptorTextLimitError";
  }
}

/** 🫀️ Awaits one progress publication while the exact original signal can interrupt it. */
export async function reportDescriptorProgress(onProgress: DescriptorProgress, signal: AbortSignal | undefined): Promise<void> {
  signal?.throwIfAborted();
  const progress = onProgress();
  if (!signal) { await progress; return; }
  let refuse!: (reason: unknown) => void;
  const interrupted = new Promise<never>((_, reject) => { refuse = reject; });
  const abort = () => { refuse(signal.reason); };
  signal.addEventListener("abort", abort, { once: true });
  try {
    if (signal.aborted) abort();
    await Promise.race([progress, interrupted]);
    signal.throwIfAborted();
  } catch (error) {
    signal.throwIfAborted();
    throw error;
  } finally { signal.removeEventListener("abort", abort); }
}

/** 📡️ Reads bounded chunks under the original cancellation and awaited progress, then releases the stream. */
export async function readDescriptorText(response: Response, onProgress: DescriptorProgress, policy: DescriptorTextPolicy, signal: AbortSignal | undefined): Promise<string> {
  signal?.throwIfAborted();
  if (validateJsonSchemaSubset(schema, policy).length) throw Error("Invalid descriptor text policy");
  const { maximumBytes, maximumCodeUnits } = policy;
  if (!response.body) return "";
  const reader = response.body.getReader(), decoder = new TextDecoder();
  let text = "", bytes = 0;
  const abort = () => { void reader.cancel(signal?.reason).catch(() => {}); };
  signal?.addEventListener("abort", abort, { once: true });
  try {
    signal?.throwIfAborted();
    for (;;) {
      const { done, value } = await reader.read();
      signal?.throwIfAborted();
      if (done) break;
      bytes += value.byteLength;
      if (bytes > maximumBytes) throw new DescriptorTextLimitError("byte");
      const chunk = decoder.decode(value, { stream: true });
      if (chunk.length > maximumCodeUnits - text.length) throw new DescriptorTextLimitError("code-unit");
      text += chunk;
      await reportDescriptorProgress(onProgress, signal);
      signal?.throwIfAborted();
    }
    const tail = decoder.decode();
    if (tail.length > maximumCodeUnits - text.length) throw new DescriptorTextLimitError("code-unit");
    return text + tail;
  } catch (error) {
    await reader.cancel(error).catch(() => {});
    signal?.throwIfAborted();
    throw error;
  } finally {
    signal?.removeEventListener("abort", abort);
    reader.releaseLock();
  }
}
