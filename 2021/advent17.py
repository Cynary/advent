#!/usr/bin/python3
"""
Day 17 of advent of code 2021
"""

import math

def part1(fname):
    """
    Part 1 of the problem.
    """
    def parse_range(s):
        """
        Parse a range string with the format [x|y]=a..b and returns (a, b)
        """
        return tuple(sorted(map(int, s.split("=")[1].split(".."))))

    with open(fname, encoding="utf-8") as f:
        x_range, y_range = [parse_range(r) for r in f.readline().strip().split(": ")[1].split(", ")]

    # Because of symmetry, the probe will always pass through 0 on the way down, and it will have
    # the same speed as when it was thrown up in the previous step, so it will have (-v-1) velocity.
    # The fastest y-velocity possible will reach the bottom of the y-range in one step, so it will
    # be (-v-1) = y_range[0]
    vertical_velocity = -y_range[0] - 1

    # The ball will go up v+(v-1)+(v-2)+...+1 = v*(v+1)/2 units.
    top_spot = vertical_velocity*(vertical_velocity+1)//2
    return top_spot

assert part1("examples/171.txt") == 45
print(part1("input/17.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    def parse_range(s):
        """
        Parse a range string with the format [x|y]=a..b and returns (a, b)
        """
        return tuple(sorted(map(int, s.split("=")[1].split(".."))))

    with open(fname, encoding="utf-8") as f:
        x_range, y_range = [parse_range(r) for r in f.readline().strip().split(": ")[1].split(", ")]

    # The main insight is that the forward and vertical velocities are independent.
    #
    # The range of possible vertical velocities is [y_range[0], -y_range[0]-1]. Let's compute the
    # range of how many steps it takes for every possible y-velocity to reach the y-range.
    #
    # range[0] <= v+(v-1)+(v-2)+...+(v-(N-1)) <= range[1]
    # range[0] <= v*N - (N-1)*N/2 <= range[1]
    # range[0] <= -N^2/2 + N*(v+1/2) <= range[1]
    #
    # These are two quadratics:
    # N^2 - N*(2*v+1) + 2*range[0] <= 0
    # N^2 - N*(2*v+1) + 2*range[1] >= 0
    #
    # The quadratic formula is (-b +- sqrt(b^2 - 4*a*c))/2*a
    vertical_velocity_to_range_map = {}
    all_max_steps = 0
    for y_velocity in range(y_range[0], -y_range[0]):
        a = 1
        b = -2*y_velocity - 1
        c0 = 2*y_range[0]
        c1 = 2*y_range[1]
        min_steps = math.ceil((-b + math.sqrt(b**2 - 4*a*c1))/(2*a))
        max_steps = math.floor((-b + math.sqrt(b**2 - 4*a*c0))/(2*a))

        if min_steps <= max_steps:
            vertical_velocity_to_range_map[y_velocity] = (min_steps, max_steps)
            all_max_steps = max(max_steps, all_max_steps)

    # Now do the same thing for the forward velocities.
    #
    # These have a special property, however - since they decrease, if the minimum number of steps
    # it takes to reach the x-range is > v, then it wouldn't actually reach the x-range.  If the max
    # number of steps to stay within the x-range is >= v, then the speed would stay in the x-range
    # forever - to cap that, we just use the max step count that we saw for the vertical velocities.
    #
    # Forward velocities can't go faster than x_range[1].  The lowest velocity is based on the
    # following inequality:
    #
    # v+(v-1)+...+1 >= x_range[0]
    # v*(v+1)/2 >= x_range[0]
    # v^2 + v - 2*x_range[0] >= 0
    # v >= (-1 + sqrt(1 + 8*x_range[0]))/2
    min_forward_velocity = math.ceil((-1 + math.sqrt(1 + 8*x_range[0]))/2)
    forward_velocity_to_range_map = {}
    for x_velocity in range(min_forward_velocity, x_range[1]+1):
        a = 1
        b = -2*x_velocity - 1
        c0 = 2*x_range[0]
        c1 = 2*x_range[1]

        min_steps = math.ceil((-b - math.sqrt(b**2 - 4*a*c0))/(2*a))

        try:
            max_steps = math.floor((-b - math.sqrt(b**2 - 4*a*c1))/(2*a))
        except ValueError:
            # This means that the velocity would forever stay in the x-range.
            #
            max_steps = all_max_steps

        if min_steps <= max_steps and min_steps <= x_velocity:
            assert max_steps < x_velocity or max_steps == all_max_steps
            forward_velocity_to_range_map[x_velocity] = (min_steps, max_steps)

    # Now just find all possible combinations.
    #
    def range_intersects(r1, r2):
        """
        Returns true if the ranges intersect.
        """
        return r1[0] <= r2[1] and r2[0] <= r1[1]
    total_possibilities = 0
    for y_velocity, step_range in vertical_velocity_to_range_map.items():
        for x_velocity, x_velocity_range in forward_velocity_to_range_map.items():
            if range_intersects(step_range, x_velocity_range):
                total_possibilities += 1

    return total_possibilities

def part22(fname):
    """
    Part 2 of the problem.
    """
    def parse_range(s):
        """
        Parse a range string with the format [x|y]=a..b and returns (a, b)
        """
        return tuple(sorted(map(int, s.split("=")[1].split(".."))))

    with open(fname, encoding="utf-8") as f:
        x_range, y_range = [parse_range(r) for r in f.readline().strip().split(": ")[1].split(", ")]

    def simulate(x_speed, y_speed):
        """
        Simulate the probe with the given speeds. Returns whether it passes through the target at
        any step.
        """
        nonlocal x_range, y_range
        x_pos = 0
        y_pos = 0
        while x_pos <= x_range[1] and y_pos >= y_range[0]:
            x_pos += x_speed
            y_pos += y_speed
            x_speed = x_speed-1 if x_speed != 0 else 0
            y_speed -= 1
            if x_range[0] <= x_pos <= x_range[1] and y_range[0] <= y_pos <= y_range[1]:
                return True
        return False

    total = 0
    for x_speed in range(x_range[1]+1):
        for y_speed in range(y_range[0], -y_range[0]):
            if simulate(x_speed, y_speed):
                total += 1
    return total

assert part2("examples/171.txt") == 112
print(part2("input/17.txt"))

assert part22("examples/171.txt") == 112
print(part22("input/17.txt"))
