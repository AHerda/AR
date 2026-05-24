#!/usr/bin/env python3
import sys

current_j = None
a_entries = []
b_entries = []
results = {}


def process(a_entries, b_entries):
    for i, a_val in a_entries:
        for k, b_val in b_entries:
            key = (int(i), int(k))
            results[key] = results.get(key, 0) + float(a_val) * float(b_val)


for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    parts = line.split()
    if len(parts) != 4:
        continue

    j, kind, idx, value = parts[0], parts[1], parts[2], parts[3]

    if j != current_j:
        if current_j is not None:
            process(a_entries, b_entries)
        current_j = j
        a_entries = []
        b_entries = []

    if kind == "A":
        a_entries.append((idx, value))
    elif kind == "B":
        b_entries.append((idx, value))

process(a_entries, b_entries)

for i, k in sorted(results):
    print(f"C {i} {k} {results[(i, k)]}")
