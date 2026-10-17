/** 📊️ Declares the real argument of the `d3-shape` stack order functions.
 *
 * `d3-shape` calls every `stackOrder*` function with the whole array of series (`d3.stack()` hands them its
 * `series` array), but `@types/d3-shape` types the parameter as one `Series`. Independent stacking oracles
 * call the order functions directly, so the array form is declared here as an overload.
 *
 * @see https://d3js.org/d3-shape/stack#stackOrderAppearance
 */
import type { Series } from "d3-shape";

declare module "d3-shape" {
  function stackOrderAppearance<Datum, Key>(series: readonly Series<Datum, Key>[]): number[];
  function stackOrderAscending<Datum, Key>(series: readonly Series<Datum, Key>[]): number[];
  function stackOrderDescending<Datum, Key>(series: readonly Series<Datum, Key>[]): number[];
  function stackOrderInsideOut<Datum, Key>(series: readonly Series<Datum, Key>[]): number[];
  function stackOrderNone<Datum, Key>(series: readonly Series<Datum, Key>[]): number[];
  function stackOrderReverse<Datum, Key>(series: readonly Series<Datum, Key>[]): number[];
}
