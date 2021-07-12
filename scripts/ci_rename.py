import glob
import os
import sys

proj = "rust-multiline-textdraw"

args = sys.argv[1:]

if len(args) == 2:
    rust, target = args
    mode = "--debug"
else:
    rust, mode, target = args

filenames = glob.glob(f"./target/**/{proj}")
if not os.path.isdir("builds"):
    os.mkdir("builds")

for filename in filenames:
    dest = "./builds/" + "-".join([proj, rust, mode[2:], target])
    os.rename(filename, dest)
