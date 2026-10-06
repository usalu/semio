import { checkedU64 } from "./common.js";
import { pollableCreate } from "./io.js";
const MAX_TIMEOUT_MS = 0x7fffffff;
function timeout(durationNs) {
    let remainingMs = Number((durationNs + 999999n) / 1000000n);
    return new Promise((resolve) => {
        const next = () => {
            if (remainingMs <= 0) {
                resolve();
                return;
            }
            const delay = Math.min(remainingMs, MAX_TIMEOUT_MS);
            remainingMs -= delay;
            setTimeout(next, delay);
        };
        next();
    });
}
export const monotonicClock = {
    resolution() {
        // usually we dont get sub-millisecond accuracy in the browser
        // Note: is there a better way to determine this?
        return BigInt(1e6);
    },
    now() {
        // performance.now() is in milliseconds, but we want nanoseconds
        return BigInt(Math.floor(performance.now() * 1e6));
    },
    subscribeInstant(instant) {
        instant = checkedU64(instant, "instant");
        const now = monotonicClock.now();
        if (instant <= now) {
            return pollableCreate();
        }
        return monotonicClock.subscribeDuration(instant - now);
    },
    subscribeDuration(duration) {
        duration = checkedU64(duration, "duration");
        if (duration === 0n) {
            return pollableCreate();
        }
        return pollableCreate(timeout(duration));
    },
};
export const wallClock = {
    now() {
        let now = Date.now(); // in milliseconds
        const seconds = BigInt(Math.floor(now / 1e3));
        const nanoseconds = (now % 1e3) * 1e6;
        return { seconds, nanoseconds };
    },
    resolution() {
        return { seconds: 0n, nanoseconds: 1e6 };
    },
};
