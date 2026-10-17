/** 🪟️ InkCanvasHost overlays: the selection chrome with its resize handles, the viewport grid and the text and table cell editor overlays. */
import { useEffect, useRef } from "react";
import { useLabel } from "@semio-tech/ui-react";
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
    <div className="absolute z-30" style={{ left: screenBounds.x, top: screenBounds.y, width: screenBounds.width, height: screenBounds.height }}>
      <div className="bg-background/95 mb-1 flex gap-1 rounded border p-1 shadow-sm">
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs"
          onMouseDown={(event) => {
            event.preventDefault();
            applyCommand("bold");
          }}
        >
          B
        </button>
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs italic"
          onMouseDown={(event) => {
            event.preventDefault();
            applyCommand("italic");
          }}
        >
          I
        </button>
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs underline"
          onMouseDown={(event) => {
            event.preventDefault();
            applyCommand("underline");
          }}
        >
          U
        </button>
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs"
          onMouseDown={(event) => {
            event.preventDefault();
            const url = window.prompt(linkUrlPromptLabel);
            if (url) applyCommand("createLink", url);
          }}
        >
          {linkLabel}
        </button>
      </div>
      <div
        ref={editorRef}
        contentEditable
        suppressContentEditableWarning
        className="text-foreground bg-background h-[calc(100%-2rem)] w-full overflow-auto rounded border p-2 outline-none"
        style={{ fontSize: block.fontSize, fontWeight: block.fontWeight, textAlign: block.align }}
        dangerouslySetInnerHTML={{ __html: inkParagraphsToHtml(block.paragraphs) }}
        onBlur={() => {
          if (!editorRef.current) return;
          onCommit(inkHtmlToParagraphs(editorRef.current));
        }}
      />
    </div>
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
    <input
      ref={inputRef}
      className="bg-background ring-primary absolute z-30 rounded border px-2 py-1 text-sm ring-2 outline-none"
      style={{ left: screenBounds.x, top: screenBounds.y, width: screenBounds.width, height: screenBounds.height }}
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
  );
}
//#endregion Overlays
//#endregion 🔖️InkCanvasHost
