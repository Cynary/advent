#!/usr/bin/python3
"""
Day 7 of advent of code 2021
"""

def solution(fname, cost_fn):
    """
    Solution of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        positions = list(map(int, f.readline().split(',')))

    min_cost = None
    for position in range(min(positions), max(positions)+1):
        cost = sum(cost_fn(pos - position) for pos in positions)
        if min_cost is None or cost < min_cost:
            min_cost = cost
    return min_cost

def part1(fname):
    """
    Part 1 of the problem.
    """
    return solution(fname, lambda x: abs(x))

assert part1("examples/71.txt") == 37
print(part1("input/7.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    return solution(fname, lambda x: (abs(x) * (abs(x) + 1)) // 2)

assert part2("examples/71.txt") == 168
print(part2("input/7.txt"))
