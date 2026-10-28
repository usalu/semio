/** 🖥️ Read-only software workspaces showing given data alongside the authoritative answer controls. */
import { useId, type ReactElement } from "react";
import type { ExcelMockup, GrasshopperMockup, Mockup, MockupEndpoint, SheetItem, Text } from "@semio-tech/course";
import type { CourseLocale } from "../🌐️i18n/🟦️.ts";

/** 🌍️ Preview content always uses the learner's explicit language. */
const localized = (value: Text, locale: CourseLocale): string => value[locale];

const phrases = {
  en: { highlight: "Highlighted", selected: "Selected", connections: "Connections", input: "Input", output: "Output", tree: "Data tree", empty: "Empty branch", from: "from", to: "to" },
  de: { highlight: "Hervorgehoben", selected: "Ausgewählt", connections: "Verbindungen", input: "Eingang", output: "Ausgang", tree: "Datenbaum", empty: "Leerer Zweig", from: "von", to: "nach" },
} as const;

/** 🔦️ The same highlight is visible and announced without naming the requested term. */
function Highlight({ shown, locale }: { readonly shown: boolean; readonly locale: CourseLocale }): ReactElement | null {
  return shown ? <span className="sr-only"> {phrases[locale].highlight}</span> : null;
}

/** 📗️ Native table semantics preserve every cell, coordinate, selection and formula. */
function Workbook({ value, locale }: { readonly value: ExcelMockup; readonly locale: CourseLocale }): ReactElement {
  const words = phrases[locale];
  const selected = value.selection;
  const address = `${value.columns[selected.column]}${selected.row + 1}`;
  const range = `${address}${selected.rowEnd !== undefined || selected.columnEnd !== undefined ? `:${value.columns[selected.columnEnd ?? selected.column]}${(selected.rowEnd ?? selected.row) + 1}` : ""}`;
  const target = (part: ExcelMockup["target"]): boolean => value.target === part;
  return <div className="course-workbook" data-highlight={target("workbook") || undefined}>
    <div className="course-workbook-title"><b>Excel</b><span>{localized(value.workbook, locale)}<Highlight shown={target("workbook")} locale={locale} /></span></div>
    <div className="course-formula-bar">
      <span className="course-name-box" data-highlight={target("name-box") || undefined}>{range}<Highlight shown={target("name-box")} locale={locale} /></span>
      <span aria-hidden="true">fx</span>
      <code data-highlight={target("formula") || undefined}>{localized(value.formula, locale) || " "}<Highlight shown={target("formula")} locale={locale} /></code>
    </div>
    <table aria-label={localized(value.sheets[value.activeSheet]!, locale)}>
      <thead><tr><th scope="col" aria-label=" " />{value.columns.map((column, index) => <th key={column} scope="col" data-highlight={target("column") && index === selected.column || undefined}>{column}<Highlight shown={target("column") && index === selected.column} locale={locale} /></th>)}</tr></thead>
      <tbody>{value.rows.map((row, rowIndex) => <tr key={rowIndex}>
        <th scope="row" data-highlight={target("row") && rowIndex === selected.row || undefined}>{rowIndex + 1}<Highlight shown={target("row") && rowIndex === selected.row} locale={locale} /></th>
        {row.map((cell, columnIndex) => {
          const isSelected = rowIndex >= selected.row && rowIndex <= (selected.rowEnd ?? selected.row) && columnIndex >= selected.column && columnIndex <= (selected.columnEnd ?? selected.column);
          const highlighted = (target("cell") || target("range")) && isSelected || target("row") && rowIndex === selected.row || target("column") && columnIndex === selected.column;
          return <td key={columnIndex} data-selected={isSelected || undefined} data-highlight={highlighted || undefined}>
            {localized(cell, locale) || " "}<span className="sr-only"> {value.columns[columnIndex]}{rowIndex + 1}{isSelected ? ` ${words.selected}` : ""}</span><Highlight shown={highlighted} locale={locale} />
          </td>;
        })}
      </tr>)}</tbody>
    </table>
    <div className="course-workbook-sheets">{value.sheets.map((sheet, index) => <span key={index} data-active={index === value.activeSheet || undefined} data-highlight={target("worksheet") && index === value.activeSheet || undefined}>{localized(sheet, locale)}<Highlight shown={target("worksheet") && index === value.activeSheet} locale={locale} /></span>)}</div>
  </div>;
}

/** 🦗️ Node geometry and directed text relations share the same port identifiers. */
function NodeGraph({ value, locale }: { readonly value: GrasshopperMockup; readonly locale: CourseLocale }): ReactElement {
  const words = phrases[locale];
  const target = value.target;
  const width = Math.max(320, value.nodes.length * 220 + 20);
  const height = Math.max(170, ...value.nodes.map((node) => 100 + Math.max(node.inputs.length, node.outputs.length) * 24 + (node.widget?.kind === "panel" ? node.widget.lines.length * 22 : node.widget?.kind === "slider" ? 64 : 0)));
  const point = (endpoint: MockupEndpoint, input: boolean): readonly [number, number] => {
    const index = value.nodes.findIndex((node) => node.id === endpoint.node);
    const node = value.nodes[index]!;
    const ports = input ? node.inputs : node.outputs;
    return [index * 220 + (input ? 20 : 200), 80 + ports.findIndex((port) => port.id === endpoint.port) * 24];
  };
  const name = (endpoint: MockupEndpoint, input: boolean): string => {
    const node = value.nodes.find((node) => node.id === endpoint.node)!;
    const port = (input ? node.inputs : node.outputs).find((port) => port.id === endpoint.port)!;
    return `${localized(node.label, locale)} · ${localized(port.label, locale)}`;
  };
  return <div className="course-grasshopper">
    <div className="course-graph" style={{ width, height }}>
      <svg width={width} height={height} aria-hidden="true">{value.wires.map((wire, index) => {
        const [x1, y1] = point(wire.from, false);
        const [x2, y2] = point(wire.to, true);
        return <path key={index} d={`M${x1},${y1} C${x1 + 65},${y1} ${x2 - 65},${y2} ${x2},${y2}`} data-highlight={target.kind === "wire" && target.index === index || undefined} />;
      })}</svg>
      {value.nodes.map((node, index) => <section key={node.id} className="course-graph-node" data-widget={node.widget?.kind ?? "component"} style={{ left: 20 + index * 220 }} data-highlight={target.kind === "node" && target.node === node.id || undefined}>
        <h5>{localized(node.label, locale)}<Highlight shown={target.kind === "node" && target.node === node.id} locale={locale} /></h5>
        <div className="course-graph-ports">{(["inputs", "outputs"] as const).map((side) => <ul key={side} aria-label={side === "inputs" ? words.input : words.output}>{node[side].map((port) => {
          const highlighted = target.kind === (side === "inputs" ? "input" : "output") && target.node === node.id && target.port === port.id;
          return <li key={port.id} data-highlight={highlighted || undefined}><span aria-hidden="true">● </span>{localized(port.label, locale)}<Highlight shown={highlighted} locale={locale} /></li>;
        })}</ul>)}</div>
        {node.widget?.kind === "slider" ? <div className="course-slider-preview">
          <div className="course-slider-track" role="img" aria-label={`${node.widget.min} ≤ ${node.widget.value} ≤ ${node.widget.max}`}><span className="course-slider-thumb" style={{ left: `${100 * (node.widget.value - node.widget.min) / (node.widget.max - node.widget.min)}%` }} /></div>
          <div className="course-slider-values"><span>{node.widget.min}</span><strong>{node.widget.value}</strong><span>{node.widget.max}</span></div>
        </div> : node.widget?.kind === "panel" ? <ol className="course-panel-values" start={0}>{node.widget.lines.map((line, index) => <li key={index}>{localized(line, locale) || " "}</li>)}</ol> : node.widget?.kind === "plugin" ? <div className="course-plugin-label">◆ {localized(node.widget.label, locale)}</div> : null}
      </section>)}
    </div>
    {value.wires.length === 0 ? null : <details className="course-graph-relations"><summary>{words.connections}</summary><ol>{value.wires.map((wire, index) => <li key={index} data-highlight={target.kind === "wire" && target.index === index || undefined}>{words.from} {name(wire.from, false)} → {words.to} {name(wire.to, true)}<Highlight shown={target.kind === "wire" && target.index === index} locale={locale} /></li>)}</ol></details>}
    {value.tree.length === 0 ? null : <section className="course-tree-panel" data-highlight={target.kind === "tree" || undefined} aria-label={words.tree}>
      <h5>{words.tree}<Highlight shown={target.kind === "tree"} locale={locale} /></h5>
      <ol>{value.tree.map((branch, branchIndex) => <li key={branch.path} data-highlight={target.kind === "branch" && target.index === branchIndex || undefined}>
        <code>{branch.path}</code><Highlight shown={target.kind === "branch" && target.index === branchIndex} locale={locale} />
        {branch.values.length === 0 ? <span>{words.empty}</span> : <ol start={0}>{branch.values.map((entry, index) => <li key={index} data-highlight={target.kind === "item" && branchIndex === 0 && target.index === index || undefined}><code>{localized(entry, locale)}</code><Highlight shown={target.kind === "item" && branchIndex === 0 && target.index === index} locale={locale} /></li>)}</ol>}
      </li>)}</ol>
    </section>}
  </div>;
}

/** 🪟️ A keyboard-scrollable, labelled preview, without controls that could alter an answer. */
export function MockupView({ value, locale }: { readonly value: Mockup; readonly locale: CourseLocale }): ReactElement {
  const id = useId();
  return <figure className="course-mockup" data-mockup={value.kind}>
    <figcaption id={id}>{localized(value.caption, locale)}</figcaption>
    <div className="course-mockup-scroll" role="region" aria-labelledby={id} tabIndex={0}>{value.kind === "excel" ? <Workbook value={value} locale={locale} /> : <NodeGraph value={value} locale={locale} />}</div>
  </figure>;
}

/** 🧷️ The optional preview stays a sibling of the item's answer controls. */
export function ItemMockup({ item, locale }: { readonly item: Pick<SheetItem, "mockup">; readonly locale: CourseLocale }): ReactElement | null {
  return item.mockup ? <MockupView value={item.mockup} locale={locale} /> : null;
}
