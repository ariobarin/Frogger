"""Compare two folders of same-named PNG frames pixel by pixel. Usage: compare.py <a dir> <b dir> [diff dir]"""
import sys
from pathlib import Path

from PIL import Image, ImageChops

a_dir, b_dir = Path(sys.argv[1]), Path(sys.argv[2])
diff_dir = Path(sys.argv[3]) if len(sys.argv) > 3 else None
names = sorted(p.name for p in a_dir.glob("*.png"))
missing = sorted(set(names) ^ set(p.name for p in b_dir.glob("*.png")))
total = differing_frames = 0
for name in names:
    if not (b_dir / name).exists():
        continue
    a = Image.open(a_dir / name).convert("RGB")
    b = Image.open(b_dir / name).convert("RGB")
    diff = ImageChops.difference(a, b)
    bbox = diff.getbbox()
    total += 1
    if bbox:
        differing_frames += 1
        count = sum(1 for p in diff.get_flattened_data() if p != (0, 0, 0))
        print(f"{name}: {count} pixels differ, within {bbox}, max {max(max(c) for c in diff.getextrema())}")
        if diff_dir:
            diff_dir.mkdir(exist_ok=True)
            Image.merge("RGB", [ch.point(lambda v: 255 if v else 0) for ch in diff.split()]).save(diff_dir / name)
print(f"{total} frames compared, {differing_frames} differ" + (f"; only in one folder: {missing}" if missing else ""))
