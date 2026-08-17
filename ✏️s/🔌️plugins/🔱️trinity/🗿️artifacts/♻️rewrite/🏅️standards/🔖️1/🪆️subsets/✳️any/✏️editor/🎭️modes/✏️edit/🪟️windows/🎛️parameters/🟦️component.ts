/** 🎛️ Trinity Rewrite editor — Parameters window: typed twin of `🦀️component.rs`'s declarative-form
 * render boundary over the RHS's declared parameters. Every value is pre-stringified on the Rust
 * side (`PropertyValue` → display string) before reaching `render()`, so the TS twin mirrors that
 * flattened shape rather than a typed `PropertyValue` union. */

export type TrinityRewriteEditParameterKind = "string" | "number" | "boolean";

/** 🎛️ One editable parameter row, mirrors the `UiFieldNode`/`UiInputNode` pair `render()` builds
 * per `Rhs::parameters` entry. */
export interface TrinityRewriteEditParameterField {
  name: string;
  kind: TrinityRewriteEditParameterKind;
  value: string;
}

export interface TrinityRewriteEditParametersViewModel {
  windowKindId: "trinity-rewrite-edit-parameters";
  bodyKey: "trinity.rewrite.edit.parameters";
  surfaceId: "trinity.rewrite.edit.parameters";
  parameters: TrinityRewriteEditParameterField[];
  editable: true;
}

export const TRINITY_REWRITE_EDIT_PARAMETERS_WINDOW_KIND_ID = "trinity-rewrite-edit-parameters" as const;
export const TRINITY_REWRITE_EDIT_PARAMETERS_BODY_KEY = "trinity.rewrite.edit.parameters" as const;
export const TRINITY_REWRITE_EDIT_PARAMETERS_SURFACE_ID = "trinity.rewrite.edit.parameters" as const;
