/** 🪝️ The reader's preview viewpoint: which of side elevation, front elevation or plan (top view) every catalog preview, frieze and thumbnail draws. A persisted local-only preference, not part of a link.
 *
 * @see ../🟦️.ts — the viewpoints
 * @see ../../💾️persistence/🟦️.ts — `Preferences.previewViewpoint`
 */
import type { ReactElement } from "react";
import { VIEWPOINTS } from "@teaching/staffage";
import type { Viewpoint } from "@teaching/staffage";
import { useArchive, useLocal } from "../../🧭️context/🟦️.tsx";
import { ViewpointSelector } from "../🎚️selector/🟦️.tsx";

/** 🪝️ The preview viewpoint and the way to change it. */
export function usePreviewViewpoint(): readonly [Viewpoint, (viewpoint: Viewpoint) => void] {
  const { store } = useArchive();
  const { preferences } = useLocal();
  return [preferences.previewViewpoint, (previewViewpoint) => store.dispatch({ type: "preferences-changed", patch: { previewViewpoint } })];
}

/** 🎚️ The selector of the preview viewpoint (the three single viewpoints, no sheet). */
export function PreviewViewpointSelector(props: { readonly className?: string }): ReactElement {
  const { language } = useArchive();
  const [viewpoint, setViewpoint] = usePreviewViewpoint();
  return <ViewpointSelector className={props.className} value={viewpoint} language={language} choices={VIEWPOINTS} onChange={(choice) => choice !== "all" && setViewpoint(choice)} />;
}
