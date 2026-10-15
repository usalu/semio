/** 🌳️ The json preview, the default json tree and the replaceable json tree renderer.
 *
 * 🧱️ Child module of {@link presentation-react | the `@semio-tech/presentation-react` barrel}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import { ephemeralBox } from "@semio-tech/framework";
// #endregion 🔌️Adapters
import { useState, type ReactNode } from "@semio-tech/ui-react";

//#region 🔖️json
//#region 🔖️Renderer
/** 🧬️ Renders parsed JSON as an interactive syntax tree. */
export interface JsonTreeRenderer {
  render(data: unknown): ReactNode;
}

export function jsonPreview(value: unknown): string {
  if (value === null) {
    return "null";
  }
  switch (typeof value) {
    case "string":
      return value.length > 48 ? `${value.slice(0, 45)}…` : value;
    case "number":
    case "boolean":
    case "undefined":
      return String(value);
    case "object":
      return Array.isArray(value) ? `Array(${value.length})` : `Object(${Object.keys(value as object).length})`;
    default:
      return String(value);
  }
}

function jsonEntries(value: unknown): readonly (readonly [string, unknown])[] {
  if (typeof value !== "object" || value === null) {
    return [];
  }
  if (Array.isArray(value)) {
    return value.map((entry, index) => [String(index), entry] as const);
  }
  return Object.entries(value as Record<string, unknown>);
}

function JsonScalar({ value }: { readonly value: unknown }): ReactNode {
  if (value === null) {
    return <span className="presentation-json-null">null</span>;
  }
  switch (typeof value) {
    case "string":
      return <span className="presentation-json-string">"{value}"</span>;
    case "number":
      return <span className="presentation-json-number">{value}</span>;
    case "boolean":
      return <span className="presentation-json-boolean">{String(value)}</span>;
    default:
      return <span className="presentation-json-unknown">{String(value)}</span>;
  }
}

function JsonBranch({ label, value, depth, defaultExpanded }: { readonly label: string; readonly value: unknown; readonly depth: number; readonly defaultExpanded: boolean }): ReactNode {
  const [expanded, setExpanded] = useState(defaultExpanded);
  const isArray = Array.isArray(value);
  const entries = jsonEntries(value);
  const canExpand = entries.length > 0;
  if (!canExpand) {
    return (
      <div className="presentation-json-line" style={{ paddingInlineStart: `${depth}ch` }}>
        <span className="presentation-json-key">{label}</span>
        <span className="presentation-json-colon">: </span>
        <JsonScalar value={value} />
      </div>
    );
  }
  return (
    <div className="presentation-json-branch">
      <button type="button" className="presentation-json-line presentation-json-toggle" style={{ paddingInlineStart: `${depth}ch` }} aria-expanded={expanded} onClick={() => setExpanded((open) => !open)}>
        <span className="presentation-json-caret" aria-hidden="true">
          {expanded ? "▾️" : "▸️"}
        </span>
        <span className="presentation-json-key">{label}</span>
        <span className="presentation-json-colon">: </span>
        <span className="presentation-json-meta">{isArray ? `[${entries.length}]` : `{${entries.length}}`}</span>
        {!expanded ? (
          <>
            <span className="presentation-json-colon"> </span>
            <span className="presentation-json-preview">{jsonPreview(value)}</span>
          </>
        ) : null}
      </button>
      {expanded ? entries.map(([key, entry]) => <JsonBranch key={key} label={isArray ? `[${key}]` : key} value={entry} depth={depth + 1} defaultExpanded={depth < 1} />) : null}
    </div>
  );
}

function DefaultJsonTree({ data }: { readonly data: unknown }): ReactNode {
  if (typeof data !== "object" || data === null) {
    return (
      <div className="presentation-json-tree">
        <JsonScalar value={data} />
      </div>
    );
  }
  const isArray = Array.isArray(data);
  const entries = jsonEntries(data);
  return (
    <div className="presentation-json-tree">
      {entries.map(([key, entry]) => (
        <JsonBranch key={key} label={isArray ? `[${key}]` : key} value={entry} depth={0} defaultExpanded />
      ))}
    </div>
  );
}

const defaultJsonTreeRenderer: JsonTreeRenderer = {
  render(data) {
    return <DefaultJsonTree data={data} />;
  },
};

const jsonTreeRenderer = ephemeralBox<JsonTreeRenderer>("framework.products.presentation.targets.react.jsonTreeRenderer", defaultJsonTreeRenderer);

/** 🔌️ Replaces the JSON tree renderer (tests or alternate renderers). */
export function setJsonTreeRenderer(renderer: JsonTreeRenderer): void {
  jsonTreeRenderer.current = renderer;
}

/** 🧬️ Renders JSON through the active {@link JsonTreeRenderer}. */
export function renderJsonTree(data: unknown): ReactNode {
  return jsonTreeRenderer.current.render(data);
}
//#endregion 🔖️Renderer
//#endregion 🔖️json
