import { composeSpecificOsCatalogV1 } from "../../../🧩️catalog/🟦️.ts";
import { GIS_INFERENCE_PRESENTATION_V1 } from "../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🪟️presentation/🟦️.tsx";
import { bootFrameworkOsDev } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts";
import { PUZZLE_BOARD_SESSION_FACTORIES } from "@semio-tech/puzzle-2d";
const operation = new AbortController();
window.addEventListener("pagehide", () => operation.abort(), { once: true });
const admission = { maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => operation.signal.aborted, progress: (event: { readonly completed: number; readonly total: number; readonly work: number }) => window.dispatchEvent(new CustomEvent("semio:catalog-progress", { detail: event })) };
const variant = import.meta.env.VITE_SEMIO_PLUGIN;
if (typeof variant !== "string" || !variant) throw new Error("Specific Dev entry requires its explicit selected variant");
const mounted = await bootFrameworkOsDev({ variant, admission, catalogRows: composeSpecificOsCatalogV1(window.location.href, admission).rows, brands: [], documentServices: [GIS_INFERENCE_PRESENTATION_V1], backboneWorkerFactory: () => new Worker(new URL("../🧩️service-composition/👷️worker/🟦️.ts", import.meta.url), { type: "module" }), surfaceSessionFactories: PUZZLE_BOARD_SESSION_FACTORIES });

if (import.meta.hot) import.meta.hot.dispose(() => mounted.dispose());
window.addEventListener("pagehide", () => mounted.dispose(), { once: true });
