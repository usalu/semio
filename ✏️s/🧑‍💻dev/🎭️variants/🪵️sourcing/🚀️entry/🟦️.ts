import { formatCatalogOwnedByV1 } from "../../../🚀️entry/🧭️format-catalog/🟦️.ts";
import { composeSpecificOsCatalogV1 } from "@semio-tech/s-dev-catalog-ts";
import { bootFrameworkOsDev } from "@semio-tech/framework-os-dev";
const operation = new AbortController();
const stop = () => operation.abort();
window.addEventListener("pagehide", stop, { once: true, signal: operation.signal });
if (import.meta.hot) import.meta.hot.dispose(stop);
const admission = {
  maxBytes: 2097152,
  maxRows: 512,
  maxEdges: 4096,
  maxWork: 65536,
  deadlineMs: performance.now() + 30000,
  now: () => performance.now(),
  cancelled: () => operation.signal.aborted,
  progress: (event: { readonly completed: number; readonly total: number; readonly work: number }) =>
    window.dispatchEvent(new CustomEvent("semio:catalog-progress", { detail: event })),
};
const variant = import.meta.env.VITE_SEMIO_PLUGIN;
if (variant !== "sourcing") throw new Error("Sourcing Dev entry requires its explicit selected variant");
const mounted = await bootFrameworkOsDev({ formatCatalog: formatCatalogOwnedByV1("s.stdio.json.standard.rfc8259.representation.document", "s.sourcing.curation@1/*"), variant, admission, catalogRows: composeSpecificOsCatalogV1(window.location.href, admission).rows, brands: [] });
if (operation.signal.aborted) mounted.dispose();
else operation.signal.addEventListener("abort", () => mounted.dispose(), { once: true });
