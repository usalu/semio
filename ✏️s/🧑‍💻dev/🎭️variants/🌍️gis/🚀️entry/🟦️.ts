import { PUZZLE_BOARD_SESSION_FACTORIES } from "@semio-tech/puzzle-2d";
import { bootGisDevelopment } from "./🧩️boot.ts";

const mounted = await bootGisDevelopment(PUZZLE_BOARD_SESSION_FACTORIES);
if (import.meta.hot) import.meta.hot.dispose(() => mounted.dispose());
window.addEventListener("pagehide", () => mounted.dispose(), { once: true });
