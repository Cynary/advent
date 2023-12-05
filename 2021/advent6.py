#!/usr/bin/python3
"""
Day 6 of advent of code 2021
"""

import collections

def solution(fname, days):
    """
    Part 1 of the problem.
    """
    timers = collections.defaultdict(lambda: 0)
    with open(fname, encoding="utf-8") as f:
        for lantern in f.readline().split(','):
            timers[int(lantern)] += 1

    for _ in range(days):
        for i in range(9):
            timers[i-1] = timers[i]
        timers[8] = timers[-1]
        timers[6] += timers[-1]
        timers[-1] = 0

    return sum(timers.values())

print(solution("examples/61.txt", 18))
assert solution("examples/61.txt", 80) == 5934
print(solution("input/6.txt", 80))

assert solution("examples/61.txt", 256) == 26984457539
print(solution("input/6.txt", 256))
