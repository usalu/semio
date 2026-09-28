//! 🧪️ The same JSON cases run against the native pixel implementation and image crate oracle.
use super::*;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()
}

fn bytes(value: &Value) -> Vec<u8> {
    value.as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u8).collect()
}

fn operation(value: &Value) -> PixelOperation {
    let amount = value["value"].as_f64().unwrap_or(0.0);
    match value["kind"].as_str().unwrap() {
        "invert" => PixelOperation::Invert,
        "grayscale" => PixelOperation::Grayscale,
        "blur" => PixelOperation::Blur(amount as u8),
        "sharpen" => PixelOperation::Sharpen(amount),
        "flipHorizontal" => PixelOperation::FlipHorizontal,
        "flipVertical" => PixelOperation::FlipVertical,
        "rotateClockwise" => PixelOperation::RotateClockwise,
        "rotateCounterclockwise" => PixelOperation::RotateCounterclockwise,
        "clear" => PixelOperation::Clear,
        "gamma" => PixelOperation::Gamma(amount),
        "threshold" => PixelOperation::Threshold(amount),
        "fill" => PixelOperation::Fill(bytes(&value["color"]).try_into().unwrap()),
        "alphaFill" => PixelOperation::AlphaFill {alpha:value["alpha"].as_u64().unwrap() as u8,opacity:value["opacity"].as_f64().unwrap()},
        "alphaStroke" => PixelOperation::AlphaStroke(PixelAlphaBrush {
            points: value["points"].as_array().unwrap().iter().map(|point| [point[0].as_f64().unwrap(),point[1].as_f64().unwrap()]).collect(),
            size:value["size"].as_f64().unwrap(),opacity:value["opacity"].as_f64().unwrap(),hardness:value["hardness"].as_f64().unwrap(),alpha:value["alpha"].as_u64().unwrap() as u8,
        }),
        "stroke" => PixelOperation::Stroke(PixelBrush {
            points: value["points"].as_array().unwrap().iter().map(|point| [point[0].as_f64().unwrap(), point[1].as_f64().unwrap()]).collect(),
            size: value["size"].as_f64().unwrap(), opacity: value["opacity"].as_f64().unwrap(), hardness: value["hardness"].as_f64().unwrap(),
            color: bytes(&value["color"]).try_into().unwrap(), erase: value["erase"].as_bool().unwrap(),
        }),
        "crop" => PixelOperation::Crop { x: value["x"].as_u64().unwrap() as u32, y: value["y"].as_u64().unwrap() as u32, width: value["width"].as_u64().unwrap() as u32, height: value["height"].as_u64().unwrap() as u32 },
        other => panic!("unhandled fixture operation {other}"),
    }
}

#[test]
fn pixel_editing_language_neutral_cases() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let image=case.get("image").unwrap_or(&fixture["image"]);
        let source=RasterImage {width:image["width"].as_u64().unwrap() as u32,height:image["height"].as_u64().unwrap() as u32,pixels:bytes(&image["pixels"])};
        let mask = case.get("selection").map(bytes);
        let mut job = PixelEditJob::new(source.clone(), operation(&case["operation"]), mask).unwrap();
        while !job.advance(1).unwrap().done {}
        assert_eq!(job.result().unwrap().pixels, bytes(&case["expected"]), "{}", case["name"]);
        if let Some(width)=case.get("width") {assert_eq!((job.result().unwrap().width,job.result().unwrap().height),(width.as_u64().unwrap() as u32,case["height"].as_u64().unwrap() as u32));}
    }
}

#[test]
fn pixel_editing_image_crate_oracle() {
    let source = RasterImage { width: 2, height: 2, pixels: bytes(&fixture()["image"]["pixels"]) };
    let oracle = image::RgbaImage::from_raw(2, 2, source.pixels.clone()).unwrap();
    for (operation, expected) in [
        (PixelOperation::FlipHorizontal, image::imageops::flip_horizontal(&oracle)),
        (PixelOperation::FlipVertical, image::imageops::flip_vertical(&oracle)),
        (PixelOperation::RotateClockwise, image::imageops::rotate90(&oracle)),
        (PixelOperation::RotateCounterclockwise, image::imageops::rotate270(&oracle)),
    ] {
        let mut job = PixelEditJob::new(source.clone(), operation, None).unwrap();
        assert!(job.advance(4).unwrap().done);
        assert_eq!(job.result().unwrap().pixels, expected.into_raw());
    }
}

#[test]
fn pixel_editing_jobs_cancel_without_publication() {
    let mut job = PixelEditJob::new(RasterImage::new(2, 2), PixelOperation::Invert, None).unwrap();
    assert_eq!(job.advance(1).unwrap().completed, 1);
    assert!(job.result().is_err());
    job.cancel();
    assert!(job.advance(1).is_err());
    assert!(job.result().is_err());
}

#[test]
fn pixel_editing_rejects_invalid_inputs() {
    assert!(PixelEditJob::new(RasterImage { width: 2, height: 2, pixels: vec![0] }, PixelOperation::Invert, None).is_err());
    assert!(PixelEditJob::new(RasterImage::new(2, 2), PixelOperation::Gamma(0.0), None).is_err());
    assert!(PixelEditJob::new(RasterImage::new(2, 2), PixelOperation::Invert, Some(vec![255])).is_err());
    assert!(validate_extent(u32::MAX, u32::MAX).is_err());
}

#[test]
fn pixel_editing_selection_language_neutral_cases() {
    for case in fixture()["selections"].as_array().unwrap() {
        let shape = &case["shape"];
        let selection = if shape["kind"] == "polygon" {
            SelectionShape::Polygon(shape["points"].as_array().unwrap().iter().map(|p| [p[0].as_f64().unwrap(), p[1].as_f64().unwrap()]).collect())
        } else {
            SelectionShape::Box { ellipse: shape["kind"] == "ellipse", x: shape["x"].as_f64().unwrap(), y: shape["y"].as_f64().unwrap(), width: shape["width"].as_f64().unwrap(), height: shape["height"].as_f64().unwrap() }
        };
        assert_eq!(selection_mask(case["width"].as_u64().unwrap_or(2) as u32, case["height"].as_u64().unwrap_or(2) as u32, &selection).unwrap(), bytes(&case["expected"]), "{}", case["name"]);
    }
}

#[test]
fn pixel_editing_bilinear_uses_premultiplied_alpha() {
    let source = RasterImage { width: 2, height: 1, pixels: vec![255,0,0,255,0,0,255,0] };
    let mut job = PixelEditJob::new(source, PixelOperation::Resize { width: 3, height: 1, bilinear: true }, None).unwrap();
    job.advance(3).unwrap();
    assert_eq!(&job.result().unwrap().pixels[4..8], &[255,0,0,128]);
}

#[test]
fn pixel_editing_stroke_interpolates_without_opacity_buildup() {
    let brush = PixelBrush { points: vec![[0.5,0.5],[4.5,0.5]], size: 1.0, opacity: 0.5, hardness: 1.0, color: [255,0,0,255], erase: false };
    let mut job = PixelEditJob::new(RasterImage::new(5,1), PixelOperation::Stroke(brush), None).unwrap();
    assert!(job.advance(5).unwrap().done);
    assert_eq!(job.result().unwrap().pixels, [255,0,0,128].repeat(5));
}

#[test]
fn alpha_brush_jobs_are_bounded_and_cancellable() {
    let mut brush=PixelAlphaBrush {points:vec![[0.5,0.5]],size:2.0,opacity:1.0,hardness:1.0,alpha:255};
    let mut job=PixelEditJob::new(RasterImage::new(2,2),PixelOperation::AlphaStroke(brush.clone()),None).unwrap();
    assert_eq!(job.recommended_grant(),256);
    assert_eq!(job.advance(1).unwrap().completed,1);
    assert!(job.result().is_err());
    job.cancel();assert!(job.result().is_err());assert!(job.advance(1).is_err());
    brush.opacity=f64::NAN;
    assert!(PixelEditJob::new(RasterImage::new(2,2),PixelOperation::AlphaStroke(brush),None).is_err());
}

#[test]
fn output_initialization_follows_pixel_grants() {
    let fixture=fixture();let case=&fixture["outputGrants"];
    let width=case["width"].as_u64().unwrap() as u32;let height=case["height"].as_u64().unwrap() as u32;
    let source=RasterImage {width,height,pixels:bytes(&case["sourcePixel"]).repeat((width*height) as usize)};
    let mut oracle=image::RgbaImage::from_raw(width,height,source.pixels.clone()).unwrap();
    image::imageops::invert(&mut oracle);
    let mut job=PixelEditJob::new(source,operation(&case["operation"]),None).unwrap();
    assert!(job.output.pixels.is_empty());assert!(job.result().is_err());
    for (grant,completed) in case["grants"].as_array().unwrap().iter().zip(case["completed"].as_array().unwrap()) {
        let expected=completed.as_u64().unwrap() as usize;
        let progress=job.advance(grant.as_u64().unwrap() as usize).unwrap();
        assert_eq!(progress.completed,expected);assert_eq!(progress.total,(width*height) as usize);
        assert_eq!(job.output.pixels.len(),expected*4);
        assert_eq!(progress.done,expected==(width*height) as usize);
        if !progress.done {assert!(job.result().is_err());}
    }
    assert_eq!(job.result().unwrap().pixels,bytes(&case["expectedPixel"]).repeat((width*height) as usize));
    assert_eq!(job.result().unwrap().pixels,oracle.into_raw());
}

#[test]
fn selection_combination_neutral_cases_and_bounded_cancellation() {
    let fixture=fixture();
    for row in fixture["selectionCombinations"].as_array().unwrap() {
        let current=row.get("current").map(bytes);let next=bytes(&row["next"]);
        let mode=match row["mode"].as_str().unwrap() {"replace"=>SelectionMerge::Replace,"add"=>SelectionMerge::Add,"subtract"=>SelectionMerge::Subtract,"intersect"=>SelectionMerge::Intersect,_=>unreachable!()};
        let mut job=SelectionCombineJob::new(current.as_deref(),&next,mode).unwrap();assert_eq!(job.result(),Err(PixelEditError::Incomplete));assert_eq!(job.advance(0).unwrap().completed,0);
        while !job.advance(1).unwrap().done {}assert_eq!(job.into_result().unwrap(),bytes(&row["expected"]));
    }
    let work=&fixture["selectionCombinationWork"];let next=vec![128;work["length"].as_u64().unwrap() as usize];let current=vec![200;next.len()];
    let mut job=SelectionCombineJob::new(Some(&current),&next,SelectionMerge::Subtract).unwrap();
    for expected in work["completed"].as_array().unwrap() {assert_eq!(job.advance(usize::MAX).unwrap().completed,expected.as_u64().unwrap() as usize);}
    assert!(job.result().unwrap().iter().all(|&value|value==72));job.cancel();assert_eq!(job.result(),Err(PixelEditError::Cancelled));assert_eq!(job.advance(1),Err(PixelEditError::Cancelled));
    let mut job=SelectionCombineJob::new(None,&next,SelectionMerge::Add).unwrap();job.advance(1).unwrap();job.cancel();assert_eq!(job.into_result(),Err(PixelEditError::Cancelled));
    assert!(SelectionCombineJob::new(None,&[],SelectionMerge::Add).is_err());assert!(SelectionCombineJob::new(Some(&[0]),&next,SelectionMerge::Add).is_err());
    let oversized=vec![0;MAX_IMAGE_PIXELS+1];assert!(SelectionCombineJob::new(None,&oversized,SelectionMerge::Add).is_err());
    assert!(current.iter().all(|&value|value==200));assert!(next.iter().all(|&value|value==128));
}

#[test]
fn selection_rasterization_is_bounded_and_cancellable(){
    let fixture=fixture();let case=&fixture["selectionRasterization"];let width=case["width"].as_u64().unwrap() as u32;let height=case["height"].as_u64().unwrap() as u32;
    let shape=SelectionShape::Polygon(case["shape"]["points"].as_array().unwrap().iter().map(|point|[point[0].as_f64().unwrap(),point[1].as_f64().unwrap()]).collect());
    let mut job=PixelSelectionJob::new(width,height,shape.clone()).unwrap();assert_eq!(job.result(),Err(PixelEditError::Incomplete));
    for (grant,completed) in case["grants"].as_array().unwrap().iter().zip(case["completed"].as_array().unwrap()){
        let progress=job.advance(grant.as_u64().unwrap() as usize).unwrap();assert_eq!(job.mask.len(),progress.completed*width as usize);assert_eq!(progress,PixelProgress{completed:completed.as_u64().unwrap() as usize,total:height as usize,done:completed.as_u64().unwrap()==u64::from(height)});if !progress.done{assert_eq!(job.result(),Err(PixelEditError::Incomplete));}
    }
    assert_eq!(job.into_result().unwrap(),bytes(&case["expected"]));
    for rows in case["cancelAfterRows"].as_array().unwrap(){let mut job=PixelSelectionJob::new(width,height,shape.clone()).unwrap();if rows.as_u64().unwrap()>0{job.advance(rows.as_u64().unwrap() as usize).unwrap();}job.cancel();assert_eq!(job.result(),Err(PixelEditError::Cancelled));assert_eq!(job.advance(1),Err(PixelEditError::Cancelled));}
    for rows in case["invalidGrants"].as_array().unwrap(){let mut job=PixelSelectionJob::new(width,height,shape.clone()).unwrap();assert!(job.advance(rows.as_u64().unwrap() as usize).is_err());assert_eq!(job.advance(1).unwrap().completed,1);}
}
