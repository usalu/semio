import { COMPONENT_RECEIPT_FILE, COMPONENT_RECEIPT_MAX_BYTES, parseComponentReceiptV1 } from "../🖨️describe/🏷️component-receipt/🧬️schema/🟦️.ts";
import { SemioFaultError, readDescriptorText, reportDescriptorProgress, DescriptorTextLimitError, type PackageReceiptIdentity, type PackageReceiptRequest, type PackageReceiptSource } from "../../../../../🔨️modules/🎠️kernel/📦️packages/🟦️typescript/🟦️.ts";

/** 🧾️ Reads the OS component receipt under its actual wire policy and returns a neutral kernel identity. */
export const componentPackageReceiptSource: PackageReceiptSource = Object.freeze({
  async read({ pluginId, moduleUrl, signal, onProgress }: PackageReceiptRequest): Promise<PackageReceiptIdentity> {
    signal?.throwIfAborted();
    const path = moduleUrl.split(/[?#]/u)[0]!, receiptUrl = path.slice(0, path.lastIndexOf("/") + 1) + COMPONENT_RECEIPT_FILE;
    const response = await fetch(receiptUrl, signal ? { signal } : undefined);
    const refused = (detail: string) => new SemioFaultError({ origin: "os", code: "plugin.descriptor-invalid", severity: "error", message: `plugin.descriptor-invalid: ${receiptUrl} ${detail}`, scope: { pluginId }, retryable: true });
    try {
      await reportDescriptorProgress(onProgress, signal);
      signal?.throwIfAborted();
      if (!response.ok) throw refused(`(HTTP ${response.status})`);
      let text: string;
      try { text = await readDescriptorText(response, onProgress, { version: 1, maximumBytes: COMPONENT_RECEIPT_MAX_BYTES, maximumCodeUnits: COMPONENT_RECEIPT_MAX_BYTES }, signal); }
      catch (error) {
        signal?.throwIfAborted();
        if (error instanceof DescriptorTextLimitError) throw refused(error.message);
        throw error;
      }
      let receipt;
      try { receipt = parseComponentReceiptV1(text); }
      catch (error) { throw refused((error as Error).message); }
      signal?.throwIfAborted();
      return { version: 1, pluginId: receipt.pluginId, packageId: receipt.packageId, componentSha256: receipt.wasmSha256 };
    } catch (error) {
      await response.body?.cancel(error).catch(() => {});
      signal?.throwIfAborted();
      throw error;
    }
  },
});
