#!/usr/bin/env python3
"""Run BlackBoldOS in QEMU interactively (serial on stdio)."""
import pathlib, shutil, subprocess, sys, os

ROOT = pathlib.Path(__file__).resolve().parent.parent

def find_qemu():
    cands = [
        r"C:\Program Files\qemu\qemu-system-x86_64.exe",
        "qemu-system-x86_64",
    ]
    for c in cands:
        if pathlib.Path(c).exists():
            return c
        q = shutil.which(c)
        if q:
            return q
    print("qemu not found")
    sys.exit(1)

def find_ovmf():
    cands = [
        ROOT / "ovmf" / "ovmf-code-x86_64.fd",
        pathlib.Path(r"C:\Program Files\qemu\share\edk2-x86_64-code.fd"),
        pathlib.Path("/usr/share/OVMF/OVMF_CODE.fd"),
        pathlib.Path("/usr/share/edk2/ovmf/OVMF_CODE.fd"),
    ]
    for c in cands:
        if c.exists():
            return str(c)
    print("ovmf not found")
    sys.exit(1)

if __name__ == "__main__":
    # ensure esp
    r = subprocess.run([sys.executable, "scripts/build.py"], cwd=ROOT)
    if r.returncode != 0:
        sys.exit(r.returncode)
    qemu = find_qemu()
    code = find_ovmf()
    cmd = [
        qemu, "-M", "q35",
        "-drive", f"if=pflash,unit=0,format=raw,file={code},readonly=on",
        "-drive", "format=raw,file=fat:rw:esp",
        "-serial", "stdio",
        "-display", "none",
        "-m", "512",
    ]
    print("+", " ".join(cmd))
    print("press Ctrl+C or close to exit, 'halt' to stop guest")
    os.execvp(cmd[0], cmd)
