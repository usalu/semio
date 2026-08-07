// #region 🧲️Header
// 💻️ framework/ui/elements/🔣IconSelector/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from "../☑️Select/🟦️component.tsx";
import { Textarea } from "../📄Textarea/🟦️component.tsx";
import { useLabel } from "../🏷️Label/🟦️component.tsx";
import { type IconSelectorMode, decodeIcon, encodeIcon, classifyIconSelectorMode, Icon } from "../🔣Icons/🟦️component.tsx";
import { Button } from "../🔘Button/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🖼️IconSelector

function stripTypstEmojiPrefixesForIconSelector(raw: string): string {
  const t = raw.trim();
  if (t.startsWith("typst:")) {
    return t.slice("typst:".length).trim();
  }
  if (t.startsWith("emoji:")) {
    return t.slice("emoji:".length).trim();
  }
  return t;
}

function innerFromIconForSelectorMode(raw: string, mode: IconSelectorMode): string {
  const icon = decodeIcon(raw);
  if (!icon) {
    return "";
  }
  switch (mode) {
    case "url":
      return icon.kind === "url" ? icon.url : "";
    case "shortcode":
      return icon.kind === "shortcode" ? icon.code : "";
    case "data":
      return icon.kind === "data" ? icon.data : "";
    case "emoji":
      return icon.kind === "emoji" ? icon.emoji : "";
    case "math":
      return icon.kind === "typst" ? icon.src : "";
    case "text":
      return icon.kind === "text" ? icon.text : "";
    case "vector":
      return icon.kind === "svg" ? icon.svg : icon.kind === "catalog" ? icon.key : "";
  }
}

function emitIconKindFromSelectorMode(mode: IconSelectorMode, inner: string): string {
  const i = inner.trim();
  if (i === "") {
    return "";
  }
  switch (mode) {
    case "url":
      return encodeIcon({ kind: "url", url: i });
    case "shortcode":
      return encodeIcon({ kind: "shortcode", code: i.replace(/^:+|:+$/g, "") });
    case "data":
      return i;
    case "emoji":
      return encodeIcon({ kind: "emoji", emoji: i });
    case "math":
      return encodeIcon({ kind: "typst", src: i });
    case "text":
      return encodeIcon({ kind: "text", text: i });
    case "vector":
      return i.includes("<svg") || i.startsWith("<?xml") ? i : encodeIcon({ kind: "catalog", key: i });
  }
}

function migrateIconKindToIconSelectorMode(prev: string, mode: IconSelectorMode, classify: (raw: string) => IconSelectorMode): string {
  const cur = classify(prev);
  if (cur === mode) {
    return prev;
  }
  if (mode === "data" || mode === "vector") {
    return cur === mode ? prev : "";
  }
  const neutral = stripTypstEmojiPrefixesForIconSelector(prev).trim();
  if (mode === "math") {
    return neutral === "" ? "" : emitIconKindFromSelectorMode("math", neutral);
  }
  if (mode === "emoji") {
    return neutral === "" ? "" : emitIconKindFromSelectorMode("emoji", neutral);
  }
  if (mode === "text") {
    return neutral === "" ? "" : emitIconKindFromSelectorMode("text", neutral);
  }
  if (mode === "url") {
    return "";
  }
  if (mode === "shortcode") {
    return "";
  }
  return "";
}

export interface IconSelectorProps {
  id: string;
  value: string;
  onChange: (next: string) => void;
  disabled?: boolean;
  uniform?: boolean;
  classifyIconSelectorMode?: (raw: string) => IconSelectorMode;
}

/** @emoji 🖼️ Canonical `iconKind` editor for all canvases. */
export function IconSelector({ id, value, onChange, disabled = false, uniform = true, classifyIconSelectorMode: classifyModeProp }: IconSelectorProps): React.ReactElement {
  const classifyMode = classifyModeProp ?? classifyIconSelectorMode;
  const activeMode = classifyMode(value);
  const fileInputRef = reactHostPort.useRef<HTMLInputElement>(null);
  const locked = disabled || !uniform;
  const editorValue = uniform ? innerFromIconForSelectorMode(value, activeMode) : "";
  const importFileLabel = useLabel("ui.common.importFile");
  const clearLabel = useLabel("ui.common.clear");
  const modeUrlLabel = useLabel("ui.iconSelector.mode.url");
  const modeShortcodeLabel = useLabel("ui.iconSelector.mode.shortcode");
  const modeMathLabel = useLabel("ui.iconSelector.mode.math");
  const modeDataLabel = useLabel("ui.iconSelector.mode.data");
  const modeEmojiLabel = useLabel("ui.iconSelector.mode.emoji");
  const modeTextLabel = useLabel("ui.iconSelector.mode.text");
  const modeVectorLabel = useLabel("ui.iconSelector.mode.vector");

  const onModeSelect = (next: string) => {
    if (locked) {
      return;
    }
    onChange(migrateIconKindToIconSelectorMode(value, next as IconSelectorMode, classifyMode));
  };

  const onEditorChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    if (locked) {
      return;
    }
    onChange(emitIconKindFromSelectorMode(activeMode, e.target.value));
  };

  const editorPlaceholder =
    activeMode === "url"
      ? "https://example.com/icon.png"
      : activeMode === "shortcode"
        ? "Shortcode (emoji alias, UI icon, or catalog id — stored as :name:)"
        : activeMode === "math"
          ? "Typst markup (e.g. $x^2$)"
          : activeMode === "data"
            ? "data:image/png;base64,… or other data:… URL"
            : activeMode === "emoji"
              ? "Emoji or Typst emoji body (stored as emoji:…)"
              : activeMode === "text"
                ? "Short label (stored as text:…)"
                : "Catalog id or inline <svg …>";

  const onPickFiles = (e: React.ChangeEvent<HTMLInputElement>) => {
    const list = e.target.files;
    const f = list?.[0];
    e.target.value = "";
    if (!f || locked) {
      return;
    }
    const isSvgMime = f.type === "image/svg+xml" || /\.svg$/i.test(f.name);
    if (isSvgMime) {
      const reader = new FileReader();
      reader.onload = () => {
        const text = typeof reader.result === "string" ? reader.result.trim() : "";
        onChange(text);
      };
      reader.readAsText(f);
      return;
    }
    const reader = new FileReader();
    reader.onload = () => {
      const url = typeof reader.result === "string" ? reader.result : "";
      onChange(url.trim());
    };
    reader.readAsDataURL(f);
  };

  const previewIcon = decodeIcon(value.trim());
  const preview = (() => {
    if (!previewIcon) {
      return <span className="text-muted-foreground text-xs">—</span>;
    }
    if (previewIcon.kind === "node") {
      return previewIcon.node;
    }
    return <Icon icon={previewIcon} size={56} />;
  })();

  return (
    <div className={cn("flex min-w-0 flex-col gap-2 rounded-md border p-2", locked && "pointer-events-none opacity-60")} data-slot="icon-selector">
      <Select id={`${id}.mode.select`} disabled={locked} onValueChange={onModeSelect} value={activeMode}>
        <SelectTrigger className="h-8 w-full min-w-0 px-2 text-xs whitespace-normal" id={`${id}.mode`}>
          <SelectValue />
        </SelectTrigger>
        <SelectContent position="popper">
          <SelectItem id={`${id}.mode.url`} value="url">
            {modeUrlLabel}
          </SelectItem>
          <SelectItem id={`${id}.mode.shortcode`} value="shortcode">
            {modeShortcodeLabel}
          </SelectItem>
          <SelectItem id={`${id}.mode.math`} value="math">
            {modeMathLabel}
          </SelectItem>
          <SelectItem id={`${id}.mode.data`} value="data">
            {modeDataLabel}
          </SelectItem>
          <SelectItem id={`${id}.mode.emoji`} value="emoji">
            {modeEmojiLabel}
          </SelectItem>
          <SelectItem id={`${id}.mode.text`} value="text">
            {modeTextLabel}
          </SelectItem>
          <SelectItem id={`${id}.mode.vector`} value="vector">
            {modeVectorLabel}
          </SelectItem>
        </SelectContent>
      </Select>
      <Textarea
        className={cn("min-h-layout-preview font-mono text-xs", (activeMode === "data" || activeMode === "vector") && "min-h-layout-preview-md")}
        id={`${id}.field`}
        key={activeMode}
        mixed={!uniform}
        onChange={onEditorChange}
        placeholder={editorPlaceholder}
        readOnly={locked}
        rows={activeMode === "data" || activeMode === "vector" ? 5 : 4}
        value={editorValue}
      />
      <div className="bg-muted/30 flex min-h-peta items-center justify-center overflow-hidden rounded-sm border px-1 py-2">{preview}</div>
      <div className="flex min-w-0 flex-wrap items-center justify-between gap-2">
        <Button className="h-7 shrink-0 gap-1 px-2 text-xs" disabled={locked} onClick={() => fileInputRef.current?.click()} type="button" variant="outline" icon="folder-open" text={importFileLabel} />
        <Button className="h-7 shrink-0 px-2 text-xs whitespace-nowrap" disabled={locked} onClick={() => onChange("")} type="button" variant="ghost" icon="x" text={clearLabel} />
      </div>
      <input accept="image/png,image/jpeg,image/webp,image/gif,image/svg+xml,.svg,.png,.jpg,.jpeg,.webp,.gif" className="hidden" onChange={onPickFiles} ref={fileInputRef} type="file" />
    </div>
  );
}

// #endregion 🖼️IconSelector
