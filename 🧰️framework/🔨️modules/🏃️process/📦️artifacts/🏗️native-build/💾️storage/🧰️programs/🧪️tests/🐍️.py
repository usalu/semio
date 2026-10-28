"""🐍️ Independent oracle for storage programs."""
import os,sys
for path in sys.argv[1:]:
    print(path, "ok" if os.path.isfile(path) and os.access(path,os.X_OK) else "no")
