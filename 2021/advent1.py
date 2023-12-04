#!/usr/bin/python3
"""
Day 1 of advent of code 2021
"""

def part1(fname):
    """
    Part 1 of the problem.
    """
    measurements = [int(l[:-1]) for l in open(fname, encoding="utf-8").readlines()]
    return sum(m1 > m0 for m0, m1 in zip(measurements, measurements[1:]))

assert part1("examples/11.txt") == 7
print(part1("input/1.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        measurements = [int(l[:-1]) for l in f.readlines()]
    windows = [m0+m1+m2 for m0, m1, m2 in zip(measurements, measurements[1:], measurements[2:])]
    return sum(m1 > m0 for m0, m1 in zip(windows, windows[1:]))

assert part2("examples/11.txt") == 5
print(part2("input/1.txt"))
