use super::*;
use semio_s_fem_elements2d::Bar2;
use semio_s_fem_elements2d::BeamEb2;
use semio_s_fem_test_support::close_job;
use semio_s_fem_test_support::pages_bytes;
use crate::engine_test_vectors::seen_of;
use semio_s_fem_test_support::TEST_GRANT;
use semio_s_fem_static::solve_linear_static;
use semio_s_fem_model::AxialSpring;
use semio_s_fem_static::Model;
use semio_s_fem_model::PlaneStress;
use semio_s_fem_model::ShellState;
use semio_s_fem_model::SolidStress;

include!("🍳️assembly-page-and-mounted-admission-laws/🦀️.rs");

include!("🧈️assembly-csr-element-build-and-job-laws/🦀️.rs");

include!("🥓️static-solve-and-rejection-laws/🦀️.rs");

include!("🥩️member-load-scalar-average-and-job-graph-laws/🦀️.rs");
