/** 🧬️ TsvSnapshot schema facet — mirrors 🦀️.rs field-for-field. IANA TSV has no
 * quoting/escaping; `records` is a raw row grid with no header/data structural distinction. */

export type TsvLineEnding = 'lf' | 'crlf';

export interface TsvSnapshot {
  schema: string;
  records: string[][];
  trailingNewline: boolean;
  lineEnding: TsvLineEnding;
}
