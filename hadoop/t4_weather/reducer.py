#!/usr/bin/env python3
import sys

current_key = None
global_min = 100.0
global_max = 0.0
global_sum = 0.0
global_count = 0


print("key,avg,min,max")


def emit(key):
    if global_count != 0:
        avg = global_sum / global_count
        print(f"{key},{avg:.2f},{global_min},{global_max}")
    else:
        print(f"{key},xxx {global_sum} xxx,{global_min},{global_max}")


for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    parts = line.split()
    if len(parts) != 5:
        continue

    key = parts[0]
    mn, mx, s, c = float(parts[1]), float(parts[2]), float(parts[3]), int(parts[4])

    if key != current_key:
        if current_key is not None:
            emit(current_key)
        current_key = key
        global_min = 100.0
        global_max = 0.0
        global_sum = 0.0
        global_count = 0

    global_min = min(global_min, mn)
    global_max = max(global_max, mx)
    global_sum += s
    global_count += c

if current_key is not None:
    emit(current_key)
