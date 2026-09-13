export type WorkspaceRootDocument = Record<string, unknown> & { workspaces?: readonly string[] };

/** 📄️ Parses the root package document without interpreting unrelated fields. */
export function parseWorkspaceRootDocument(source: string): WorkspaceRootDocument {
  const document = JSON.parse(source) as unknown;
  if (!document || typeof document !== "object" || Array.isArray(document)) throw new Error("Root package.json must contain an object");
  const workspaces = (document as { workspaces?: unknown }).workspaces;
  if (workspaces !== undefined && (!Array.isArray(workspaces) || workspaces.some((entry) => typeof entry !== "string"))) throw new Error("Root package.json workspaces must be a string array");
  return document as WorkspaceRootDocument;
}

/** 🧾️ Projects canonical workspace membership while retaining every unrelated root field. */
export function renderWorkspaceRootDocument(document: WorkspaceRootDocument, workspaces: readonly string[]): string {
  return `${JSON.stringify({ ...document, workspaces: [...workspaces] }, null, 2)}\n`;
}
