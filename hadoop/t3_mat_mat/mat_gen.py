#!/usr/bin/env python3
import random
import sys

rows1 = int(sys.argv[1]) if len(sys.argv) > 1 else 5
cols1_rows2 = int(sys.argv[2]) if len(sys.argv) > 2 else 5
cols2 = int(sys.argv[3]) if len(sys.argv) > 3 else 5
density = float(sys.argv[4]) if len(sys.argv) > 4 else 0.2


filename = "../input/t3"
with open(filename, "w") as f:
    for i in range(rows1):
        for j in range(cols1_rows2):
            if random.random() < density:
                value = round(random.uniform(-1 * cols1_rows2, cols1_rows2), 2)
                f.write(f"A {i} {j} {value}\n")

    for j in range(cols1_rows2):
        for k in range(cols2):
            if random.random() < density:
                value = round(random.uniform(-1 * cols1_rows2, cols1_rows2), 2)
                f.write(f"B {j} {k} {value}\n")

print(
    f"Generated {rows1}x{cols1_rows2} matrix and {cols1_rows2}x{cols2} with ~{int(density * 100)}% density-> {filename}"
)
