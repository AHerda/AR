#!/usr/bin/env python3
import random
import sys

rows = int(sys.argv[1]) if len(sys.argv) > 1 else 5
cols = int(sys.argv[2]) if len(sys.argv) > 2 else 5
density = float(sys.argv[3]) if len(sys.argv) > 3 else 0.4

with open("../input/t2", "w") as f:
    for i in range(rows):
        for j in range(cols):
            if random.random() < density:
                value = round(random.uniform(-1 * cols, cols), 2)
                f.write(f"M {i} {j} {value}\n")

    for j in range(cols):
        value = random.randint(-1 * cols, cols)
        f.write(f"V {j} {value}\n")

print(
    f"Generated {rows}x{cols} matrix with ~{int(density * 100)}% density + vector of size {cols} -> input.txt"
)
