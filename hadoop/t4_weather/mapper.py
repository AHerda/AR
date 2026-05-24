#!/usr/bin/env python3
import sys

year_filter = sys.argv[1] if len(sys.argv) > 1 else "2025"

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    parts = line.split(",")
    if len(parts) < 4:
        continue

    station, date, measure_type, value = parts[0], parts[1], parts[2], parts[3]

    if date == "DATE":
        continue

    if not date.startswith(year_filter):
        continue

    if measure_type not in ("TMAX", "TMIN", "TAVG") or station != "ACW00011647":
        continue

    try:
        temp = float(value)
    except ValueError:
        continue

    if temp == 9999:
        continue

    temp = temp / 10.0
    month = date[4:6]
    print(f"{month} {measure_type} {temp}")
