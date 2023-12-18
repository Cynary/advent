#!/usr/bin/python3
"""
Day 18 of advent of code 2023
"""

from collections import defaultdict

deltas = {
    'R': (0, 1),
    '0': (0, 1),
    'L': (0, -1),
    '2': (0, -1),
    'U': (-1, 0),
    '3': (-1, 0),
    'D': (1, 0),
    '1': (1, 0),

}

def part1(fname):
    """
    Part 1 of the problem.
    """
    min_r, min_c = 0, 0
    max_r, max_c = 0, 0
    current = (0,0)
    path = [current]
    with open(fname, encoding="utf-8") as f:
        for line in f.readlines():
            direction, count, _ = line.split()
            for _ in range(int(count)):
                current = tuple(current[i] + deltas[direction][i] for i in range(2))
                path.append(current) # type: ignore
                min_r = min(min_r, current[0])
                min_c = min(min_c, current[1])

                max_r = max(max_r, current[0])
                max_c = max(max_c, current[1])

    path = set((r-min_r, c-min_c) for r, c in path)
    max_r -= min_r
    max_c -= min_c
    # for r in range(max_r+1):
    #     for c in range(max_c+1):
    #         if (r, c) in path:
    #             print('#', end='')
    #         else:
    #             print('.', end='')
    #     print()

    # Cheating.
    start_point = (max_r//2, max_c//2)

    def neighbors(point):
        (r, c) = point
        return [
            (r-1, c),
            (r+1, c),
            (r, c-1),
            (r, c+1),
        ]

    size = len(path)
    added = set([start_point])
    queue = [start_point]
    while queue:
        size += 1
        current = queue.pop()
        for n in neighbors(current):
            if n in added or n in path:
                continue
            added.add(n)
            queue.append(n)

    return size


# part1("examples/181.txt")
# part1("input/18.txt")

# assert part1("examples/181.txt") == 62
# print(part1("input/18.txt"))

def part2(fname, count_fn=lambda x: (x[2][-2], int(x[2][2:-2], 16))):
    """
    Part 1 of the problem.
    """
    min_r, min_c = 0, 0
    max_r, max_c = 0, 0
    current = (0,0)
    column_to_rows = defaultdict(list)
    connected = defaultdict(set)
    with open(fname, encoding="utf-8") as f:
        lines = f.readlines()
        for i, line in enumerate(lines):
            print(f"processed {i/len(lines):.2%} lines")
            direction, count = count_fn(line.split())
            prev = current
            if direction in ('U', 'D'):
                current = tuple(current[i] + deltas[direction][i]*count for i in range(2))
            else:
                column_to_rows[current[1]].append((current[0], direction))
                for i in range(int(count)):
                    current = tuple(current[j] + deltas[direction][j] for j in range(2))
                    column_to_rows[current[1]].append((current[0], direction))
            connected[prev].add(current)
            connected[current].add(prev)
            min_r = min(min_r, current[0])
            min_c = min(min_c, current[1])

            max_r = max(max_r, current[0])
            max_c = max(max_c, current[1])

    size = 0
    for c in range(min_c, max_c+1):
        rows = sorted(column_to_rows[c])
        if len(rows) == 0:
            continue
        enter_direction = rows[0][1]
        size += 1
        for i, (r, direction) in enumerate(rows[1:]):
            if direction == enter_direction:
                if (r, c) in connected[(rows[i][0], c)]:
                    size += r - rows[i][0]
                else:
                    size += 1
            else:
                size += r - rows[i][0]

    print(size)
    return size


assert part2("examples/181.txt", count_fn=lambda x: (x[0], int(x[1]))) == 62
print(part2("input/18.txt", count_fn=lambda x: (x[0], int(x[1]))))

assert part2("examples/181.txt") == 952408144115
print(part2("input/18.txt"))
