/** 🎛️ Trinity Rewriting editor — Parameters window: typed twin of `🦀️.rs`'s declarative-form
 * render boundary over the RHS's declared parameters. Every value is pre-stringified on the Rust
 * side (`PropertyValue` → display string) before reaching `render()`, so the TS twin mirrors that
 * flattened shape rather than a typed `PropertyValue` union. */

export type TrinityRewritingEditParameterKind = "string" | "number" | "boolean";

/** 🎛️ One editable parameter row, mirrors the `UiFieldNode`/`UiInputNode` pair `render()` builds
 * per `Rhs::parameters` entry. */
export interface TrinityRewritingEditParameterField {
  name: string;
  kind: TrinityRewritingEditParameterKind;
  value: string;
}

export interface TrinityRewritingEditParametersViewModel {
  windowKindId: "trinity-rewriting-edit-parameters";
  bodyKey: "trinity.rewriting.edit.parameters";
  surfaceId: "trinity.rewriting.edit.parameters";
  parameters: TrinityRewritingEditParameterField[];
  editable: true;
}

export const TRINITY_REWRITING_EDIT_PARAMETERS_WINDOW_KIND_ID = "trinity-rewriting-edit-parameters" as const;
export const TRINITY_REWRITING_EDIT_PARAMETERS_BODY_KEY = "trinity.rewriting.edit.parameters" as const;
export const TRINITY_REWRITING_EDIT_PARAMETERS_SURFACE_ID = "trinity.rewriting.edit.parameters" as const;
