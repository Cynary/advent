#!/usr/bin/python3
"""
Day 8 of advent of code 2023
"""

import math

def smallest_multiple(numbers):
    """
    Calculates the smallest multiple of multiple numbers.
    """
    lcm = numbers[0]
    for i in range(1, len(numbers)):
        lcm = lcm * numbers[i] // math.gcd(lcm, numbers[i])
    return lcm


def solution(fname, is_start, is_end):
    """
    Solution of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        instructions = f.readline().strip()
        f.readline()
        network = {
            node: targets[1:-2].split(', ')
            for (node, targets) in (edge.split(' = ') for edge in f.readlines())
        }

    current_nodes_fake = set(node for node in network.keys() if is_start(node))
    steps_total = []

    for node in current_nodes_fake:
        steps = 0
        current_nodes = [node]
        while any(not is_end(node) for node in current_nodes):
            ind = 1 if instructions[steps%len(instructions)] == 'R' else 0
            current_nodes = set(network[node][ind] for node in current_nodes)
            steps += 1
        steps_total.append(steps)
    return smallest_multiple(steps_total)

def part1(fname):
    """
    Part 1 of the problem.
    """
    return solution(fname, lambda node: node == 'AAA', lambda node: node == 'ZZZ')

assert part1("examples/81.txt") == 2
assert part1("examples/82.txt") == 6
print(part1("input/8.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    return solution(fname, lambda node: node[-1] == 'A', lambda node: node[-1] == 'Z')

assert part2("examples/83.txt") == 6
print(part2("input/8.txt"))
