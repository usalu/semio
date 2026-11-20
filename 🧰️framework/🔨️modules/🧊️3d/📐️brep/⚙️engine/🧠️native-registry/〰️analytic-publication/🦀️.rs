//! 🛠️ Fixed-size analytic requests and budgeted publication of their topology.

use super::*;

use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};

/// 〰️ Fixed-size parameters of an analytic curve an original caller may publish.
pub enum AnalyticCurveParameters {
    Line { start: EVec3, end: EVec3 },
    Circle { center: EVec3, normal: EVec3, radius: f64 },
    Ellipse { center: EVec3, normal: EVec3, major_radius: f64, minor_radius: f64 },
}

/// 🧭️ Exact transforms that can retain an analytic source's original parameterization.
#[derive(Clone, Copy)]
pub enum AnalyticTransform {
    Translate(EVec3),
    Rotate {origin: EVec3, axis: EVec3, angle: f64},
    Scale {center: EVec3, factors: EVec3},
    Mirror {origin: EVec3, normal: EVec3},
    Copy,
}

impl AnalyticTransform {
    pub(super) fn map(self) -> Result<Affine3, ValueError> {
        let finite = |v: EVec3| v.iter().all(|x| x.is_finite());
        let axis_ok = |v: EVec3| finite(v) && v[0].hypot(v[1]).hypot(v[2]).is_finite() && v[0].hypot(v[1]).hypot(v[2]) > 1e-9;
        let map = match self {
            Self::Translate(offset) if finite(offset) => Affine3::translation(vec3(offset)),
            Self::Rotate{origin,axis,angle} if finite(origin) && axis_ok(axis) && angle.is_finite() => Affine3::rotation_about(pnt(origin),vec3(axis),angle),
            Self::Scale{center,factors} if finite(center) && finite(factors) && factors.iter().all(|&v| v!=0.0) => Affine3::scaling(pnt(center),vec3(factors)),
            Self::Mirror{origin,normal} if finite(origin) && axis_ok(normal) => Affine3::mirror(pnt(origin),vec3(normal)),
            Self::Copy => Affine3::IDENTITY,
            _ => return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"analytic transform requires finite, nondegenerate parameters")),
        };
        Ok(map)
    }
}


use semio_framework_3d_brep_spline_construction::{ArcSplineBuild, CoonsSurfaceBuild, CurveSplineBuild, GridSurfaceBuild};

#[derive(semio_framework_value::RetireOwned)]
enum SplineConstruction {
    Grid(GridSurfaceBuild),
    Coons(CoonsSurfaceBuild),
    Curve(CurveSplineBuild),
}

/// 🪪️ Keeps an analytic entity and its handle until the original session publishes both.
#[derive(semio_framework_value::RetireOwned)]
pub struct AnalyticPublication {
    owner: usize,
    entity: Option<Entity>,
    kind: &'static str,
    handle: [u8; 64],
    arc: Option<ArcSplineBuild>,
    spline: Option<SplineConstruction>,
}

impl AnalyticPublication {
    pub fn construction_demand(&self) -> Option<RetirementDemand> {
        match &self.spline {
            Some(SplineConstruction::Grid(grid)) => Some(grid.demand()),
            Some(SplineConstruction::Coons(coons)) => Some(coons.demand()),
            Some(SplineConstruction::Curve(curve)) => Some(curve.demand()),
            None => self.arc.as_ref().map(ArcSplineBuild::demand),
        }
    }

    /// 🧮️ Identifies the next point in an original curve or surface interpolation list.
    pub fn next_flat_sample(&self) -> Option<usize> {
        match &self.spline {
            Some(SplineConstruction::Grid(grid)) => grid.next_point_index(),
            Some(SplineConstruction::Curve(curve)) => curve.next_sample(),
            _ => None,
        }
    }

    /// 🌈️ Identifies the next original point and boundary of an interpolating Coons patch.
    pub fn next_coons_point(&self) -> Option<(usize, usize)> {
        if let Some(SplineConstruction::Coons(coons)) = &self.spline { coons.next_sample() } else { None }
    }

    /// 🔧️ Retains one allocated buffer, sampled point, or original fitting event.
    pub fn step_construction(&mut self, sample: Option<EVec3>, effect:&mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress,ValueError> {
        if let Some(spline) = &mut self.spline {
            let (progress, ready) = match spline {
                SplineConstruction::Grid(grid) => (grid.step(sample, effect)?, grid.ready()),
                SplineConstruction::Coons(coons) => (coons.step(sample, effect)?, coons.ready()),
                SplineConstruction::Curve(curve) => (curve.step(sample, effect)?, curve.ready()),
            };
            if ready {
                match self.spline.take().expect("completed spline construction") {
                    SplineConstruction::Grid(grid) => {
                        let label = grid.label();
                        self.entity = Some(Entity::Surface(grid.surface(), label));
                    }
                    SplineConstruction::Curve(curve) => {
                        let label = curve.label();
                        self.entity = Some(Entity::Curve(curve.curve(), label));
                    }
                    SplineConstruction::Coons(coons) => {
                        self.spline = Some(SplineConstruction::Grid(coons.into_grid()?));
                    }
                }
            }
            return Ok(progress);
        }
        let Some(arc) = self.arc.as_mut() else { return Ok(Default::default()); };
        let (progress, completed) = arc.step(effect)?;
        if let Some(curve) = completed {
            self.entity = Some(Entity::Curve(curve, arc.label()));
            self.arc = None;
        }
        Ok(progress)
    }

    pub fn kind(&self) -> &'static str { self.kind }

    pub fn handle(&self) -> &str {
        std::str::from_utf8(&self.handle).expect("the digest uses lowercase hexadecimal")
    }
}

/// 🏷️ Encodes an original native geometry kind and persistent label as a canonical live key.
pub fn primitive_handle(kind: &[u8], label: u64) -> [u8; 64] {
    let mut decimal = [0; 20];
    let mut hash = semio_framework_hash::Hasher::new();
    hash.update(kind).update(PersistentLabel(label).decimal(&mut decimal).as_bytes());
    let digest = hash.finalize();
    const HEX: &[u8; 16] = b"0123456789abcdef";
    std::array::from_fn(|index| {
        let byte = digest.as_bytes()[index / 2];
        HEX[(if index % 2 == 0 { byte >> 4 } else { byte & 15 }) as usize]
    })
}


pub fn analytic_arc_recipe(center:EVec3,normal:EVec3,radius:f64,start:f64,end:f64) -> Result<semio_framework_3d_brep_curve::ArcSplineRecipe,ValueError> {
    let invalid=||ValueError::literal(ValueRefusalKind::InvalidValue,"arc requires positive radius and a finite increasing angle interval");
    if !radius.is_finite() || radius<=0.0 || end-start<=1e-15 { return Err(invalid()); }
    let frame=analytic_frame(center,normal)?;
    semio_framework_3d_brep_curve::ArcSplineRecipe::new(frame,[radius;2],(start,end),std::f64::consts::TAU/3.0)
        .filter(|recipe|recipe.control_count()<=400_001).ok_or_else(invalid)
}

fn finite_affine_frame(frame: &Frame3) -> bool {
    let finite = |v: NativeVec3| [v.x,v.y,v.z].iter().all(|x| x.is_finite());
    [frame.origin.x,frame.origin.y,frame.origin.z].iter().all(|x| x.is_finite())
        && finite(frame.x) && finite(frame.y) && finite(frame.z) && frame.z.normalized().is_some()
}

impl BrepRegistry {
    /// 📌️ Plans an analytic curve from finite parameters and reserves only a monotone label.
    pub fn plan_analytic_curve(&mut self, parameters: AnalyticCurveParameters) -> Result<AnalyticPublication, ValueError> {
        let finite = |point: EVec3| point.iter().all(|value| value.is_finite());
        let positive = |size: f64| size.is_finite() && size > 0.0;
        let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "analytic curve requires finite, nondegenerate parameters");
        let curve = match parameters {
            AnalyticCurveParameters::Line { start, end } => {
                let dir = pnt(end) - pnt(start);
                let length = dir.x.hypot(dir.y).hypot(dir.z);
                if !finite(start) || !finite(end) || !length.is_finite() || length <= 1e-9 { return Err(invalid()); }
                Curve3::Line { origin: pnt(start), dir }
            }
            AnalyticCurveParameters::Circle { center, normal, radius } => {
                if !positive(radius) { return Err(invalid()); }
                Curve3::Circle { frame: analytic_frame(center, normal)?, radius }
            }
            AnalyticCurveParameters::Ellipse { center, normal, major_radius, minor_radius } => {
                if !positive(major_radius) || !positive(minor_radius) { return Err(invalid()); }
                Curve3::Ellipse { frame: analytic_frame(center, normal)?, major_radius, minor_radius }
            }
        };
        Ok(self.plan_prepared_analytic_curve(curve))
    }

    /// 📦️ Stages the fixed analytic curve produced by an exact native construction.
    pub fn plan_prepared_analytic_curve(&mut self, curve: Curve3) -> AnalyticPublication {
        let label = self.body.new_label();
        AnalyticPublication {
            owner: std::ptr::from_ref(&*self) as usize, entity: Some(Entity::Curve(curve, label)),
            kind: "curve", handle: primitive_handle(b"Curve:", label.0), arc:None, spline:None,
        }
    }

    /// 🌙️ Opens exact rational arc construction without allocating its control lattice.
    pub fn plan_analytic_arc(&mut self,center:EVec3,normal:EVec3,radius:f64,start:f64,end:f64) -> Result<AnalyticPublication,ValueError> {
        let recipe=analytic_arc_recipe(center,normal,radius,start,end)?;
        let label=self.body.new_label();
        Ok(AnalyticPublication{
            owner:std::ptr::from_ref(&*self) as usize,entity:None,kind:"curve",handle:primitive_handle(b"Curve:",label.0),
            arc:Some(ArcSplineBuild::new(recipe, label)),
            spline:None,
        })
    }

    /// ➿️ Plans an original NURBS curve through the ordered source samples.
    pub fn plan_interpolated_curve(&mut self, count: usize, degree: usize) -> Result<AnalyticPublication, ValueError> {
        let label = self.body.new_label();
        let curve = CurveSplineBuild::interpolation(count, degree, label)?;
        Ok(self.plan_spline_curve(curve))
    }

    /// 📉️ Admits an original endpoint-constrained curve fit.
    pub fn plan_approximated_curve(&mut self, count: usize, degree: usize, controls: usize) -> Result<AnalyticPublication, ValueError> {
        let label = self.body.new_label();
        let curve = CurveSplineBuild::approximation(count, degree, controls, label)?;
        Ok(self.plan_spline_curve(curve))
    }

    fn plan_spline_curve(&self, curve: CurveSplineBuild) -> AnalyticPublication {
        let label = curve.label();
        AnalyticPublication {
            owner: std::ptr::from_ref(self) as usize, entity: None, kind: "curve",
            handle: primitive_handle(b"Curve:", label.0), arc: None, spline: Some(SplineConstruction::Curve(curve)),
        }
    }

    /// 🌈️ Plans a Coons surface from the original sizes of four ordered boundary lists.
    pub fn plan_coons_surface(&mut self, boundaries: [usize; 4]) -> Result<AnalyticPublication, ValueError> {
        let label = self.body.new_label();
        let coons = CoonsSurfaceBuild::new(boundaries, label)?;
        Ok(AnalyticPublication {
            owner: std::ptr::from_ref(&*self) as usize, entity: None, kind: "surface",
            handle: primitive_handle(b"Surface:", label.0), arc: None, spline: Some(SplineConstruction::Coons(coons)),
        })
    }

    /// 🪡️ Admits a bounded original NURBS grid fit before copying any data samples.
    pub fn plan_interpolated_grid(&mut self, rows: usize, columns: usize, degree_u: usize, degree_v: usize) -> Result<AnalyticPublication, ValueError> {
        let label = self.body.new_label();
        let grid = GridSurfaceBuild::new(rows, columns, degree_u, degree_v, label)?;
        Ok(AnalyticPublication {
            owner: std::ptr::from_ref(&*self) as usize, entity: None, kind: "surface",
            handle: primitive_handle(b"Surface:", label.0), arc: None, spline: Some(SplineConstruction::Grid(grid)),
        })
    }

    /// 📄️ Plans a plane from its finite origin and normal without allocating geometry.
    pub fn plan_analytic_plane(&mut self, origin: EVec3, normal: EVec3) -> Result<AnalyticPublication, ValueError> {
        let frame = analytic_frame(origin, normal)?;
        let label = self.body.new_label();
        Ok(AnalyticPublication {
            owner: std::ptr::from_ref(&*self) as usize, entity: Some(Entity::Surface(Surface::Plane { frame }, label)),
            kind: "surface", handle: primitive_handle(b"Surface:", label.0), arc:None, spline:None,
        })
    }

    /// 📐️ Plans one affine image of an original analytic curve or plane.
    pub fn plan_analytic_transform(&mut self, source: &str, transform: AnalyticTransform) -> Result<AnalyticPublication, ValueError> {
        let map = transform.map()?;
        let original = self.live.get(source).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue,"analytic transform requires a live handle"))?;
        let entity = match original {
            Entity::Curve(curve @ Curve3::Line { .. }, _) => {
                let out = curve.transformed(&map);
                let Curve3::Line { origin, dir } = out else { unreachable!() };
                if ![origin.x,origin.y,origin.z,dir.x,dir.y,dir.z].iter().all(|v| v.is_finite()) || dir.x.hypot(dir.y).hypot(dir.z)==0.0 {
                    return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"transformed line is non-finite or collapsed"));
                }
                let label = self.body.new_label();
                (Entity::Curve(Curve3::Line {origin,dir},label),"curve",primitive_handle(b"Curve:",label.0))
            }
            Entity::Curve(curve @ (Curve3::Circle{..} | Curve3::Ellipse{..}),_) if map.is_similarity().is_some() => {
                let out = curve.transformed(&map);
                let valid = match &out {
                    Curve3::Circle{frame,radius} => radius.is_finite() && *radius > 0.0 && finite_affine_frame(frame),
                    Curve3::Ellipse{frame,major_radius,minor_radius} => [*major_radius,*minor_radius].iter().all(|&v| v.is_finite() && v>0.0) && finite_affine_frame(frame),
                    _ => false,
                };
                if !valid { return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"transformed conic is non-finite or collapsed")); }
                let label = self.body.new_label();
                (Entity::Curve(out,label),"curve",primitive_handle(b"Curve:",label.0))
            }
            Entity::Curve(curve @ (Curve3::Circle { .. } | Curve3::Ellipse { .. }), _) => {
                let recipe = curve.affine_conic_recipe(map).ok_or_else(|| ValueError::literal(
                    ValueRefusalKind::InvalidValue, "transformed conic requires a finite rational interval"))?;
                if recipe.control_count() > 400_001 {
                    return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "transformed conic exceeds the original arc storage limit"));
                }
                let label = self.body.new_label();
                return Ok(AnalyticPublication {
                    owner: std::ptr::from_ref(&*self) as usize, entity: None, kind: "curve",
                    handle: primitive_handle(b"Curve:",label.0), arc: Some(ArcSplineBuild::new(recipe, label)), spline: None,
                });
            }
            Entity::Surface(plane @ Surface::Plane { .. }, _) => {
                let out = plane.transformed(&map);
                let Surface::Plane { frame } = out else { unreachable!() };
                if !finite_affine_frame(&frame) { return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"transformed plane is non-finite or collapsed")); }
                let label = self.body.new_label();
                (Entity::Surface(Surface::Plane { frame },label),"surface",primitive_handle(b"Surface:",label.0))
            }
            Entity::Surface(surface @ (Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } | Surface::Torus { .. }), _)
                if map.is_similarity().is_some() => {
                let out = surface.transformed(&map);
                let positive = |radius: f64| radius.is_finite() && radius > 0.0;
                let valid = match &out {
                    Surface::Cylinder { frame, radius } | Surface::Sphere { frame, radius } => finite_affine_frame(frame) && positive(*radius),
                    Surface::Cone { frame, half_angle } => finite_affine_frame(frame) && half_angle.tan().is_finite(),
                    Surface::Torus { frame, major_radius, minor_radius } => finite_affine_frame(frame) && positive(*major_radius) && positive(*minor_radius),
                    _ => false,
                };
                if !valid { return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"transformed analytic surface is non-finite or collapsed")); }
                let label = self.body.new_label();
                (Entity::Surface(out,label),"surface",primitive_handle(b"Surface:",label.0))
            }
            _ => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"this affine image requires funded topology or spline production")),
        };
        Ok(AnalyticPublication {owner:std::ptr::from_ref(&*self) as usize,entity:Some(entity.0),kind:entity.1,handle:entity.2,arc:None,spline:None})
    }

    /// 📏️ Quotes one live-index insertion without expanding the rest of the session.
    pub fn analytic_publication_demand(&self, plan: &AnalyticPublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        let key = plan.handle();
        if plan.owner != std::ptr::from_ref(self) as usize || plan.entity.is_none() || self.live.contains_key(key) {
            return Err(ValueError::invariant("analytic geometry no longer has its original unpublished session"));
        }
        Ok(RetirementDemand {
            capacity_bytes: self.live.next_insert_capacity_byte_demand(key, copy)?,
            depth: self.live.next_insert_depth_demand(key)?,
            ..Default::default()
        })
    }

    /// 🏗️ Reserves one actual live-index page without publishing its entity.
    pub fn reserve_analytic_publication(&mut self, plan: &AnalyticPublication, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        self.analytic_publication_demand(plan, grant.maximum_copy_bytes)?;
        self.live.reserve_insert_step(plan.handle(), grant).map_err(|(error, receipt)| error.with_retained_progress(receipt))
    }

    /// 🌱️ Transfers the admitted live key and original analytic geometry into the reserved index.
    pub fn publish_analytic_geometry(&mut self, plan: &mut AnalyticPublication, key: String, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, (ValueError, String)> {
        if key != plan.handle() {
            return Err((ValueError::invariant("analytic geometry live key changed"), key));
        }
        let demand = match self.analytic_publication_demand(plan, grant.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(error) => return Err((error, key)),
        };
        if demand.capacity_bytes != 0 {
            return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit, "analytic geometry requires an admitted live slot"), key));
        }
        let Some(entity) = plan.entity.take() else {
            return Err((ValueError::invariant("analytic geometry is already published"), key));
        };
        let (previous, receipt) = match self.live.insert_reserved(key, entity, grant) {
            Ok(result) => result,
            Err((error, key, entity)) => { plan.entity = Some(entity); return Err((error, key)); }
        };
        assert!(previous.is_none());
        self.advance_live_revision();
        Ok(receipt)
    }

}

fn analytic_frame(origin: EVec3, normal: EVec3) -> Result<Frame3, ValueError> {
    let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "analytic frame requires a finite origin and nonzero normal");
    let length = normal[0].hypot(normal[1]).hypot(normal[2]);
    if origin.iter().any(|value| !value.is_finite()) || !length.is_finite() || length <= f64::EPSILON {
        return Err(invalid());
    }
    Frame3::from_normal(pnt(origin), vec3(normal.map(|component| component / length))).ok_or_else(invalid)
}

