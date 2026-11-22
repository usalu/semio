"""🐍️ Independent model of the test discovery bound (stdlib only): counts the entries (files and directories, the unit bun reports) of a tree outside the directories the root bunfig ignores.
usage: python3 -I 🐍️.py ignored <root> <name>..."""
import os,sys
def ignored(root,names):
    total=0
    for current,directories,files in os.walk(root):
        if os.path.basename(current) in names or any(part in names for part in os.path.relpath(current,root).split(os.sep)):
            total+=len(files)+len(directories)+(os.path.basename(current) in names)
    return total
print(ignored(sys.argv[2],set(sys.argv[3:])))
