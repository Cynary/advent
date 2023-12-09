#!/usr/bin/python3
"""
Day 18 of advent of code 2021
"""

import math
import functools

class Number: #pylint: disable=too-few-public-methods
    """
    Represents an element in a snailfish number.
    """
    def __init__(self, value, depth=0):
        self.value = value
        self.depth = depth

    def __repr__(self):
        return f"Number({self.value},{self.depth})"

def explode(number):
    """
    Explode the number if necessary.
    """
    for i, n in enumerate(number):
        if n.depth > 4:
            assert i != len(number)-1
            assert number[i+1].depth == n.depth
            if i != 0:
                number[i-1].value += n.value
            if i != len(number)-2:
                number[i+2].value += number[i+1].value
            number.pop(i+1)
            n.value = 0
            n.depth -= 1
            return True
    return False

def split(number):
    """
    Split the number if necessary.
    """
    for i, n in enumerate(number):
        if n.value >= 10:
            
            number.insert(i+1, Number(math.ceil(n.value/2), n.depth+1))
            n.depth += 1
            n.value //= 2
            return True
    return False

def reduce(number):
    """
    Reduces the number
    """
    while True:
        if not explode(number):
            if not split(number):
                return

def add(n1, n2):
    """
    Add two snailfish number
    """
    number = [Number(n.value, n.depth+1) for n in n1 + n2]
    reduce(number)
    return number

def magnitude(number, index=0, depth=1):
    """
    Return the magnitude of the number and the index at which the number at given depth ends.
    """
    if number[index].depth == depth:
        left_value = number[index].value
        index += 1
    else:
        left_value, index = magnitude(number, index, depth+1)

    if number[index].depth == depth:
        right_value = number[index].value
        index += 1
    else:
        right_value, index = magnitude(number, index, depth+1)

    return 3*left_value + 2*right_value, index

def parse_numbers(fname):
    """
    Parse the numbers from the file.
    """
    with open(fname, encoding="utf-8") as f:
        numbers = []
        for line in f.readlines():
            current_depth = 0
            number = []
            for c in line.strip():
                if c == ',':
                    assert current_depth > 0
                elif c == '[':
                    current_depth += 1
                elif c == ']':
                    current_depth -= 1
                else:
                    # No number greater than 9, otherwise would have been split.
                    assert '0' <= c <= '9', c
                    number.append(Number(int(c), current_depth))
            assert current_depth == 0
            numbers.append(number)
    return numbers

def part1(fname):
    """
    Part 1 of the problem.
    """
    numbers = parse_numbers(fname)
    result = functools.reduce(add, numbers)
    return magnitude(result)[0]

assert part1("examples/181.txt") == 4140
print(part1("input/18.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    numbers = parse_numbers(fname)
    max_mag = -1
    for i, n in enumerate(numbers):
        for n2 in numbers[i+1:]:
            max_mag = max(max_mag, magnitude(add(n, n2))[0], magnitude(add(n2, n))[0])
    return max_mag

assert part2("examples/181.txt") == 3993
print(part2("input/18.txt"))
