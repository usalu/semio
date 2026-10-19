/** 🔎️ Code lenses and decorations
 *
 * Definition code lenses, entity-id decorations and the gutter icon.
 */

// #region 🔌️Adapters
import { ephemeralBox } from "@semio-tech/framework";
import * as path from "path";
import * as vscode from "vscode";
import { buildEntityIdRegex, ENTITY_EMOJIS } from "../🌪️entity-emoji-registry/🟦️.ts";
import { getWorkspaceRoot } from "../🎼️utilities/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🪵️Providers
function isDefinitionEntityId(id: string): boolean {
  for (const [emoji, entityKind] of ENTITY_EMOJIS.entries()) {
    if (id.includes(emoji) && entityKind.startsWith("definition-")) {
      return true;
    }
  }
  return false;
}

const NATIVE_DEFINITION_SYMBOL_KINDS = new Set<vscode.SymbolKind>([
  vscode.SymbolKind.Class,
  vscode.SymbolKind.Constant,
  vscode.SymbolKind.Constructor,
  vscode.SymbolKind.Enum,
  vscode.SymbolKind.EnumMember,
  vscode.SymbolKind.Field,
  vscode.SymbolKind.Function,
  vscode.SymbolKind.Interface,
  vscode.SymbolKind.Method,
  vscode.SymbolKind.Module,
  vscode.SymbolKind.Namespace,
  vscode.SymbolKind.Property,
  vscode.SymbolKind.Struct,
  vscode.SymbolKind.TypeParameter,
  vscode.SymbolKind.Variable,
]);

function getDocumentRelativePath(document: vscode.TextDocument): string | null {
  if (document.uri.scheme !== "file") return null;
  const root = getWorkspaceRoot();
  if (!root) return null;
  const relativePath = path.relative(root, document.uri.fsPath).replace(/\\/g, "/");
  if (!relativePath || relativePath.startsWith("..")) return null;
  return relativePath;
}

function buildNativeDefinitionScope(relativePath: string, definitionName: string): string {
  return `${relativePath}§${definitionName}`;
}

function isDocumentSymbol(value: vscode.DocumentSymbol | vscode.SymbolInformation): value is vscode.DocumentSymbol {
  return "selectionRange" in value;
}

function collectNativeDefinitionFallbackRanges(document: vscode.TextDocument): Array<{ name: string; range: vscode.Range }> {
  const patternsByLanguage: Partial<Record<string, RegExp[]>> = {
    typescript: [
      /^\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?(?:default\s+)?class\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?interface\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?type\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?enum\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\b/,
    ],
    typescriptreact: [
      /^\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?(?:default\s+)?class\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?interface\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?type\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?enum\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\b/,
    ],
    javascript: [/^\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\b/, /^\s*(?:export\s+)?(?:default\s+)?class\s+([A-Za-z_$][\w$]*)\b/, /^\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\b/],
    javascriptreact: [/^\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\b/, /^\s*(?:export\s+)?(?:default\s+)?class\s+([A-Za-z_$][\w$]*)\b/, /^\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\b/],
    go: [/^\s*func\s+(?:\([^)]+\)\s*)?([A-Za-z_][\w]*)\b/, /^\s*type\s+([A-Za-z_][\w]*)\b/, /^\s*const\s+([A-Za-z_][\w]*)\b/, /^\s*var\s+([A-Za-z_][\w]*)\b/],
  };
  const patterns = patternsByLanguage[document.languageId] ?? [];
  if (patterns.length === 0) return [];
  const results: Array<{ name: string; range: vscode.Range }> = [];
  const seen = new Set<string>();
  for (let lineIndex = 0; lineIndex < document.lineCount; lineIndex++) {
    const textLine = document.lineAt(lineIndex).text;
    for (const pattern of patterns) {
      const match = pattern.exec(textLine);
      if (!match?.[1]) continue;
      const name = match[1];
      const key = `${lineIndex}:${name}`;
      if (seen.has(key)) continue;
      seen.add(key);
      const startCharacter = textLine.indexOf(name);
      if (startCharacter < 0) continue;
      results.push({
        name,
        range: new vscode.Range(lineIndex, startCharacter, lineIndex, startCharacter + name.length),
      });
      break;
    }
  }
  return results;
}

async function collectNativeDefinitionCodeLenses(document: vscode.TextDocument, token: vscode.CancellationToken): Promise<vscode.CodeLens[]> {
  const relativePath = getDocumentRelativePath(document);
  if (!relativePath) return [];
  const symbols = (await vscode.commands.executeCommand<Array<vscode.DocumentSymbol | vscode.SymbolInformation>>("vscode.executeDocumentSymbolProvider", document.uri)) ?? [];
  const lenses: vscode.CodeLens[] = [];
  const linesSeen = new Set<number>();

  const addLens = (name: string, range: vscode.Range): void => {
    if (!name) return;
    const line = range.start.line;
    if (linesSeen.has(line)) return;
    linesSeen.add(line);
    lenses.push(
      new vscode.CodeLens(range, {
        title: "Navigate to",
        command: "compose.navigateToDefinition",
        arguments: [{ filePath: relativePath, definition: { range: { start: range.start.line + 1, end: range.end.line + 1 } } }],
      }),
    );
  };

  const visitDocumentSymbol = (symbol: vscode.DocumentSymbol): void => {
    if (token.isCancellationRequested) return;
    if (NATIVE_DEFINITION_SYMBOL_KINDS.has(symbol.kind)) {
      addLens(symbol.name, symbol.selectionRange);
    }
    for (const child of symbol.children) {
      visitDocumentSymbol(child);
    }
  };

  for (const symbol of symbols) {
    if (token.isCancellationRequested) break;
    if (isDocumentSymbol(symbol)) {
      visitDocumentSymbol(symbol);
    } else if (NATIVE_DEFINITION_SYMBOL_KINDS.has(symbol.kind)) {
      addLens(symbol.name, symbol.location.range);
    }
  }

  for (const fallback of collectNativeDefinitionFallbackRanges(document)) {
    addLens(fallback.name, fallback.range);
  }

  return lenses;
}

/**
 * 🧬️ ComposeCodeLensProvider provides navigation CodeLenses for entity IDs.
 * It uses the ENTITY_ID_REGEX built dynamically from the ENTITY_EMOJIS registry.
 **/
export class ComposeCodeLensProvider implements vscode.CodeLensProvider {
  async provideCodeLenses(document: vscode.TextDocument, token: vscode.CancellationToken): Promise<vscode.CodeLens[]> {
    const lenses: vscode.CodeLens[] = [];
    const text = document.getText();
    const regex = buildEntityIdRegex();
    let match;

    while ((match = regex.exec(text)) !== null) {
      if (token.isCancellationRequested) break;
      const id = match[1] || match[3];
      const uri = match[2];

      const startPos = document.positionAt(match.index);
      const endPos = document.positionAt(match.index + match[0].length);
      const range = new vscode.Range(startPos, endPos);

      lenses.push(
        new vscode.CodeLens(range, {
          title: "Navigate to",
          command: "compose.navigate",
          arguments: [uri || id],
        }),
      );
    }

    lenses.push(...(await collectNativeDefinitionCodeLenses(document, token)));

    return lenses;
  }
}

/**
 * 🤝️ composeGutterIcon.current holds the data fields for a composeGutterIcon.current record.
 **/
export const composeGutterIcon = ephemeralBox<vscode.TextEditorDecorationType | undefined>("framework.products.repo.modules.client.vscode.packages.typescript.extension.ts.composeGutterIcon", undefined);

/** 🔺️ updateComposeDecorations holds the data fields for a updateComposeDecorations record.
 **/
export function updateComposeDecorations(editor: vscode.TextEditor) {
  if (!editor || !composeGutterIcon.current) return;
  const text = editor.document.getText();
  const regex = buildEntityIdRegex();
  const decorations: vscode.DecorationOptions[] = [];
  let match;

  while ((match = regex.exec(text)) !== null) {
    const startPos = editor.document.positionAt(match.index);
    const endPos = editor.document.positionAt(match.index + match[0].length);
    decorations.push({ range: new vscode.Range(startPos, endPos) });
  }

  editor.setDecorations(composeGutterIcon.current, decorations);
}
// #endregion 🪵️Providers
