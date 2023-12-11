#!/usr/bin/python3
"""
Day 10 of advent of code 2023
"""

import collections

# | is a vertical pipe connecting north and south.
# - is a horizontal pipe connecting east and west.
# L is a 90-degree bend connecting north and east.
# J is a 90-degree bend connecting north and west.
# 7 is a 90-degree bend connecting south and west.
# F is a 90-degree bend connecting south and east.
# . is ground; there is no pipe in this tile.
# S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

pipe_adjacencies = {
    'S': [(0, 1), (0, -1), (1, 0), (-1, 0)],
    '|': [(1, 0), (-1, 0)],
    'L': [(0, 1), (-1, 0)],
    'J': [(0, -1), (-1, 0)],
    '7': [(0, -1), (1, 0)],
    'F': [(0, 1), (1, 0)],
    '-': [(0, 1), (0, -1)],
    '.': []
}

def transform(state, row, col):
    """
    Transform a state into a list of adjacent states.
    """
    adj = pipe_adjacencies[state]
    return [(row + r, col + c) for r, c in adj]

def part1(fname):
    """
    Part 1 of the problem.
    """
    adj = {}
    start = None
    with open(fname, encoding="utf-8") as f:
        for ri, row in enumerate(f.readlines()):
            for ci, state in enumerate(row.strip()):
                adj[(ri, ci)] = transform(state, ri, ci)
                if state == 'S':
                    start = (ri, ci)

    # Fixup the adjacency - true adjacencies are only possible if they're bidirectional.
    for state, adj_states in adj.items():
        adj[state] = [adj_state for adj_state in adj_states if state in adj.get(adj_state, [])]

    assert start is not None
    visited = set([start])
    queue = [(0, start)]
    max_cost = 0
    while queue:
        cost, curr = queue.pop(0)
        assert cost >= max_cost
        max_cost = cost
        for adj_state in adj.get(curr, []):
            if adj_state not in visited:
                visited.add(adj_state)
                queue.append((cost+1, adj_state)) # type: ignore

    return max_cost

assert part1("examples/101.txt") == 8
assert part1("examples/102.txt") == 4
print(part1("input/10.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    adj = collections.defaultdict(list)
    start = None
    max_col = 0
    max_row = 0
    with open(fname, encoding="utf-8") as f:
        for ri, row in enumerate(f.readlines()):
            for ci, state in enumerate(row.strip()):
                adj[(ri, ci)] = transform(state, ri, ci)
                if state == 'S':
                    start = (ri*2, ci*2)
                max_col = max(max_col, ci)
            max_row = max(max_row, ri)

    # Fixup the adjacency - true adjacencies are only possible if they're bidirectional.
    for state, adj_states in adj.items():
        adj[state] = [adj_state for adj_state in adj_states if state in adj.get(adj_state, [])]

    # "Blow up" the map by a factor of 2, but keeping pipes connected so that the loop remains.
    blowup = collections.defaultdict(list)
    for (r,c), adj_states in adj.items():
        for (ar, ac) in adj_states:
            middle = ((r*2 + (ar-r)), (c*2 + (ac-c)))
            blowup[(r*2, c*2)].append(middle)
            blowup[middle].append((r*2, c*2))

    # Mark all the nodes in the loop.
    #
    assert start is not None
    queue = [(set([start]), start)]
    visited = set()
    loop = None
    while loop is None:
        path, curr = queue.pop()
        if curr in visited:
            continue
        visited.add(curr)
        for adj_state in blowup[curr]:
            if adj_state == start and len(path) > 2:
                loop = path
                break

            if adj_state not in visited:
                queue.append((path | set([adj_state]), adj_state))

    unfilled = set((r,c) for r in range(max_row*2 + 1) for c in range(max_col*2 + 1)
                   if (r,c) not in loop)

    inner_count = 0
    while len(unfilled) != 0:
        start_point = unfilled.pop()
        is_inner = True
        queue = [start_point]
        visited = set([start_point])
        orig_tile_count = 0
        while queue:
            r, c = queue.pop(0)
            if r%2 == 0 and c%2 == 0:
                orig_tile_count += 1
            neighbors = [(r+dr, c+dc) for dr in (-1,0,1) for dc in (-1,0,1)]
            for state in neighbors:
                if state in visited or state in loop:
                    continue

                rr, cc = state
                if rr < 0 or rr > max_row*2 or cc < 0 or cc > max_col*2:
                    is_inner = False
                    continue

                assert state in unfilled, state
                unfilled.discard(state)
                visited.add(state)
                queue.append(state)
        if is_inner:
            inner_count += orig_tile_count
    return inner_count

assert part2("examples/103.txt") == 4
print("Solved examples/103.txt")
assert part2("examples/104.txt") == 4
print("Solved examples/104.txt")
assert part2("examples/105.txt") == 8
print("Solved examples/105.txt")
assert part2("examples/106.txt") == 10
print("Solved examples/106.txt")
print(part2("input/10.txt"))
