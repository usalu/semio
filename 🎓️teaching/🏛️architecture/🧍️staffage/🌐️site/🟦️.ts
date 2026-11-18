/** 🧍️ Entry of the architectural staffage archive website: arms the stale-chunk recovery, loads the semio styles, then mounts the archive app on `#root`.
 * The document assumes no language; the app sets `lang` once it knows its reader's language.
 * @see ./🛟️recovery/🟦️.ts — reload once when a redeploy removed the chunks this document names
 * @see ./🎨️.css — the semio design system chain the archive renders with
 * @see ../🎯️targets/⚛️react/📦️packages/🟦️typescript/📚️app/🟦️.tsx — the archive app */
import { recoverStaleChunks } from "./🛟️recovery/🟦️.ts";
import "./🎨️.css";
import { mountArchive } from "@teaching/staffage-react/app";

recoverStaleChunks({ target: window, storage: () => window.sessionStorage, reload: () => window.location.reload(), now: () => Date.now() });
const root = document.getElementById("root");
if (root) mountArchive(root);
