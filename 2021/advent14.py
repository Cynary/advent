#!/usr/bin/python3
"""
Day 14 of advent of code 2021
"""

import collections

def part1(fname, steps=10):
    """
    Part 1 of the problem.
    """
    transformations = {}
    with open(fname, encoding="utf-8") as f:
        polymer = f.readline().strip()
        f.readline()

        transformations = {
            pair: middle for (pair,middle) in map(lambda x: x.strip().split(" -> "), f.readlines())
        }

    current_polymer_pairs = collections.defaultdict(int)
    for i in range(len(polymer)-1):
        current_polymer_pairs[polymer[i:i+2]] += 1

    for _ in range(steps):
        new_polymer_pairs = collections.defaultdict(int)
        for pair, count in current_polymer_pairs.items():
            assert pair in transformations, pair
            new_polymer_pairs[pair[0] + transformations[pair]] += count
            new_polymer_pairs[transformations[pair] + pair[1]] += count
        current_polymer_pairs = new_polymer_pairs

    counts = collections.defaultdict(int)

    # The very last element isn't counted because we only take into account the first element in
    # every pair, so count it here.
    counts[polymer[-1]] += 1
    for pair, count in current_polymer_pairs.items():
        counts[pair[0]] += count

    return max(counts.values()) - min(counts.values())

assert part1("examples/141.txt") == 1588
print(part1("input/14.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    return part1(fname, 40)

assert part2("examples/141.txt") == 2188189693529
print(part2("input/14.txt"))
