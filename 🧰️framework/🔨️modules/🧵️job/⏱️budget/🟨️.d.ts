/** ⏱️ Types for the JavaScript twin of the shared monotonic clock conversion schema (`🟨️.js`).
 *
 * The implementation is authored in plain JavaScript on purpose — it is the byte-identical oracle the
 * Rust and TypeScript clocks are checked against, and it must load in a bare browser page with no
 * transform step. This sibling declaration is how the `.js` module enters a `allowJs: false` program.
 */

/** ⏱️ Converts milliseconds to whole microseconds, or `null` when the value is not a finite unsigned 64-bit count. */
export function microsecondsFromMilliseconds(milliseconds: number): bigint | null;

/** 🎟️ Mandatory original normal job memory authority, independent of fuel and deadline. */
export interface JobRetainedGrant { maximumItems:number; maximumCopyBytes:number; maximumCapacityBytes:number; maximumReleaseBytes:number; maximumDepth:number; }
/** 🧾️ Actual child receipts accepted by the same original job step. */
export interface JobRetainedProgress { copiedItems:number; copiedBytes:number; retainedCapacityBytes:number; releasedBytes:number; }
export function remainingRetainedGrant(grant:JobRetainedGrant,progress:JobRetainedProgress):JobRetainedGrant;
/** ⚠️ Failed actual ingress carries its original physical delta beside the mutated recipient. */
export interface JobRetainedIngressError extends RangeError {retainedProgress:JobRetainedProgress;}
export function consumeRetainedProgress(grant:JobRetainedGrant,previous:JobRetainedProgress,actual:JobRetainedProgress):JobRetainedProgress;
