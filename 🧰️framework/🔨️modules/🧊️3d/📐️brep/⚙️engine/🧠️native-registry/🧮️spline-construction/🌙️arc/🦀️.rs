//! 🌙️ Exact rational arc storage admitted one retained operation at a time.

use super::{Curve3, PersistentLabel, Pnt3};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};

#[derive(semio_framework_value::RetireOwned)]
pub struct ArcSplineBuild {
    recipe:semio_framework_3d_brep_curve::ArcSplineRecipe,
    label:PersistentLabel,
    knots:Vec<f64>,
    controls:Vec<Pnt3>,
    weights:Vec<f64>,
}

impl ArcSplineBuild {
    /// 🌱️ Holds the exact arc recipe without allocating its control lattice.
    pub fn new(recipe: semio_framework_3d_brep_curve::ArcSplineRecipe, label: PersistentLabel) -> Self {
        Self { recipe, label, knots: Vec::new(), controls: Vec::new(), weights: Vec::new() }
    }

    /// 🏷️ Returns the label reserved by the owning publication.
    pub fn label(&self) -> PersistentLabel { self.label }

    pub fn demand(&self) -> RetirementDemand {
        let (count,capacity)=if self.knots.capacity()==0 {(self.recipe.knot_count(),size_of::<f64>())}
            else if self.controls.capacity()==0 {(self.recipe.control_count(),size_of::<Pnt3>())}
            else if self.weights.capacity()==0 {(self.recipe.control_count(),size_of::<f64>())}
            else {(0,0)};
        let copy=if count!=0 {0}
            else if self.knots.len()<self.recipe.knot_count(){size_of::<f64>()}
            else if self.controls.len()<self.recipe.control_count(){size_of::<Pnt3>()+size_of::<f64>()}
            else{0};
        RetirementDemand{depth:1,capacity_bytes:count*capacity,copy_bytes:copy,..Default::default()}
    }

    /// 🌙️ Retains one arc buffer or control and yields the curve on its final funded turn.
    pub fn step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<(RetainedCloneProgress, Option<Curve3>), ValueError> {
        let demand=self.demand();
        let grant=effect.grant();
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand) || !effect.performed(1)? {return Ok((Default::default(), None));}
        fn reserve<T>(rows:&mut Vec<T>,count:usize) -> Result<usize,ValueError> {
            rows.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"arc control storage allocation failed"))?;
            Ok(rows.capacity()*size_of::<T>())
        }
        let mut completed = None;
        let mut receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        if self.knots.capacity()==0 {receipt.retained_capacity_bytes=reserve(&mut self.knots,self.recipe.knot_count())?;}
        else if self.controls.capacity()==0 {receipt.retained_capacity_bytes=reserve(&mut self.controls,self.recipe.control_count())?;}
        else if self.weights.capacity()==0 {receipt.retained_capacity_bytes=reserve(&mut self.weights,self.recipe.control_count())?;}
        else if self.knots.len()<self.recipe.knot_count() {self.knots.push(self.recipe.knot(self.knots.len()));}
        else if self.controls.len()<self.recipe.control_count() {
            let (p,w)=self.recipe.control(self.controls.len());
            if ![p.x,p.y,p.z,w].iter().all(|v|v.is_finite()) || w<=0.0 {return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"arc control is non-finite or singular"));}
            self.controls.push(p);
            self.weights.push(w);
        } else {
            let knots = semio_framework_3d_brep_curve::bspline::KnotVector { knots: std::mem::take(&mut self.knots), degree: 2 };
            completed = Some(Curve3::Nurbs { knots, controls: std::mem::take(&mut self.controls), weights: std::mem::take(&mut self.weights) });
        }
        if !receipt.fits(grant) {return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"arc control storage exceeded its grant").with_retained_progress(receipt));}
        Ok((receipt, completed))
    }
}
