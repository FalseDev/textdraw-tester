import glob
import os
import sys

args = sys.argv[1:]

if len(args) == 2:
    rust, target = args
    mode = "--debug"
else:
    rust, mode, target = args

filenames = glob.glob("./target/**/rocket-trial")
if not os.path.isdir("builds"):
    os.mkdir("builds")

for filename in filenames:
    dest = "./builds/" + "-".join(["rocket-trial", rust, mode[2:], target])
    os.rename(filename, dest)
