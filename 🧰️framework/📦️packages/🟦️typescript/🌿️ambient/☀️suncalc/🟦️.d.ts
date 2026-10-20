/** ☀️ Surface of `suncalc` used by the independent sun-path oracles.
 *
 * `suncalc` ships without types and `@types/suncalc` is not part of the workspace, so the one position call the oracles make is declared here. Delete this file the moment the package gains types.
 *
 * @see https://github.com/mourner/suncalc#reference
 */
declare module "suncalc" {
  interface SunPosition {
    readonly altitude: number;
    readonly azimuth: number;
  }
  const SunCalc: {
    getPosition(date: Date, latitude: number, longitude: number): SunPosition;
  };
  export default SunCalc;
}
