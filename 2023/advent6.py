#!/usr/bin/python3
"""
Day 6 of advent of code 2023
"""

import math

def possible_solutions(time, dist):
    """
    Finds the number of hold times that beats the distance in the given time.
    """
    a = -1
    b = time
    c = -dist
    min_hold_time = int((-b + math.sqrt(b**2 - 4*a*c)) // (2*a) + 1)
    max_hold_time = math.ceil((-b - math.sqrt(b**2 - 4*a*c)) / (2*a))

    return max_hold_time - min_hold_time

def part1(fname):
    """
    Part 1 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        times = [int(t) for t in  f.readline().split()[1:]]
        dists = [int(d) for d in  f.readline().split()[1:]]

    possible_mult = 1
    for t, d in zip(times, dists):
        possible_mult *= possible_solutions(t, d)

    return possible_mult

assert part1("examples/61.txt") == 288
print(part1("input/6.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        time = int(f.readline().strip().replace(' ', '').split(':')[1])
        dist = int(f.readline().strip().replace(' ', '').split(':')[1])

    return possible_solutions(time, dist)

assert part2("examples/61.txt") == 71503
print(part2("input/6.txt"))
