"""🔢 Verify authored energy results remain visible with their formula references."""
import ast,importlib,json,unittest
from pathlib import Path
import jsonschema,numpy as np
from manim import Camera,tempconfig

r=importlib.import_module('🎓️teaching.🏛️architecture.📊️excel.🎬️clip.⚡️energy.🐍️')
HERE=Path(__file__).parent
CASE=json.loads((HERE/'🔣️.json').read_text(encoding='utf-8'))
SCHEMA=json.loads((HERE/'🔣️.schema.json').read_text(encoding='utf-8'))

class EnergyResult(unittest.TestCase):
 def test_authored_result_windows(self):
  jsonschema.validate(CASE,SCHEMA)
  tree=ast.parse(Path(r.__file__).read_text(encoding='utf-8'))
  cls=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=='Beat4_Energy')
  method=next(n for n in cls.body if isinstance(n,ast.FunctionDef) and any(isinstance(x,ast.Call) and isinstance(x.func,ast.Attribute) and x.func.attr=='step' and x.args and isinstance(x.args[0],ast.Constant) and x.args[0].value=='volume' for x in ast.walk(n)))
  with tempconfig({'dry_run':True,'pixel_width':320,'pixel_height':180,'frame_rate':3,'background_color':'#000000'}):
   for lang in CASE['languages']:
    scene=r.Beat4_Energy();scene.language=lambda:lang;scene._prepare()
    for row in CASE['cases']:
     assignment=next(n for n in method.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='new' for t in n.targets) and isinstance(n.value,ast.Call) and any(isinstance(a,ast.Constant) and a.value==row['cell'] for a in n.value.args))
     stage=next(n for n in method.body if isinstance(n,ast.Expr) and n.lineno>assignment.lineno and isinstance(n.value,ast.Call) and isinstance(n.value.func,ast.Attribute) and n.value.func.attr=='step' and n.value.args and isinstance(n.value.args[0],ast.Constant) and n.value.args[0].value==row['stage'])
     edits=[n for n in method.body if isinstance(n,ast.Expr) and assignment.lineno<n.lineno<stage.lineno and isinstance(n.value,ast.Call) and isinstance(n.value.func,ast.Attribute) and n.value.func.attr=='edit' and isinstance(n.value.func.value,ast.Name) and n.value.func.value.id=='new']
     env={'self':scene,'T':scene.T,'GOOD':r.GOOD}
     exec(compile(ast.fix_missing_locations(ast.Module(body=[assignment,*edits],type_ignores=[])),r.__file__,'exec'),env)
     win=env['new'];win.update(0)
     cell=row['cell'].split(':')[0]
     self.assertIsNone(win.state['editing'],(lang,cell))
     self.assertEqual(win.state['selected'],row['cell'])
     for value_cell in (f'B{i}' for i in range(8,17)):
      text=scene._show(scene.T,value_cell)
      glyph=r._visuals._workbook._label(text,win.font,r._visuals._workbook.CELL_INK)
      self.assertLessEqual(glyph.width,win._room(value_cell,'r'),(lang,value_cell,text))
     self.assertEqual(win.view.formula(scene.T,cell),row['formula'])
     self.assertEqual((win.rows[0],win.rows[-1]),tuple(row['rows']))
     bars=[x for x in win.formula_bar[1].get_family() if hasattr(x,'lines')]
     self.assertEqual(''.join(bars[0].lines),row['formula'])
     self.assertGreaterEqual(win.formula_bar[1].get_bottom()[1],win.rect('B8')[1])
     self.assertGreaterEqual(win.get_left()[0],-scene.camera.frame_width/2)
     self.assertLessEqual(win.get_right()[0],scene.camera.frame_width/2)
     self.assertAlmostEqual(float(scene.vs.a[(scene.T,cell)]),row['value'],delta=max(0.01,abs(row['value'])*0.001))
     edited=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color='#000000',background_opacity=1)
     edited.capture_mobjects([win]);a=np.asarray(edited.get_image(),dtype=int)
     fresh=scene._window(scene.T,tuple(row['rows']),row['cell'],'A:B');fresh.update(0)
     oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color='#000000',background_opacity=1)
     oracle.capture_mobjects([fresh]);b=np.asarray(oracle.get_image(),dtype=int)
     left,top,right,bottom=win.rect(cell);fw=scene.camera.frame_width;fh=scene.camera.frame_height
     x0=int((left/fw+.5)*320)+1;x1=int((right/fw+.5)*320)-1;y0=int((.5-top/fh)*180)+1;y1=int((.5-bottom/fh)*180)-1
     self.assertLessEqual(int(np.abs(a[y0:y1,x0:x1]-b[y0:y1,x0:x1]).max()),3,(lang,cell))
     ref_assignment=next(n for n in method.body if isinstance(n,ast.Assign) and assignment.lineno<n.lineno<stage.lineno
                         and any(isinstance(t,ast.Name) and t.id=='refs' for t in n.targets))
     exec(compile(ast.fix_missing_locations(ast.Module(body=[ref_assignment],type_ignores=[])),r.__file__,'exec'),env)
     refs=env['refs']
     self.assertEqual(len(refs.submobjects),len(row['refs']),(lang,cell))
     extra=None
     if row['stage']=='annual':
      self.assertEqual(scene._show(scene.T,'B15'),CASE['b15'])
      left,top,right,bottom=win.rect('B15')
      label=r._visuals._workbook._put(CASE['b15'],right-.12,(top+bottom)/2,'r',win.font,r._visuals._workbook.CELL_INK)
      paper=r._engine.Rectangle(width=right-left,height=top-bottom,stroke_width=0,fill_color='#FFFFFF',fill_opacity=1).move_to([(left+right)/2,(top+bottom)/2,0])
      full=Camera(pixel_width=854,pixel_height=480,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color='#000000',background_opacity=1)
      full.capture_mobjects([win]);actual=np.asarray(full.get_image(),dtype=int)
      full.reset();full.capture_mobjects([paper,label]);ideal=np.asarray(full.get_image(),dtype=int)
      fx0=int((left/fw+.5)*854)+2;fx1=int((right/fw+.5)*854)-2
      fy0=int((.5-top/fh)*480)+2;fy1=int((.5-bottom/fh)*480)-2
      self.assertLessEqual(int(np.abs(actual[fy0:fy1,fx0:fx1]-ideal[fy0:fy1,fx0:fx1]).max()),3,lang)
      extra_assignment=next(n for n in method.body if isinstance(n,ast.Assign) and assignment.lineno<n.lineno<stage.lineno
                            and any(isinstance(t,ast.Name) and t.id=='per_area' for t in n.targets))
      exec(compile(ast.fix_missing_locations(ast.Module(body=[extra_assignment],type_ignores=[])),r.__file__,'exec'),env)
      extra=env['per_area']
      extra_fade=next(n for n in ast.walk(stage) if isinstance(n,ast.Call) and isinstance(n.func,ast.Attribute) and n.func.attr=='FadeIn'
                      and n.args and isinstance(n.args[0],ast.Name) and n.args[0].id=='per_area')

     fade=next(n for n in ast.walk(stage) if isinstance(n,ast.Call) and isinstance(n.func,ast.Attribute) and n.func.attr=='FadeIn'
               and n.args and isinstance(n.args[0],ast.Name) and n.args[0].id=='refs')
     scene.frames=[];scene.win=win;scene.add(win)
     animations=[eval(compile(ast.Expression(fade),r.__file__,'eval'),{'_engine':r._engine,'refs':refs})]
     if extra is not None:animations.append(eval(compile(ast.Expression(extra_fade),r.__file__,'eval'),{'_engine':r._engine,'per_area':extra}))
     scene.play(*animations,run_time=.34)
     if extra is not None:
      full.reset();full.capture_mobjects([win,refs,extra]);framed=np.asarray(full.get_image(),dtype=int)
      prefix=r._visuals._workbook._label('14',win.font,r._visuals._workbook.CELL_INK)
      px0=max(fx0,int((label.get_left()[0]/fw+.5)*854)-1)
      px1=min(fx1,px0+int(np.ceil(prefix.width/fw*854))+2)
      expected_dark=(ideal[fy0:fy1,px0:px1,:3].max(axis=2)<120)
      actual_dark=(framed[fy0:fy1,px0:px1,:3].max(axis=2)<120)
      self.assertGreater(int(expected_dark.sum()),2,lang)
      self.assertFalse(np.any(expected_dark & ~actual_dark),(lang,'leading digits'))
     cleanup=next(n for n in method.body if isinstance(n,ast.Expr) and n.lineno>stage.lineno
                  and any(isinstance(x,ast.Name) and x.id=='refs' for x in ast.walk(n))
                  and isinstance(n.value,ast.Call))
     env['phi']=r._engine.VGroup()
     exec(compile(ast.fix_missing_locations(ast.Module(body=[cleanup],type_ignores=[])),r.__file__,'exec'),env)
     remaining={id(x) for x in scene.get_mobject_family_members()}
     if {id(x) for x in refs.get_family()} & remaining:
      self.assertTrue(any(x is refs for x in scene.frames))
      replacement=scene._window(scene.T,(8,16),'B10','A:B')
      scene.play(*scene._swap(replacement),run_time=.34)
     remaining={id(x) for x in scene.get_mobject_family_members()}
     self.assertFalse({id(x) for x in refs.get_family()} & remaining)
     if extra is not None:self.assertFalse({id(x) for x in extra.get_family()} & remaining)

if __name__=='__main__':unittest.main()
