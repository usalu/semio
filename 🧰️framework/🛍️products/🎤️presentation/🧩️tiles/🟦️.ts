/** 🧩️ Presentation tiles: tiling figures, splitting a figure grid into tiles and the tile play that seeds, moves and resizes tile drafts and builds their morph prompt.
 *
 * @see ../🎭️embodiments/🟦️.ts — the figure embodiment
 * @see ../🧱️vocabulary/🟦️.ts — the dispositions
 * @see ../🟦️.ts — the package entry that publishes this module
 */
import type { ParticipantEmphasis, Participant, DispositionPosition, DispositionStyle, Disposition } from "../🧱️vocabulary/🟦️.ts";
import type { FigureMosaicGrid, FigureEmbodiment } from "../🎭️embodiments/🟦️.ts";

//#region 🔖️Tile
/** 📕️ Spec for {@link tile}: one cropped figure embodiment from a source image. */
export interface TileSpec {
	readonly id: string;
	readonly source: string;
	readonly crop: DispositionPosition;
	readonly alt?: string;
	readonly sourceAspect?: number;
	readonly mosaic?: FigureMosaicGrid;
}

/** 🀄️ Produces one cropped {@link FigureEmbodiment} from a source figure. */
export function tile(spec: TileSpec): FigureEmbodiment {
	return {
		kind: "figure",
		id: spec.id,
		src: spec.source,
		alt: spec.alt,
		crop: spec.crop,
		...(spec.sourceAspect !== undefined ? { sourceAspect: spec.sourceAspect } : {}),
		...(spec.mosaic !== undefined ? { mosaic: spec.mosaic } : {}),
	};
}
//#endregion 🔖️Tile

//#region 🔖️Split
/** 📐️ One grid cell placement and source crop (internal to {@link splitFigureGrid}). */
export interface SplitGridCell {
	readonly key: string;
	readonly crop: DispositionPosition;
	readonly position: DispositionPosition;
	readonly emphasis?: ParticipantEmphasis;
	readonly style?: DispositionStyle;
}

/** 📏️ Spec for {@link splitFigureGrid}: uniform rows×columns inside a slide frame. */
export interface SplitFigureGridSpec {
	readonly rows: number;
	readonly columns: number;
	readonly frame: DispositionPosition;
	readonly gap?: number;
	readonly emphasis?: ParticipantEmphasis;
	readonly keyPrefix?: string;
}

/** ✂️ Builds grid cells that pack a figure into a normalized slide frame (gap=0 reconstructs the frame). */
export function splitFigureGrid(spec: SplitFigureGridSpec): SplitGridCell[] {
	const { rows, columns, frame, gap = 0, emphasis, keyPrefix = "tile" } = spec;
	if (rows < 1 || columns < 1) {
		throw new Error(`splitFigureGrid requires rows and columns >= 1 (got ${rows}×${columns}).`);
	}
	const cellWidth = (frame.width - gap * (columns - 1)) / columns;
	const cellHeight = (frame.height - gap * (rows - 1)) / rows;
	const cells: SplitGridCell[] = [];
	const cropWidth = frame.width / columns;
	const cropHeight = frame.height / rows;
	for (let row = 0; row < rows; row += 1) {
		for (let column = 0; column < columns; column += 1) {
			cells.push({
				key: `${keyPrefix}-r${row}-c${column}`,
				crop: {
					x: frame.x + column * cropWidth,
					y: frame.y + row * cropHeight,
					width: cropWidth,
					height: cropHeight,
				},
				position: {
					x: frame.x + column * (cellWidth + gap),
					y: frame.y + row * (cellHeight + gap),
					width: cellWidth,
					height: cellHeight,
				},
				...(emphasis ? { emphasis } : {}),
			});
		}
	}
	return cells;
}

/** 💿️ Union of normalized source-image crops. */
export function unionSourceCrops(crops: readonly DispositionPosition[]): DispositionPosition {
	if (crops.length === 0) {
		throw new Error("unionSourceCrops: no crops.");
	}
	let minX = 1;
	let minY = 1;
	let maxX = 0;
	let maxY = 0;
	for (const crop of crops) {
		minX = Math.min(minX, crop.x);
		minY = Math.min(minY, crop.y);
		maxX = Math.max(maxX, crop.x + crop.width);
		maxY = Math.max(maxY, crop.y + crop.height);
	}
	return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}

/** 📔️ Spec for {@link split}: grid of tile participants, embodiments, and dispositions. */
export interface SplitSpec {
	readonly source: string;
	readonly rows: number;
	readonly columns: number;
	readonly frame: DispositionPosition;
	readonly gap?: number;
	readonly emphasis?: ParticipantEmphasis;
	readonly keyPrefix?: string;
	readonly alt?: string;
	readonly embodimentIdSuffix?: string;
	readonly sourceAspect?: number;
}

/** 🏺️ Artifacts produced by the split template (one participant and disposition per grid cell). */
export interface SplitArtifacts {
	readonly participants: readonly Participant[];
	readonly embodiments: readonly FigureEmbodiment[];
	readonly dispositions: readonly Disposition[];
}

/** 🏠️ Produces a grid of tile figure embodiments with one disposition per cell. */
export function split(spec: SplitSpec): SplitArtifacts {
	const suffix = spec.embodimentIdSuffix ?? "figure";
	const cells = splitFigureGrid({
		rows: spec.rows,
		columns: spec.columns,
		frame: spec.frame,
		gap: spec.gap,
		emphasis: spec.emphasis,
		keyPrefix: spec.keyPrefix,
	});
	const participants: Participant[] = [];
	const embodiments: FigureEmbodiment[] = [];
	const dispositions: Disposition[] = [];
	for (const cell of cells) {
		const participantId = cell.key;
		const embodimentId = `${participantId}-${suffix}`;
		participants.push({ id: participantId });
		embodiments.push(
			tile({
				id: embodimentId,
				source: spec.source,
				crop: cell.crop,
				alt: spec.alt,
				sourceAspect: spec.sourceAspect,
				mosaic: { rows: spec.rows, columns: spec.columns, frame: spec.frame },
			}),
		);
		dispositions.push({
			participantId,
			embodimentId,
			emphasis: cell.emphasis ?? spec.emphasis ?? "active",
			position: cell.position,
			...(cell.style ? { style: cell.style } : {}),
		});
	}
	return { participants, embodiments, dispositions };
}

/** 🔪️ Replaces slide positions on split dispositions matched by participant id. */
export function remapSplitDispositions(
	dispositions: readonly Disposition[],
	positionsByParticipantId: Readonly<Record<string, DispositionPosition>>,
): Disposition[] {
	return dispositions.map((disposition) => {
		const position = positionsByParticipantId[disposition.participantId];
		return position ? { ...disposition, position } : disposition;
	});
}
//#endregion 🔖️Split

//#region 🔖️TilePlay
/** 📙️ One editable source-image crop for the presentation tile play (overlap allowed). */
export interface FigureTileDraft {
	readonly id: string;
	readonly name: string;
	readonly crop: DispositionPosition;
}

/** 🖼️ Source figure loaded in the tile play (normalized {@link DispositionPosition.frame}). */
export type FigureTileMediaKind = "figure" | "video" | "pdf";

/** 📄️ Default PDF page width÷height (ISO A4 at 72 dpi) for tile play aspect before page probe. */
export const FIGURE_TILE_PDF_PAGE_ASPECT = 595 / 842;

export interface FigureTileSource {
	readonly src: string;
	readonly kind?: FigureTileMediaKind;
	readonly sourceAspect?: number;
	readonly frame: DispositionPosition;
	readonly pdfPage?: number;
}

/** 🔎️ Resolves tile-play media kind from a browser file MIME type and optional name. */
export function figureTileMediaKindFromFile(mime: string, fileName = ""): FigureTileMediaKind | null {
	const normalized = mime.toLowerCase().split(";")[0]?.trim() ?? "";
	if (normalized.startsWith("video/")) {
		return "video";
	}
	if (normalized === "application/pdf") {
		return "pdf";
	}
	if (normalized.startsWith("image/")) {
		return "figure";
	}
	const ext = fileName.split(".").pop()?.toLowerCase() ?? "";
	if (ext === "pdf") {
		return "pdf";
	}
	if (["mp4", "webm", "ogg", "ogv", "mov", "m4v", "mkv"].includes(ext)) {
		return "video";
	}
	if (["png", "jpg", "jpeg", "gif", "webp", "svg", "bmp", "avif", "ico", "tif", "tiff"].includes(ext)) {
		return "figure";
	}
	return null;
}

/** ✅️ True when the file is supported by presentation tile play. */
export function isFigureTileMediaFile(mime: string, fileName = ""): boolean {
	return figureTileMediaKindFromFile(mime, fileName) !== null;
}

/** ⊡ Eight resize handles for {@link resizeNormalizedRect}. */
export type NormalizedRectHandle = "nw" | "n" | "ne" | "e" | "se" | "s" | "sw" | "w";

export const NORMALIZED_RECT_MIN_FRACTION = 0.02;

/** 🌱️ Spec for {@link populateTileDraftsFromGrid}. */
export interface FigureTileGridSeedSpec {
	readonly source: FigureTileSource;
	readonly rows: number;
	readonly columns: number;
	readonly gap?: number;
	readonly keyPrefix?: string;
}

/** 🔢️ Clamps a unit-interval scalar. */
export function clampNormalizedFraction(value: number): number {
	return Math.min(1, Math.max(0, value));
}

/** ↔ Moves a normalized rectangle by fractional deltas (overlap allowed). */
export function moveNormalizedRect(
	rect: DispositionPosition,
	dx: number,
	dy: number,
): DispositionPosition {
	let x = clampNormalizedFraction(rect.x + dx);
	let y = clampNormalizedFraction(rect.y + dy);
	if (x + rect.width > 1) {
		x = 1 - rect.width;
	}
	if (y + rect.height > 1) {
		y = 1 - rect.height;
	}
	return { x, y, width: rect.width, height: rect.height };
}

/** 🧲️ Resizes a normalized rectangle from one handle by fractional deltas. */
export function resizeNormalizedRect(
	rect: DispositionPosition,
	handle: NormalizedRectHandle,
	dx: number,
	dy: number,
	minSize: number = NORMALIZED_RECT_MIN_FRACTION,
): DispositionPosition {
	let { x, y, width, height } = rect;
	if (handle.includes("e")) {
		width += dx;
	}
	if (handle.includes("w")) {
		x += dx;
		width -= dx;
	}
	if (handle.includes("s")) {
		height += dy;
	}
	if (handle.includes("n")) {
		y += dy;
		height -= dy;
	}
	width = Math.max(minSize, width);
	height = Math.max(minSize, height);
	x = clampNormalizedFraction(x);
	y = clampNormalizedFraction(y);
	if (x + width > 1) {
		width = 1 - x;
	}
	if (y + height > 1) {
		height = 1 - y;
	}
	width = Math.max(minSize, width);
	height = Math.max(minSize, height);
	return { x, y, width, height };
}

/** 🔠️ Parses grid engagement tokens such as `3x5` or `3×5`. */
export function parseGridEngagement(text: string): { readonly rows: number; readonly columns: number } | null {
	const match = text.trim().match(/^(\d+)\s*[x×]\s*(\d+)$/iu);
	if (!match) {
		return null;
	}
	const rows = Number.parseInt(match[1]!, 10);
	const columns = Number.parseInt(match[2]!, 10);
	if (!Number.isFinite(rows) || !Number.isFinite(columns) || rows < 1 || columns < 1) {
		return null;
	}
	return { rows, columns };
}

/** 🚧️ Seeds {@link FigureTileDraft} rows from {@link splitFigureGrid} inside {@link FigureTileSource.frame}. */
export function populateTileDraftsFromGrid(spec: FigureTileGridSeedSpec): FigureTileDraft[] {
	const cells = splitFigureGrid({
		rows: spec.rows,
		columns: spec.columns,
		frame: spec.source.frame,
		gap: spec.gap,
		keyPrefix: spec.keyPrefix,
	});
	return cells.map((cell) => ({
		id: cell.key,
		name: cell.key,
		crop: cell.crop,
	}));
}

function formatTilePlayDispositionPosition(position: DispositionPosition): string {
	return `{ x: ${position.x.toFixed(6)}, y: ${position.y.toFixed(6)}, width: ${position.width.toFixed(6)}, height: ${position.height.toFixed(6)} }`;
}

/** 📋️ Builds a natural-language LLM prompt embedding tile morph parameters for deck authoring. */
export function buildTileMorphPrompt(source: FigureTileSource, drafts: readonly FigureTileDraft[]): string {
	const kind = source.kind ?? "figure";
	const lines: string[] = [
		"Wire a one-to-many morph for `@semio-tech/framework-presentation-core` using the parameters below.",
		"",
		"## Source media",
		`- kind: ${kind}`,
		`- src: ${JSON.stringify(source.src)}`,
		...(source.sourceAspect !== undefined ? [`- sourceAspect: ${source.sourceAspect}`] : []),
		...(kind === "pdf" && source.pdfPage !== undefined ? [`- pdfPage: ${source.pdfPage}`] : []),
		`- frame: ${formatTilePlayDispositionPosition(source.frame)}`,
		"",
		"## Tiles (normalized source crops; overlap allowed)",
	];
	for (const draft of drafts) {
		lines.push(`- ${draft.name} (${draft.id}): crop ${formatTilePlayDispositionPosition(draft.crop)}`);
	}
	const embodimentHint =
		kind === "video"
			? "Use `video(...)` embodiments for tile participants and the source clip."
			: kind === "pdf"
				? "Use `pdf(...)` embodiments for tile participants and the source document page."
				: "Register one participant per tile with a `tile(...)` figure embodiment using each crop above.";
	lines.push(
		"",
		"## Task",
		`1. ${embodimentHint}`,
		"2. On the source slide, place the full media with `morphTo: MorphToSlot[]` pointing at each tile participant (use each tile crop region as the ghost source position during the morph).",
		"3. Follow the catalogue pattern in `mit-bestand/präsentation/33.projektetage/spec.ts` (`split`, `MorphToSlot`, `DispositionPosition`, semantic participant ids).",
		"4. Use reveal.js auto-animate; morph from the actual disposition including ephemeral modifications.",
	);
	return lines.join("\n");
}
//#endregion 🔖️TilePlay
