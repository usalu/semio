
/** 🧭️ The one shared, compile-time-enforced state model every rendered UI element carries:
 * `state` × `status` × `hover` × `selected`. Mirrors the Rust `UiState`/`UiStatus`/`UiPresence`
 * model in `ui_wgpu` (see `framework/ui/wgpu/rs/lib.rs`'s 🔖️Presence region) — string-literal unions here,
 * since this package has no dependency on the Rust crate or its generated bindings. `Hidden` makes
 * every other axis irrelevant: a hidden element is not rendered at all. */
export const UI_STATES = ["introducing", "celebrating", "previewed", "normal", "disabled", "hidden"] as const;
export type UiState = (typeof UI_STATES)[number];

/** 🧭️ The activity lifecycle of a UI element, orthogonal to {@link UiState} and composable with it. */
export const UI_STATUSES = ["waiting", "loading", "idle", "finished"] as const;
export type UiStatus = (typeof UI_STATUSES)[number];

export interface UiElementState {
  readonly state: UiState;
  readonly status: UiStatus;
  readonly hover: boolean;
  readonly selected: boolean;
}

const DEFAULT_UI_ELEMENT_STATE: UiElementState = { state: "normal", status: "idle", hover: false, selected: false };

/** 🧭️ Fills in every axis with its default (`normal`/`idle`/`false`/`false`) — the one place that convention lives. */
export function resolveElementState(partial?: Partial<UiElementState>): UiElementState {
  return { ...DEFAULT_UI_ELEMENT_STATE, ...partial };
}

/** 🙈️ `true` only for `state === "hidden"` — callers must not render/lay out/hit-test the element. */
export function elementStateHidden(s: Pick<UiElementState, "state">): boolean {
  return s.state === "hidden";
}

/** 🧭️ The one function that emits the shared `data-ui-*` attribute vocabulary every element's
 * markup carries. Two axes carry a SECOND attribute alongside `data-ui-state`/`data-ui-status`:
 * `introducing` also stamps `data-introduced="true"` — the exact attribute `UIIntroduction`'s
 * tour-driven reveal already stamps imperatively (see `framework/ui/js/react/index.tsx`), so an authored
 * `state: "introducing"` and a live tour step converge on the identical CSS rule
 * (`[data-introduced="true"]` in `🖌️ui.css`) with no duplicate styling to maintain. `celebrating`
 * mirrors this exactly with `data-celebrated="true"`, converging with `celebrateElements()`'s
 * transient imperative stamp on the same `[data-celebrated="true"]` rule. All axes are omitted at
 * their default value — the DOM stays clean when nothing is going on. A hidden element gets `{}`:
 * callers must not render it at all, so there is nothing to attribute. */
export type UiElementStateAttributes = {
  readonly "data-ui-state"?: "introducing" | "celebrating" | "previewed" | "disabled";
  readonly "data-introduced"?: "true";
  readonly "data-celebrated"?: "true";
  readonly "data-ui-status"?: "waiting" | "loading" | "finished";
  readonly "data-ui-hover"?: "true";
  readonly "data-ui-selected"?: "true";
};

export function elementStateAttributes(s: UiElementState): UiElementStateAttributes {
  if (elementStateHidden(s)) return {};
  const attrs: { -readonly [K in keyof UiElementStateAttributes]?: UiElementStateAttributes[K] } = {};
  if (s.state === "introducing") {
    attrs["data-ui-state"] = "introducing";
    attrs["data-introduced"] = "true";
  } else if (s.state === "celebrating") {
    attrs["data-ui-state"] = "celebrating";
    attrs["data-celebrated"] = "true";
  } else if (s.state === "previewed" || s.state === "disabled") {
    attrs["data-ui-state"] = s.state;
  }
  if (s.status !== "idle") attrs["data-ui-status"] = s.status;
  if (s.hover) attrs["data-ui-hover"] = "true";
  if (s.selected) attrs["data-ui-selected"] = "true";
  return attrs;
}

/** 🎨️ The shared precedence resolver for renderers that can't use CSS/data-attributes at all
 * (3D fills, canvas emissive channels — `Orb`/`Geometry`/world-mesh materials): collapses the four
 * axes to a single fill "kind" a caller maps to its own color table. Precedence, most to least
 * specific: `disabled` > `celebrated` > `selected` > `previewed` > `hovered` > `neutral`. `hidden`
 * resolves to `null` — nothing to fill, the caller must not render the element at all. */
export const ELEMENT_FILL_KINDS = ["disabled", "celebrated", "selected", "previewed", "hovered", "neutral"] as const;
export type ElementFillKind = (typeof ELEMENT_FILL_KINDS)[number];

export function resolveElementFillKind(s: UiElementState): ElementFillKind | null {
  if (elementStateHidden(s)) return null;
  if (s.state === "disabled") return "disabled";
  if (s.state === "celebrating") return "celebrated";
  if (s.selected) return "selected";
  if (s.state === "previewed") return "previewed";
  if (s.hover) return "hovered";
  return "neutral";
}
