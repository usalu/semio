//! 🧪️ The PNG encode job's declared birth bytes equal the layout of its defining retirement cursor.
use super::*;

#[test]
fn png_encode_job_birth_bytes_equal_its_defining_cursor_layout() {
    use semio_framework_value::retirement::{retirement_cursor_birth_demand,RetireOwned};
    let job=PngEncodeJob::new(RasterImage::new(2,2)).unwrap();
    let layout=std::mem::size_of::<<PngEncodeJob<RasterImage> as RetireOwned>::RetirementCursor>();
    assert_eq!(job.retirement_birth_bytes(),Some(layout));
    let demand=retirement_cursor_birth_demand(&job).unwrap();
    assert_eq!(demand.capacity_bytes,layout);
    let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::retain(job);
    owner.abandon();
}
