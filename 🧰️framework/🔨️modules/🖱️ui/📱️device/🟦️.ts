// #region 🧲️Header
/** @emoji 📱️ The ONE viewport-breakpoint policy of the shell, written once and read by every target.
 *
 * `AGENTS.md` orders the devices desktop → mobile → tablet, and the settings surface has always exposed
 * all three (`ElementsSurfaceDevice`); until this module the AUTOMATIC detection was binary, so `tablet`
 * existed only as a manual override a user had to find. The policy lives here — not in the React target
 * and not in the wgpu dock — because both renderers must answer the same width with the same device or
 * the two shells stop being byte-parity.
 *
 * Pure: no React, no DOM, no media-query object. The React target turns these into `matchMedia`
 * subscriptions; the wgpu target compares its own `screen_w`. Both go through
 * {@link elementsSurfaceDeviceForWidth}.
 *
 * @see `🧰️framework/🔨️modules/🖱️ui/📱️device/🧪️tests/🔬️unit/🟦️.ts` — including the parity assertion that
 * reads the wgpu dock's own Rust constants off disk. */
// #endregion 🧲️Header

// #region 📱️Device
/** 📱️ Document-level device chrome a shell paints for — the automatic value comes from
 * {@link elementsSurfaceDeviceForWidth}, and a user may still pin one in settings. */
export type ElementsSurfaceDevice = "desktop" | "tablet" | "mobile";

/** 📱️ Widest viewport that is still a PHONE. One flat panel stack, no dock anchors, no tab drag.
 * Byte-parity with `dock::MODE_DOCK_MOBILE_MAX_WIDTH_PX` in
 * `📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs`. */
export const UI_MOBILE_MAX_WIDTH_PX = 767;

/** 📱️ Widest viewport that is still a TABLET: wide enough for the dock anchors a phone cannot carry,
 * too narrow for the desktop's simultaneous multi-panel layout. Byte-parity with
 * `dock::MODE_DOCK_TABLET_MAX_WIDTH_PX`. */
export const UI_TABLET_MAX_WIDTH_PX = 1023;

/** 📱️ Shared viewport breakpoint below which shells switch to the automatic mobile device. */
export const UI_MOBILE_MEDIA_QUERY = `(max-width: ${UI_MOBILE_MAX_WIDTH_PX}px)`;

/** 📱️ Shared viewport band that is a tablet: above the phone breakpoint, at or below the tablet one.
 * Deliberately a BAND, not `(max-width: …)` — a phone also matches `max-width: 1023px`, and a query
 * that both devices match cannot decide between them. */
export const UI_TABLET_MEDIA_QUERY = `(min-width: ${UI_MOBILE_MAX_WIDTH_PX + 1}px) and (max-width: ${UI_TABLET_MAX_WIDTH_PX}px)`;

/**
 * 📱️ The device a viewport width IS. The ONE place the two thresholds are compared.
 *
 * Both bounds are inclusive maxima, matching the wgpu dock's `screen_w <= MAX_WIDTH_PX` and the CSS
 * `(max-width: Npx)` media feature — 767 is a phone, 768 is a tablet, 1023 is a tablet, 1024 is a
 * desktop. A non-finite width reads as desktop: the widest layout is the one that degrades most safely
 * when the measurement itself is missing (SSR, a pane measured before layout).
 */
export const elementsSurfaceDeviceForWidth = (widthPx: number): ElementsSurfaceDevice => {
  if (!Number.isFinite(widthPx)) return "desktop";
  if (widthPx <= UI_MOBILE_MAX_WIDTH_PX) return "mobile";
  if (widthPx <= UI_TABLET_MAX_WIDTH_PX) return "tablet";
  return "desktop";
};

/** 📱️ The device two already-evaluated media-query matches imply — the React target's path, where the
 * width itself is never read. `mobile` wins over `tablet` so a stale or overlapping pair of matches can
 * never resolve to the wider layout on a phone. */
export const elementsSurfaceDeviceForMatches = (matches: { readonly mobile: boolean; readonly tablet: boolean }): ElementsSurfaceDevice =>
  matches.mobile ? "mobile" : matches.tablet ? "tablet" : "desktop";

/** 📱️ Whether a device collapses the eight dock anchors into ONE flat panel stack. Only a phone does:
 * a tablet is wide enough to carry the anchors, which is the whole reason it is its own breakpoint. */
export const elementsSurfaceDeviceIsMobile = (device: ElementsSurfaceDevice): boolean => device === "mobile";

/** 📱️ Whether a device may drag tabs between dock anchors — false on a phone, where there is nothing to
 * drop into (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s `startTabDrag: mobile ? noopDrag : startTabDrag`). */
export const elementsSurfaceDeviceSupportsTabDrag = (device: ElementsSurfaceDevice): boolean => device !== "mobile";
// #endregion 📱️Device
