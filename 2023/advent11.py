#!/usr/bin/python3
"""
Day 11 of advent of code 2023
"""

def neighbors(r, c):
    return [
        (r-1, c),
        (r+1, c),
        (r, c-1),
        (r, c+1),
    ]

def solution(fname, expansion):
    """
    Solution of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        dr = 0
        galaxies = set()
        column_galaxies = set()
        for r, l in enumerate(f.readlines()):
            has_galaxy = False
            for c, symbol in enumerate(l.strip()):
                if symbol == '#':
                    galaxies.add((r+dr, c))
                    column_galaxies.add(c)
                    has_galaxy = True
            if not has_galaxy:
                dr += expansion

    dc = [0] * (max(c for _,c in galaxies)+1)
    for c in range(len(dc)): #pylint: disable=consider-using-enumerate
        dc[c] = dc[c-1]
        if c not in column_galaxies:
            dc[c] += expansion

    galaxies = set((r, c+dc[c]) for r, c in galaxies)

    total = 0
    for gal1 in galaxies:
        for gal2 in galaxies:
            total += abs(gal1[0]-gal2[0]) + abs(gal1[1]-gal2[1])
    return total//2

def part1(fname):
    return solution(fname, 1)

assert part1("examples/111.txt") == 374
print("Solved example")
print(part1("input/11.txt"))

def part2(fname):
    return solution(fname, 999999)

assert solution("examples/111.txt", 9) == 1030
assert solution("examples/111.txt", 99) == 8410
print("Solved part 2 examples")
print(part2("input/11.txt"))
