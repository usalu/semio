// #region 🔌️Adapters
import * as React from "react";
import { SharedBorderRow, SharedBorderRowCell } from "@semio-tech/ui-react";
// #endregion 🔌️Adapters

/** 📊️ One labelled figure of {@link StatTiles}. */
export interface StatTile {
  readonly label: string;
  readonly value: React.ReactNode;
}

/** 📊️ A set of labelled figures as ONE full-bleed band (only its bottom line is its own; the page edges and the band above own the rest): rows of `columns` tiles that share single hairlines (never one border per tile),
 * the last row padded with empty cells so every column keeps its width.
 *
 * @see ../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪢️SharedBorderRow/🟦️.tsx */
export function StatTiles({ tiles, columns, "data-slot": slot = "admin-stat-tiles" }: { readonly tiles: readonly StatTile[]; readonly columns: number; readonly "data-slot"?: string }): React.ReactElement {
  const rows: StatTile[][] = [];
  for (let index = 0; index < tiles.length; index += columns) rows.push(tiles.slice(index, index + columns));
  return (
    <SharedBorderRow orientation="vertical" edges={["bottom"]} data-slot={slot}>
      {rows.map((row, rowIndex) => (
        <SharedBorderRow key={rowIndex}>
          {row.map((tile) => (
            <SharedBorderRowCell key={tile.label} className="flex-1 flex-col items-start justify-start gap-single p-single" data-slot="admin-stat-tile">
              <span className="text-sm text-muted-foreground">{tile.label}</span>
              <span className="break-words text-lg font-semibold text-emphasized">{tile.value}</span>
            </SharedBorderRowCell>
          ))}
          {Array.from({ length: columns - row.length }, (_, filler) => (
            <SharedBorderRowCell key={`filler-${filler}`} aria-hidden="true" className="flex-1 p-single" />
          ))}
        </SharedBorderRow>
      ))}
    </SharedBorderRow>
  );
}
