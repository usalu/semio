/** 🧱️ Semantic table and definition-list shells shared by the detail sections: captioned, scoped headers, horizontally scrollable on small screens.
 * @see https://www.w3.org/WAI/tutorials/tables/ — accessible tables */
import type { ReactElement, ReactNode } from "react";
import { formatNumber, decimalsFor } from "@teaching/staffage";
import type { Language, ParameterSource } from "@teaching/staffage";
import { SharedBorderRow, cn } from "../../🪟️chrome/🟦️.tsx";

/** 📊️ A captioned table inside a scroll container; `head` is the column header cells. */
export function DataTable(props: { readonly caption: ReactNode; readonly head: readonly ReactNode[]; readonly children: ReactNode; readonly className?: string }): ReactElement {
  return (
    <div data-shared-border-cell="" className={cn("max-w-full overflow-x-auto", props.className)} tabIndex={0} role="region" aria-label={typeof props.caption === "string" ? props.caption : undefined}>
      <table className="w-full border-collapse text-left text-xs">
        <caption className="border-b border-border bg-muted/40 p-double text-left text-xs font-semibold">{props.caption}</caption>
        <thead>
          <tr>
            {props.head.map((cell, index) => (
              <th key={index} scope="col" className="whitespace-nowrap border-b border-border px-double py-single text-[0.65rem] font-medium uppercase tracking-wide text-muted-foreground">
                {cell}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>{props.children}</tbody>
      </table>
    </div>
  );
}

/** 📋️ A body row: the first cell is the row header. */
export function Row(props: { readonly head: ReactNode; readonly cells: readonly ReactNode[] }): ReactElement {
  return (
    <tr className="border-b border-border/60 align-top">
      <th scope="row" className="px-double py-single text-left font-medium">
        {props.head}
      </th>
      {props.cells.map((cell, index) => (
        <td key={index} className="px-double py-single font-mono">
          {cell}
        </td>
      ))}
    </tr>
  );
}

/** 🧾️ A term and its value in a definition list: a cell of the {@link FactList} column. */
export function Fact(props: { readonly term: ReactNode; readonly children: ReactNode }): ReactElement {
  return (
    <div data-shared-border-cell="" className="flex min-w-0 flex-wrap items-baseline justify-between gap-x-double gap-y-0.5 px-double py-single">
      <dt className="text-[0.65rem] uppercase tracking-wide text-muted-foreground">{props.term}</dt>
      <dd className="m-0 break-words font-mono text-xs">{props.children}</dd>
    </div>
  );
}

/** 🧾️ A definition list whose facts are the cells of one column. */
export function FactList(props: { readonly children: ReactNode; readonly "data-totals"?: string }): ReactElement {
  return <SharedBorderRow as="dl" orientation="vertical" {...(props["data-totals"] === undefined ? {} : { "data-totals": props["data-totals"] })}>{props.children}</SharedBorderRow>;
}

/** 🔢️ A number to three significant digits in the reader's number format. */
export const num = (value: number, language: Language, significant = 3): string => formatNumber(value, decimalsFor(Math.abs(value), significant), language);

/** 🔬️ A number in scientific notation with the language's decimal separator. */
export function sci(value: number, language: Language): string {
  if (value === 0) return "0";
  const [mantissa = "0", exponent = "0"] = value.toExponential(2).split("e");
  return `${language === "de" ? mantissa.replace(".", ",") : mantissa}·10^${Number(exponent)}`;
}

/** 🏷️ The parameter sources in display order. */
export const SOURCES: readonly ParameterSource[] = ["measured", "literature", "calibrated", "authored-estimate", "illustrative"];
