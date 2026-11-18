import catalog from "../🔣️.json";

/** 🌐️ The production host play is served from; this module is the browser-safe pane catalog, free of build-time inventory. */
export const PLAY_HOST = catalog.host;
/** 🏠️ The OS shell variant — it hosts apps rather than being one, so play never lists it. */
export const PLAY_HOST_VARIANT = catalog.hostVariant;
/** 🗂️ The authored pane groups in grid order. */
export const PLAY_GROUPS = catalog.groups;

/** 🎡️ One authored pane of the play grid, tagged with the group it is listed under. Mirrors
 * `🧬️schema/🔣️.json`'s `PlayPane` — `example` is the curated `PlayExampleId` the pane boots on and is
 * absent exactly for the panes whose app publishes no example for its dialect; `exampleLabel` rides with
 * it and is the text that example RENDERS in the pane's navbar picker (its `label.native.en`), so an
 * acceptance run can read the pane's chrome without resolving every plugin descriptor first. Stated rather
 * than inferred from the JSON module, because a catalog of heterogeneous rows infers as a union that
 * hides the optional key. */
export type PlayRuntimePane = {
  readonly variant: string;
  readonly label: string;
  readonly shortLabel?: string;
  readonly tagline: string;
  readonly description: string;
  readonly icon: string;
  readonly example?: string;
  readonly exampleLabel?: string;
  readonly group: string;
};

/** 📋️ Every authored pane in grid order (row-major, groups in authored order). */
export const PLAY_RUNTIME_PANES: readonly PlayRuntimePane[] = catalog.groups.flatMap(group => group.panes.map(pane => ({ ...(pane as Omit<PlayRuntimePane, "group">), group: group.id })));
