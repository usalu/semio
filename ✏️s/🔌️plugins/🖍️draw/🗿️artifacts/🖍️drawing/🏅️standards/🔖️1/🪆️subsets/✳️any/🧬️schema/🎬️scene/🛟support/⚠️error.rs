//! ⚠️ Shared document-scene failure carried by preparation and raster.
use std::sync::Arc;
#[derive(Clone,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub enum DocumentSceneMessage{Literal(&'static str),Owned(Arc<String>)}
impl AsRef<str> for DocumentSceneMessage{fn as_ref(&self)->&str{match self{Self::Literal(value)=>value,Self::Owned(value)=>value.as_str()}}}
impl From<&'static str> for DocumentSceneMessage{fn from(value:&'static str)->Self{Self::Literal(value)}}
impl PartialEq<str> for DocumentSceneMessage{fn eq(&self,other:&str)->bool{self.as_ref()==other}}
impl PartialEq<&str> for DocumentSceneMessage{fn eq(&self,other:&&str)->bool{self.as_ref()==*other}}
#[derive(Clone,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub enum DocumentSceneError{Invalid(DocumentSceneMessage),PathRaster(semio_s_artifact_draw_drawing_geometry::raster::PathRasterError),AffineImage(semio_framework_pixels::affine_sampling::AffineImageError),Incomplete,Cancelled}
impl DocumentSceneError{pub fn literal(message:&'static str)->Self{Self::Invalid(DocumentSceneMessage::Literal(message))}pub fn held()->Self{Self::literal("Scene preparation lost its working state")}pub fn owned(message:Arc<String>)->Self{Self::Invalid(DocumentSceneMessage::Owned(message))}}
#[path="../📋️prepare/⚠️failure/🦀️.rs"]
mod original_failure;
impl std::fmt::Display for DocumentSceneError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str(self.message())}}
impl From<semio_s_artifact_draw_drawing_geometry::raster::PathRasterError> for DocumentSceneError{fn from(error:semio_s_artifact_draw_drawing_geometry::raster::PathRasterError)->Self{Self::PathRaster(error)}}
impl From<semio_framework_pixels::affine_sampling::AffineImageError> for DocumentSceneError{fn from(error:semio_framework_pixels::affine_sampling::AffineImageError)->Self{Self::AffineImage(error)}}
impl std::error::Error for DocumentSceneError{}
impl From<&'static str> for DocumentSceneError{fn from(message:&'static str)->Self{Self::literal(message)}}
impl From<semio_framework_2d::flatten::PathFlattenError> for DocumentSceneError{fn from(error:semio_framework_2d::flatten::PathFlattenError)->Self{match error{semio_framework_2d::flatten::PathFlattenError::Invalid(message)=>Self::literal(message),semio_framework_2d::flatten::PathFlattenError::Incomplete=>Self::Incomplete,semio_framework_2d::flatten::PathFlattenError::Cancelled=>Self::Cancelled}}}
impl From<semio_framework_2d::stroke::StrokeOutlineError> for DocumentSceneError{fn from(error:semio_framework_2d::stroke::StrokeOutlineError)->Self{match error{semio_framework_2d::stroke::StrokeOutlineError::Invalid(message)=>Self::literal(message),semio_framework_2d::stroke::StrokeOutlineError::Incomplete=>Self::Incomplete,semio_framework_2d::stroke::StrokeOutlineError::Cancelled=>Self::Cancelled}}}
impl From<semio_framework_2d::booleans::BooleanError> for DocumentSceneError{fn from(error:semio_framework_2d::booleans::BooleanError)->Self{match error{semio_framework_2d::booleans::BooleanError::Invalid(message)=>Self::literal(message),semio_framework_2d::booleans::BooleanError::Incomplete=>Self::Incomplete,semio_framework_2d::booleans::BooleanError::Cancelled=>Self::Cancelled}}}
