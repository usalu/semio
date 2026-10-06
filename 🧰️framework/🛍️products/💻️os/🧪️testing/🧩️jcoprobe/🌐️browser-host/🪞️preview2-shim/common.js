export const MAX_U64 = (1n << 64n) - 1n;
/** Validate and return a WebAssembly u64 value. */
export function checkedU64(value, name) {
    if (typeof value !== "bigint" || value < 0n || value > MAX_U64) {
        throw new TypeError(`${name} must be a valid u64`);
    }
    return value;
}
/** Convert a WebAssembly u64 value to a lossless JavaScript number. */
export function checkedU64AsNumber(value, name) {
    checkedU64(value, name);
    if (value > BigInt(Number.MAX_SAFE_INTEGER)) {
        throw new RangeError(`${name} exceeds JavaScript's safe integer range`);
    }
    return Number(value);
}
