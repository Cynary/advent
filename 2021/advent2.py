#!/usr/bin/python3
"""
Day 2 of advent of code 2021
"""

class Position:
    """
    Repesents the position of the submarine.
    """
    def __init__(self, starting_horizontal=0, starting_depth=0):
        self.horizontal = starting_horizontal
        self.depth = starting_depth

    def forward(self, amount):
        """
        Moves the submarine forward.
        """
        self.horizontal += amount

    def down(self, amount):
        """
        Moves the submarine down.
        """
        self.depth += amount

    def up(self, amount):
        """
        Moves the submarine up.
        """
        self.depth -= amount
        assert self.depth >= 0

def solution(position_type, fname):
    """
    Problem solution.
    """
    pos = position_type()
    with open(fname, encoding="utf-8") as f:
        for (command, amount) in (l[:-1].split(' ') for l in f.readlines()):
            getattr(pos, command)(int(amount))
    return pos.horizontal * pos.depth

def part1(fname):
    """
    Part 1 of the problem.
    """
    return solution(Position, fname)

assert part1("examples/21.txt") == 150
print(part1("input/2.txt"))

class Position2:
    """
    Position of the submarine, taking into account aim.
    """
    def __init__(self):
        self.aim = 0
        self.horizontal = 0
        self.depth = 0

    def forward(self, amount):
        """
        Moves the submarine forward.
        """
        self.horizontal += amount
        self.depth += amount * self.aim

    def down(self, amount):
        """
        Moves the aim down.
        """
        self.aim += amount

    def up(self, amount):
        """
        Moves the aim up.
        """
        self.aim -= amount

def part2(fname):
    """
    Part 2 of the problem.
    """
    return solution(Position2, fname)

assert part2("examples/21.txt") == 900
print(part2("input/2.txt"))
