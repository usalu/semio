/** 📏️ A horizontal extent of a chip, in any one coordinate space shared by the cards of a stack. */
export interface CardBandSpan {
  readonly left: number;
  readonly right: number;
}

/** 🃏️ What the interlock of a stack needs to know about one card: the natural heights of its title-chip row and of its
 * footer-chip row (0 without footer chips) and the extents of the chips in those rows. */
export interface CardBandCard {
  readonly cap: number;
  readonly footer: number;
  readonly capChips: readonly CardBandSpan[];
  readonly footerChips: readonly CardBandSpan[];
}

/** 🪢️ How a card meets the one before it: `none` (no footer chip above, the cap row starts on the bottom line), `shared`
 * (the footer chips and the cap chips use one band of `band` px, the card is lifted by `lift`) or `stacked` (they would
 * collide, so the footer row is followed by the cap row, each in its own band, nothing overlaps). `cap` is the minimum
 * height of the lower card's cap row and `footer` that of the upper card's footer row. */
export interface CardBand {
  readonly mode: "none" | "shared" | "stacked";
  readonly lift: number;
  readonly cap: number;
  readonly footer: number;
}

/** 📏️ Chips of one band that are closer than this (but do not abut) would read as two lines with a gap between them, so they are stacked instead. */
export const CARD_BAND_MIN_GAP = 40;
const ABUT = 0.25;

const touch = (a: CardBandSpan, b: CardBandSpan): boolean => {
  const gap = Math.max(a.left - b.right, b.left - a.right);
  return gap < -1e-6 || (gap > ABUT && gap < CARD_BAND_MIN_GAP);
};

/** 🧮️ Resolves how each card of a stack meets its predecessor: entry `i` describes card `i + 1` against card `i`. Pure and
 * language-agnostic (see `bands.cases.json`): the interlocked band is as high as the taller of the footer row above and the
 * cap row below, so the lower card's cap row starts on the upper card's bottom line and its footer chip row ends on the
 * lower card's top line; chips that would meet or nearly meet (a gap under {@link CARD_BAND_MIN_GAP}, abutting chips excepted) in that band make it two bands instead.
 *
 * @see ./bands.cases.json */
export function resolveCardBands(cards: readonly CardBandCard[]): readonly CardBand[] {
  const bands: CardBand[] = [];
  for (let index = 1; index < cards.length; index++) {
    const above = cards[index - 1]!, below = cards[index]!;
    if (above.footer <= 0) bands.push({ mode: "none", lift: 0, cap: 0, footer: 0 });
    else if (above.footerChips.some(footer => below.capChips.some(cap => touch(footer, cap)))) bands.push({ mode: "stacked", lift: 0, cap: 0, footer: 0 });
    else {
      const band = Math.max(above.footer, below.cap);
      bands.push({ mode: "shared", lift: band, cap: band, footer: band });
    }
  }
  return bands;
}
