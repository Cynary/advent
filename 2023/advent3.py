#!/usr/bin/python3
"""
Day 3 of advent of code.
"""
def adjacent(grid, r, c):
    """
    Returns the adjacent cells to the given cell.
    """
    return ((grid[rr][cc], (rr, cc))
            for rr in range(r-1, r+2) for cc in range(c-1, c+2) if (rr, cc) != (r, c))

def part1(fname):
    """
    Part 1 of the problem.
    """
    grid = open(fname, encoding="utf-8").readlines()

    # Add sentinels so we don't have to worry about it.
    #
    grid += ['.'*len(grid[0]),]
    assert all(len(l) == len(grid[0]) for l in grid)

    total = 0
    current_number = 0
    next_to_symbol = False

    def finish_number():
        nonlocal total, current_number, next_to_symbol
        if next_to_symbol:
            total += current_number
        current_number = 0
        next_to_symbol = False

    for i, line in enumerate(grid[:-1]):
        for j, c in enumerate(line):
            if ord('0') <= ord(c) <= ord('9'):
                current_number = current_number * 10 + int(c)
                next_to_symbol = next_to_symbol or \
                      any(not (ord('0') <= ord(n) <= ord('9') or n in ('.', '\n'))
                          for n, _ in adjacent(grid, i, j))
            else:
                finish_number()
        finish_number()

    return total

assert part1("examples/31.txt") == 4361
print(part1("input/3.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    grid = open(fname, encoding="utf-8").readlines()

    # Add sentinels so we don't have to worry about it.
    #
    grid += ['.'*len(grid[0]),]
    assert all(len(l) == len(grid[0]) for l in grid)

    current_number = 0
    current_stars = set()
    stars = {}

    def finish_number():
        nonlocal current_number, current_stars, stars
        for star in current_stars:
            if star not in stars:
                stars[star] = []
            stars[star].append(current_number)
        current_stars = set()
        current_number = 0

    for i, line in enumerate(grid[:-1]):
        for j, c in enumerate(line):
            if ord('0') <= ord(c) <= ord('9'):
                current_number = current_number * 10 + int(c)
                for n, coords in adjacent(grid, i, j):
                    if n == '*':
                        current_stars.add(coords)
            else:
                finish_number()
        finish_number()

    return sum(s[0]*s[1] for s in stars.values() if len(s) == 2)

assert part2("examples/32.txt") == 467835
print(part2("input/3.txt"))
