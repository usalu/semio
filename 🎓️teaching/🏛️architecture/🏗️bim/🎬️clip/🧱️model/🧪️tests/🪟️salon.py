"""🪟 Original outboard salon bay, fixed-panel guard and folded canopy interfaces."""
import importlib,json,math
from pathlib import Path
a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def test_original_bay_dimensions_outboard_plane_fixed_guard_and_fold_contact():
    """🏛️ Check original dimensional registration and actual material interfaces independently."""
    from shapely.geometry import Polygon,Point
    from scipy.spatial import ConvexHull
    from scipy.optimize import linprog
    import numpy as np
    c=json.loads(Path(__file__).with_suffix(".json").read_text());m=a.load_model();f=a._filling_geometry(m,a._find(m,"GL-TERRACE"));assert abs(f["rough_width"]-c["width"])<1e-8;assert abs(f["rough_height"]-c["rough_height"])<1e-8;assert abs(f["overall_height"]-c["overall_height"])<1e-8
    glass=[p for p in f["parts"] if p["role"]=="glass"];fixed=[p for p in glass if p["panel_index"]==0];assert all(abs(sum(v[1] for v in p["outline"])/len(p["outline"])-c["glass_plane"])<1e-8 for p in fixed);assert max(p["z1"] for p in glass)<=6.34+1e-8
    column=a.column_geometry(m,a._find(m,"7-FIRST"));assert any(a.part_regions(p,c["column_probe"][2]) for p in column["parts"])
    def overlap(p,q):
        u,v=ConvexHull(a.part_vertices(p)),ConvexHull(a.part_vertices(q));planes=np.concatenate((u.equations,v.equations));fit=linprog([0,0,0,-1],A_ub=np.column_stack((planes[:,:3],np.ones(len(planes)))),b_ub=-planes[:,3],bounds=[(None,None)]*3+[(0,None)],method="highs");return fit.success and fit.x[3]>1e-8
    assert not any(overlap(p,q) for p in f["parts"] for q in column["parts"]),"Glazing assembly intersects retained post7"
    rail=a.solid_geometry("SALON-HANDRAIL",m);shell=[p for p in rail["parts"] if p["role"]=="hollow_handrail_shell"];assert abs((min(p["z0"] for p in shell)+max(p["z1"] for p in shell))/2-3.17-c["rail_center_height"])<1e-8;assert max(v[0] for p in shell for v in a.part_vertices(p))<14.25;assert min(v[0] for p in shell for v in a.part_vertices(p))<9.7
    canopy=a.solid_geometry("SALON-CANOPY",m);assert len(canopy["parts"])==3;beam=a.solid_geometry("BEAM-PRIMARY-R-6-34",m);assert not any(overlap(p,q) for p in canopy["parts"] for q in beam["parts"]),"Fold duplicates actual RC beam"
    assert abs(min(v[1] for p in canopy["parts"] for v in a.part_vertices(p))-c["canopy_beam_face"])<1e-8;assert abs(max(v[1] for p in canopy["parts"] for v in a.part_vertices(p))-c["glass_plane"]-c["canopy_projection"])<1e-8
    underside=[Polygon(r) for p in canopy["parts"] if abs(p["z0"]-c["canopy_frame_contact"])<1e-8 for r in a.part_regions(p,c["canopy_frame_contact"]+1e-8)];head=[Polygon(p["outline"]) for p in f["parts"] if abs(p["z1"]-c["canopy_frame_contact"])<1e-8];assert sum(p.intersection(q).area for p in underside for q in head)>.01

    expected=sum(Polygon(profile).area*c["width"] for profile in c["fold_profiles"]);assert abs(a.quantities("SALON-CANOPY",m)["volume"]-expected)<1e-8
    outside=[Polygon(a.part_section(p,c["canopy_beam_face"]+1e-8,1)) for p in canopy["parts"] if a.part_section(p,c["canopy_beam_face"]+1e-8,1)];inside=[Polygon(a.part_section(p,c["canopy_beam_face"]-1e-8,1)) for p in beam["parts"] if a.part_section(p,c["canopy_beam_face"]-1e-8,1)];assert sum(p.intersection(q).area for p in outside for q in inside)>.5
