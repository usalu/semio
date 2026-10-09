//! ⏱️ JavaScript implementation of the shared monotonic clock conversion schema.

//#region ⏱️Clock
export function microsecondsFromMilliseconds(milliseconds) {
  const microseconds = milliseconds * 1_000;
  return Number.isFinite(microseconds) && microseconds >= 0 && microseconds < 18_446_744_073_709_551_616 ? BigInt(Math.floor(microseconds)) : null;
}

//#endregion ⏱️Clock


const retainedAxes = ["maximumItems", "maximumCopyBytes", "maximumCapacityBytes", "maximumReleaseBytes", "maximumDepth"];
const retainedReceiptAxes = ["copiedItems", "copiedBytes", "retainedCapacityBytes", "releasedBytes"];
function retainedNumbers(value, axes) {
  if (value === null || typeof value !== "object" || Object.keys(value).length !== axes.length || axes.some(axis => !Object.hasOwn(value, axis) || !Number.isSafeInteger(value[axis]) || value[axis] < 0)) throw new RangeError("original retained authority has malformed independent axes");
}
/** 🎟️ Quotes independently unspent credits of the same original job step authority. */
export function remainingRetainedGrant(grant, progress) {
  retainedNumbers(grant, retainedAxes);retainedNumbers(progress, retainedReceiptAxes);
  const remaining = {...grant};
  for (let index = 0; index < retainedReceiptAxes.length; index++) {
    const axis = retainedAxes[index], actual = progress[retainedReceiptAxes[index]];
    remaining[axis] = Math.max(0,grant[axis]-actual);
  }
  return remaining;
}
/** 🧾️ Records actual child effects in the original recipient before refusing exceeded authority. */
export function consumeRetainedProgress(grant, previous, actual) {
  const remaining = remainingRetainedGrant(grant, previous);retainedNumbers(actual, retainedReceiptAxes);
  let admitted = true;const next = {};
  for (let index = 0; index < retainedReceiptAxes.length; index++) {
    const axis = retainedReceiptAxes[index],authority = retainedAxes[index];
    admitted &&= previous[axis] <= grant[authority] && actual[axis] <= remaining[authority];
    next[axis] = previous[axis]+actual[axis];
    if (!Number.isSafeInteger(next[axis])) { const error=new RangeError("actual job receipt exceeds exact address space");error.retainedProgress=actual;throw error; }
  }
  Object.assign(previous,next);
  if (!admitted) { const error=new RangeError("actual job receipt exceeds original retained authority");error.retainedProgress=actual;throw error; }
  return previous;
}
