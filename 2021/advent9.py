#!/usr/bin/python3
"""
Day 9 of advent of code 2021
"""

def adjacent(grid, r, c):
    """
    Returns the heights of the adjacent cells.
    """
    for delta in (-1, 1):
        yield grid[r+delta][c]
        yield grid[r][c+delta]

def get_grid(fname):
    """
    Generates a 2D grid with sentinels.
    """
    with open(fname, encoding="utf-8") as f:
        # Use the '\n' as a sentinel, and add a sentinel row at the bottom.
        grid = [[int(c) if c != '\n' else 9 for c in line] for line in f.readlines()]
        grid.append([9] * len(grid[0]))

    return grid

def low_points(grid):
    """
    Generates all the low point coordinates
    """
    row_count = len(grid)-1
    col_count = len(grid[0])-1
    for r in range(row_count):
        for c in range(col_count):
            if all(grid[r][c] < adj for adj in adjacent(grid, r, c)):
                yield (r, c)

def part1(fname):
    """
    Part 1 of the problem.
    """
    grid = get_grid(fname)
    return sum(1+grid[r][c] for (r, c) in low_points(grid))

assert part1("examples/91.txt") == 15
print(part1("input/9.txt"))

def flood(grid, r, c):
    """
    Floods the grid from the given coordinate, stopping at 9s, and returning the total number of
    flooded cells.
    """
    if grid[r][c] == 9:
        return 0

    grid[r][c] = 9
    return 1 + sum(flood(grid, r+delta, c) for delta in (-1, 1)) + sum(flood(grid, r, c+delta) for delta in (-1, 1))

def part2(fname):
    """
    Part 2 of the problem.
    """
    grid = get_grid(fname)
    basins = sorted(flood(grid, r, c) for (r, c) in low_points(grid))
    return basins[-1] * basins[-2] * basins[-3]

assert part2("examples/91.txt") == 1134
print(part2("input/9.txt"))
