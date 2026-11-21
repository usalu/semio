/** 🧍️ Entry of the architectural staffage archive website: loads the semio styles and mounts the archive app on `#root`.
 * The document assumes no language; the app sets `lang` once it knows its reader's language.
 * @see ./🛟️recovery/🟦️.ts — restores the original pathname when the archive mounts
 * @see ./🎨️.css — the semio design system chain the archive renders with
 * @see ../🎯️targets/⚛️react/📦️packages/🟦️typescript/📚️app/🟦️.tsx — the archive app */
import { STALE_CHUNK_MOUNTED_EVENT } from "./🛟️recovery/🟦️.ts";
import "./🎨️.css";
import { mountArchive } from "@teaching/staffage-react/app";

const root = document.getElementById("root");
if (root) {
  mountArchive(root);
  window.dispatchEvent(new Event(STALE_CHUNK_MOUNTED_EVENT));
}
