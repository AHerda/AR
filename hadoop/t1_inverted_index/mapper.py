#!/usr/bin/env python3

import sys
import os
import re

filename = os.path.basename(os.environ.get("map_input_file", "unknown"))

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    words = re.sub(r'[^a-z\s]', '', line.lower()).split()
    for word in words:
        print(f"{word}\t{filename}")
