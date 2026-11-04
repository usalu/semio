/** 👆️ A long press: holding one pointer still for `holdMs` fires once; moving beyond `slopPx`, releasing early or cancelling disarms it. */
export type LongPressV1 = Readonly<{ down: (x: number, y: number) => void; move: (x: number, y: number) => void; up: () => void; cancel: () => void }>;

export function createLongPressV1(options: Readonly<{ holdMs: number; slopPx: number; setTimer: (callback: () => void, milliseconds: number) => () => void; onLongPress: (x: number, y: number) => void }>): LongPressV1 {
  let origin: { x: number; y: number } | null = null;
  let disarm: (() => void) | null = null;
  const clear = (): void => {
    disarm?.();
    disarm = null;
    origin = null;
  };
  return {
    down: (x, y) => {
      clear();
      origin = { x, y };
      disarm = options.setTimer(() => {
        const at = origin;
        clear();
        if (at !== null) options.onLongPress(at.x, at.y);
      }, options.holdMs);
    },
    move: (x, y) => {
      if (origin !== null && Math.hypot(x - origin.x, y - origin.y) > options.slopPx) clear();
    },
    up: clear,
    cancel: clear,
  };
}
