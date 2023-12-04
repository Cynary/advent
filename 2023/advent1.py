#!/usr/bin/python3
"""
Solution for day 1 of advent of code.
"""

def part1(fname):
    """
    Part 1 of the problem.
    """
    total = 0
    with open(fname, encoding="utf-8") as f:
        for l in f.readlines():
            for val in l:
                if ord('0') <= ord(val) <= ord('9'):
                    total += int(val) * 10
                    break
            for val in reversed(l):
                if ord('0') <= ord(val) <= ord('9'):
                    total += int(val)
                    break
    return total

assert part1("examples/11.txt") == 142
print(part1("input/1.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    candidates = {
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
        "1": 1,
        "2": 2,
        "3": 3,
        "4": 4,
        "5": 5,
        "6": 6,
        "7": 7,
        "8": 8,
        "9": 9,
    }
    total = 0
    with open(fname, encoding="utf-8") as f:
        for l in f.readlines():
            first_digit = (len(l), -1) # (index, value)
            last_digit = (-1, -1) # (index, value)
            for (candidate, val) in candidates.items():
                index = l.find(candidate)
                while index != -1:
                    if index < first_digit[0]:
                        first_digit = (index, val)
                    if index > last_digit[0]:
                        last_digit = (index, val)
                    index = l.find(candidate, index+1)
            number = first_digit[1] * 10 + last_digit[1]
            total += number
    return total

assert part2("examples/12.txt") == 281
print(part2("input/1.txt"))
