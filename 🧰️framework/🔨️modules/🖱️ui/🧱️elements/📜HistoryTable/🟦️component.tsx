// #region 🧲️Header
// 💻️ framework/ui/elements/📜HistoryTable/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { TableAvatar } from "../👤Avatar/🟦️component.tsx";
import { type ElementProps } from "../🐹️ElementProps/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🗄️HistoryTable
/** @emoji 🗄️ Author of a VCS checkpoint. */
export interface HistoryColumnAuthor {
  readonly id: string;
  readonly name: string;
  readonly avatar?: string;
}

/** @emoji 🗄️ One row of a checkpoint ancestor graph — mirrors the plugin-side `HistoryColumn` (see `vcs::HistoryColumn`). */
export interface HistoryColumn {
  readonly checkpointId: string;
  readonly timestamp: string;
  readonly labels: readonly string[];
  readonly authors: readonly HistoryColumnAuthor[];
  readonly parentCheckpointId?: string;
  readonly description?: string;
  readonly lane: number;
  readonly alternativeIds: readonly string[];
}

export interface HistoryTableProps extends ElementProps {
  readonly columns: readonly HistoryColumn[];
  readonly className?: string;
  readonly laneWidth?: number;
  readonly onSelectCheckpoint?: (checkpointId: string) => void;
}

const HISTORY_GRAPH_GUIDE_CLASS = "bg-muted-foreground/40";
const HISTORY_ROW_SHELL_CLASS = "relative h-workbench min-h-workbench max-h-workbench select-none overflow-hidden min-w-0";
const HISTORY_LANE_PITCH = 16;
const HISTORY_LANE_PAD = 8;
const HISTORY_AUTHOR_SLOT = 40;
const HISTORY_GRAPH_STROKE = "color-mix(in oklab, var(--muted-foreground) 40%, transparent)";

function historyLaneCount(columns: readonly HistoryColumn[]): number {
  return Math.max(1, ...columns.map((column) => column.lane + 1));
}

function historyGraphWidth(laneCount: number, laneWidth?: number): number {
  return laneWidth ?? Math.max(56, HISTORY_LANE_PAD * 2 + laneCount * HISTORY_LANE_PITCH);
}

function historyGraphColumnWidth(laneCount: number, laneWidth?: number): number {
  return historyGraphWidth(laneCount, laneWidth) + HISTORY_AUTHOR_SLOT;
}

function historyLaneX(lane: number, laneCount: number, graphWidth: number): number {
  if (laneCount <= 1) return graphWidth / 2;
  return HISTORY_LANE_PAD + lane * HISTORY_LANE_PITCH + HISTORY_LANE_PITCH / 2;
}

function historyRowIndexByCheckpointId(columns: readonly HistoryColumn[]): Map<string, number> {
  return new Map(columns.map((column, index) => [column.checkpointId, index]));
}

function historyRowLaneGuides(columns: readonly HistoryColumn[], laneCount: number): boolean[][] {
  const guides = Array.from({ length: columns.length }, () => Array.from({ length: laneCount }, () => false));
  const rowById = historyRowIndexByCheckpointId(columns);
  for (const [rowIndex, column] of columns.entries()) {
    guides[rowIndex]![column.lane] = true;
    const parentRow = column.parentCheckpointId ? rowById.get(column.parentCheckpointId) : undefined;
    if (parentRow === undefined) continue;
    const parentLane = columns[parentRow]!.lane;
    if (column.lane === parentLane) {
      for (let row = rowIndex + 1; row < parentRow; row += 1) guides[row]![column.lane] = true;
      continue;
    }
    const elbowRow = rowIndex + 1 < parentRow ? rowIndex + 1 : parentRow;
    for (let row = rowIndex + 1; row <= elbowRow; row += 1) guides[row]![column.lane] = true;
    for (let row = elbowRow; row < parentRow; row += 1) guides[row]![parentLane] = true;
  }
  return guides;
}

function HistoryGraphSvg({ columns, graphWidth, rowHeight }: { readonly columns: readonly HistoryColumn[]; readonly graphWidth: number; readonly rowHeight: number }): React.ReactElement {
  const laneCount = historyLaneCount(columns);
  const rowById = historyRowIndexByCheckpointId(columns);
  const guides = historyRowLaneGuides(columns, laneCount);
  const height = Math.max(columns.length, 1) * rowHeight;
  return (
    <svg width={graphWidth} height={height} className="absolute inset-0 block pointer-events-none" aria-hidden>
      {guides.map((rowGuides, rowIndex) =>
        rowGuides.map((active, lane) => {
          if (!active) return null;
          const x = historyLaneX(lane, laneCount, graphWidth);
          const y0 = rowIndex * rowHeight;
          const y1 = y0 + rowHeight;
          return <line key={`guide-${rowIndex}-${lane}`} x1={x} y1={y0} x2={x} y2={y1} stroke={HISTORY_GRAPH_STROKE} strokeWidth={1} />;
        }),
      )}
      {columns.map((column, rowIndex) => {
        const parentRow = column.parentCheckpointId ? rowById.get(column.parentCheckpointId) : undefined;
        if (parentRow === undefined) return null;
        const x0 = historyLaneX(column.lane, laneCount, graphWidth);
        const x1 = historyLaneX(columns[parentRow]!.lane, laneCount, graphWidth);
        const y0 = rowIndex * rowHeight + rowHeight / 2;
        const y1 = parentRow * rowHeight + rowHeight / 2;
        if (x0 === x1) {
          return <line key={`${column.checkpointId}-stem`} x1={x0} y1={y0} x2={x1} y2={y1} stroke={HISTORY_GRAPH_STROKE} strokeWidth={1.5} />;
        }
        const elbowY = (rowIndex + 1) * rowHeight;
        return <path key={`${column.checkpointId}-stem`} d={`M ${x0} ${y0} L ${x0} ${elbowY} L ${x1} ${elbowY} L ${x1} ${y1}`} fill="none" stroke={HISTORY_GRAPH_STROKE} strokeWidth={1.5} />;
      })}
      {columns.map((column, rowIndex) => (
        <circle key={`${column.checkpointId}-node`} cx={historyLaneX(column.lane, laneCount, graphWidth)} cy={rowIndex * rowHeight + rowHeight / 2} r={3} fill="var(--foreground)" />
      ))}
    </svg>
  );
}

function HistoryRowAuthors({ column }: { readonly column: HistoryColumn }): React.ReactElement {
  return <div className="flex shrink-0 -space-x-2">{column.authors.length > 0 ? column.authors.map((author) => <TableAvatar key={author.id} id={author.id} name={author.name} icon={author.avatar} />) : <TableAvatar id="unknown" name="?" />}</div>;
}

function HistoryRowLabels({ column }: { readonly column: HistoryColumn }): React.ReactElement {
  return (
    <div className="flex min-w-0 flex-wrap items-center gap-1">
      {column.labels.length > 0 ? (
        column.labels.map((label) => (
          <span key={`${column.checkpointId}-${label}`} className="rounded px-1.5 py-0.5 text-2xs bg-[var(--accent)] text-[var(--accent-foreground)]">
            {label}
          </span>
        ))
      ) : (
        <span className="text-2xs text-muted-foreground">checkpoint</span>
      )}
    </div>
  );
}

/**
 * SVG ancestor-graph history table: swimlane guides, elbow connectors between forked checkpoints,
 * commit nodes, per-row author avatars and alternative-name label chips.
 **/
export const HistoryTable: React.FC<HistoryTableProps> = ({ id, columns, className, laneWidth, onSelectCheckpoint }) => {
  const sorted = React.useMemo(() => columns, [columns]);
  const laneCount = historyLaneCount(sorted);
  const graphWidth = historyGraphWidth(laneCount, laneWidth);
  const graphColumnWidth = historyGraphColumnWidth(laneCount, laneWidth);
  const rowProbeRef = React.useRef<HTMLDivElement>(null);
  const [rowHeight, setRowHeight] = React.useState(30);
  React.useLayoutEffect(() => {
    const height = rowProbeRef.current?.offsetHeight;
    if (height && height > 0) setRowHeight(height);
  }, [sorted.length]);
  const graphHeight = Math.max(sorted.length, 1) * rowHeight;
  return (
    <div id={id} className={cn("text-xs", className)}>
      {sorted.length === 0 ? (
        <div className="px-single py-single text-muted-foreground">—</div>
      ) : (
        <div
          className="relative grid min-w-0"
          style={{
            gridTemplateColumns: `auto ${graphColumnWidth}px minmax(0, 1fr)`,
            gridTemplateRows: `repeat(${sorted.length}, minmax(0, auto))`,
          }}
        >
          <div className="pointer-events-none relative" style={{ gridColumn: 2, gridRow: `1 / ${sorted.length + 1}`, height: graphHeight }}>
            <HistoryGraphSvg columns={sorted} graphWidth={graphWidth} rowHeight={rowHeight} />
          </div>
          {sorted.map((column, index) => {
            const nodeX = historyLaneX(column.lane, laneCount, graphWidth);
            const authorLeft = Math.max(nodeX - 12, 0);
            return (
              <React.Fragment key={column.checkpointId}>
                <div
                  ref={index === 0 ? rowProbeRef : undefined}
                  className={cn(HISTORY_ROW_SHELL_CLASS, "grid items-center border-b border-[var(--border)]", onSelectCheckpoint && "cursor-pointer")}
                  style={{ gridColumn: "1 / 3", gridRow: index + 1, gridTemplateColumns: `auto ${graphColumnWidth}px` }}
                  onClick={onSelectCheckpoint ? () => onSelectCheckpoint(column.checkpointId) : undefined}
                >
                  <div className="flex items-center px-single">
                    <HistoryRowLabels column={column} />
                  </div>
                  <div className="relative">
                    <div className={cn("pointer-events-none absolute top-1/2 h-px -translate-y-1/2", HISTORY_GRAPH_GUIDE_CLASS)} style={{ left: 0, width: authorLeft }} aria-hidden />
                    <div className="absolute top-1/2 -translate-y-1/2" style={{ left: authorLeft }}>
                      <HistoryRowAuthors column={column} />
                    </div>
                  </div>
                </div>
                <div className={cn(HISTORY_ROW_SHELL_CLASS, "flex items-center border-b border-[var(--border)] px-single")} style={{ gridColumn: 3, gridRow: index + 1 }}>
                  <span className="min-w-0 truncate text-muted-foreground" title={column.description ?? ""}>
                    {column.description ?? ""}
                  </span>
                </div>
              </React.Fragment>
            );
          })}
        </div>
      )}
    </div>
  );
};

HistoryTable.displayName = "HistoryTable";
// #endregion 🗄️HistoryTable
