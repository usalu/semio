/** 🪟️ InkCanvasHost overlays: the selection chrome with its resize handles, the viewport grid and the text and table cell editor overlays. */
import { useEffect, useRef } from "../../⚛️react-port/🟦️.ts";
import { Input, SharedBorderRow, useLabel } from "@semio-tech/ui-react";
import type { InkBounds, InkTableItem, InkTextItem, InkTextParagraph } from "../🧾️ink-model/🟦️.ts";
import { inkHtmlToParagraphs, inkParagraphsToHtml } from "../🌳️ink-document-ops/🟦️.ts";

//#region 🔖️InkCanvasHost
//#region Overlays
export function InkTextEditorOverlay({ block, screenBounds, onCommit, onCancel }: { readonly block: InkTextItem; readonly screenBounds: InkBounds; readonly onCommit: (paragraphs: readonly InkTextParagraph[]) => void; readonly onCancel: () => void }) {
  const editorRef = useRef<HTMLDivElement | null>(null);
  const linkLabel = useLabel("ui.ink.link");
  const linkUrlPromptLabel = useLabel("ui.ink.linkUrlPrompt");
  const applyCommand = (command: string, value?: string) => {
    editorRef.current?.focus();
    document.execCommand(command, false, value);
  };
  useEffect(() => {
    const editor = editorRef.current;
    if (!editor) return;
    editor.focus();
    const selection = window.getSelection();
    const range = document.createRange();
    range.selectNodeContents(editor);
    selection?.removeAllRanges();
    selection?.addRange(range);
    const onKeyDown = (event: globalThis.KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        onCancel();
      }
    };
    editor.addEventListener("keydown", onKeyDown);
    return () => editor.removeEventListener("keydown", onKeyDown);
  }, [onCancel]);
  return (
    <SharedBorderRow data-ink-editor-overlay="" orientation="vertical" className="bg-background/95 absolute z-30 shadow-sm" style={{ left: screenBounds.x, top: screenBounds.y, width: screenBounds.width, height: screenBounds.height }}>
      <SharedBorderRow className="shrink-0">
        <button
          type="button"
          data-shared-border-cell="" className="hover:bg-muted px-2 py-0.5 text-xs"
          onMouseDown={(event) => {
            event.preventDefault();
            applyCommand("bold");
          }}
        >
          B
        </button>
        <button
          type="button"
          data-shared-border-cell="" className="hover:bg-muted px-2 py-0.5 text-xs italic"
          onMouseDown={(event) => {
            event.preventDefault();
            applyCommand("italic");
          }}
        >
          I
        </button>
        <button
          type="button"
          data-shared-border-cell="" className="hover:bg-muted px-2 py-0.5 text-xs underline"
          onMouseDown={(event) => {
            event.preventDefault();
            applyCommand("underline");
          }}
        >
          U
        </button>
        <button
          type="button"
          data-shared-border-cell="" className="hover:bg-muted px-2 py-0.5 text-xs"
          onMouseDown={(event) => {
            event.preventDefault();
            const url = window.prompt(linkUrlPromptLabel);
            if (url) applyCommand("createLink", url);
          }}
        >
          {linkLabel}
        </button>
      </SharedBorderRow>
      <div
        ref={editorRef}
        contentEditable
        suppressContentEditableWarning
        data-shared-border-cell="" className="text-foreground bg-background min-h-0 w-full flex-1 overflow-auto p-2 outline-none"
        style={{ fontSize: block.fontSize, fontWeight: block.fontWeight, textAlign: block.align }}
        dangerouslySetInnerHTML={{ __html: inkParagraphsToHtml(block.paragraphs) }}
        onBlur={() => {
          if (!editorRef.current) return;
          onCommit(inkHtmlToParagraphs(editorRef.current));
        }}
      />
    </SharedBorderRow>
  );
}

export function InkTableCellEditorOverlay({
  block,
  row,
  col,
  screenBounds,
  onCommit,
  onCancel,
}: {
  readonly block: InkTableItem;
  readonly row: number;
  readonly col: number;
  readonly screenBounds: InkBounds;
  readonly onCommit: (content: string, advance?: boolean) => void;
  readonly onCancel: () => void;
}) {
  const inputRef = useRef<HTMLInputElement | null>(null);
  useEffect(() => {
    inputRef.current?.focus();
    inputRef.current?.select();
  }, []);
  return (
    <div data-ink-editor-overlay="" className="absolute z-30 flex" style={{ left: screenBounds.x, top: screenBounds.y, width: screenBounds.width, height: screenBounds.height }}>
      <Input
        id="ink-table-cell-editor"
        ref={inputRef}
        className="text-sm"
        defaultValue={block.rows[row]?.[col]?.content ?? ""}
        onKeyDown={(event) => {
          if (event.key === "Escape") {
            event.preventDefault();
            onCancel();
          }
          if (event.key === "Enter" || event.key === "Tab") {
            event.preventDefault();
            onCommit(event.currentTarget.value, true);
          }
        }}
        onBlur={(event) => onCommit(event.currentTarget.value)}
      />
    </div>
  );
}
//#endregion Overlays
//#endregion 🔖️InkCanvasHost
