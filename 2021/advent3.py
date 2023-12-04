#!/usr/bin/python3
"""
Day 3 of advent of code 2021
"""
import collections

def part1(fname):
    """
    Part 1 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        counts = collections.defaultdict(lambda: [0, 0])
        for line in f.readlines():
            for i, c in enumerate(line[:-1]):
                counts[i][int(c)] += 1
    gamma = int(''.join('1' if counts[i][1] > counts[i][0] else '0' for i in range(len(counts))), 2)
    eps = int(''.join('1' if counts[i][0] > counts[i][1] else '0' for i in range(len(counts))), 2)
    return gamma*eps

assert part1("examples/31.txt") == 198
print(part1("input/3.txt"))

def part2(fname):
    """
    Part 1 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        numbers = [num[:-1] for num in f.readlines()]

    def successive_filtering(criteria_fn):
        """
        Filter the numbers by the given criteria function.
        """
        nonlocal numbers
        candidates = numbers
        index = 0
        while len(candidates) > 1:
            counts = [sum(num[index] == '0' for num in candidates),
                      sum(num[index] == '1' for num in candidates)]

            candidates = [num for num in candidates if criteria_fn(counts, int(num[index]))]
            index += 1
        return int(''.join(candidates[0]), 2)

    oxygen = successive_filtering(lambda counts, x: x == int(counts[1] >= counts[0]))
    co2 = successive_filtering(lambda counts, x: x != int(counts[1] >= counts[0]))

    return oxygen*co2

assert part2("examples/31.txt") == 230
print(part2("input/3.txt"))
