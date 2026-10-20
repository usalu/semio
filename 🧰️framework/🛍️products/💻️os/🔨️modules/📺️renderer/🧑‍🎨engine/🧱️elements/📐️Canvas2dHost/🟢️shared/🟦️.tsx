/** 🟢️ The shared declarations of Canvas2dHost. */
// 🐢️ `CAMERA_SYNC_DEBOUNCE_MS` is also referenced (unqualified) by `World3dHost`'s still-barrel-resident
// camera-dispatch debounce — exported so that outside reference keeps resolving after this extraction.
export const CAMERA_SYNC_DEBOUNCE_MS = 120;

export const DRAG_OVER_THROTTLE_MS = 50;

export const DRAG_OVER_THROTTLE_DISTANCE = 4;

export function clientPointOverRect(clientX: number, clientY: number, rect: DOMRect): boolean {
  return clientX >= rect.left && clientX <= rect.right && clientY >= rect.top && clientY <= rect.bottom;
}
