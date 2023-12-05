#!/usr/bin/python3
"""
Day 11 of advent of code 2021
"""

ROWS=10
COLUMNS=10
CELLS = list((r, c) for r in range(ROWS) for c in range(COLUMNS))

def adjacent(r, c):
    for dr in range(-1, 2):
        for dc in range(-1, 2):
            if dr == 0 and dc == 0:
                continue
            if r + dr < 0 or r + dr >= ROWS:
                continue
            if c + dc < 0 or c + dc >= COLUMNS:
                continue
            yield r + dr, c + dc

def flash(grid, r, c):
    """
    Flash the octopus
    """
    flashes = 1
    for rr, cc in adjacent(r, c):
        if grid[rr][cc] != 10:
            grid[rr][cc] += 1
            if grid[rr][cc] == 10:
                flashes += flash(grid, rr, cc)
    return flashes

def step(grid):
    """
    Step the grid
    """
    flashes = 0
    for r, c in CELLS:
        if grid[r][c] != 10:
            grid[r][c] += 1
            if grid[r][c] == 10:
                flashes += flash(grid, r, c)

    for r, c in CELLS:
        if grid[r][c] == 10:
            grid[r][c] = 0

    return flashes

def part1(fname):
    """
    Part 1 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        grid = [list(map(int, line.strip())) for line in f.readlines()]
    flashes = 0
    for _ in range(100):
        flashes += step(grid)
    return flashes

assert part1("examples/111.txt") == 1656
print(part1("input/11.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        grid = [list(map(int, line.strip())) for line in f.readlines()]
    flash_step = 1
    while True:
        if step(grid) == ROWS*COLUMNS:
            break
        flash_step += 1
    return flash_step

assert part2("examples/111.txt") == 195
print(part2("input/11.txt"))
