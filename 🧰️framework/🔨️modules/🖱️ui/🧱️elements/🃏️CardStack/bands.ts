/** 📐️ The smallest height at or above `height` that is a whole number of device pixels (`pixel` is one device pixel in CSS px), so a card's top and bottom lines never fall between pixels and the line of the abutting neighbour coincides with it.
 *
 * @see ./bands.cases.json */
export const snapCardHeight = (height: number, pixel: number): number => Math.ceil(height / pixel - 1e-4) * pixel;
