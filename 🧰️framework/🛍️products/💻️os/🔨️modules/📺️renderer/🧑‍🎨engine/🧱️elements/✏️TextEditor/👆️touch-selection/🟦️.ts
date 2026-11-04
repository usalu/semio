/** 👆️ Touch word selection for the text editor: a long press, or a second tap close in time and place, selects the word under the finger. Mouse and pen keep their own paths (`detail >= 2`). */
export const TOUCH_LONG_PRESS_MS_V1 = 500;
export const TOUCH_DOUBLE_TAP_MS_V1 = 350;
export const TOUCH_SLOP_PX_V1 = 10;

export type TouchSelectionOptionsV1 = Readonly<{
  now: () => number;
  setTimer: (callback: () => void, milliseconds: number) => () => void;
  selectWord: (x: number, y: number) => void;
}>;

export type TouchSelectionV1 = Readonly<{
  down: (x: number, y: number) => void;
  move: (x: number, y: number) => void;
  up: () => void;
  cancel: () => void;
}>;

/** 👆️ The recognizer: `down` arms a long press; `move` beyond the slop (a scroll or drag) or `cancel` disarms it; `up` before the timer is a tap, and a tap near the previous tap in time and place selects the word. Returns whether a down was consumed as a selection by the callback only. */
export function createTouchSelectionV1(options: TouchSelectionOptionsV1): TouchSelectionV1 {
  let origin: { x: number; y: number } | null = null;
  let disarm: (() => void) | null = null;
  let fired = false;
  let lastTap: { x: number; y: number; at: number } | null = null;
  const clear = (): void => {
    disarm?.();
    disarm = null;
  };
  const far = (x: number, y: number, from: { x: number; y: number }): boolean => Math.hypot(x - from.x, y - from.y) > TOUCH_SLOP_PX_V1;
  return {
    down: (x, y) => {
      clear();
      fired = false;
      origin = { x, y };
      disarm = options.setTimer(() => {
        fired = true;
        lastTap = null;
        if (origin !== null) options.selectWord(origin.x, origin.y);
      }, TOUCH_LONG_PRESS_MS_V1);
    },
    move: (x, y) => {
      if (origin !== null && far(x, y, origin)) {
        clear();
        origin = null;
      }
    },
    up: () => {
      clear();
      if (origin === null || fired) {
        origin = null;
        return;
      }
      const at = options.now();
      if (lastTap !== null && at - lastTap.at <= TOUCH_DOUBLE_TAP_MS_V1 && !far(origin.x, origin.y, lastTap)) {
        options.selectWord(origin.x, origin.y);
        lastTap = null;
      } else {
        lastTap = { ...origin, at };
      }
      origin = null;
    },
    cancel: () => {
      clear();
      origin = null;
    },
  };
}
