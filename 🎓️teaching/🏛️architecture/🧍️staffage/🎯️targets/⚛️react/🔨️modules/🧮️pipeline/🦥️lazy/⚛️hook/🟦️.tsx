/** ⚛️ The React side of {@link ensureViewpoint}: a hook that reports whether a viewpoint can be drawn and a gate that shows a polite status in the reader's language until it can.
 *
 * @see ../🟦️.ts — the loader */
import { useEffect, useState, type ReactElement, type ReactNode } from "react";
import type { Language } from "@teaching/staffage";
import { translator } from "../../../🌐️i18n/🟦️.ts";
import { ensureViewpoint, viewpointReady } from "../🟦️.ts";

/** 🪝️ Whether the viewpoint can be drawn; loads it the first time it is asked for. */
export function useViewpointReady(viewpoint: string): boolean {
  const [ready, setReady] = useState(() => viewpointReady(viewpoint));
  useEffect(() => {
    if (viewpointReady(viewpoint)) return setReady(true);
    let current = true;
    setReady(false);
    void ensureViewpoint(viewpoint).then(() => current && setReady(true));
    return () => {
      current = false;
    };
  }, [viewpoint]);
  return ready && viewpointReady(viewpoint);
}

/** 🚪️ Renders `children` once the viewpoint can be drawn, a busy status in the language before. */
export function ViewpointGate(props: { readonly viewpoint: string; readonly language: Language; readonly children: ReactNode }): ReactElement {
  const ready = useViewpointReady(props.viewpoint);
  if (ready) return <>{props.children}</>;
  return (
    <div role="status" aria-busy="true" lang={props.language} data-loading={props.viewpoint} data-staffage-figure="">
      {translator(props.language)("viewpoints.loading")}
    </div>
  );
}
