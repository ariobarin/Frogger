# /// script
# dependencies = ["playwright", "pillow"]
# ///
"""Runs a parity script through the TeaVM build (FroggerFramesWeb, built into target/site) in Chrome and
compares each shot with the Java version's frame of the same name (jvm/FroggerFrames writes them).
A probe script compares the sprite scaling with jvm/ScaleProbe's output instead.

Usage (after `mvn -P parity package` in teavm/):
  uv run frames.py <script> <java frames dir>
  uv run frames.py probe <ScaleProbe output>
"""
import functools
import http.server
import sys
import threading
import zlib
from pathlib import Path

from PIL import Image
from playwright.sync_api import sync_playwright

SITE = Path(__file__).resolve().parent / "target" / "site"
# the Java version runs from the repo root, where it reads this scores file
SCORES = Path(__file__).resolve().parents[2] / "scores.txt"


class Quiet(http.server.SimpleHTTPRequestHandler):
    def log_message(self, *args):
        pass


def serve(folder):
    handler = functools.partial(Quiet, directory=str(folder))
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    return f"http://127.0.0.1:{server.server_port}/"


def run(script_text):
    lines = []
    with sync_playwright() as pw:
        browser = pw.chromium.launch(channel="chrome")
        page = browser.new_page()
        page.on("console", lambda m: lines.append(m.text))
        page.on("pageerror", lambda e: lines.append(f"page error: {e}"))
        page.route("**/script.txt", lambda route: route.fulfill(body=script_text))
        page.route("**/scores.txt", lambda route: route.fulfill(body=SCORES.read_text()))
        page.goto(serve(SITE) + "index.html?script=script.txt")
        while "done" not in lines:
            page.wait_for_timeout(200)
        browser.close()
    return lines


def main():
    script, reference = sys.argv[1], Path(sys.argv[2])
    if script == "probe":
        got = [line for line in run("probe\n") if ":" in line and not line.startswith("page error")]
        want = reference.read_text().splitlines()
        differ = sum(a != b for a, b in zip(got, want)) + abs(len(got) - len(want))
        print(f"{len(want)} scaling cases, {differ} differ")
        return
    lines = run(Path(script).read_text())
    shots = {}
    for line in lines:
        if line.startswith("shot "):
            _, name, crc = line.split()
            shots[name] = int(crc)
        elif line != "done":
            print(line)
    same = differ = missing = 0
    for name, crc in shots.items():
        png = reference / f"{name}.png"
        if not png.exists():
            missing += 1
            continue
        if zlib.crc32(Image.open(png).convert("RGB").tobytes()) == crc:
            same += 1
        else:
            differ += 1
            print(f"{name}: differs")
    print(f"{same + differ} frames compared, {differ} differ" + (f"; {missing} have no Java frame" if missing else ""))


if __name__ == "__main__":
    main()
