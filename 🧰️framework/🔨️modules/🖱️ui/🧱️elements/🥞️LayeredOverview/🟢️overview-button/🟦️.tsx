/** 🟢️ The overview button phase of LayeredOverview. */
import { CatalogIcon as Icon } from "../../🔣️Icons/🟦️.tsx";
import type { usePanesState } from "../🔶️panes/🟦️.tsx";
import type { useMoveState } from "../🔸️move/🟦️.tsx";

/** 🔵 Builds overviewButton of LayeredOverview. */
export function buildOverviewButton({ interactive, opened, labels, close }: Pick<ReturnType<typeof usePanesState>, "interactive" | "opened" | "labels"> & Pick<ReturnType<typeof useMoveState>, "close">) {
  const overviewButton =
    !interactive || opened === null || labels.overview === undefined ? null : (
      <button
        type="button"
        data-layered-overview-button=""
        data-level="dialog"
        aria-keyshortcuts="Escape"
        onClick={close}
        className="ui-glass absolute right-double top-double z-40 inline-flex items-center gap-single border border-border-normal px-single py-half text-sm font-medium text-foreground outline-none transition-colors hover:border-border-emphasized focus-visible:ring-2 focus-visible:ring-ring motion-reduce:transition-none"
      >
        <Icon icon="layout-grid" size="small" />
        {labels.overview}
      </button>
    );
  return { overviewButton };
}
