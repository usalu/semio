/// 🔬️ Examples: one evaluation law per bundled example.

//#region 🔖️Examples
#[test]
fn rectangle_wire_preview_evaluates_to_an_open_wire() {
    assert_example(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪢️rectangle-wire-preview/🖼️assets/🪢️rectangle-wire-preview/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪢️rectangle-wire-preview/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn rectangle_extrude_volume_evaluates_to_the_analytic_box() {
    assert_example(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️rectangle-extrude-volume/🖼️assets/📦️rectangle-extrude-volume/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️rectangle-extrude-volume/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn face_sweep_extrude_evaluates_to_a_closed_solid() {
    assert_example(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧹️face-sweep-extrude/🖼️assets/🧹️face-sweep-extrude/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧹️face-sweep-extrude/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn hexagonal_mushroom_column_evaluates_to_the_analytic_prism() {
    assert_example(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍄️hexagonal-mushroom-column/🖼️assets/🍄️hexagonal-mushroom/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍄️hexagonal-mushroom-column/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn box_shell_preview_evaluates_to_a_hollow_solid() {
    assert_example(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🐚️box-shell-preview/🖼️assets/🐚️box-shell-preview/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🐚️box-shell-preview/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn box_fillet_preview_evaluates_to_a_rounded_solid() {
    assert_example(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📐️box-fillet-preview/🖼️assets/📐️box-fillet-preview/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📐️box-fillet-preview/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn sphere_box_fuse_evaluates_to_the_union_volume() {
    assert_example(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧲️sphere-box-fuse/🖼️assets/🧲️sphere-box-fuse/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧲️sphere-box-fuse/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}

#[test]
fn sphere_cut_with_torus_evaluates_to_the_difference_volume() {
    assert_example(include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍩️sphere-cut-with-torus/🖼️assets/🍩️sphere-cut-with-torus/🗣️.dsl.semio"), include_str!("../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍩️sphere-cut-with-torus/🧪️tests/🧫️fixtures/🧩️example/🔣️.json"));
}
//#endregion 🔖️Examples
