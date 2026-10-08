import { bootFrameworkOsDev } from "../🟦️.ts";
import { admitPluginCatalogV1 } from "../../🔌️plugin/📇️registry/🟦️.ts";
const operation = new AbortController();
window.addEventListener("pagehide", () => operation.abort(), { once: true });
const admission = { maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => operation.signal.aborted, progress: (event: { readonly completed: number; readonly total: number; readonly work: number }) => window.dispatchEvent(new CustomEvent("semio:catalog-progress", { detail: event })) };
const rows = admitPluginCatalogV1({ version: 1, targets: [], hosts: [], playgrounds: [] }, admission);
const mounted = await bootFrameworkOsDev({ catalogRows: rows, variant: "", admission, brands: [] });
window.addEventListener("pagehide", () => mounted.dispose(), { once: true });
