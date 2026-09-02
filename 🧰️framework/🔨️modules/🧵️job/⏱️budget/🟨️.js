//! ⏱️ JavaScript implementation of the shared monotonic clock conversion schema.

//#region ⏱️Clock
export function microsecondsFromMilliseconds(milliseconds) {
  const microseconds = milliseconds * 1_000;
  return Number.isFinite(microseconds) && microseconds >= 0 && microseconds < 18_446_744_073_709_551_616 ? BigInt(Math.floor(microseconds)) : null;
}

//#endregion ⏱️Clock
