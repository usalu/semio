/** 📈️ Surface of `d3-regression` used by the independent regression oracles.
 *
 * `d3-regression` ships without types and `@types/d3-regression` is not part of the workspace, so the five fitting generators the oracles call are declared here. Delete this file the moment the package gains types.
 *
 * @see https://github.com/HarryStevens/d3-regression#readme
 */
declare module "d3-regression" {
  type Accessor<Datum> = (datum: Datum, index: number, data: readonly Datum[]) => number;
  interface Fit extends Array<[number, number]> {
    readonly rSquared: number;
    predict(x: number): number;
  }
  interface TwoTermFit extends Fit {
    readonly a: number;
    readonly b: number;
  }
  interface PolynomialFit extends Fit {
    readonly coefficients: readonly number[];
  }
  interface Generator<Datum, Result> {
    (data: readonly Datum[]): Result;
    x(): Accessor<Datum>;
    x(accessor: Accessor<Datum>): this;
    y(): Accessor<Datum>;
    y(accessor: Accessor<Datum>): this;
    domain(): [number, number] | undefined;
    domain(domain: [number, number] | undefined): this;
  }
  interface PolynomialGenerator<Datum> extends Generator<Datum, PolynomialFit> {
    order(): number;
    order(order: number): this;
  }
  export function regressionLinear<Datum = [number, number]>(): Generator<Datum, TwoTermFit>;
  export function regressionLog<Datum = [number, number]>(): Generator<Datum, TwoTermFit>;
  export function regressionPow<Datum = [number, number]>(): Generator<Datum, TwoTermFit>;
  export function regressionExp<Datum = [number, number]>(): Generator<Datum, TwoTermFit>;
  export function regressionPoly<Datum = [number, number]>(): PolynomialGenerator<Datum>;
}
