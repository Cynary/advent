#!/usr/bin/python3
"""
Day 10 of advent of code 2021
"""

def solution(fname):
    """
    Solution of the problem.
    """
    opens = set('([{<')
    closes = set(')]}>')
    close_to_open = {')': '(', ']': '[', '}': '{', '>': '<'}
    point_map = {
        ')': 3,
        ']': 57,
        '}': 1197,
        '>': 25137
    }

    complete_point_map = {
            '(': 1,
            '[': 2,
            '{': 3,
            '<': 4
    }

    error_points = 0
    complete_scores = []
    with open(fname, encoding="utf-8") as f:
        for line in f.readlines():
            stack = []
            for c in line.strip():
                if c in opens:
                    stack.append(c)
                elif c in closes:
                    if len(stack) == 0 or stack[-1] != close_to_open[c]:
                        error_points += point_map[c]
                        break
                    stack.pop()
            else:
                this_complete_points = 0
                for c in reversed(stack):
                    this_complete_points = this_complete_points*5 + complete_point_map[c]
                complete_scores.append(this_complete_points)

    complete_scores.sort()
    complete_points = complete_scores[len(complete_scores) // 2]
    return (error_points, complete_points)

def part1(fname):
    """
    Part 1 of the problem.
    """
    return solution(fname)[0]

assert part1("examples/101.txt") == 26397
print(part1("input/10.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    return solution(fname)[1]

assert part2("examples/101.txt") == 288957
print(part2("input/10.txt"))
