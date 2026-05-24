#!/usr/bin/env python3
import sys

current_key = None
local_min = 100.0
local_max = 0.0
local_sum = 0.0
local_count = 0


def emit(key, mn, mx, s, c):
    print(f"{key.split()[0]} {mn} {mx} {s} {c}")


for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    parts = line.split()
    if len(parts) != 3:
        continue

    month, measure_type, temp = parts[0], parts[1], float(parts[2])
    key = f"{month} {measure_type}"

    if key != current_key:
        if current_key is not None:
            emit(current_key, local_min, local_max, local_sum, local_count)
        current_key = key
        local_min = 100.0
        local_max = 0.0
        local_sum = 0.0
        local_count = 0

    if measure_type == "TMIN":
        local_min = min(local_min, temp)
    if measure_type == "TMAX":
        local_max = max(local_max, temp)
    if measure_type == "TAVG":
        local_sum += temp
        local_count += 1

if current_key is not None:
    emit(current_key, local_min, local_max, local_sum, local_count)
