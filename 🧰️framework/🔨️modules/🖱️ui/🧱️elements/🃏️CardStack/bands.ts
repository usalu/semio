/** 📏️ Chips of one row that are closer than this (but do not abut) would read as two lines with a sliver between them, so they stretch to abut instead. */
export const CARD_BAND_MIN_GAP = 40;
const ABUT = 0.25;

/** 🫸️ Whether two chips of one row separated by `gap` px read as two lines with a small gap: closer than {@link CARD_BAND_MIN_GAP} but not abutting. Chip rows of a frame then let the chips abut and share one seam.
 *
 * @see ./bands.cases.json */
export const chipGapReadsAsSliver = (gap: number): boolean => gap > ABUT && gap < CARD_BAND_MIN_GAP;

/** 📐️ The smallest height at or above `height` that is a whole number of device pixels (`pixel` is one device pixel in CSS px), so a card's top and bottom lines never fall between pixels and the line of the abutting neighbour coincides with it.
 *
 * @see ./bands.cases.json */
export const snapCardHeight = (height: number, pixel: number): number => Math.ceil(height / pixel - 1e-4) * pixel;
