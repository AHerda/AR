#!/usr/bin/env python3
import sys

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    parts = line.split(" ")
    if len(parts) < 3 or len(parts) > 4:
        continue

    kind = parts[0]

    if kind == "M" and len(parts) == 4:
        row, col, value = parts[1], parts[2], parts[3]
        print(f"{col} M {row} {value}")

    elif kind == "V" and len(parts) == 3:
        row, value = parts[1], parts[2]
        print(f"{row} V {value}")
