"""🖼️ Immutable screen snapshots of vector drawings for whole-object presentation."""
import types
import numpy as np
from PIL import Image
from manim import Camera, ImageMobject

class Snapshot(ImageMobject):
    """🖼️ A camera-aligned immutable raster presentation of vector geometry."""

def capture(drawing, camera):
    """📸 Capture painter order at the camera's pixel and frame dimensions."""
    target = Camera(frame_center=np.array(camera.frame_center, copy=True), pixel_width=camera.pixel_width, pixel_height=camera.pixel_height,
                    frame_width=camera.frame_width, frame_height=camera.frame_height, frame_rate=camera.frame_rate,
                    background_opacity=0, cairo_line_width_multiple=camera.cairo_line_width_multiple, use_z_index=camera.use_z_index)
    target.capture_mobjects([drawing])
    rgba = np.array(target.get_image())
    alpha = rgba[:, :, 3].astype(float)
    rgba[:, :, :3] = np.where(alpha[:, :, None] > 0, np.minimum(255, np.rint(rgba[:, :, :3].astype(float) * 255 / np.maximum(alpha[:, :, None], 1))), 0).astype(np.uint8)
    image = Snapshot(rgba)
    image.stretch_to_fit_width(camera.frame_width).stretch_to_fit_height(camera.frame_height).move_to(camera.frame_center)
    image.snapshot_points = image.points.copy()
    image.snapshot_center = np.array(camera.frame_center, copy=True)
    image.snapshot_frame = (camera.frame_width, camera.frame_height)
    if not hasattr(camera, "_snapshot_display"):
        original = camera.display_image_mobject
        def display(self, item, pixels):
            if not isinstance(item, Snapshot):
                return original(item, pixels)
            if item.pixel_array.shape[:2] != pixels.shape[:2] or not np.allclose(item.points, item.snapshot_points) or not np.allclose(self.frame_center, item.snapshot_center) or (self.frame_width, self.frame_height) != item.snapshot_frame:
                raise ValueError("A drawing snapshot requires its original fixed camera and frame alignment")
            self.overlay_PIL_image(pixels, Image.fromarray(item.get_pixel_array(), mode="RGBA"))
        camera._snapshot_display = original
        camera.display_image_mobject = types.MethodType(display, camera)
    return image
