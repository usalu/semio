/** 🧰️ Shared constants and helpers of the extension-host suites: the expected commands, constraints and views, the workspace and fixture openers, the diagnostics wait, the definition entity scan and the code lens readers. */
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { buildEntityIdRegex, ENTITY_EMOJIS } from "../../🌪️entity-emoji-registry/🟦️.ts";

// #region 🧪️OwnedExtensionHostTestContract
type OwnedExtensionHostTestContext = { timeout(milliseconds: number): void };
type OwnedExtensionHostTestBody = (this: OwnedExtensionHostTestContext) => void | Promise<void>;
declare function suite(title: string, body: OwnedExtensionHostTestBody): void;
declare function test(title: string, body: OwnedExtensionHostTestBody): void;
declare function suiteSetup(body: OwnedExtensionHostTestBody): void;

// #endregion 🧪️OwnedExtensionHostTestContract
// #region 🎞️Constants
export const EXPECTED_COMMANDS = [
  "compose.copyId",
  "compose.mailto",
  "compose.openLink",
  "compose.refreshMonorepo",
  "compose.refreshCodebase",
  "compose.refreshItem",
  "compose.filter.search",
  "compose.filter.toggle",
  "compose.filter.toggleYear",
  "compose.filter.toggleMonth",
  "compose.filter.toggleDay",
  "compose.navigateToBundle",
  "compose.navigateToFolder",
  "compose.navigateToFile",
  "compose.navigateToSection",
  "compose.navigateToDefinition",
  "compose.navigate",
  "compose.navigateTo",
  "compose.ticketOpen",
  "compose.copyCheckpointSha",
  "compose.openCheckpointInGitHub",
  "compose.open"
];
export const EXPECTED_CONSTRAINTS = [
  "guid-unique",
  "type-name-unique",
  "design-name-unique",
  "piece-name-unique",
  "quality-name-unique",
  "port-name-unique",
  "file-name-unique",
  "folder-name-unique",
  "connector-name-unique",
  "model-name-unique",
  "layer-path-unique",
];
export const EXPECTED_VIEWS = ["compose.monorepo", "compose.filter"];

// #endregion 🎞️Constants
// #region 🎼️Utilities
export function getWorkspaceRoot(): string {
  return vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? path.join(__dirname, "../../../..");
}

function getFixturePath(relativePath: string): string {
  return path.join(getWorkspaceRoot(), "compose", "assets", relativePath);
}

export async function openWorkspaceDocument(...relativePaths: string[]): Promise<vscode.TextDocument> {
  const workspaceRoot = getWorkspaceRoot();
  const searchRoots = [workspaceRoot, path.join(workspaceRoot, ".."), path.join(workspaceRoot, "..", "..")];

  for (const relativePath of relativePaths) {
    for (const searchRoot of searchRoots) {
      const documentPath = path.resolve(searchRoot, relativePath);
      if (fs.existsSync(documentPath)) {
        return vscode.workspace.openTextDocument(vscode.Uri.file(documentPath));
      }
    }
  }

  throw new Error(`Workspace document not found. Tried: ${relativePaths.join(", ")}`);
}

export async function openFixture(relativePath: string): Promise<vscode.TextDocument> {
  const fixturePath = getFixturePath(relativePath);
  if (!fs.existsSync(fixturePath)) {
    throw new Error(`Fixture not found at ${fixturePath}`);
  }
  const fixtureUri = vscode.Uri.file(fixturePath);
  const document = await vscode.workspace.openTextDocument(fixtureUri);
  return document;
}

export async function waitForDiagnostics(uri: vscode.Uri, timeout = 5000): Promise<vscode.Diagnostic[]> {
  await new Promise((resolve) => setTimeout(resolve, timeout));
  return vscode.languages.getDiagnostics(uri).filter((d) => d.source === "compose");
}

function isDefinitionEntityId(id: string): boolean {
  for (const [emoji, entityKind] of ENTITY_EMOJIS.entries()) {
    if (id.includes(emoji) && entityKind.startsWith("definition-")) {
      return true;
    }
  }
  return false;
}

export function collectDefinitionEntityIds(text: string): string[] {
  const regex = buildEntityIdRegex();
  const ids = new Set<string>();

  for (const match of text.matchAll(regex)) {
    const id = match[1] || match[3];
    if (id && isDefinitionEntityId(id)) {
      ids.add(id);
    }
  }

  return Array.from(ids);
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

function getDocumentRelativePath(document: vscode.TextDocument): string {
  const workspaceRoot = getWorkspaceRoot();
  return path.relative(workspaceRoot, document.uri.fsPath).replace(/\\/g, "/");
}

function isDocumentSymbol(value: vscode.DocumentSymbol | vscode.SymbolInformation): value is vscode.DocumentSymbol {
  return "selectionRange" in value;
}

function collectNativeDefinitionFallbackEntries(document: vscode.TextDocument): Array<{ name: string; line: number }> {
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
  const entries: Array<{ name: string; line: number }> = [];
  for (let lineIndex = 0; lineIndex < document.lineCount; lineIndex++) {
    const textLine = document.lineAt(lineIndex).text;
    for (const pattern of patterns) {
      const match = pattern.exec(textLine);
      if (!match?.[1]) continue;
      entries.push({ name: match[1], line: lineIndex });
      break;
    }
  }
  return entries;
}

export async function collectNativeDefinitionScopes(document: vscode.TextDocument): Promise<string[]> {
  const relativePath = getDocumentRelativePath(document);
  const symbols = (await vscode.commands.executeCommand<Array<vscode.DocumentSymbol | vscode.SymbolInformation>>("vscode.executeDocumentSymbolProvider", document.uri)) ?? [];
  const scopes: string[] = [];
  const linesSeen = new Set<number>();

  const addScope = (name: string, kind: vscode.SymbolKind, range: vscode.Range): void => {
    if (!name || !NATIVE_DEFINITION_SYMBOL_KINDS.has(kind)) return;
    const line = range.start.line;
    if (linesSeen.has(line)) return;
    linesSeen.add(line);
    scopes.push(`${relativePath}§${name}`);
  };

  const visitDocumentSymbol = (symbol: vscode.DocumentSymbol): void => {
    addScope(symbol.name, symbol.kind, symbol.selectionRange);
    for (const child of symbol.children) {
      visitDocumentSymbol(child);
    }
  };

  for (const symbol of symbols) {
    if (isDocumentSymbol(symbol)) {
      visitDocumentSymbol(symbol);
    } else {
      addScope(symbol.name, symbol.kind, symbol.location.range);
    }
  }

  for (const fallback of collectNativeDefinitionFallbackEntries(document)) {
    if (linesSeen.has(fallback.line)) continue;
    linesSeen.add(fallback.line);
    scopes.push(`${relativePath}§${fallback.name}`);
  }

  return scopes;
}

export async function getCodeLenses(document: vscode.TextDocument): Promise<vscode.CodeLens[]> {
  await vscode.window.showTextDocument(document, { preview: true, preserveFocus: false });
  await new Promise((resolve) => setTimeout(resolve, 250));
  return (await vscode.commands.executeCommand<vscode.CodeLens[]>("vscode.executeCodeLensProvider", document.uri)) ?? [];
}
// #endregion 🎼️Utilities
