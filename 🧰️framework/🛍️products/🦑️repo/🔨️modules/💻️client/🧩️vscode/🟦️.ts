// #region 🧲️Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// VS Code extension providing read-only monorepo source navigation and language editing.

// #endregion 🧲️Header

// #region 🔌️Adapters
import * as vscode from "vscode";
import { isKitDocument, validateKitDocument } from "./🧾️kit-validation/🟦️.ts";
import { kitDiagnosticCollection, outputChannel } from "./🎩️globals/🟦️.ts";
import { log, logError } from "./🎼️utilities/🟦️.ts";
import { invalidateTreeNodeCache } from "./🏬️uri-resolution/🟦️.ts";
import { filterProvider, FilterTreeDataProvider, monorepoProvider, MonorepoTreeDataProvider, SectionsTreeDataProvider } from "./🌳️tree-providers/🟦️.ts";
import { ComposeCodeLensProvider, composeGutterIcon, updateComposeDecorations } from "./🔎️code-lens/🟦️.ts";
import { registerCommands } from "./⌨️commands/🟦️.ts";
// #endregion 🔌️Adapters

// #region 📜️Activation
// Activation MUST handle extension activation, command registration, and lifecycle management.

/**
 * registerSidebarViews holds the data fields for a registerSidebarViews record.
 **/
function registerSidebarViews(context: vscode.ExtensionContext): void {
  filterProvider.current = new FilterTreeDataProvider();
  vscode.window.registerTreeDataProvider("compose.filter", filterProvider.current);

  monorepoProvider.current = new MonorepoTreeDataProvider(filterProvider.current);
  vscode.window.registerTreeDataProvider("compose.monorepo", monorepoProvider.current);

  const sectionsProvider = new SectionsTreeDataProvider(context);
  vscode.window.registerTreeDataProvider("compose.sections", sectionsProvider);
}

/**
 * Activates the repo VS Code extension and registers all providers and commands.
 *Implementations MUST register sidebar views, commands, diagnostics, and event handlers.
 **/
export function activate(context: vscode.ExtensionContext) {
  outputChannel.current = vscode.window.createOutputChannel("repo");
  context.subscriptions.push(outputChannel.current);
  log("[ACTIVATION] repo extension activating...");

  try {
    registerSidebarViews(context);
    registerCommands(context);

    kitDiagnosticCollection.current = vscode.languages.createDiagnosticCollection("compose-kit");
    context.subscriptions.push(kitDiagnosticCollection.current);

    composeGutterIcon.current = vscode.window.createTextEditorDecorationType({
      gutterIconPath: vscode.Uri.file(context.asAbsolutePath("🔣️compose_codeicon.svg")),
      gutterIconSize: "contain",
    });

    context.subscriptions.push(vscode.languages.registerCodeLensProvider({ pattern: "**/*" }, new ComposeCodeLensProvider()));

    context.subscriptions.push(
      vscode.window.onDidChangeActiveTextEditor((editor) => {
        if (editor) updateComposeDecorations(editor);
      }),
    );

    if (vscode.window.activeTextEditor) {
      updateComposeDecorations(vscode.window.activeTextEditor);
    }

    context.subscriptions.push(
      vscode.workspace.onDidSaveTextDocument((document) => {
        invalidateTreeNodeCache();
        monorepoProvider.current?.refresh();
        if (isKitDocument(document)) validateKitDocument(document);
      }),
    );

    context.subscriptions.push(
      vscode.workspace.onDidOpenTextDocument((document) => {
        if (isKitDocument(document)) validateKitDocument(document);
      }),
    );

    const kitDebounceTimers = new Map<string, ReturnType<typeof setTimeout>>();
    context.subscriptions.push(
      vscode.workspace.onDidChangeTextDocument((event) => {
        if (vscode.window.activeTextEditor && event.document === vscode.window.activeTextEditor.document) {
          updateComposeDecorations(vscode.window.activeTextEditor);
        }
        const document = event.document;
        if (!isKitDocument(document)) return;
        const key = document.uri.toString();
        const existing = kitDebounceTimers.get(key);
        if (existing) clearTimeout(existing);
        kitDebounceTimers.set(
          key,
          setTimeout(() => {
            kitDebounceTimers.delete(key);
            if (isKitDocument(document)) validateKitDocument(document);
          }, 1500),
        );
      }),
    );

    context.subscriptions.push(
      vscode.workspace.registerTextDocumentContentProvider("repo", {
        provideTextDocumentContent(uri: vscode.Uri): string {
          const repoUri = `repo://${uri.authority.toLowerCase()}${uri.path.toLowerCase()}`;
          vscode.commands.executeCommand("compose.navigate", repoUri);
          return "";
        },
      }),
    );

    context.subscriptions.push(
      vscode.window.registerUriHandler({
        handleUri(uri: vscode.Uri) {
          const repoUri = `repo://${uri.authority.toLowerCase()}${uri.path.toLowerCase()}`;
          vscode.commands.executeCommand("compose.navigate", repoUri);
        },
      }),
    );

    setTimeout(() => {
      vscode.workspace.textDocuments.forEach((document) => {
        if (isKitDocument(document)) {
          validateKitDocument(document);
        }
      });
    }, 100);

    log("[ACTIVATION] repo extension activated.");
  } catch (e) {
    logError("[ACTIVATION] Failed to activate extension:", e);
  }
}

/**
 * Deactivates the repo VS Code extension and releases resources.
 *Implementations MUST clean up any active subscriptions.
 **/
export function deactivate() {}
// #endregion 📜️Activation
