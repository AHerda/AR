#!/usr/bin/env python3
import sys

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    parts = line.split()
    if len(parts) != 4:
        continue

    kind, row, col, value = parts[0], parts[1], parts[2], parts[3]

    if kind == "A":
        i, j = row, col
        print(f"{j} A {i} {value}")

    elif kind == "B":
        j, k = row, col
        print(f"{j} B {k} {value}")
