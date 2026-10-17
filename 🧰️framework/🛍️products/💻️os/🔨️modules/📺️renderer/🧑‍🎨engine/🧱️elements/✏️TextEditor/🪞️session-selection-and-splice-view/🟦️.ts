/** 🪞️ Text editor session selection and splice view: the session selection read and the splice view display. */
import { scalarOfUtf8OffsetV1, utf8OffsetOfScalarV1, type TextEditorSpliceViewV1 } from "@semio-tech/framework";
import type { FrameworkEditorSession } from "../🏷️editor-types/🟦️.ts";

//#region 🔖️TextEditorHost
/** 🔢️ A wasm editor session's selection in scalars of `text` (the session speaks UTF-8 bytes); the text's end without one. */
export function textEditorSessionSelectionV1(session: FrameworkEditorSession | null, text: string): { readonly anchor: number; readonly caret: number } {
  const end = Array.from(text).length;
  return session === null ? { anchor: end, caret: end } : { anchor: scalarOfUtf8OffsetV1(text, session.anchor()), caret: scalarOfUtf8OffsetV1(text, session.caret()) };
}

/** 🖼️ Shows what splice typing answered (ticket 26/09/23 C12, `semio.ui.scene.text-splice.v1`): the text, then the selection in
 * the session's UTF-8 bytes; nothing for `null` (the editor already shows it). */
export function showTextEditorSpliceViewV1(session: FrameworkEditorSession | null, view: TextEditorSpliceViewV1): void {
  if (session === null || view === null) return;
  session.setText(view.text);
  session.setSelectionRange(utf8OffsetOfScalarV1(view.text, view.anchor), utf8OffsetOfScalarV1(view.text, view.caret));
}
//#endregion 🔖️TextEditorHost
