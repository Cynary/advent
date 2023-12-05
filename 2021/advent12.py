#!/usr/bin/python3
"""
Day 12 of advent of code 2021
"""
import collections

def solution(fname, double_visit):
    """
    Solution of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        edges = [line.strip().split('-') for line in f.readlines()]

    adj = collections.defaultdict(list)
    for a, b in edges:
        adj[a].append(b)
        adj[b].append(a)

    print(adj)

    def is_small(edge):
        """
        Returns whether edge's first character is lower case.
        """
        return edge[0].islower()

    def dfs(node='start', visited=set(), doubled=None, path=[]): #pylint: disable=dangerous-default-value
        """
        Go through all possible paths that visit small caves at most once.
        """
        nonlocal adj, is_small
        if node == 'end':
            return 1

        paths = 0
        if is_small(node):
            if node in visited:
                assert doubled is None
                doubled = node
            else:
                visited.add(node)
        for neighbor in adj[node]:
            if neighbor not in visited or (doubled is None and neighbor != 'start'):
                paths += dfs(neighbor, visited, doubled, path)
        if is_small(node) and doubled != node:
            visited.discard(node)
        return paths

    return dfs(doubled='start' if not double_visit else None)

def part1(fname):
    """
    Part 1 of the problem.
    """
    return solution(fname, False)

# assert part1("examples/121.txt") == 10
# assert part1("examples/122.txt") == 226
# assert part1("examples/123.txt") == 19
# print(part1("input/12.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    return solution(fname, True)

assert part2("examples/121.txt") == 36
assert part2("examples/123.txt") == 103
assert part2("examples/122.txt") == 3509
print(part2("input/12.txt"))
