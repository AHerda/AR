#!/usr/bin/env python3
import sys

current_col = None
matrix_entries = []
vector_value = None
row_sums = {}
total_rows = 0
vector_size = 0


def process(entries, vec_val):
    if vec_val is None:
        return
    vec_val = float(vec_val)
    for row, val in entries:
        row = int(row)
        row_sums[row] = row_sums.get(row, 0) + float(val) * vec_val


for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    parts = line.split(" ")
    if len(parts) < 3 or len(parts) > 4:
        continue

    col = parts[0]
    kind = parts[1]

    if col != current_col:
        if current_col is not None:
            process(matrix_entries, vector_value)
        current_col = col
        matrix_entries = []
        vector_value = None

    if kind == "M":
        row = int(parts[2])
        total_rows = max(total_rows, row + 1)
        matrix_entries.append((parts[2], parts[3]))
    elif kind == "V":
        vector_size += 1
        vector_value = parts[2]

process(matrix_entries, vector_value)

total_rows = max(total_rows, vector_size)

for i in range(total_rows):
    print(f"{i} {row_sums.get(i, 0.0)}")
