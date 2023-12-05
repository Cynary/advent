#!/usr/bin/python3
"""
Day 5 of advent of code 2023
"""

def part1(fname):
    """
    Part 1 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        seeds = map(int, f.readline().split(':')[1].split())
        possibilities = list(seeds)
        f.readline()
        for _ in range(7):
            f.readline()
            line = f.readline().strip()
            is_set = [False] * len(possibilities)
            while line != "":
                dest_start, src_start, range_length = map(int, line.split())
                for i, poss in enumerate(possibilities):
                    if not is_set[i] and src_start <= poss < src_start + range_length:
                        possibilities[i] = dest_start + poss - src_start
                        is_set[i] = True
                line = f.readline().strip()

    return min(possibilities)

print(part1("examples/51.txt"))
assert part1("examples/51.txt") == 35
print(part1("input/5.txt"))

def intersects(r1_start, r1_end, r2_start, r2_end):
    """
    Returns true if the two ranges intersect (end is exclusive)
    """
    return r1_start < r2_end and r2_start < r1_end

def split_range(r1_start, r1_end, r2_start, r2_end):
    """
    Returns a list of ranges that are in r1 but not in r2, assuming they intersect.
    """
    ret = []
    if r1_start < r2_start:
        ret.append((r1_start, r2_start))
    if r1_end > r2_end:
        ret.append((r2_end, r1_end))
    return ret

def part2(fname):
    """
    Part 1 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        seeds = list(map(int, f.readline().split(':')[1].split()))
        possibilities = []
        for i in range(0, len(seeds), 2):
            possibilities.append((seeds[i], seeds[i]+seeds[i+1]))
        f.readline()
        for _ in range(7):
            f.readline()
            line = f.readline().strip()
            is_set = [False] * len(possibilities)
            while line != "":
                dest_start, src_start, range_length = map(int, line.split())
                src_end = src_start + range_length
                dest_end = dest_start + range_length
                for i, (p_start, p_end) in enumerate(possibilities):
                    if not is_set[i] and intersects(p_start, p_end, src_start, src_end):
                        possibilities[i] = (max(dest_start + p_start - src_start, dest_start),
                                            min(dest_start + p_end - src_start, dest_end))
                        is_set[i] = True
                        for extra in split_range(p_start, p_end, src_start, src_end):
                            possibilities.append(extra)
                            is_set.append(False)
                line = f.readline().strip()

    return min(possibilities)[0]

assert part2("examples/51.txt") == 46
print(part2("input/5.txt"))
