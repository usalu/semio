export type BrowserAssetFailureDisposition = "cancelled" | "missing" | "fault";

/** 🧯️ Separates one unavailable asset from cancellation and renderer-credit faults. */
export function browserAssetFailureDisposition(error: unknown, referenceImage: boolean): BrowserAssetFailureDisposition {
  if (error instanceof DOMException && error.name === "AbortError") return "cancelled";
  if (referenceImage || error instanceof TypeError) return "missing";
  const detail = error instanceof Error ? error.message : String(error);
  if (detail.startsWith("asset-fetch-status:") || detail.startsWith("asset-response-short-read:")) return "missing";
  return "fault";
}
