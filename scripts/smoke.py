#!/usr/bin/env python3
"""Smoke test: build, boot in QEMU, check banner + shell commands over serial."""
import pathlib, shutil, subprocess, sys, time, re, os

ROOT = pathlib.Path(__file__).resolve().parent.parent
ESP = ROOT / "esp"
SERIAL_LOG = ROOT / "serial.log"

def find_qemu():
    cands = [
        os.environ.get("QEMU"),
        r"C:\Program Files\qemu\qemu-system-x86_64.exe",
        "qemu-system-x86_64",
        "qemu-system-x86_64.exe",
    ]
    for c in cands:
        if not c:
            continue
        p = shutil.which(c) if not pathlib.Path(c).exists() else c
        if p and pathlib.Path(p).exists() or shutil.which(str(c)):
            return p if pathlib.Path(str(p or c)).exists() else shutil.which(str(c))
    # fallback to PATH lookup
    q = shutil.which("qemu-system-x86_64")
    if q:
        return q
    print("qemu not found")
    sys.exit(1)

def find_ovmf_code():
    cands = [
        ROOT / "ovmf" / "ovmf-code-x86_64.fd",
        pathlib.Path(r"C:\Program Files\qemu\share\edk2-x86_64-code.fd"),
        pathlib.Path("/usr/share/OVMF/OVMF_CODE.fd"),
        pathlib.Path("/usr/share/edk2/ovmf/OVMF_CODE.fd"),
        pathlib.Path("/usr/share/qemu/OVMF_CODE.fd"),
    ]
    for c in cands:
        if c and c.exists():
            return str(c)
    print("OVMF code not found, tried:", cands)
    sys.exit(1)

def build():
    import build as b
    # reuse build.py logic
    r = subprocess.run([sys.executable, "scripts/build.py"], cwd=ROOT)
    if r.returncode != 0:
        print("build failed")
        sys.exit(1)

def send_slow(proc, s, delay=0.015):
    for ch in s:
        proc.stdin.write(ch)
        proc.stdin.flush()
        time.sleep(delay)
    time.sleep(0.4)

def main():
    build()
    qemu = find_qemu()
    code = find_ovmf_code()
    print(f"qemu: {qemu}\novmf: {code}")
    if SERIAL_LOG.exists():
        SERIAL_LOG.unlink()
    cmd = [
        qemu, "-M", "q35",
        "-drive", f"if=pflash,unit=0,format=raw,file={code},readonly=on",
        "-drive", "format=raw,file=fat:rw:esp",
        "-serial", "stdio",
        "-display", "none",
        "-no-reboot", "-m", "512",
    ]
    print("+", " ".join(cmd))
    p = subprocess.Popen(cmd, cwd=ROOT, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                         stderr=subprocess.STDOUT, text=True, bufsize=0)
    import threading
    chunks = []
    def reader():
        try:
            while True:
                c = p.stdout.read(1)
                if not c:
                    break
                chunks.append(c)
        except Exception:
            pass
    t = threading.Thread(target=reader, daemon=True)
    t.start()
    time.sleep(5)
    checks = []
    def check(name, cond):
        print(f"{'PASS' if cond else 'FAIL'}: {name}")
        checks.append(cond)

    # wait for prompt
    time.sleep(1)
    send_slow(p, "help\n")
    send_slow(p, "echo smoke-ok\n")
    send_slow(p, "version\n")
    send_slow(p, "about\n")
    send_slow(p, "mem\n")
    send_slow(p, "\n")  # empty line should not crash
    send_slow(p, "echo " + "A"*150 + "\n")  # long line truncation
    time.sleep(1)
    p.terminate()
    try:
        p.wait(timeout=5)
    except Exception:
        p.kill()
    time.sleep(0.5)
    out = "".join(chunks)
    SERIAL_LOG.write_text(out, errors="ignore")
    clean = re.sub(r"\x1b\[[0-9;=?]*[A-Za-z]", "", out)
    # print tail for debug
    idx = clean.find("BlackBoldOS")
    print("--- serial tail ---")
    print(clean[idx:idx+5000][-5000:] if idx != -1 else clean[-4000:])
    print("-------------------")
    check("banner BlackBoldOS", "BlackBoldOS" in clean)
    check("boot ok", "boot ok" in clean)
    check("idt ok", "idt ok" in clean)
    check("help lists", "commands:" in clean and "help" in clean)
    check("echo smoke-ok", "smoke-ok" in clean)
    check("version", "v0.1" in clean)
    check("about", "freestanding" in clean or "minimal" in clean)
    check("mem entries", "memmap" in clean and "usable total" in clean)
    check("empty line survived (still prompted)", clean.count("> ") >= 4)
    # long line should be truncated to 128, not crash; we sent 150 A's, expect at least 100 back
    check("long line truncated, no crash", clean.count("A") >= 100)
    if all(checks):
        print("SMOKE PASS")
        return 0
    print("SMOKE FAIL")
    return 1

if __name__ == "__main__":
    sys.exit(main())
