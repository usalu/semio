import { GIS_INFERENCE_PRESENTATION_V1 } from "../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🪟️presentation/🟦️.tsx";
import { bootFrameworkOsDev } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts";
import { PUZZLE_BOARD_SESSION_FACTORIES } from "@semio-tech/puzzle-2d";
const mounted = await bootFrameworkOsDev({ brands: [], documentServices: [GIS_INFERENCE_PRESENTATION_V1], backboneWorkerFactory: () => new Worker(new URL("../🧩️service-composition/👷️worker/🟦️.ts", import.meta.url), { type: "module" }), surfaceSessionFactories: PUZZLE_BOARD_SESSION_FACTORIES });

if (import.meta.hot) import.meta.hot.dispose(() => mounted.dispose());
