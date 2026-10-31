"""🎨 Native material-cell coverage behind the drafting renderer interface."""
import importlib

def material_fill(actor):
    """🖌️ Rasterize adjacent material fills with one pixel owner while keeping boundary ink antialiased."""
    camera=importlib.import_module("manim.camera.camera");original=camera.Camera.apply_fill
    if not getattr(original,"_semio_material_coverage",False):
        def apply_fill(self,context,drawable):
            """📷 Isolate cell-fill coverage policy without changing other native camera operations."""
            if not getattr(drawable,"_semio_material_cell",False):
                return original(self,context,drawable)
            context.save()
            try:
                context.set_antialias(camera.cairo.ANTIALIAS_NONE)
                return original(self,context,drawable)
            finally:
                context.restore()
        apply_fill._semio_material_coverage=True
        camera.Camera.apply_fill=apply_fill
    actor._semio_material_cell=True
    return actor
