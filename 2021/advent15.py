#!/usr/bin/python3
"""
Day 15 of advent of code 2021
"""

import heapq


def neighbors(grid, coords):
    """
    Returns the neighbors of the given coordinate.
    """
    (x, y) = coords
    return [(grid[yy][xx], (xx, yy)) for xx, yy in [(x-1, y), (x+1, y), (x, y-1), (x, y+1)]
            if 0 <= xx < len(grid[0]) and 0 <= yy < len(grid)
        ]

def solution(grid):
    """
    Solution (dijkstra) of the problem.
    """
    start = (0, 0)
    goal = (len(grid)-1, len(grid[0])-1)
    queue = [(0, start)] # (cost, coords)
    visited = set()

    while True:
        cost, coords = heapq.heappop(queue)
        if coords == goal:
            return cost

        if coords in visited:
            continue

        visited.add(coords)
        for ncost, ncoords in neighbors(grid, coords):
            heapq.heappush(queue, (cost+ncost, ncoords))

def part1(fname):
    """
    Part 1 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        grid = [list(map(int, line.strip())) for line in f.readlines()]
    return solution(grid)

assert part1("examples/151.txt") == 40
print(part1("input/15.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        grid = [list(map(int, line.strip())) for line in f.readlines()]
    
    repeats = 5

    # First repeat horizontally
    #
    width = len(grid[0])
    for row in grid:
        for _ in range(width*(repeats-1)):
            new_val = row[-width]+1 if row[-width] != 9 else 1
            row.append(new_val)

    # Then repeat vertically
    height = len(grid)
    for _ in range((repeats-1)*height):
        grid.append([risk+1 if risk != 9 else 1 for risk in grid[-height]])

    return solution(grid)

assert part2("examples/151.txt") == 315
print(part2("input/15.txt"))
