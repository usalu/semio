/** 🖱️ Gives a mouse press back to the element under it. three's OrbitControls captures the pointer on its event source, which retargets the trailing pointerup and click to that source and starves every host listener below it (instance picks, the host's own release handler); its move and up listeners live on the document, so the capture is not needed. Touch and pen keep their implicit capture. */
export function releaseWorldOrbitMouseCapture(target: Pick<Element, "hasPointerCapture" | "releasePointerCapture">, event: Pick<PointerEvent, "pointerId" | "pointerType">): boolean {
  if (event.pointerType !== "mouse" || !target.hasPointerCapture(event.pointerId)) return false;
  target.releasePointerCapture(event.pointerId);
  return true;
}
