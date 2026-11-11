"""🐍️ Independent model of the guard (stdlib only): prints the open files ceiling of every fixture row, and counts the open descriptors of a pid the way `lsof` lists them (a second tool, not the library under test).
usage: python3 -I 🐍️.py ceilings <fixtures.json> | descriptors <pid> | alive <pid>"""
import json,os,subprocess,sys
DEFAULT,MINIMUM,VARIABLE=8192,256,"SEMIO_TEST_OPEN_FILES_CEILING"
def ceiling(environment):
    text=environment.get(VARIABLE)
    if text is None: return DEFAULT
    try:
        value=float(text) if text.strip() else 0.0
        return int(value) if value==int(value) and value>=MINIMUM else DEFAULT
    except (ValueError,OverflowError): return DEFAULT
def descriptors(pid):
    out=subprocess.run(["lsof","-p",str(pid),"-n","-P","-w","-Ff"],capture_output=True,text=True).stdout
    return sum(1 for line in out.splitlines() if line[:1]=="f" and line[1:].isdigit())
def alive(pid):
    try: os.kill(pid,0); return True
    except ProcessLookupError: return False
    except PermissionError: return True
if sys.argv[1]=="alive": print("1" if alive(int(sys.argv[2])) else "0")
elif sys.argv[1]=="ceilings": print(json.dumps([ceiling(row["environment"]) for row in json.load(open(sys.argv[2]))["ceilings"]]))
else: print(descriptors(int(sys.argv[2])))
