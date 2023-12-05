#!/usr/bin/python3
"""
Day 13 of advent of code 2021
"""

def do_fold(points, dimension, line):
    """
    Fold the points along the line in the given dimension.
    """
    folded_points = [list(point) for point in points if point[dimension] > line]
    for point in folded_points:
        points.discard(tuple(point))
        point[dimension] = line - (point[dimension] - line)
        points.add(tuple(point))

def solution(fname):
    """
    Solution of the problem.
    """
    dimensions = {
        'fold along x': 0,
        'fold along y': 1,
    }
    with open(fname, encoding="utf-8") as f:
        points, folds = f.read().strip().split('\n\n')
    points = set(tuple(map(int, line.split(','))) for line in points.split('\n'))
    folds = [(dimensions[d], int(line))
             for d, line in map(lambda x: x.split('='), folds.split('\n'))]
    do_fold(points, *folds[0])
    part1 = len(points)

    for fold in folds[1:]:
        do_fold(points, *fold)

    x_count = max(point[0] for point in points) + 1
    y_count = max(point[1] for point in points) + 1
    grid = [['.'] * x_count for _ in range(y_count)]
    for point in points:
        grid[point[1]][point[0]] = '#'

    for line in grid:
        print(''.join(line))

    return part1

assert solution("examples/131.txt") == 17
print(solution("input/13.txt"))
