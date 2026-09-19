/** ⏱️ Types for the JavaScript twin of the shared monotonic clock conversion schema (`🟨️.js`).
 *
 * The implementation is authored in plain JavaScript on purpose — it is the byte-identical oracle the
 * Rust and TypeScript clocks are checked against, and it must load in a bare browser page with no
 * transform step. This sibling declaration is how the `.js` module enters a `allowJs: false` program.
 */

/** ⏱️ Converts milliseconds to whole microseconds, or `null` when the value is not a finite unsigned 64-bit count. */
export function microsecondsFromMilliseconds(milliseconds: number): bigint | null;
