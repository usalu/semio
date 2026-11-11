import ast,importlib,json,unittest
from pathlib import Path
import jsonschema,numpy as np
from manim import Camera,ValueTracker,tempconfig
from shapely.geometry import box
r=importlib.import_module('🎓️teaching.🏛️architecture.📊️excel.🎬️clip.📍️references.🐍️')
case=json.loads(Path(__file__).with_name('🔣️.json').read_text(encoding='utf-8'))
schema=json.loads(Path(__file__).with_name('🔣️.schema.json').read_text(encoding='utf-8'))
class Roof(unittest.TestCase):
 def test_basis_lifetime_and_layout(self):
  tree=ast.parse(Path(r.__file__).read_text(encoding='utf-8'))
  cls=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=='Beat2_References')
  method=next(n for n in cls.body if isinstance(n,ast.FunctionDef) and n.name=='present_references')
  def step(name):
   return next(n for n in ast.walk(method) if isinstance(n,ast.Call) and isinstance(n.func,ast.Attribute) and n.func.attr=='step' and n.args and isinstance(n.args[0],ast.Constant) and n.args[0].value==name)
  def animation(step_node,kind):
   return next(n for n in ast.walk(step_node) if isinstance(n,ast.Call) and isinstance(n.func,ast.Attribute) and n.func.attr==kind and n.args and isinstance(n.args[0],ast.Name) and n.args[0].id=='basis')
  with tempconfig({'dry_run':True,'pixel_width':854,'pixel_height':480,'frame_rate':3,'background_color':'#000000'}):
   for lang in case['languages']:
    scene=r.Beat2_References();scene.language=lambda:lang
    basis=scene._basis_labels()
    win=scene._window(r._xl.workbook(),r.SHEET,'A:D',(1,10),'D3',language=lang)
    viewport=box(-scene.camera.frame_width/2,-scene.camera.frame_height/2,scene.camera.frame_width/2,scene.camera.frame_height/2)
    labels=[box(x.get_left()[0],x.get_bottom()[1],x.get_right()[0],x.get_top()[1]) for x in basis]
    self.assertTrue(all(viewport.covers(x) for x in labels))
    self.assertFalse(labels[0].intersects(labels[1]))
    self.assertTrue(all(x.bounds[1]>win.get_top()[1] for x in labels))
    env={'_m':r._m,'basis':basis}
    for name,kind in (('hook','FadeIn'),('relative','FadeOut')):
     node=animation(step(name),kind)
     scene.play(eval(compile(ast.Expression(node),r.__file__,'eval'),env),run_time=.34)
    self.assertFalse({id(x) for x in basis.get_family()} & {id(x) for x in scene.get_mobject_family_members()})
    camera=Camera(pixel_width=854,pixel_height=480,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color='#000000',background_opacity=1)
    camera.capture_mobjects([win,*scene._basis_labels()])
    self.assertGreater(int(np.asarray(camera.get_image())[:65,:,:3].max()),0)

 def test_live_fill_edit_lifetimes(self):
  tree=ast.parse(Path(r.__file__).read_text(encoding='utf-8'))
  cls=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=='Beat2_References')
  methods={n.name:n for n in cls.body if isinstance(n,ast.FunctionDef)}
  rows=(('present_references','win4','absolute',r.SHEET,'C:G',(2,12),'G3'),
        ('_stage_two_way','flux','mixed2','Program','A:E',(18,21),'C19'),
        ('_stage_two_way','win5','mixed','Program','A:E',(10,13),'B11'))
  with tempconfig({'dry_run':True,'pixel_width':320,'pixel_height':180,'frame_rate':3}):
   for lang in case['languages']:
    for method_name,name,stage,sheet,cols,rowspec,selected in rows:
     scene=r.Beat2_References();scene.language=lambda:lang
     win=scene._window(r._xl.workbook(),sheet,cols,rowspec,selected,language=lang)
     method=methods[method_name]
     stage_node=next(n for n in method.body if isinstance(n,ast.Expr) and isinstance(n.value,ast.Call)
                     and isinstance(n.value.func,ast.Attribute) and n.value.func.attr=='step' and n.value.args
                     and isinstance(n.value.args[0],ast.Constant) and n.value.args[0].value==stage)
     edits=[n for n in method.body if isinstance(n,ast.Expr) and isinstance(n.value,ast.Call) and isinstance(n.value.func,ast.Attribute)
            and n.value.func.attr=='edit' and isinstance(n.value.func.value,ast.Name) and n.value.func.value.id==name and n.lineno<stage_node.lineno]
     for edit in edits:
      exec(compile(ast.fix_missing_locations(ast.Module(body=[edit],type_ignores=[])),r.__file__,'exec'),{name:win})
     self.assertIsNone(win.state['editing'],(lang,name,stage))
     self.assertEqual(len(win._edit_overlay().submobjects),0,(lang,name,stage))

 def test_live_roof_result(self):
  jsonschema.validate(case,schema)
  tree=ast.parse(Path(r.__file__).read_text(encoding='utf-8'))
  cls=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=='Beat2_References')
  method=next(n for n in cls.body if isinstance(n,ast.FunctionDef) and n.name=='present_references')
  start=next(i for i,n in enumerate(method.body) if isinstance(n,ast.Assign) and any(isinstance(t,ast.Attribute) and t.attr=='formula_pieces' for t in n.targets))
  end=next(i for i in range(start+1,len(method.body)) if isinstance(method.body[i],ast.Expr) and 'self.w.animate.set_value' in ast.unparse(method.body[i]))
  code=compile(ast.fix_missing_locations(ast.Module(body=method.body[start:end+1],type_ignores=[])),r.__file__,'exec')
  with tempconfig({'dry_run':True,'pixel_width':320,'pixel_height':180,'frame_rate':3,'background_color':'#000000'}):
   for lang in case['languages']:
    scene=r.Beat2_References();scene.language=lambda:lang;scene.w=ValueTracker(case['inputStart'])
    book=r._xl.workbook()
    def build():
     w=scene._window(book,r.SHEET,'A:D',(1,10),'D3',language=lang)
     w.bind('D3',lambda:scene.xl_display(scene.w.get_value()*case['length'],'0.0 "m²"',lang))
     return w
    win=build();win.edit('=B2*B3');scene.add(win)
    expected=build();expected.edit(None)
    env={'self':scene,'win':win,'width0':case['inputStart'],'marks':[],'rest':[],'_m':r._m,'blue':'#4472C4','red':'#FF0000'}
    oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color='#000000',background_opacity=1)
    left,top,right,bottom=win.rect('D3');fw=scene.camera.frame_width;fh=scene.camera.frame_height
    x0=int((left/fw+.5)*320)+3;x1=int((right/fw+.5)*320)-3
    y0=int((.5-top/fh)*180)+3;y1=int((.5-bottom/fh)*180)-3
    differences=[];original=scene.renderer.update_frame
    def capture(*args,**kwargs):
     result=original(*args,**kwargs)
     expected.update(0);oracle.reset();oracle.capture_mobjects([expected])
     a=np.asarray(scene.camera.get_image(),dtype=int);b=np.asarray(oracle.get_image(),dtype=int)
     differences.append(int(np.abs(a[y0:y1,x0:x1]-b[y0:y1,x0:x1]).max()))
     return result
    scene.renderer.update_frame=capture
    exec(code,env)
    self.assertAlmostEqual(float(np.multiply(scene.w.get_value(),case['length'])),12.0)
    self.assertIsNone(win.state['editing'])
    self.assertGreaterEqual(len(differences),4)
    self.assertLessEqual(max(differences),3,(lang,differences))
if __name__=='__main__':unittest.main()
