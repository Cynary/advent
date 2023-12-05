#!/usr/bin/python3
"""
Day 5 of advent of code 2021
"""

import collections

def unit(val):
    """
    Return the unit of the given value.
    """
    return 1 if val > 0 else -1 if val < 0 else 0

def line_range(start, end):
    """
    Return the line between the two points.
    """
    current = tuple(start)
    end = tuple(end)
    while current != end:
        yield current
        current = (current[0]+unit(end[0]-current[0]), current[1]+unit(end[1]-current[1]))
    yield current

def solution(diagonals, fname):
    """
    Solution for the problem.
    """
    points = collections.defaultdict(lambda: 0)
    with open(fname, encoding="utf-8") as f:
        for line in f.readlines():
            start, end = line[:-1].split(" ->")
            start = [int(num) for num in start.split(",")]
            end = [int(num) for num in end.split(",")]

            if start[0] == end[0] or start[1] == end[1] or diagonals:
                for point in line_range(start, end):
                    points[point] += 1

    return sum(1 for point in points if points[point] > 1)

def part1(fname):
    """
    Part 1 of the problem.
    """
    return solution(False, fname)

assert part1("examples/51.txt") == 5
print(part1("input/5.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    return solution(True, fname)

assert part2("examples/51.txt") == 12
print(part2("input/5.txt"))
