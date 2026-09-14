/** 🌍 Geography: the TypeScript twin of `semio-viz-geo`. The fourteen registered projections with
 * d3-geo's rotate → raw → scale/translate pipeline and their inverses, the graticule generator, a
 * GeoJSON path renderer writing into the kernel's path context, and extent fitting.
 * @see ../../../🖋️latex/semio-viz-geo.sty
 */
import { vizPathRecorder, type VizPathCommand, type VizPathContext } from "../✒️mark/🟦️.ts";
import type { VizExtent, VizPoint, VizProjectionKind } from "../🧬️schema/🟦️.ts";

//#region 🔖️Spherical
const EPSILON = 1e-6;
const RADIANS = Math.PI / 180;
const DEGREES = 180 / Math.PI;
const HALF_PI = Math.PI / 2;
const TAU = 2 * Math.PI;

/** 🌐 A raw projection: radians in, unit-plane coordinates out, with its inverse. */
export type VizRawProjection = { (lambda: number, phi: number): VizPoint; invert?(x: number, y: number): VizPoint };

function asin(x: number): number {
  return x > 1 ? HALF_PI : x < -1 ? -HALF_PI : Math.asin(x);
}

function acos(x: number): number {
  return x > 1 ? 0 : x < -1 ? Math.PI : Math.acos(x);
}

function rotationLambda(deltaLambda: number): VizRawProjection {
  const forward = ((lambda: number, phi: number): VizPoint => {
    const l = lambda + deltaLambda;
    return [l > Math.PI ? l - TAU : l < -Math.PI ? l + TAU : l, phi];
  }) as VizRawProjection;
  forward.invert = (lambda: number, phi: number) => {
    const l = lambda - deltaLambda;
    return [l > Math.PI ? l - TAU : l < -Math.PI ? l + TAU : l, phi];
  };
  return forward;
}

function rotationPhiGamma(deltaPhi: number, deltaGamma: number): VizRawProjection {
  const cosDeltaPhi = Math.cos(deltaPhi);
  const sinDeltaPhi = Math.sin(deltaPhi);
  const cosDeltaGamma = Math.cos(deltaGamma);
  const sinDeltaGamma = Math.sin(deltaGamma);
  const forward = ((lambda: number, phi: number): VizPoint => {
    const cosPhi = Math.cos(phi);
    const x = Math.cos(lambda) * cosPhi;
    const y = Math.sin(lambda) * cosPhi;
    const z = Math.sin(phi);
    const k = z * cosDeltaPhi + x * sinDeltaPhi;
    return [Math.atan2(y * cosDeltaGamma - k * sinDeltaGamma, x * cosDeltaPhi - z * sinDeltaPhi), asin(k * cosDeltaGamma + y * sinDeltaGamma)];
  }) as VizRawProjection;
  forward.invert = (lambda: number, phi: number) => {
    const cosPhi = Math.cos(phi);
    const x = Math.cos(lambda) * cosPhi;
    const y = Math.sin(lambda) * cosPhi;
    const z = Math.sin(phi);
    const k = z * cosDeltaGamma - y * sinDeltaGamma;
    return [Math.atan2(y * cosDeltaGamma + z * sinDeltaGamma, x * cosDeltaPhi + k * sinDeltaPhi), asin(k * cosDeltaPhi - x * sinDeltaPhi)];
  };
  return forward;
}

const rotationIdentity = ((lambda: number, phi: number): VizPoint => [lambda > Math.PI ? lambda - TAU : lambda < -Math.PI ? lambda + TAU : lambda, phi]) as VizRawProjection;
rotationIdentity.invert = rotationIdentity as unknown as (x: number, y: number) => VizPoint;

function composeRaw(a: VizRawProjection, b: VizRawProjection): VizRawProjection {
  const forward = ((x: number, y: number): VizPoint => {
    const p = a(x, y);
    return b(p[0], p[1]);
  }) as VizRawProjection;
  if (a.invert && b.invert) {
    forward.invert = (x: number, y: number) => {
      const p = b.invert!(x, y);
      return a.invert!(p[0], p[1]);
    };
  }
  return forward;
}

/** 🌐 The three-angle rotation d3 applies before the raw projection. */
export function vizRotateRadians(deltaLambda: number, deltaPhi: number, deltaGamma: number): VizRawProjection {
  const l = deltaLambda % TAU;
  if (l) return deltaPhi || deltaGamma ? composeRaw(rotationLambda(l), rotationPhiGamma(deltaPhi, deltaGamma)) : rotationLambda(l);
  return deltaPhi || deltaGamma ? rotationPhiGamma(deltaPhi, deltaGamma) : rotationIdentity;
}

/** 🌐 Great-circle distance in radians between two `[lon, lat]` points. */
export function vizGeoDistance(a: VizPoint, b: VizPoint): number {
  const [lon0, lat0] = [a[0] * RADIANS, a[1] * RADIANS];
  const [lon1, lat1] = [b[0] * RADIANS, b[1] * RADIANS];
  const deltaLambda = lon1 - lon0;
  const cosPhi0 = Math.cos(lat0);
  const sinPhi0 = Math.sin(lat0);
  const cosPhi1 = Math.cos(lat1);
  const sinPhi1 = Math.sin(lat1);
  const x = cosPhi1 * Math.sin(deltaLambda);
  const y = cosPhi0 * sinPhi1 - sinPhi0 * cosPhi1 * Math.cos(deltaLambda);
  const z = sinPhi0 * sinPhi1 + cosPhi0 * cosPhi1 * Math.cos(deltaLambda);
  return Math.atan2(Math.sqrt(x * x + y * y), z);
}
//#endregion 🔖️Spherical

//#region 🔖️RawProjections
/** 🌐 `equirectangular`. */
export const rawEquirectangular = ((lambda: number, phi: number): VizPoint => [lambda, phi]) as VizRawProjection;
rawEquirectangular.invert = (x: number, y: number) => [x, y];

/** 🌐 `mercator`. */
export const rawMercator = ((lambda: number, phi: number): VizPoint => [lambda, Math.log(Math.tan((HALF_PI + phi) / 2))]) as VizRawProjection;
rawMercator.invert = (x: number, y: number) => [x, 2 * Math.atan(Math.exp(y)) - HALF_PI];

/** 🌐 `transverse-mercator`. */
export const rawTransverseMercator = ((lambda: number, phi: number): VizPoint => [Math.log(Math.tan((HALF_PI + phi) / 2)), -lambda]) as VizRawProjection;
rawTransverseMercator.invert = (x: number, y: number) => [-y, 2 * Math.atan(Math.exp(x)) - HALF_PI];

function azimuthalRaw(scale: (cxcy: number) => number): VizRawProjection {
  return ((lambda: number, phi: number): VizPoint => {
    const cx = Math.cos(lambda);
    const cy = Math.cos(phi);
    const k = scale(cx * cy);
    if (k === Number.POSITIVE_INFINITY) return [2, 0];
    return [k * cy * Math.sin(lambda), k * Math.sin(phi)];
  }) as VizRawProjection;
}

function azimuthalInvert(angle: (z: number) => number): (x: number, y: number) => VizPoint {
  return (x, y) => {
    const z = Math.sqrt(x * x + y * y);
    const c = angle(z);
    const sc = Math.sin(c);
    const cc = Math.cos(c);
    return [Math.atan2(x * sc, z * cc), asin(z && (y * sc) / z)];
  };
}

/** 🌐 `azimuthal-equal-area`. */
export const rawAzimuthalEqualArea = azimuthalRaw((cxcy) => Math.sqrt(2 / (1 + cxcy)));
rawAzimuthalEqualArea.invert = azimuthalInvert((z) => 2 * asin(z / 2));

/** 🌐 `azimuthal-equidistant`. */
export const rawAzimuthalEquidistant = azimuthalRaw((c) => {
  const a = acos(c);
  return a && a / Math.sin(a);
});
rawAzimuthalEquidistant.invert = azimuthalInvert((z) => z);

/** 🌐 `gnomonic`. */
export const rawGnomonic = ((lambda: number, phi: number): VizPoint => {
  const cy = Math.cos(phi);
  const k = Math.cos(lambda) * cy;
  return [(cy * Math.sin(lambda)) / k, Math.sin(phi) / k];
}) as VizRawProjection;
rawGnomonic.invert = azimuthalInvert(Math.atan);

/** 🌐 `orthographic`. */
export const rawOrthographic = ((lambda: number, phi: number): VizPoint => [Math.cos(phi) * Math.sin(lambda), Math.sin(phi)]) as VizRawProjection;
rawOrthographic.invert = azimuthalInvert(asin);

/** 🌐 `stereographic`. */
export const rawStereographic = ((lambda: number, phi: number): VizPoint => {
  const cy = Math.cos(phi);
  const k = 1 + Math.cos(lambda) * cy;
  return [(cy * Math.sin(lambda)) / k, Math.sin(phi) / k];
}) as VizRawProjection;
rawStereographic.invert = azimuthalInvert((z) => 2 * Math.atan(z));

function tany(y: number): number {
  return Math.tan((HALF_PI + y) / 2);
}

/** 🌐 `conic-conformal` at two standard parallels, in radians. */
export function rawConicConformal(y0: number, y1: number): VizRawProjection {
  const cy0 = Math.cos(y0);
  const n = y0 === y1 ? Math.sin(y0) : Math.log(cy0 / Math.cos(y1)) / Math.log(tany(y1) / tany(y0));
  const f = (cy0 * tany(y0) ** n) / n;
  if (!n) return rawMercator;
  const forward = ((lambda: number, phi: number): VizPoint => {
    let p = phi;
    if (f > 0) {
      if (p < -HALF_PI + EPSILON) p = -HALF_PI + EPSILON;
    } else if (p > HALF_PI - EPSILON) p = HALF_PI - EPSILON;
    const r = f / tany(p) ** n;
    return [r * Math.sin(n * lambda), f - r * Math.cos(n * lambda)];
  }) as VizRawProjection;
  forward.invert = (x: number, y: number) => {
    const fy = f - y;
    const sign = Math.sign(n) * Math.sqrt(x * x + fy * fy);
    let l = Math.atan2(x, Math.abs(fy)) * Math.sign(fy);
    if (fy * n < 0) l -= Math.PI * Math.sign(x) * Math.sign(fy);
    return [l / n, 2 * Math.atan((f / sign) ** (1 / n)) - HALF_PI];
  };
  return forward;
}

function rawCylindricalEqualArea(phi0: number): VizRawProjection {
  const cosPhi0 = Math.cos(phi0);
  const forward = ((lambda: number, phi: number): VizPoint => [lambda * cosPhi0, Math.sin(phi) / cosPhi0]) as VizRawProjection;
  forward.invert = (x: number, y: number) => [x / cosPhi0, asin(y * cosPhi0)];
  return forward;
}

/** 🌐 `conic-equal-area` at two standard parallels, in radians. */
export function rawConicEqualArea(y0: number, y1: number): VizRawProjection {
  const sy0 = Math.sin(y0);
  const n = (sy0 + Math.sin(y1)) / 2;
  if (Math.abs(n) < EPSILON) return rawCylindricalEqualArea(y0);
  const c = 1 + sy0 * (2 * n - sy0);
  const r0 = Math.sqrt(c) / n;
  const forward = ((lambda: number, phi: number): VizPoint => {
    const r = Math.sqrt(c - 2 * n * Math.sin(phi)) / n;
    const x = lambda * n;
    return [r * Math.sin(x), r0 - r * Math.cos(x)];
  }) as VizRawProjection;
  forward.invert = (x: number, y: number) => {
    const r0y = r0 - y;
    let l = Math.atan2(x, Math.abs(r0y)) * Math.sign(r0y);
    if (r0y * n < 0) l -= Math.PI * Math.sign(x) * Math.sign(r0y);
    return [l / n, asin((c - (x * x + r0y * r0y) * n * n) / (2 * n))];
  };
  return forward;
}

/** 🌐 `conic-equidistant` at two standard parallels, in radians. */
export function rawConicEquidistant(y0: number, y1: number): VizRawProjection {
  const cy0 = Math.cos(y0);
  const n = y0 === y1 ? Math.sin(y0) : (cy0 - Math.cos(y1)) / (y1 - y0);
  const g = cy0 / n + y0;
  if (Math.abs(n) < EPSILON) return rawEquirectangular;
  const forward = ((lambda: number, phi: number): VizPoint => {
    const gy = g - phi;
    const nx = n * lambda;
    return [gy * Math.sin(nx), g - gy * Math.cos(nx)];
  }) as VizRawProjection;
  forward.invert = (x: number, y: number) => {
    const gy = g - y;
    let l = Math.atan2(x, Math.abs(gy)) * Math.sign(gy);
    if (gy * n < 0) l -= Math.PI * Math.sign(x) * Math.sign(gy);
    return [l / n, g - Math.sign(n) * Math.sqrt(x * x + gy * gy)];
  };
  return forward;
}

const EE_A1 = 1.340264;
const EE_A2 = -0.081106;
const EE_A3 = 0.000893;
const EE_A4 = 0.003796;
const EE_M = Math.sqrt(3) / 2;

/** 🌐 `equal-earth`. */
export const rawEqualEarth = ((lambda: number, phi: number): VizPoint => {
  const l = asin(EE_M * Math.sin(phi));
  const l2 = l * l;
  const l6 = l2 * l2 * l2;
  return [(lambda * Math.cos(l)) / (EE_M * (EE_A1 + 3 * EE_A2 * l2 + l6 * (7 * EE_A3 + 9 * EE_A4 * l2))), l * (EE_A1 + EE_A2 * l2 + l6 * (EE_A3 + EE_A4 * l2))];
}) as VizRawProjection;
rawEqualEarth.invert = (x: number, y: number) => {
  let l = y;
  let l2 = l * l;
  let l6 = l2 * l2 * l2;
  for (let i = 0; i < 12; i += 1) {
    const fy = l * (EE_A1 + EE_A2 * l2 + l6 * (EE_A3 + EE_A4 * l2)) - y;
    const fpy = EE_A1 + 3 * EE_A2 * l2 + l6 * (7 * EE_A3 + 9 * EE_A4 * l2);
    l -= fy / fpy;
    l2 = l * l;
    l6 = l2 * l2 * l2;
    if (Math.abs(fy / fpy) < 1e-12) break;
  }
  return [(EE_M * x * (EE_A1 + 3 * EE_A2 * l2 + l6 * (7 * EE_A3 + 9 * EE_A4 * l2))) / Math.cos(l), asin(Math.sin(l) / EE_M)];
};

/** 🌐 `natural-earth`. */
export const rawNaturalEarth = ((lambda: number, phi: number): VizPoint => {
  const phi2 = phi * phi;
  const phi4 = phi2 * phi2;
  return [lambda * (0.8707 - 0.131979 * phi2 + phi4 * (-0.013791 + phi4 * (0.003971 * phi2 - 0.001529 * phi4))), phi * (1.007226 + phi2 * (0.015085 + phi4 * (-0.044475 + 0.028874 * phi2 - 0.005916 * phi4)))];
}) as VizRawProjection;
//#endregion 🔖️RawProjections

//#region 🔖️Projection
/** 🌐 A projection: the whole rotate → raw → scale/translate pipeline, and its inverse. */
export type VizProjection = {
  (point: VizPoint): VizPoint;
  invert(point: VizPoint): VizPoint | null;
  scale(): number;
  translate(): VizPoint;
  rotate(): readonly [number, number, number];
  center(): VizPoint;
  withScale(k: number): VizProjection;
  withTranslate(t: VizPoint): VizProjection;
  withRotate(r: readonly [number, number, number?]): VizProjection;
  withCenter(c: VizPoint): VizProjection;
};

/** 🌐 Settings of a projection, all of them optional and all documented by their d3 defaults. */
export type VizProjectionSettings = { readonly scale?: number; readonly translate?: VizPoint; readonly rotate?: readonly [number, number, number?]; readonly center?: VizPoint; readonly reflectX?: boolean; readonly reflectY?: boolean };

/** 🌐 Wraps a raw projection into the full pipeline, exactly like d3's `projection()`. */
export function vizProjection(raw: VizRawProjection, settings: VizProjectionSettings = {}): VizProjection {
  const k = settings.scale ?? 150;
  const [tx, ty] = settings.translate ?? [480, 250];
  const [rl, rp, rg] = settings.rotate ?? [0, 0, 0];
  const [cl, cp] = settings.center ?? [0, 0];
  const sx = settings.reflectX === true ? -1 : 1;
  const sy = settings.reflectY === true ? -1 : 1;
  const rotate = vizRotateRadians(rl * RADIANS, rp * RADIANS, (rg ?? 0) * RADIANS);
  const rotated = composeRaw(rotate, raw);
  const centred = raw(cl * RADIANS, cp * RADIANS);
  const dx = tx - k * sx * centred[0];
  const dy = ty + k * sy * centred[1];
  const projection = ((point: VizPoint): VizPoint => {
    const p = rotated(point[0] * RADIANS, point[1] * RADIANS);
    return [dx + k * sx * p[0], dy - k * sy * p[1]];
  }) as VizProjection;
  projection.invert = (point: VizPoint) => {
    if (rotated.invert === undefined) return null;
    const p = rotated.invert(((point[0] - dx) / k) * sx, ((dy - point[1]) / k) * sy);
    return p === null ? null : [p[0] * DEGREES, p[1] * DEGREES];
  };
  projection.scale = () => k;
  projection.translate = () => [tx, ty];
  projection.rotate = () => [rl, rp, rg ?? 0];
  projection.center = () => [cl, cp];
  projection.withScale = (scale) => vizProjection(raw, { ...settings, scale });
  projection.withTranslate = (translate) => vizProjection(raw, { ...settings, translate });
  projection.withRotate = (rotation) => vizProjection(raw, { ...settings, rotate: rotation });
  projection.withCenter = (center) => vizProjection(raw, { ...settings, center });
  return projection;
}

/** 🌐 Options one of the fourteen registered projections accepts. */
export type VizProjectionOptions = VizProjectionSettings & { readonly parallels?: readonly [number, number] };

/** 🌐 The default scale, rotation and centre d3 gives each registered projection. */
const PROJECTION_DEFAULTS: Readonly<Record<VizProjectionKind, VizProjectionSettings & { parallels?: readonly [number, number] }>> = {
  equirectangular: { scale: 152.63 },
  mercator: { scale: 961 / TAU },
  "transverse-mercator": { scale: 159.155, rotate: [0, 0, 90] },
  "azimuthal-equal-area": { scale: 124.75 },
  "azimuthal-equidistant": { scale: 79.4188 },
  gnomonic: { scale: 144.049 },
  orthographic: { scale: 249.5 },
  stereographic: { scale: 250 },
  "conic-conformal": { scale: 109.5, parallels: [30, 30] },
  "conic-equal-area": { scale: 155.424, center: [0, 33.6442], parallels: [0, 60] },
  "conic-equidistant": { scale: 131.154, center: [0, 13.9389], parallels: [0, 60] },
  albers: { scale: 1070, translate: [480, 250], rotate: [96, 0], center: [-0.6, 38.7], parallels: [29.5, 45.5] },
  "equal-earth": { scale: 177.158 },
  "natural-earth": { scale: 175.295 },
};

/** 🌍 `\SemioVizCoordinate{geographic}`: builds one of the fourteen registered projections. */
export function vizGeoProjection(kind: VizProjectionKind, options: VizProjectionOptions = {}): VizProjection {
  const defaults = PROJECTION_DEFAULTS[kind];
  const parallels = options.parallels ?? defaults.parallels ?? [0, 0];
  const settings: VizProjectionSettings = { ...defaults, ...options };
  switch (kind) {
    case "mercator":
      return vizProjection(rawMercator, settings);
    case "transverse-mercator":
      return vizProjection(rawTransverseMercator, settings);
    case "azimuthal-equal-area":
      return vizProjection(rawAzimuthalEqualArea, settings);
    case "azimuthal-equidistant":
      return vizProjection(rawAzimuthalEquidistant, settings);
    case "gnomonic":
      return vizProjection(rawGnomonic, settings);
    case "orthographic":
      return vizProjection(rawOrthographic, settings);
    case "stereographic":
      return vizProjection(rawStereographic, settings);
    case "conic-conformal":
      return vizProjection(rawConicConformal(parallels[0] * RADIANS, parallels[1] * RADIANS), settings);
    case "conic-equal-area":
    case "albers":
      return vizProjection(rawConicEqualArea(parallels[0] * RADIANS, parallels[1] * RADIANS), settings);
    case "conic-equidistant":
      return vizProjection(rawConicEquidistant(parallels[0] * RADIANS, parallels[1] * RADIANS), settings);
    case "equal-earth":
      return vizProjection(rawEqualEarth, settings);
    case "natural-earth":
      return vizProjection(rawNaturalEarth, settings);
    default:
      return vizProjection(rawEquirectangular, settings);
  }
}
//#endregion 🔖️Projection

//#region 🔖️Path
/** 🗺️ The GeoJSON subset the print kernel draws. */
export type VizGeoGeometry =
  | { readonly type: "Point"; readonly coordinates: VizPoint }
  | { readonly type: "MultiPoint"; readonly coordinates: readonly VizPoint[] }
  | { readonly type: "LineString"; readonly coordinates: readonly VizPoint[] }
  | { readonly type: "MultiLineString"; readonly coordinates: readonly (readonly VizPoint[])[] }
  | { readonly type: "Polygon"; readonly coordinates: readonly (readonly VizPoint[])[] }
  | { readonly type: "MultiPolygon"; readonly coordinates: readonly (readonly (readonly VizPoint[])[])[] }
  | { readonly type: "GeometryCollection"; readonly geometries: readonly VizGeoGeometry[] }
  | { readonly type: "Feature"; readonly geometry: VizGeoGeometry }
  | { readonly type: "FeatureCollection"; readonly features: readonly { readonly geometry: VizGeoGeometry }[] };

function eachRing(geometry: VizGeoGeometry, visit: (points: readonly VizPoint[], closed: boolean) => void): void {
  switch (geometry.type) {
    case "Point":
      visit([geometry.coordinates], false);
      return;
    case "MultiPoint":
      for (const point of geometry.coordinates) visit([point], false);
      return;
    case "LineString":
      visit(geometry.coordinates, false);
      return;
    case "MultiLineString":
      for (const line of geometry.coordinates) visit(line, false);
      return;
    case "Polygon":
      for (const ring of geometry.coordinates) visit(ring, true);
      return;
    case "MultiPolygon":
      for (const polygon of geometry.coordinates) for (const ring of polygon) visit(ring, true);
      return;
    case "GeometryCollection":
      for (const child of geometry.geometries) eachRing(child, visit);
      return;
    case "Feature":
      eachRing(geometry.geometry, visit);
      return;
    default:
      for (const feature of geometry.features) eachRing(feature.geometry, visit);
  }
}

/** 🗺️ `\SemioVizGeoPath`: renders a GeoJSON geometry through a projection into path commands.
 * Point geometries are drawn as circles of `pointRadius`, exactly as d3's `geoPath` does. */
export function vizGeoPath(geometry: VizGeoGeometry, projection: VizProjection, options: { pointRadius?: number } = {}, context?: VizPathContext): VizPathCommand[] {
  const recorder = context === undefined ? vizPathRecorder() : undefined;
  const sink = context ?? recorder!;
  const pointRadius = options.pointRadius ?? 4.5;
  const isPointOnly = geometry.type === "Point" || geometry.type === "MultiPoint";
  eachRing(geometry, (points, closed) => {
    if (isPointOnly) {
      for (const point of points) {
        const p = projection(point);
        sink.moveTo(p[0] + pointRadius, p[1]);
        sink.arc(p[0], p[1], pointRadius, 0, TAU);
      }
      return;
    }
    const projected = points.map((point) => projection(point));
    const drawn = closed && projected.length > 1 && projected[0]![0] === projected[projected.length - 1]![0] && projected[0]![1] === projected[projected.length - 1]![1] ? projected.slice(0, -1) : projected;
    drawn.forEach((point, i) => {
      if (i === 0) sink.moveTo(point[0], point[1]);
      else sink.lineTo(point[0], point[1]);
    });
    if (closed) sink.closePath();
  });
  return recorder?.commands ?? [];
}

/** 🗺️ The planar bounding box of a projected geometry. */
export function vizGeoBounds(geometry: VizGeoGeometry, projection: VizProjection): VizExtent {
  let x0 = Number.POSITIVE_INFINITY;
  let y0 = Number.POSITIVE_INFINITY;
  let x1 = Number.NEGATIVE_INFINITY;
  let y1 = Number.NEGATIVE_INFINITY;
  eachRing(geometry, (points) => {
    for (const point of points) {
      const [x, y] = projection(point);
      if (x < x0) x0 = x;
      if (x > x1) x1 = x;
      if (y < y0) y0 = y;
      if (y > y1) y1 = y;
    }
  });
  return { x0, y0, x1, y1 };
}

/** 🗺️ The planar area of a projected geometry, in square figure millimetres. */
export function vizGeoArea(geometry: VizGeoGeometry, projection: VizProjection): number {
  let total = 0;
  eachRing(geometry, (points, closed) => {
    if (!closed) return;
    const projected = points.map((point) => projection(point));
    let area = 0;
    for (let i = 0, n = projected.length; i < n; i += 1) {
      const a = projected[i]!;
      const b = projected[(i + 1) % n]!;
      area += a[0] * b[1] - b[0] * a[1];
    }
    total += area / 2;
  });
  return Math.abs(total);
}

/** 🗺️ The planar centroid of a projected geometry. */
export function vizGeoCentroid(geometry: VizGeoGeometry, projection: VizProjection): VizPoint {
  let cx = 0;
  let cy = 0;
  let weight = 0;
  eachRing(geometry, (points, closed) => {
    const projected = points.map((point) => projection(point));
    if (!closed) {
      for (const point of projected) {
        cx += point[0];
        cy += point[1];
        weight += 1;
      }
      return;
    }
    let area = 0;
    let sx = 0;
    let sy = 0;
    for (let i = 0, n = projected.length; i < n; i += 1) {
      const a = projected[i]!;
      const b = projected[(i + 1) % n]!;
      const cross = a[0] * b[1] - b[0] * a[1];
      area += cross;
      sx += (a[0] + b[0]) * cross;
      sy += (a[1] + b[1]) * cross;
    }
    if (area !== 0) {
      cx += sx / 3;
      cy += sy / 3;
      weight += area;
    }
  });
  return weight === 0 ? [Number.NaN, Number.NaN] : [cx / weight, cy / weight];
}
//#endregion 🔖️Path

//#region 🔖️Graticule
function stepRange(start: number, stop: number, step: number): number[] {
  const out: number[] = [];
  if (!(step > 0)) return out;
  const n = Math.max(0, Math.ceil((stop - start) / step));
  for (let i = 0; i < n; i += 1) out.push(start + i * step);
  return out;
}

/** 🌐 Options of `\SemioVizGeoGraticule`, with d3's defaults. */
export type VizGraticuleOptions = { readonly extent?: readonly [VizPoint, VizPoint]; readonly step?: readonly [number, number]; readonly stepMajor?: readonly [number, number]; readonly precision?: number };

/** 🌐 The meridian and parallel grid, as a `MultiLineString`. */
export function vizGraticule(options: VizGraticuleOptions = {}): { readonly type: "MultiLineString"; readonly coordinates: VizPoint[][] } {
  const [[x0, y0], [x1, y1]] = options.extent ?? [
    [-180, -90 + EPSILON],
    [180, 90 - EPSILON],
  ];
  const [dx, dy] = options.step ?? [10, 10];
  const [bigX, bigY] = options.stepMajor ?? [90, 360];
  const precision = options.precision ?? 2.5;
  const alongY = (from: number, to: number, step: number) => (x: number): VizPoint[] => [...stepRange(from, to - EPSILON, step), to].map((y) => [x, y] as VizPoint);
  const alongX = (from: number, to: number, step: number) => (y: number): VizPoint[] => [...stepRange(from, to - EPSILON, step), to].map((x) => [x, y] as VizPoint);
  const majorMeridian = alongY(y0, y1, 90);
  const majorParallel = alongX(x0, x1, precision);
  const minorMeridian = alongY(y0, y1, 90);
  const minorParallel = alongX(x0, x1, precision);
  return {
    type: "MultiLineString",
    coordinates: [
      ...stepRange(Math.ceil(x0 / bigX) * bigX, x1, bigX).map(majorMeridian),
      ...stepRange(Math.ceil(y0 / bigY) * bigY, y1, bigY).map(majorParallel),
      ...stepRange(Math.ceil(x0 / dx) * dx, x1, dx)
        .filter((x) => Math.abs(x % bigX) > EPSILON)
        .map(minorMeridian),
      ...stepRange(Math.ceil(y0 / dy) * dy, y1, dy)
        .filter((y) => Math.abs(y % bigY) > EPSILON)
        .map(minorParallel),
    ],
  };
}
//#endregion 🔖️Graticule

//#region 🔖️Fit
/** 🗺️ Rescales and recentres a projection so a geometry exactly fills an extent. */
export function vizGeoFitExtent(projection: VizProjection, extent: readonly [VizPoint, VizPoint], geometry: VizGeoGeometry, raw: VizRawProjection, settings: VizProjectionSettings = {}): VizProjection {
  const unit = vizProjection(raw, { ...settings, scale: 150, translate: [0, 0] });
  const bounds = vizGeoBounds(geometry, unit);
  const width = extent[1][0] - extent[0][0];
  const height = extent[1][1] - extent[0][1];
  const k = Math.min(width / (bounds.x1 - bounds.x0), height / (bounds.y1 - bounds.y0));
  const x = extent[0][0] + (width - k * (bounds.x1 + bounds.x0)) / 2;
  const y = extent[0][1] + (height - k * (bounds.y1 + bounds.y0)) / 2;
  void projection;
  return vizProjection(raw, { ...settings, scale: 150 * k, translate: [x, y] });
}

/** 🗺️ Fits a geometry into a `width × height` box anchored at the origin. */
export function vizGeoFitSize(projection: VizProjection, size: readonly [number, number], geometry: VizGeoGeometry, raw: VizRawProjection, settings: VizProjectionSettings = {}): VizProjection {
  return vizGeoFitExtent(
    projection,
    [
      [0, 0],
      [size[0], size[1]],
    ],
    geometry,
    raw,
    settings,
  );
}
//#endregion 🔖️Fit
