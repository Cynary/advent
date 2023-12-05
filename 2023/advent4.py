#!/usr/bin/python3
"""
Day 4 of advent of code 2023
"""

def part1(fname):
    """
    Part 1 of the problem.
    """
    score = 0
    with open(fname, encoding="utf-8") as f:
        for i, line in enumerate(f.readlines()):
            card, lists = line.split(": ")
            assert card.split()[1] == f"{i+1}", line
            winning, have = lists.split(" | ")
            winning = set(int(num) for num in winning.split())
            have = [int(num) for num in have.split()]
            matches = sum(num in winning for num in have)
            if matches > 0:
                score += 2**(sum(num in winning for num in have)-1)
    return score

assert part1("examples/41.txt") == 13
print(part1("input/4.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    cards = []
    with open(fname, encoding="utf-8") as f:
        for i, line in enumerate(f.readlines()):
            card, lists = line.split(": ")
            assert card.split()[1] == f"{i+1}", line
            winning, have = lists.split(" | ")
            winning = set(int(num) for num in winning.split())
            have = [int(num) for num in have.split()]
            matches = sum(num in winning for num in have)
            cards.append(matches)

    accum = []
    for i, card in enumerate(reversed(cards)):
        assert card <= i
        total = 1
        for j in range(i-card, i):
            total += accum[j]
        accum.append(total)

    return sum(accum)

assert part2("examples/41.txt") == 30
print(part2("input/4.txt"))
