import { S_FORMAT_CATALOG_V1 } from "../../../🚀️entry/🧭️format-catalog/🟦️.ts";
import { composeSpecificOsCatalogV1 } from "@semio-tech/s-dev-catalog-ts";
import { bootFrameworkOsDev } from "@semio-tech/framework-os-dev";

const variant = import.meta.env.VITE_SEMIO_PLUGIN;
if (variant !== "raster") throw new Error("Raster Dev requires the raster playground");
const operation = new AbortController();
window.addEventListener("pagehide", () => operation.abort(), { once: true });
const admission = {
  maxBytes: 2_097_152,
  maxRows: 512,
  maxEdges: 4096,
  maxWork: 65_536,
  deadlineMs: performance.now() + 30_000,
  now: () => performance.now(),
  cancelled: () => operation.signal.aborted,
  progress: (detail: { readonly completed: number; readonly total: number; readonly work: number }) =>
    window.dispatchEvent(new CustomEvent("semio:catalog-progress", { detail })),
};
const mounted = await bootFrameworkOsDev({ formatCatalog: S_FORMAT_CATALOG_V1,
  variant,
  admission,
  catalogRows: composeSpecificOsCatalogV1(window.location.href, admission).rows,
  brands: [],
});
if (import.meta.hot) import.meta.hot.dispose(() => mounted.dispose());
window.addEventListener("pagehide", () => mounted.dispose(), { once: true });
