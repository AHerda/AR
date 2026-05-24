#!/usr/bin/env python3

import sys

current_word = None
docs = set()

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    parts = line.split("\t")
    if len(parts) != 2:
        continue
    word, doc = parts

    if word != current_word:
        if current_word is not None:
            print(f"{current_word} -> [{', '.join(sorted(docs))}]")
        current_word = word
        docs = {doc}
    else:
        docs.add(doc)

if current_word is not None:
    print(f"{current_word} ->[{', '.join(sorted(docs))}]")
