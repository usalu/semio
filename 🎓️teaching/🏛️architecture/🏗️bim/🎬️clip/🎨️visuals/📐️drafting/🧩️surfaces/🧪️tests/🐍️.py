"""🧪 Independent exact material-skin and production cell-boundary contracts."""
import importlib,json,unittest,sys
sys.dont_write_bytecode=True
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import unary_union
from jsonschema import Draft202012Validator
PARENT="🎓️teaching.🏛️architecture.🏗️bim.🎬️clip"
skin=importlib.import_module(PARENT+".🎨️visuals.📐️drafting.🧩️surfaces.🐍️")
RESULTS={}

def box_facets(bounds,material="a"):
    """📦 Independently author six oriented rectangular box faces."""
    a,b=bounds; vertices=[[x,y,z]for z in (a[2],b[2])for y in (a[1],b[1])for x in (a[0],b[0])]
    faces=[([0,2,3,1],[0,0,-1]),([4,5,7,6],[0,0,1]),([0,1,5,4],[0,-1,0]),([2,6,7,3],[0,1,0]),([0,4,6,2],[-1,0,0]),([1,3,7,5],[1,0,0])]
    return [{"rings":[[vertices[i]for i in face]],"normal":normal,"material":material}for face,normal in faces]

def annular_facets(case):
    """🪈 Independently author closed radial annular cells with an actual finite bore."""
    outer,inner=case["outer"],case["inner"];lo,hi=case["z"];out=[]
    for index,a in enumerate(outer):
        j=(index+1)%len(outer);ring=[a,outer[j],inner[j],inner[index]]
        out.extend([{"rings":[[[x,y,lo]for x,y in reversed(ring)]],"normal":[0,0,-1],"material":"metal"},{"rings":[[[x,y,hi]for x,y in ring]],"normal":[0,0,1],"material":"metal"}])
        for a,b in zip(ring,ring[1:]+ring[:1]):
            dx,dy=b[0]-a[0],b[1]-a[1]
            out.append({"rings":[[[*a,lo],[*b,lo],[*b,hi],[*a,hi]]],"normal":[dy,-dx,0],"material":"metal"})
    return out

def oracle(facets):
    """📐 Group exact source planes but independently overlay both outward orientations with GEOS."""
    groups={}
    for facet in facets:
        key,axes,plane,sign=skin.plane_chart(facet)
        if key is None:continue
        polygon=Polygon([[float(p[i])for i in axes]for p in facet["rings"][0]],[[[float(p[i])for i in axes]for p in ring]for ring in facet["rings"][1:]])
        groups.setdefault((facet["material"],key),{1:[],-1:[]})[sign].append(polygon)
    return {(material,key,sign):unary_union(values[sign]).difference(unary_union([polygon for (other_material,other_key),other in groups.items()if other_key==key for polygon in other[-sign]]))for (material,key),values in groups.items()for sign in (1,-1)}

class MaterialSkin(unittest.TestCase):
    def certify(self,facets):
        expected=oracle(facets);actual=skin.material_skin(facets);grouped={}
        for facet in actual:
            key,axes,plane,sign=skin.plane_chart(facet)
            self.assertIsNotNone(key)
            polygon=Polygon([[p[i]for i in axes]for p in facet["rings"][0]],[[[p[i]for i in axes]for p in ring]for ring in facet["rings"][1:]])
            self.assertTrue(polygon.is_valid)
            grouped.setdefault((facet["material"],key,sign),[]).append(polygon)
        for key,want in expected.items():
            got=unary_union(grouped[key]) if grouped.get(key) else Polygon();self.assertLess(got.symmetric_difference(want).area,1e-12)
            if not want.is_empty:self.assertLess(got.boundary.symmetric_difference(want.boundary).length,1e-12)
            else:self.assertTrue(got.is_empty)
            self.assertAlmostEqual(sum(p.area for p in grouped.get(key,[])),want.area,places=12)
        return actual
    def test_language_neutral_skin_matches_independent_geos(self):
        path=Path(__file__).with_name("🔣️.json");fixture=json.loads(path.read_text(encoding="utf8"));schema=json.loads(path.with_name("🔣️.schema.json").read_text(encoding="utf8"));Draft202012Validator(schema).validate(fixture)
        for case in fixture["cases"]:
            with self.subTest(case=case["id"]):
                facets=annular_facets(case["annulus"]) if "annulus"in case else [facet for index,bounds in enumerate(case["boxes"])for facet in box_facets(bounds,case.get("materials",["a"]*len(case["boxes"]))[index])]
                output=self.certify(facets);RESULTS[case["id"]]={"input":len(facets),"output":len(output)}
    def test_tap_hall_cancelled_seams_node_crossing_edges(self):
        """🚰 Preserve the real coplanar material opening across canceled tile seams."""
        path=Path(__file__).with_name("🔣️tap.json")
        fixture=json.loads(path.read_text(encoding="utf-8"))
        schema=json.loads(path.with_name("🔣️tap.schema.json").read_text(encoding="utf-8"))
        Draft202012Validator(schema).validate(fixture)
        facets=fixture["facets"]
        positive=unary_union([Polygon([[p[1],p[2]]for p in f["rings"][0]])for f in facets if f["normal"][0]>0])
        negative=unary_union([Polygon([[p[1],p[2]]for p in f["rings"][0]])for f in facets if f["normal"][0]<0])
        expected=positive.difference(negative)
        actual=skin.material_skin(facets)
        polygons=[Polygon([[p[1],p[2]]for p in f["rings"][0]],[[[p[1],p[2]]for p in ring]for ring in f["rings"][1:]])for f in actual if f["normal"][0]>0]
        merged=unary_union(polygons)
        self.assertTrue(all(p.is_valid for p in polygons))
        self.assertLess(merged.symmetric_difference(expected).area,1e-14)
        self.assertLess(abs(merged.boundary.length-expected.boundary.length),1e-10)

    def test_actual_beam_projection_crossing_winding_matches_geos(self):
        """🧱 Preserve the exact captured beam boundary until its final screen conversion."""
        from fractions import Fraction
        from shapely.validation import make_valid
        primitives=importlib.import_module(PARENT+".🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives")
        path=Path(__file__).with_name("🔣️projection.json")
        fixture=json.loads(path.read_text(encoding="utf8"))
        Draft202012Validator(json.loads(path.with_name("🔣️projection.schema.json").read_text(encoding="utf8"))).validate(fixture)
        facets=fixture["facets"];matrix=fixture["matrix"]
        lookup={(float(Fraction(*map(int,p[0]))),float(Fraction(*map(int,p[1])))):
                (Fraction(*map(int,p[0])),Fraction(*map(int,p[1]))) for p in fixture["exact_ring"]}
        facets[0]["rings"][0]=[[*lookup[tuple(p[:2])],Fraction(p[2])] for p in facets[0]["rings"][0]]
        original=facets[0]["rings"][0]
        faces=[primitives._poly(f["rings"],None,"beam") for f in facets]
        prim=primitives.Prim("beam",{},0,(0,0),faces,skin=tuple(f["material"] for f in facets))
        self.assertEqual(prim.faces[0][1][0],list(map(tuple,original)))
        axon=importlib.import_module(PARENT+".🎨️visuals.🏛️savoye.🧊️axon.🐍️")
        lifted=axon._lift(prim,0.,.5)
        self.assertEqual(lifted.faces[0][1][0][0],(original[0][0],original[0][1],original[0][2]/2))
        projected=[{"rings":face[1],"material":material} for face,material in zip(prim.faces,prim.skin)]
        result=skin.projected_material_regions(projected,matrix)
        self.assertTrue(result)
        self.assertTrue(all(isinstance(v,Fraction) for r in result for ring in r["rings"] for point in ring for v in point))
        def project(ring):
            return [[sum(float(p[i])*matrix[i][j] for i in range(3)) for j in range(2)] for p in ring]
        def polys(shape):
            if shape.geom_type=="Polygon":return [shape]
            return [p for child in getattr(shape,"geoms",()) for p in polys(child)]
        expected=unary_union([p for f in facets for p in polys(make_valid(Polygon(project(f["rings"][0]),[project(r) for r in f["rings"][1:]])))])
        actual=unary_union([p for r in result for p in polys(make_valid(Polygon(r["rings"][0],r["rings"][1:])))])
        self.assertLess(actual.symmetric_difference(expected).area,1e-10)
        def cross(a,b,c):
            return (b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])
        def on(a,b,p):
            return cross(a,b,p)==0 and all(min(a[k],b[k])<=p[k]<=max(a[k],b[k]) for k in (0,1))
        for region in result:
            for ring in region["rings"]:
                for i in range(len(ring)):
                    a,b=ring[i],ring[(i+1)%len(ring)]
                    for j in range(i+1,len(ring)):
                        if j-i in (1,len(ring)-1):continue
                        c,d=ring[j],ring[(j+1)%len(ring)]
                        self.assertFalse(on(a,b,c) or on(a,b,d) or on(c,d,a) or on(c,d,b))
                        self.assertFalse(cross(a,b,c)*cross(a,b,d)<0 and cross(c,d,a)*cross(c,d,b)<0)

    def test_material_skin_preserves_exact_intersection_coordinates(self):
        """📐 Carry exact rational material vertices through the native surface output."""
        from fractions import Fraction
        facets=[
            {"rings":[[[0,0,0],[1,3,0],[0,3,0]]],"normal":[0,0,1],"material":{"id":"x"}},
            {"rings":[[[0,1,0],[2,1,0],[2,2,0],[0,2,0]]],"normal":[0,0,1],"material":{"id":"x"}},
        ]
        actual=skin.material_skin(facets)
        self.assertTrue(any(p[0]==Fraction(1,3) and isinstance(p[0],Fraction)
                            for f in actual for ring in f["rings"] for p in ring))

    def test_axon_centres_exact_coordinates_before_screen_conversion(self):
        """🔭 Preserve a sub-float model offset when centering the view."""
        from fractions import Fraction
        axon=importlib.import_module(PARENT+".🎨️visuals.🏛️savoye.🧊️axon.🐍️")
        view=axon.AxonView((0,0,0),1,(10**16,0,0),0)
        point=(Fraction(10**16)+Fraction(1,3),Fraction(0),Fraction(0))
        actual=view.project([point])[0]
        expected=view.M[0]/3
        self.assertAlmostEqual(actual[0],expected[0],places=14)
        self.assertAlmostEqual(actual[1],expected[1],places=14)
        self.assertTrue((view(*point)==actual).all())

    def test_progress_cancellation_and_owned_output(self):
        facets=box_facets([[0,0,0],[1,1,1]]);events=[];one=skin.material_skin(facets,progress=events.append);one[0]["rings"][0][0][0]=900
        self.assertNotEqual(one,skin.material_skin(facets));self.assertTrue(events)
        with self.assertRaises(skin.SurfaceCancelled):skin.material_skin(facets,cancelled=lambda:True)

    def test_actual_source_gravel_production_boundary(self):
        root=next(p for p in Path(__file__).resolve().parents if(p/"AGENTS.md").is_file()and(p/"package.json").is_file());source=root/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json"
        model=json.loads(source.read_text(encoding="utf8"));element=next(e for e in model["elements"]if e["tag"]=="FIN-PILOTIS-GRAVEL")
        primitives=importlib.import_module(PARENT+".🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives")
        item={**element,**element["geometry"]};item["parts"]=[item["parts"][i]for i in (0,21)]
        actual=primitives._surface_prims(item,{},3.17,6.60,0)
        upper=[face for prim in actual for face in prim.faces if face[0]=="poly"and face[2][2]>.99 and all(p[2]==-.15 for p in face[1][0])]
        expected=unary_union([Polygon([[p[0],p[1]]for p in part["mesh"]["vertices"] if p[2]==-.15])for part in item["parts"]])
        polygons=[Polygon([[p[0],p[1]]for p in face[1][0]],[[[p[0],p[1]]for p in ring]for ring in face[1][1:]])for face in upper]
        self.assertLess(sum(p.boundary.length for p in polygons)-expected.boundary.length,1e-12)
        self.assertLess(unary_union(polygons).symmetric_difference(expected).area,1e-12)

    def test_language_neutral_visible_material_edges_match_geos(self):
        from shapely.geometry import LineString
        path=Path(__file__).with_name("🔣️.json");fixture=json.loads(path.read_text(encoding="utf8"));schema=json.loads(path.with_name("🔣️.schema.json").read_text(encoding="utf8"));Draft202012Validator(schema).validate(fixture)
        for case in fixture["visible_cases"]:
            with self.subTest(case=case["id"]):
                regions=skin.projected_material_regions(case["facets"],((1,0),(0,1),(0,0)))
                edges=[{"points":[a,b],"material":r["material"]}for r in regions for ring in r["rings"]for a,b in zip(ring,ring[1:]+ring[:1])]
                for material in {f["material"]for f in case["facets"]}:
                    expected=unary_union([Polygon([p[:2]for p in f["rings"][0]])for f in case["facets"]if f["material"]==material]).boundary
                    actual=unary_union([LineString([p[:2]for p in edge["points"]])for edge in edges if edge["material"]==material])
                    self.assertEqual(actual.symmetric_difference(expected).length,0)
                    self.assertEqual(sum(LineString([p[:2]for p in e["points"]]).length for e in edges if e["material"]==material),expected.length)
        with self.assertRaises(skin.SurfaceCancelled):skin.projected_material_regions(fixture["visible_cases"][0]["facets"],((1,0),(0,1),(0,0)),cancelled=lambda:True)

    def test_actual_painter_inks_only_visible_material_regions_and_reuses_native_boundaries(self):
        import ast,math
        from fractions import Fraction
        from types import SimpleNamespace
        import numpy as np
        from shapely.geometry import LineString
        primitives=importlib.import_module(PARENT+".🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives")
        source=Path(primitives.__file__).with_name("🐍️.py");tree=ast.parse(source.read_text(encoding="utf8"));painter=next(n for n in tree.body if isinstance(n,ast.ClassDef)and n.name=="_Painter");body=next(n for n in painter.body if isinstance(n,ast.FunctionDef)and n.name=="_skin")
        records=[]
        class Group:
            def __init__(self):self.items=[]
            def add(self,*items):self.items.extend(items);return self
        def multi(rings,fill,opacity,stroke,width,stroke_opacity):
            records.append(("fill",rings,stroke,width));return object()
        def segments(pairs,stroke,width,opacity):
            records.append(("ink",pairs,stroke,width));return object()
        def region(rings,fill,opacity,stroke,width,stroke_opacity):
            records.append(("fill",rings,stroke,width));return object()
        class View:
            turns=0;centre=np.zeros(3);point=np.zeros(3);scale=1.
            def facing(self,n):return -n[0]-n[1]+n[2]
            def project(self,ring,dz=0):
                a=np.asarray(ring);return np.column_stack((.866*(a[:,0]-a[:,1]),.5*(a[:,0]+a[:,1])+a[:,2]+dz,np.zeros(len(a))))
        namespace={"np":np,"math":math,"Fraction":Fraction,"_engine":SimpleNamespace(VGroup=Group),"_primitives":primitives,"_multi":multi,"_d":SimpleNamespace(region=region,segments=segments,material_fill=lambda actor:actor),"brightness":lambda n,t:.8}
        exec(compile(ast.Module(body=[body],type_ignores=[]),str(source),"exec"),namespace)
        bounds=[[[0,0,0],[1,1,1]],[[1,0,0],[2,1,1]]];facets=[f for b in bounds for f in box_facets(b)]
        native=skin.material_skin(facets);prim=primitives.Prim("fixture",{},0,(0,0),[("poly",f["rings"],f["normal"],"gravel")for f in native],skin=tuple(f["material"]for f in native))
        view=View();self_view=SimpleNamespace(view=view,base=lambda p,r,l:"#aaa",style=lambda *args:("normal","#aaa",1.,"#111",1.,.8))
        namespace["_skin"](self_view,prim,"architecture",0)
        self.assertTrue(prim.outlines);self.assertTrue(all(stroke is None and width==0 for kind,rings,stroke,width in records if kind=="fill"))
        self.assertTrue(all(sum(a[0]*b[1]-a[1]*b[0]for a,b in zip(ring,list(ring[1:])+[ring[0]]))>0 for kind,rings,stroke,width in records if kind=="fill"for ring in rings))
        emitted=unary_union([LineString((a[:2],b[:2]))for kind,pairs,stroke,width in records if kind=="ink"for a,b in pairs])
        visible=[f for f in native if view.facing(f["normal"])>1e-6];expected=unary_union([Polygon(view.project(f["rings"][0])[:,:2],list(view.project(r)[:,:2]for r in f["rings"][1:]))for f in visible]).boundary
        self.assertLess(emitted.symmetric_difference(expected).length,1e-12)
        first=next(iter(prim.outlines.values()));namespace["_skin"](self_view,prim,"architecture",0);self.assertIs(next(iter(prim.outlines.values())),first)
        original=skin.projected_material_regions;calls=[]
        try:
            def replacement(*args,**kwargs):
                calls.append(True);return original(*args,**kwargs)
            skin.projected_material_regions=replacement;namespace["_skin"](self_view,prim,"architecture",0);self.assertEqual(len(calls),1)
        finally:skin.projected_material_regions=original

    def test_noncovering_annular_tube_uses_the_actual_production_skin_dispatch(self):
        root=next(p for p in Path(__file__).resolve().parents if(p/"AGENTS.md").is_file()and(p/"package.json").is_file());model=json.loads((root/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json").read_text(encoding="utf8"));element=next(e for e in model["elements"]if e["tag"]=="TUBE-STAIR-CENTRAL")
        primitives=importlib.import_module(PARENT+".🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives");items={e["tag"]:e for e in model["elements"]}
        slabs=[]
        for role,level in (("floor","first"),("roof","roof")):
            e=items[primitives._c.role(role,model)];slabs.append({"tag":e["tag"],"guid":e["guid"],"level":level,"storey":level,"parts":[{"outline":[[0,0],[1,0],[1,1],[0,1]],"z0":3.,"z1":3.1}]})
        item={**element,**element["geometry"],"domains":element["metadata"]["domains"]}
        g={"box":{"length":19.,"width":21.5},"storeys":{"ground":0.,"first":3.17,"roof":6.6},"pilotis":[],"slabs":slabs,"walls":[],"fillings":[],"railings":[item],"services":[]}
        pieces=[p for p in primitives._build(g,model)if p.tag==element["tag"]]
        self.assertTrue(pieces);self.assertTrue(all(p.skin is not None for p in pieces))
        z=max(p[2]for part in item["parts"]for f in primitives._part_faces(part,"metal")for p in f[1][0])
        source=[Polygon([p[:2]for p in f[1][0]])for part in item["parts"]for f in primitives._part_faces(part,"metal")if f[2][2]>.99 and all(p[2]==z for p in f[1][0])]
        actual=[Polygon([p[:2]for p in f[1][0]],[[p[:2]for p in r]for r in f[1][1:]])for prim in pieces for f in prim.faces if f[2][2]>.99 and all(p[2]==z for p in f[1][0])]
        expected=unary_union(source);self.assertGreater(len(expected.interiors),0)
        self.assertEqual(unary_union(actual).symmetric_difference(expected).area,0)
        self.assertLess(abs(sum(p.boundary.length for p in actual)-expected.boundary.length),1e-12)
