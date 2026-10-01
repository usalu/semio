const TEMPLATE = String.fromCharCode(96);
export type StylingSourceLanguageV1 = "typescript" | "rust" | "css";

/** 📖️Extracts literal styling data while preserving physical source lines. */
export function stylingSourceDataV1(text: string, language: StylingSourceLanguageV1): string {
  if (!["typescript", "rust", "css"].includes(language)) throw Error("Invalid styling source language");
  const output: string[] = [], blank = (value: string): void => { output.push(value.replace(/[^\n]/g, " ")); };
  const comment = (start: number, block: boolean): number => {
    let cursor = start + 2, depth = 1;
    while (cursor < text.length) {
      if (!block && text[cursor] === "\n") break;
      if (block && language === "rust" && text.startsWith("/*", cursor)) { depth++; cursor += 2; }
      else if (block && text.startsWith("*/", cursor)) { cursor += 2; if (--depth === 0) break; }
      else cursor++;
    }
    blank(text.slice(start, cursor));
    return cursor;
  };
  const literal = (start: number, delimiter: string, hashes?: string, formatted = false): number => {
    let cursor = start + 1;
    blank(delimiter);
    while (cursor < text.length) {
      const close = text.indexOf(delimiter, cursor), escape = hashes === undefined ? text.indexOf("\\", cursor) : -1, expression = hashes === undefined && delimiter === TEMPLATE ? text.indexOf("$" + "{", cursor) : -1, field = formatted ? text.indexOf("{", cursor) : -1;
      const marker = Math.min(...[close, escape, expression, field, text.length].filter(index => index >= cursor));
      if (marker > cursor) { output.push(text.slice(cursor, marker)); cursor = marker; }
      if (cursor === text.length) break;
      const char = text[cursor]!;
      if (char === delimiter && (hashes === undefined || text.startsWith(hashes, cursor + 1))) {
        blank(text.slice(cursor, cursor + 1 + (hashes?.length ?? 0)));
        return cursor + 1 + (hashes?.length ?? 0);
      }
      if (formatted && char === "{") {
        const end = text.indexOf("}", cursor + 1);
        if (text.startsWith("{{", cursor)) { output.push("{"); cursor += 2; }
        else if (end >= 0 && (close === -1 || end < close)) { blank(text.slice(cursor, end + 1)); cursor = end + 1; }
        else { output.push(char); cursor++; }
      } else if (hashes === undefined && delimiter === TEMPLATE && text.startsWith("$" + "{", cursor)) {
        blank(text.slice(cursor, cursor + 2));
        cursor = code(cursor + 2, 1);
      } else if (hashes === undefined && char === "\\") {
        const tail = text.slice(cursor, cursor + 12), unicode = /^\\u(?:\{([0-9a-fA-F]{1,6})\}|([0-9a-fA-F]{4}))/.exec(tail), hex = /^\\x([0-9a-fA-F]{2})/.exec(tail);
        if (unicode || hex) {
          const escape = unicode ?? hex!, value = Number.parseInt(unicode ? unicode[1] ?? unicode[2]! : hex![1]!, 16);
          if (value > 0x10ffff) throw Error("Invalid Unicode source escape");
          output.push(String.fromCodePoint(value).replace(/[\r\n]/g, " "));
          cursor += escape[0].length;
        } else {
          const value = text[cursor + 1] ?? "";
          output.push(value === "\n" ? "\n" : /[nrt]/.test(value) ? " " : value);
          cursor += 2;
        }
      } else { output.push(char); cursor++; }
    }
    return cursor;
  };
  const code = (start: number, depth = 0): number => {
    let cursor = start;
    const markers = /[/"'\x60{}()]/g, parentheses: boolean[] = [];
    let controlClose = false;
    while (cursor < text.length) {
      markers.lastIndex = cursor;
      const marker = markers.exec(text), end = marker?.index ?? text.length;
      if (text.slice(cursor, end).trim()) controlClose = false;
      if (language === "css") output.push(text.slice(cursor, end)); else blank(text.slice(cursor, end));
      cursor = end;
      if (cursor === text.length) break;
      const char = text[cursor]!;
      if (text.startsWith("/*", cursor)) cursor = comment(cursor, true);
      else if (language !== "css" && text.startsWith("//", cursor)) cursor = comment(cursor, false);
      else if (language === "rust" && char === "'") {
        const character = /^'(?:[^'\\\r\n]|\\(?:u\{[a-fA-F0-9]{1,6}\}|x[a-fA-F0-9]{2}|[^\r\n]))'/u.exec(text.slice(cursor, cursor + 16));
        const width = character?.[0].length ?? 1;
        blank(text.slice(cursor, cursor + width));
        cursor += width;
      }
      else if (char === '"' || (language === "typescript" && (char === "'" || char === TEMPLATE))) {
        controlClose = false;
        const raw = language === "rust" ? /(?:^|[^a-zA-Z0-9_])b?r(#{0,255})$/.exec(text.slice(Math.max(0, cursor - 258), cursor)) : null;
        const formatted = language === "rust" && /\b(?:format|format_args|print|println|eprint|eprintln|write|writeln)!\s*\([^"'{}]*$/.test(text.slice(Math.max(0, cursor - 2048), cursor));
        cursor = literal(cursor, char, raw?.[1], formatted);
      } else if (language === "typescript" && char === "/" && (controlClose || /[=([{,:;!?&|+*%~^<>-]$|(?:return|throw|case)$/.test(text.slice(Math.max(0, cursor - 128), cursor).trimEnd()))) {
        controlClose = false;
        const start = cursor++, brackets: string[] = [];
        while (cursor < text.length && text[cursor] !== "\n") {
          if (text[cursor] === "\\") cursor += 2;
          else if (text[cursor] === "[") { brackets.push("["); cursor++; }
          else if (text[cursor] === "]") { brackets.pop(); cursor++; }
          else if (text[cursor] === "/" && brackets.length === 0) { cursor++; break; }
          else cursor++;
        }
        blank(text.slice(start, cursor));
      } else {
        if (language === "typescript" && char === "(") parentheses.push(/\b(?:if|for|while|switch|catch|with)\s*$/.test(text.slice(Math.max(0, cursor - 128), cursor)));
        else if (language === "typescript" && char === ")") controlClose = parentheses.pop() ?? false;
        else controlClose = false;
        if (depth && char === "{") depth++;
        if (depth && char === "}" && --depth === 0) { blank(char); return cursor + 1; }
        if (language === "css") output.push(char); else blank(char);
        cursor++;
      }
    }
    return cursor;
  };
  code(0);
  return output.join("");
}
