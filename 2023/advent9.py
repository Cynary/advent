#!/usr/bin/python3
"""
Day 9 of advent of code 2023
"""

def solution(fname):
    """
    Solution of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        value_histories = [list(map(int, line.split())) for line in f]
    
    sum_next_values = 0
    sum_prev_values = 0
    for history in value_histories:
        derive_count = 0
        derivative = history
        starts = []

        # Derive
        while not all(i == derivative[0] for i in derivative):
            starts.append(derivative[0])
            derivative = [derivative[i+1] - derivative[i] for i in range(len(derivative)-1)]
            derive_count += 1

        derivative.append(derivative[-1])
        derivative.append(derivative[-1])

        # Integrate
        for _ in range(derive_count):
            integral = [starts.pop()-derivative[0]]
            for d in derivative:
                integral.append(integral[-1] + d)
            derivative = integral

        assert all(derivative[i+1] == history[i] for i in range(len(history)))
        sum_next_values += derivative[-1]
        sum_prev_values += derivative[0]

    return sum_next_values, sum_prev_values

def part1(fname):
    """
    Part 1 of the problem.
    """
    return solution(fname)[0]

assert part1("examples/91.txt") == 114
print(part1("input/9.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    return solution(fname)[1]

assert part2("examples/91.txt") == 2
print(part2("input/9.txt"))
