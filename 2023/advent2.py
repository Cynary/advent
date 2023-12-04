#!/usr/bin/python3
"""
Day 2 of advent of code.
"""

def part1(fname):
    """
    Part 1 of the problem.
    """
    max_cubes = {
        "red": 12,
        "green": 13,
        "blue": 14,
    }

    id_sum = 0
    with open(fname, encoding="utf-8") as f:
        for i, game in enumerate(f.readlines()):
            game_id, sets = game[:-1].split(": ")
            assert game_id == f"Game {i+1}"
            sets = sets.split("; ")
            possible = True
            for individual_set in sets:
                for count, cube_type in (s.split(" ") for s in individual_set.split(", ")):
                    if int(count) > max_cubes[cube_type]:
                        possible = False
                        break

            if possible:
                id_sum += i + 1

    return id_sum

assert part1("examples/21.txt") == 8
print(part1("input/2.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    power_sum = 0
    with open(fname, encoding="utf-8") as f:
        for game in f.readlines():
            sets = game[:-1].split(": ")[1].split("; ")
            maxes = {}
            for individual_set in sets:
                for count, cube_type in (s.split(" ") for s in individual_set.split(", ")):
                    maxes[cube_type] = int(count) if cube_type not in maxes \
                          else max(maxes[cube_type], int(count))
            power_sum += maxes["red"] * maxes["green"] * maxes["blue"]
    return power_sum

assert part2("examples/22.txt") == 2286
print(part2("input/2.txt"))
