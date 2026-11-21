import { bootGisDevelopment } from "./🧩️boot.ts";

const mounted = await bootGisDevelopment();
if (import.meta.hot) import.meta.hot.dispose(() => mounted.dispose());
window.addEventListener("pagehide", () => mounted.dispose(), { once: true });
