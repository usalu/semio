//! 🌀️ Analytic and free-form 3D curves ([`Curve3`]) and their 2D parameter-space counterparts
//! ([`Curve2`], the pcurve representation). Every variant supports position/derivative evaluation
//! and an *exact* [`Curve3::to_nurbs`]/[`Curve2::to_nurbs`] conversion — the single representation
//! every downstream algorithm (intersection, tessellation, STEP export) can fall back to when it
//! doesn't have an analytic fast path for a particular curve kind.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/➰️curve` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, together with
//! its `🪢️bspline`/`🎢️bezier`/`✂️curve-ops` siblings as local children (per this file's own
//! pre-mounted-stub note — no 1:1 file mapping), mirroring the `⚙️engine` → `🟫️step`/`📦️mesh-io`
//! local-mount pattern from wave PEEL3.
//!
//! Crate cut of `semio-framework-3d-brep-representation` (wave E4 source admission ceiling, `📓️2026-10-14-wave-e4-3d-architect-bim.md`).

extern crate semio_framework_value_derive as value_derive;

// #region 🔖️Submodules

#[path = "🎢️bezier/🦀️.rs"]
pub mod bezier;
#[path = "🪢️bspline/🦀️.rs"]
pub mod bspline;
#[path = "✂️curve-ops/🦀️.rs"]
pub mod curve_ops;

// #endregion 🔖️Submodules

use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, Vec2, Vec3};
use bspline::{curve_derivatives_rational, de_boor, KnotVector};

// #region 🔖️Curve3

/// 🌀️ A 3D curve. Each variant's *natural* domain is documented on the variant; a curve's actual
/// used range (e.g. an edge's `(t0, t1)`) is stored by the topology layer, not here — this keeps
/// geometry shareable between edges that trim the same underlying curve differently.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(crate = "::semio_framework_value")]
pub enum Curve3 {
    /// 🌀️ `origin + t·dir`. Natural domain `(-∞, ∞)`. `dir` need not be unit.
    Line { origin: Pnt3, dir: Vec3 },
    /// 🌀️ A full circle in `frame`'s xy-plane. Natural domain `[0, 2π)`, periodic.
    Circle { frame: Frame3, radius: f64 },
    /// 🌀️ A full ellipse in `frame`'s xy-plane (`frame.x` = major axis, `frame.y` = minor axis).
    /// Natural domain `[0, 2π)`, periodic.
    Ellipse { frame: Frame3, major_radius: f64, minor_radius: f64 },
    /// 🌀️ A rational B-spline curve. Natural domain = the knot vector's domain.
    Nurbs { knots: KnotVector, controls: Vec<Pnt3>, weights: Vec<f64> },
}


/// 🌀️ An explicit rational-NURBS representation, returned by [`Curve3::to_nurbs`] /
/// [`Curve2::to_nurbs`] — every curve kind's common denominator.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct NurbsCurve3 {
    pub knots: KnotVector,
    pub controls: Vec<Pnt3>,
    pub weights: Vec<f64>,
}

impl Curve3 {
    pub fn domain(&self) -> (f64, f64) {
        match self {
            Curve3::Line { .. } => (f64::NEG_INFINITY, f64::INFINITY),
            Curve3::Circle { .. } | Curve3::Ellipse { .. } => (0.0, std::f64::consts::TAU),
            Curve3::Nurbs { knots, .. } => knots.domain(),
        }
    }
    pub fn is_periodic(&self) -> bool {
        match self {
            Curve3::Circle { .. } | Curve3::Ellipse { .. } => true,
            Curve3::Nurbs { knots, .. } => knots.is_periodic(),
            Curve3::Line { .. } => false,
        }
    }
    pub fn period(&self) -> Option<f64> {
        match self {
            Curve3::Circle { .. } | Curve3::Ellipse { .. } => Some(std::f64::consts::TAU),
            Curve3::Nurbs { knots, .. } if knots.is_periodic() => {
                let (lo, hi) = knots.domain();
                Some(hi - lo)
            }
            _ => None,
        }
    }
    pub fn eval(&self, t: f64) -> Pnt3 {
        match self {
            Curve3::Line { origin, dir } => *origin + *dir * t,
            Curve3::Circle { frame, radius } => frame.to_world(Pnt3::new(radius * t.cos(), radius * t.sin(), 0.0)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => frame.to_world(Pnt3::new(major_radius * t.cos(), minor_radius * t.sin(), 0.0)),
            Curve3::Nurbs { knots, controls, weights } => eval_nurbs_curve(knots, controls, weights, t),
        }
    }
    /// 🧮️ Samples a position and up to two derivatives with bounded original scratch.
    pub fn jet_original(&self, t:f64, order:usize) -> Option<(Pnt3,Vec3,Vec3)> {
        if !t.is_finite() || order>2 { return None; }
        if let Self::Nurbs{knots,controls,weights}=self {
            if controls.len()!=weights.len() { return None; }
            let homogeneous=bspline::homogeneous_jet_original(knots,controls.len(),t,order,|i|{
                let p=controls.get(i)?;
                let w=*weights.get(i)?;
                Some([p.x*w,p.y*w,p.z*w,w])
            })?;
            let [p,d,dd]=bspline::project_original_jet(homogeneous,order)?;
            return Some((Pnt3::new(p[0],p[1],p[2]),Vec3::new(d[0],d[1],d[2]),Vec3::new(dd[0],dd[1],dd[2])));
        }
        let position=self.eval(t);
        let tangent=if order>0 { self.d1(t) } else { Vec3::ZERO };
        let acceleration=if order>1 { self.d2(t) } else { Vec3::ZERO };
        [position.x,position.y,position.z,tangent.x,tangent.y,tangent.z,acceleration.x,acceleration.y,acceleration.z]
            .iter().all(|v|v.is_finite()).then_some((position,tangent,acceleration))
    }


    /// 📐️ Gives the finite extent of a standalone spline's nonempty knot intervals.
    pub fn original_spline_span_range(&self) -> Option<(usize,usize)> {
        let Self::Nurbs{knots,controls,weights}=self else {return None;};
        let count=controls.len();
        let degree=knots.degree;
        (degree<=bspline::ORIGINAL_SPLINE_MAXIMUM_DEGREE && count>degree && weights.len()==count &&
         knots.knots.len()==count.checked_add(degree)?.checked_add(1)?).then_some((degree,count))
    }

    /// 🎚️ Samples inside one authored span, preserving both limits at repeated knots.
    pub fn original_spline_span_point(&self,span:usize,t:f64) -> Option<Pnt3> {
        let Self::Nurbs{knots,controls,weights}=self else {return None;};
        let h=bspline::homogeneous_jet_at_span_original(knots,controls.len(),t,0,span,|i|{
            let p=controls.get(i)?;
            let w=*weights.get(i)?;
            Some([p.x*w,p.y*w,p.z*w,w])
        })?;
        let [p,_,_]=bspline::project_original_jet(h,0)?;
        Some(Pnt3::new(p[0],p[1],p[2]))
    }

    /// 📏️ Bounds chord error with local homogeneous derivative control hulls.
    pub fn original_spline_subdivision(&self, span:usize, tolerance:f64) -> Option<(f64,f64,usize)> {
        let Self::Nurbs{knots,controls,weights}=self else {return None;};
        let (degree,end)=self.original_spline_span_range()?;
        if !(degree..end).contains(&span) || !tolerance.is_finite() || tolerance<=0.0 {return None;}
        let u=&knots.knots;
        let (lo,hi)=(u[span],u[span+1]);
        let width=hi-lo;
        if !width.is_finite() || width<0.0 {return None;}
        if width==0.0 {return Some((lo,hi,0));}
        let start=span-degree;
        let anchor=controls[start];
        let local_weights=&weights[start..=span];
        if local_weights.iter().any(|v|!v.is_finite() || *v<=0.0) {return None;}
        let scale=local_weights.iter().copied().fold(0.0,f64::max);
        let w_min=local_weights.iter().copied().fold(f64::INFINITY,f64::min)/scale;
        if w_min<=0.0 {return None;}
        let mut entries=[(Vec3::ZERO,0.0);bspline::ORIGINAL_SPLINE_MAXIMUM_DEGREE+1];
        let mut radius=0.0_f64;
        let mut h=[0.0_f64;3];
        let mut w=[0.0_f64;3];
        for (i,entry) in entries.iter_mut().take(degree+1).enumerate() {
            let d=controls[start+i]-anchor;
            let weight=local_weights[i]/scale;
            radius=radius.max(d.x.hypot(d.y).hypot(d.z));
            *entry=(d*weight,weight);
        }
        for order in 1..=2.min(degree) {
            let length=degree+1-order;
            for j in 0..length {
                let divisor=u[start+j+degree+1]-u[start+j+order];
                if !divisor.is_finite() || divisor<0.0 {return None;}
                let factor=if divisor==0.0 {0.0} else {length as f64 / divisor};
                let v=(entries[j+1].0-entries[j].0)*factor;
                let dw=(entries[j+1].1-entries[j].1)*factor;
                if ![v.x,v.y,v.z,dw].iter().all(|v|v.is_finite()){return None;}
                entries[j]=(v,dw);
                h[order]=h[order].max(v.x.hypot(v.y).hypot(v.z));
                w[order]=w[order].max(dw.abs());
            }
        }
        let speed=(h[1]+radius*w[1])/w_min;
        let acceleration=(h[2]+radius*w[2]+2.0*speed*w[1])/w_min;
        let segments=width*(acceleration/(8.0*tolerance)).sqrt();
        if !segments.is_finite() {return None;}
        Some((lo,hi,(segments*(1.0+1e-12)).ceil().max(1.0) as usize))
    }

    /// 🌀️ First derivative `dC/dt`.
    pub fn d1(&self, t: f64) -> Vec3 {
        match self {
            Curve3::Line { dir, .. } => *dir,
            Curve3::Circle { frame, radius } => frame.to_world_vector(Vec3::new(-radius * t.sin(), radius * t.cos(), 0.0)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => frame.to_world_vector(Vec3::new(-major_radius * t.sin(), minor_radius * t.cos(), 0.0)),
            Curve3::Nurbs { .. } => self.derivatives(t, 1)[1],
        }
    }
    /// 🌀️ Second derivative `d²C/dt²`.
    pub fn d2(&self, t: f64) -> Vec3 {
        match self {
            Curve3::Line { .. } => Vec3::ZERO,
            Curve3::Circle { frame, radius } => frame.to_world_vector(Vec3::new(-radius * t.cos(), -radius * t.sin(), 0.0)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => frame.to_world_vector(Vec3::new(-major_radius * t.cos(), -minor_radius * t.sin(), 0.0)),
            Curve3::Nurbs { .. } => self.derivatives(t, 2)[2],
        }
    }
    /// 🌀️ All derivatives `d^0C/dt^0 .. d^orderC/dt^order` (index 0 = position) in one pass — exact
    /// through the rational de Boor `A_k(u)` recurrence for [`Curve3::Nurbs`] (any `order`, not
    /// just 1/2), closed-form for the analytic kinds. `d1`/`d2` are thin wrappers over this.
    pub fn derivatives(&self, t: f64, order: usize) -> Vec<Vec3> {
        match self {
            Curve3::Nurbs { knots, controls, weights } => {
                let controls_h: Vec<Vec<f64>> = controls.iter().zip(weights).map(|(p, &w)| vec![p.x * w, p.y * w, p.z * w, w]).collect();
                curve_derivatives_rational(knots, &controls_h, t, order).into_iter().map(|v| Vec3::new(v[0], v[1], v[2])).collect()
            }
            _ => (0..=order)
                .map(|k| match k {
                    0 => self.eval(t).to_vec(),
                    1 => self.d1_analytic(t),
                    2 => self.d2_analytic(t),
                    _ => Vec3::ZERO,
                })
                .collect(),
        }
    }
    /// 🌀️ First derivative for the non-`Nurbs` (analytic) variants only — factored out of
    /// [`Self::d1`]/[`Self::derivatives`] so the latter can call it without recursing back into
    /// itself for the `Nurbs` case.
    fn d1_analytic(&self, t: f64) -> Vec3 {
        match self {
            Curve3::Line { dir, .. } => *dir,
            Curve3::Circle { frame, radius } => frame.to_world_vector(Vec3::new(-radius * t.sin(), radius * t.cos(), 0.0)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => frame.to_world_vector(Vec3::new(-major_radius * t.sin(), minor_radius * t.cos(), 0.0)),
            Curve3::Nurbs { .. } => Vec3::ZERO,
        }
    }
    /// 🌀️ Second derivative for the non-`Nurbs` (analytic) variants only, see [`Self::d1_analytic`].
    fn d2_analytic(&self, t: f64) -> Vec3 {
        match self {
            Curve3::Line { .. } => Vec3::ZERO,
            Curve3::Circle { frame, radius } => frame.to_world_vector(Vec3::new(-radius * t.cos(), -radius * t.sin(), 0.0)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => frame.to_world_vector(Vec3::new(-major_radius * t.cos(), -minor_radius * t.sin(), 0.0)),
            Curve3::Nurbs { .. } => Vec3::ZERO,
        }
    }
    pub fn tangent(&self, t: f64) -> Option<Vec3> {
        self.d1(t).normalized()
    }
    /// 🌀️ Signed curvature magnitude `|C' × C''| / |C'|³` (the standard space-curve formula).
    pub fn curvature(&self, t: f64) -> f64 {
        let d1 = self.d1(t);
        let d2 = self.d2(t);
        let speed = d1.norm();
        if speed <= f64::EPSILON {
            return 0.0;
        }
        d1.cross(d2).norm() / speed.powi(3)
    }
    /// 🌀️ An exact rational-NURBS representation over `domain` (required for [`Curve3::Line`],
    /// whose natural domain is unbounded). Arcs longer than 120° are split into equal spans of
    /// the standard rational-quadratic circular-arc construction for numerical conditioning.
    /// The returned curve traces exactly the same points over `domain` and agrees with `self` at
    /// `domain.0`/`domain.1` and every span breakpoint in between — but, as for any rational
    /// quadratic circle/ellipse representation, its *own* parametrization is not angle-linear
    /// except at those breakpoints (a well-known property of the construction, not an
    /// approximation: every point it produces still lies exactly on the circle/ellipse).
    pub fn to_nurbs(&self, domain: (f64, f64)) -> NurbsCurve3 {
        match self {
            Curve3::Line { origin, dir } => {
                let p0 = *origin + *dir * domain.0;
                let p1 = *origin + *dir * domain.1;
                NurbsCurve3 { knots: KnotVector::new(vec![domain.0, domain.0, domain.1, domain.1], 1, 2).unwrap(), controls: vec![p0, p1], weights: vec![1.0, 1.0] }
            }
            Curve3::Circle { frame, radius } => arc_to_nurbs(frame, *radius, *radius, domain),
            Curve3::Ellipse { frame, major_radius, minor_radius } => arc_to_nurbs(frame, *major_radius, *minor_radius, domain),
            Curve3::Nurbs { knots, controls, weights } => NurbsCurve3 { knots: knots.clone(), controls: controls.clone(), weights: weights.clone() },
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn eval_nurbs_curve(knots: &KnotVector, controls: &[Pnt3], weights: &[f64], t: f64) -> Pnt3 {
    let hx: Vec<f64> = controls.iter().zip(weights).map(|(p, w)| p.x * w).collect();
    let hy: Vec<f64> = controls.iter().zip(weights).map(|(p, w)| p.y * w).collect();
    let hz: Vec<f64> = controls.iter().zip(weights).map(|(p, w)| p.z * w).collect();
    let w = de_boor(knots, weights, t);
    Pnt3::new(de_boor(knots, &hx, t) / w, de_boor(knots, &hy, t) / w, de_boor(knots, &hz, t) / w)
}

/// 🌀️ Converts a circular/elliptical arc over `domain` into an exact rational-quadratic NURBS,
/// splitting into `⌈span / 120°⌉` equal-angle spans (the standard well-conditioned construction:
/// each span's middle control point sits at `radius / cos(half-span)` with weight `cos(half-span)`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn arc_to_nurbs(frame: &Frame3, radius_x: f64, radius_y: f64, domain: (f64, f64)) -> NurbsCurve3 {
    arc_to_nurbs_with_span(frame, radius_x, radius_y, domain, std::f64::consts::TAU / 3.0)
}

/// 🌀️ The exact conic-arc-to-NURBS construction a rational quadratic Bezier span reproduces the
/// circle/ellipse's OWN shape exactly, but — since `cos`/`sin` are transcendental — its parameter
/// is a Möbius (tan-half-angle) reparametrization of angle, not angle itself: a span's error away
/// from the angle-linear point is `O(half_span³)` (peaks at `≈ 0.0321·radius·half_span³`, derived
/// via a series expansion of the closed-form rational Bezier around `half_span = 0`, verified
/// numerically), zero only at each span's two endpoints and exact midpoint. [`arc_to_nurbs`] caps
/// spans at a fixed 120° (adequate when only the SHAPE, not the parameter-to-angle correspondence,
/// matters — the common case); [`refined_max_span`] picks a caller-chosen tolerance instead, for
/// the rarer case (`Curve3::transformed`'s non-similarity fallback) that needs the NURBS to
/// reproduce `(radius·cos t, radius·sin t)` to a numeric tolerance at arbitrary `t`, not just as a
/// set of points.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
/// 🪜️ Exact rational-quadratic control and knot recipe for a bounded conic arc.
#[derive(Clone,Copy)]
pub struct ArcSplineRecipe {
    frame:Frame3,
    radii:[f64;2],
    range:(f64,f64),
    spans:usize,
    affine:Option<Affine3>,
}

semio_framework_value::artifact_retire_leaf!(ArcSplineRecipe);

impl ArcSplineRecipe {
    pub fn new(frame:Frame3,radii:[f64;2],range:(f64,f64),maximum_span:f64) -> Option<Self> {
        let sweep=range.1-range.0;
        let spans=(sweep/maximum_span).ceil().max(1.0) as usize;
        if !sweep.is_finite() || sweep<0.0 || !maximum_span.is_finite() || maximum_span<=0.0 ||
           radii.iter().any(|r|!r.is_finite()) || spans>(usize::MAX-4)/2 { return None; }
        Some(Self{frame,radii,range,spans,affine:None})
    }

    pub fn control_count(self) -> usize {2*self.spans+1}
    pub fn knot_count(self) -> usize {self.control_count()+3}

    fn angle(self,span:usize) -> f64 {
        if span==self.spans {self.range.1}
        else {self.range.0+(self.range.1-self.range.0)*span as f64/self.spans as f64}
    }

    pub fn knot(self,index:usize) -> f64 {
        if index<3 {self.range.0}
        else if index>=self.knot_count()-3 {self.range.1}
        else {self.angle(1+(index-3)/2)}
    }

    pub fn control(self,index:usize) -> (Pnt3,f64) {
        let start=self.angle(index/2);
        let (angle,weight)=if index%2==0 {(start,1.0)} else {
            let half=(self.angle(index/2+1)-start)*0.5;
            (start+half,half.cos())
        };
        let [rx,ry]=self.radii;
        let point=self.frame.to_world(Pnt3::new(rx*angle.cos()/weight,ry*angle.sin()/weight,0.0));
        (self.affine.map_or(point, |affine| affine.apply_point(point)),weight)
    }

    fn materialize(self) -> NurbsCurve3 {
        let (controls,weights)=(0..self.control_count()).map(|i|self.control(i)).unzip();
        let knots=(0..self.knot_count()).map(|i|self.knot(i)).collect();
        NurbsCurve3{knots:KnotVector{knots,degree:2},controls,weights}
    }
}

fn arc_to_nurbs_with_span(frame:&Frame3,radius_x:f64,radius_y:f64,domain:(f64,f64),max_span:f64) -> NurbsCurve3 {
    ArcSplineRecipe::new(*frame,[radius_x,radius_y],domain,max_span).expect("valid rational conic interval").materialize()
}

// #endregion 🔖️Curve3

// #region 🔖️Curve2

/// 🌀️ A 2D curve, used as the pcurve type: a curve living in a face's `(u, v)` parameter domain.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(crate = "::semio_framework_value")]
pub enum Curve2 {
    Line { origin: Pnt2, dir: Vec2 },
    Circle { center: Pnt2, radius: f64 },
    Ellipse { center: Pnt2, x_axis: Vec2, major_radius: f64, minor_radius: f64 },
    Nurbs { knots: KnotVector, controls: Vec<Pnt2>, weights: Vec<f64> },
}


impl Curve2 {
    /// ➰️ The same curve translated by `delta` in parameter space — the exact operation, per
    /// kind, not a refit. A p-curve on a periodic surface is only defined up to whole periods in
    /// the periodic directions, and the branch a p-curve was born in (whatever the surface
    /// inversion that produced it happened to return) need not be the branch the face's own
    /// boundary ring is written in; imprinting one into the other without re-aligning it produces a
    /// UV polygon whose pieces sit periods apart, which has no interior at all.
    pub fn translated(&self, delta: Vec2) -> Curve2 {
        match self {
            Curve2::Line { origin, dir } => Curve2::Line { origin: *origin + delta, dir: *dir },
            Curve2::Circle { center, radius } => Curve2::Circle { center: *center + delta, radius: *radius },
            Curve2::Ellipse { center, x_axis, major_radius, minor_radius } => Curve2::Ellipse { center: *center + delta, x_axis: *x_axis, major_radius: *major_radius, minor_radius: *minor_radius },
            Curve2::Nurbs { knots, controls, weights } => Curve2::Nurbs { knots: knots.clone(), controls: controls.iter().map(|p| *p + delta).collect(), weights: weights.clone() },
        }
    }
    pub fn domain(&self) -> (f64, f64) {
        match self {
            Curve2::Line { .. } => (f64::NEG_INFINITY, f64::INFINITY),
            Curve2::Circle { .. } | Curve2::Ellipse { .. } => (0.0, std::f64::consts::TAU),
            Curve2::Nurbs { knots, .. } => knots.domain(),
        }
    }
    /// 📌️ Samples an original planar trim curve without heap scratch.
    pub fn sample_original(&self,t:f64) -> Option<Pnt2> {
        if let Self::Nurbs{knots,controls,weights}=self {
            if controls.len()!=weights.len() { return None; }
            let homogeneous=bspline::homogeneous_jet_original(knots,controls.len(),t,0,|i|{
                let p=controls.get(i)?;
                let w=*weights.get(i)?;
                Some([p.x*w,p.y*w,0.0,w])
            })?;
            let [p,_,_]=bspline::project_original_jet(homogeneous,0)?;
            return Some(Pnt2::new(p[0],p[1]));
        }
        let p=self.eval(t);
        (p.x.is_finite()&&p.y.is_finite()).then_some(p)
    }

    pub fn eval(&self, t: f64) -> Pnt2 {
        match self {
            Curve2::Line { origin, dir } => *origin + *dir * t,
            Curve2::Circle { center, radius } => *center + Vec2::new(radius * t.cos(), radius * t.sin()),
            Curve2::Ellipse { center, x_axis, major_radius, minor_radius } => {
                let x = x_axis.normalized().unwrap_or(Vec2::new(1.0, 0.0));
                let y = x.perp();
                *center + x * (major_radius * t.cos()) + y * (minor_radius * t.sin())
            }
            Curve2::Nurbs { knots, controls, weights } => eval_nurbs_curve2(knots, controls, weights, t),
        }
    }
    pub fn d1(&self, t: f64) -> Vec2 {
        match self {
            Curve2::Line { dir, .. } => *dir,
            Curve2::Circle { radius, .. } => Vec2::new(-radius * t.sin(), radius * t.cos()),
            Curve2::Ellipse { x_axis, major_radius, minor_radius, .. } => {
                let x = x_axis.normalized().unwrap_or(Vec2::new(1.0, 0.0));
                let y = x.perp();
                x * (-major_radius * t.sin()) + y * (minor_radius * t.cos())
            }
            Curve2::Nurbs { knots, controls, weights } => {
                let controls_h: Vec<Vec<f64>> = controls.iter().zip(weights).map(|(p, &w)| vec![p.x * w, p.y * w, w]).collect();
                let derivs = curve_derivatives_rational(knots, &controls_h, t, 1);
                Vec2::new(derivs[1][0], derivs[1][1])
            }
        }
    }
    /// 🌀️ Second derivative `d²C/dt²`, exact via the rational de Boor recurrence for
    /// [`Curve2::Nurbs`], closed-form for the analytic kinds.
    pub fn d2(&self, t: f64) -> Vec2 {
        match self {
            Curve2::Line { .. } => Vec2::ZERO,
            Curve2::Circle { radius, .. } => Vec2::new(-radius * t.cos(), -radius * t.sin()),
            Curve2::Ellipse { x_axis, major_radius, minor_radius, .. } => {
                let x = x_axis.normalized().unwrap_or(Vec2::new(1.0, 0.0));
                let y = x.perp();
                x * (-major_radius * t.cos()) + y * (-minor_radius * t.sin())
            }
            Curve2::Nurbs { knots, controls, weights } => {
                let controls_h: Vec<Vec<f64>> = controls.iter().zip(weights).map(|(p, &w)| vec![p.x * w, p.y * w, w]).collect();
                let derivs = curve_derivatives_rational(knots, &controls_h, t, 2);
                Vec2::new(derivs[2][0], derivs[2][1])
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn eval_nurbs_curve2(knots: &KnotVector, controls: &[Pnt2], weights: &[f64], t: f64) -> Pnt2 {
    let hx: Vec<f64> = controls.iter().zip(weights).map(|(p, w)| p.x * w).collect();
    let hy: Vec<f64> = controls.iter().zip(weights).map(|(p, w)| p.y * w).collect();
    let w = de_boor(knots, weights, t);
    Pnt2::new(de_boor(knots, &hx, t) / w, de_boor(knots, &hy, t) / w)
}

// #endregion 🔖️Curve2

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔁️Transform

use semio_framework_3d_brep_vector::matrix::Affine3;

impl Curve3 {
    /// 🌀️ Exact affine transform. `Line` stays exact under ANY invertible affine map — a line's
    /// image under any invertible linear map is again a line, with no orthonormality or domain
    /// constraint (unlike every other variant). `Circle`/`Ellipse` stay analytic under a similarity
    /// (uniform scale/rotation/translation, optionally with reflection — [`semio_framework_3d_brep_vector::matrix::Affine3::is_similarity`]),
    /// with `frame` mapped via [`Frame3::transformed`] and `radius`/`major_radius`/`minor_radius`
    /// scaled uniformly; a non-similarity map converts to the equivalent exact NURBS
    /// ([`Self::to_nurbs`] over the curve's own bounded natural domain) and transforms its control
    /// points (rational weights are affine-invariant, so they carry over unchanged either way).
    /// `Nurbs` always just transforms its control points.
    pub fn transformed(&self, map: &Affine3) -> Curve3 {
        match self {
            Curve3::Line { origin, dir } => Curve3::Line { origin: map.apply_point(*origin), dir: map.apply_vector(*dir) },
            Curve3::Nurbs { knots, controls, weights } => Curve3::Nurbs { knots: knots.clone(), controls: controls.iter().map(|p| map.apply_point(*p)).collect(), weights: weights.clone() },
            Curve3::Circle { frame, radius } => match map.is_similarity() {
                Some((_, scale, _)) => Curve3::Circle { frame: frame.transformed(map, scale), radius: radius * scale },
                None => self.transformed_via_nurbs(map),
            },
            Curve3::Ellipse { frame, major_radius, minor_radius } => match map.is_similarity() {
                Some((_, scale, _)) => Curve3::Ellipse { frame: frame.transformed(map, scale), major_radius: major_radius * scale, minor_radius: minor_radius * scale },
                None => self.transformed_via_nurbs(map),
            },
        }
    }
    /// 🌀️ The shared non-similarity fallback: convert to NURBS over the curve's own bounded
    /// natural domain, then transform every control point (weights unchanged). `Circle`/`Ellipse`
    /// use [`refined_max_span`] (not the coarser 120°-span default) so `transformed.eval(t)` stays
    /// within `1e-9` of `map.apply_point(self.eval(t))` at every `t`, not just at span boundaries
    /// — required because [`Self::transformed`]'s own contract is a pointwise pushforward
    /// (`transformed.eval(t) == map(self.eval(t))`), not merely a shape-preserving conversion.
    fn transformed_via_nurbs(&self, map: &Affine3) -> Curve3 {
        let nurbs = self.affine_conic_recipe(*map).expect("valid rational conic interval").materialize();
        Curve3::Nurbs { knots: nurbs.knots, controls: nurbs.controls, weights: nurbs.weights }
    }

    /// 🧭️ Plans the cold rational conic image without allocating knots or controls.
    pub fn affine_conic_recipe(&self, map: Affine3) -> Option<ArcSplineRecipe> {
        let (frame, radii) = match self {
            Curve3::Circle { frame, radius } => (*frame, [*radius; 2]),
            Curve3::Ellipse { frame, major_radius, minor_radius } => (*frame, [*major_radius, *minor_radius]),
            _ => return None,
        };
        let mut recipe = ArcSplineRecipe::new(frame, radii, self.domain(), refined_max_span(radii[0].max(radii[1])))?;
        recipe.affine = Some(map);
        Some(recipe)
    }
}

/// 🌀️ The largest per-span half-angle that keeps [`arc_to_nurbs_with_span`]'s parametrization
/// within `1e-9` of the true `radius·(cos t, sin t)` point at every `t` (not just span
/// boundaries), from the leading-order error bound `peak ≈ 0.0321·radius·half_span³` documented
/// on [`arc_to_nurbs_with_span`], solved for `half_span` and halved again for margin against the
/// series' next (`O(half_span⁵)`) term. Never coarser than the standard 120° span.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn refined_max_span(radius: f64) -> f64 {
    let tol = 1e-9;
    let r = radius.abs().max(1e-9);
    let half_span = (tol / (0.0321 * r)).cbrt() * 0.5;
    (2.0 * half_span).min(std::f64::consts::TAU / 3.0)
}

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️transform/🦀️.rs"]
mod transform_tests;
// #endregion 🔖️Tests

// #endregion 🔁️Transform
