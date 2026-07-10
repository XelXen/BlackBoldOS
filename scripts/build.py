#!/usr/bin/env python3
"""Build BlackBoldOS kernel + prepare ESP dir for QEMU fat boot."""
import pathlib, shutil, subprocess, sys, urllib.request, tarfile, os

ROOT = pathlib.Path(__file__).resolve().parent.parent
TARGET = ROOT / "target" / "x86_64-unknown-none" / "debug" / "blackboldos"
ESP = ROOT / "esp"
LIMINE_DIR = ROOT / "limine-bin"
BOOTX64 = LIMINE_DIR / "BOOTX64.EFI"
LIMINE_URL = "https://github.com/Limine-Bootloader/Limine/releases/latest/download/limine-binary.tar.gz"

def run(cmd):
    print("+", " ".join(str(c) for c in cmd))
    r = subprocess.run(cmd, cwd=ROOT)
    if r.returncode != 0:
        sys.exit(r.returncode)

def ensure_kernel():
    run([sys.executable, "-m", "cargo", "build"] if False else ["cargo", "build"])
    # cargo build via shell to use .cargo config
    if not TARGET.exists():
        # try with explicit target (config already sets it)
        run(["cargo", "build"])
    assert TARGET.exists(), "kernel ELF missing"

def ensure_limine():
    if BOOTX64.exists():
        return
    LIMINE_DIR.mkdir(exist_ok=True)
    tgz = LIMINE_DIR / "limine-binary.tar.gz"
    if not tgz.exists():
        print(f"downloading limine {LIMINE_URL}")
        urllib.request.urlretrieve(LIMINE_URL, tgz)
    print("extracting BOOTX64.EFI")
    with tarfile.open(tgz, "r:gz") as tf:
        for m in tf.getmembers():
            if m.name.endswith("BOOTX64.EFI"):
                f = tf.extractfile(m)
                assert f
                BOOTX64.write_bytes(f.read())
                break
    assert BOOTX64.exists(), "BOOTX64.EFI not found in tar"

def make_esp():
    if ESP.exists():
        shutil.rmtree(ESP)
    (ESP / "EFI" / "BOOT").mkdir(parents=True)
    (ESP / "boot" / "limine").mkdir(parents=True)
    shutil.copy(BOOTX64, ESP / "EFI" / "BOOT" / "BOOTX64.EFI")
    shutil.copy(TARGET, ESP / "boot" / "blackboldos.elf")
    # use repo limine.conf
    src_conf = ROOT / "limine.conf"
    shutil.copy(src_conf, ESP / "boot" / "limine" / "limine.conf")
    print(f"esp ready at {ESP}")

if __name__ == "__main__":
    # build kernel first
    print("== cargo build ==")
    r = subprocess.run(["cargo", "build"], cwd=ROOT)
    if r.returncode != 0:
        sys.exit(r.returncode)
    ensure_limine()
    make_esp()
    print("build ok. run with:")
    print("  python scripts/run.py   (or see README)")
