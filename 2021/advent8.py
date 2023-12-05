#!/usr/bin/python3
"""
Day 8 of advent of code 2021
"""

def part1(fname):
    """
    Part 1 of the problem.
    """
    total = 0
    with open(fname, encoding="utf-8") as f:
        for note in f.readlines():
            _, outputs = note.strip().split('|')
            for output in outputs.split():
                total += int(len(output) in (2, 4, 3, 7))
    return total

assert part1("examples/81.txt") == 26
print(part1("input/8.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    digits_to_segments = {
        0: set(('a', 'b', 'c', 'e', 'f', 'g')),
        1: set(('c', 'f')),
        2: set(('a', 'c', 'd', 'e', 'g')),
        3: set(('a', 'c', 'd', 'f', 'g')),
        4: set(('b', 'c', 'd', 'f')),
        5: set(('a', 'b', 'd', 'f', 'g')),
        6: set(('a', 'b', 'd', 'e', 'f', 'g')),
        7: set(('a', 'c', 'f')),
        8: set(('a', 'b', 'c', 'd', 'e', 'f', 'g')),
        9: set(('a', 'b', 'c', 'd', 'f', 'g'))
    }

    segments_to_digits = {tuple(sorted(v)): k for (k, v) in digits_to_segments.items()}

    all_segments = set(chr(c) for c in range(ord('a'), ord('g')+1))
    length_to_possibilities = {}
    for v in digits_to_segments.values():
        length_to_possibilities[len(v)] = length_to_possibilities.get(len(v), set()).union(v)

    total = 0

    def resolve(possibilities, digits):
        """
        Constraint propagation.
        """
        nonlocal length_to_possibilities
        if all(len(v) == 1 for v in possibilities.values()):
            for digit in digits.split():
                segments = tuple(sorted(set().union(*[possibilities[c] for c in digit])))
                if segments not in segments_to_digits:
                    return None
            return possibilities

        initial_possibilities = None
        while initial_possibilities != possibilities:
            initial_possibilities = {k: v.copy() for (k, v) in possibilities.items()}
            for digit in digits.split():
                total_possibilities = set().union(*[possibilities[c] for c in digit])

                # First, only take into account possibilities based on length.
                #
                total_possibilities &= length_to_possibilities[len(digit)]

                for c in digit:
                    possibilities[c] = possibilities[c] & total_possibilities

                if len(possibilities) < len(digit):
                    return None

                if len(possibilities) == len(digit):
                    for c in possibilities.keys():
                        if c not in digit:
                            possibilities[c] -= total_possibilities

            for c, v in possibilities.items():
                if len(v) == 1:
                    for c2 in possibilities.keys():
                        if c2 != c:
                            possibilities[c2] -= v

            if any(len(v) == 0 for v in possibilities.values()):
                return None

        if all(len(v) == 1 for v in possibilities.values()):
            for digit in digits.split():
                segments = tuple(sorted(set().union(*[possibilities[c] for c in digit])))
                if segments not in segments_to_digits:
                    return None
            return possibilities

        for c, v in possibilities.items():
            assert len(v) >= 1
            if len(v) != 1:
                for possible in v:
                    new_possibilities = {k: v.copy() for (k, v) in possibilities.items()}
                    new_possibilities[c] = set(possible)
                    result = resolve(new_possibilities, digits)
                    if result is not None:
                        return result

        return None

    with open(fname, encoding="utf-8") as f:
        for note in f.readlines():
            possibilities = {chr(c): all_segments.copy() for c in range(ord('a'), ord('g')+1)}
            digits, outputs = note.strip().split('|')
            possibilities = resolve(possibilities, digits)
            assert possibilities is not None

            number = 0
            for output in outputs.split():
                output_set = set().union(*[possibilities[c] for c in output])
                output_set = tuple(sorted(output_set & length_to_possibilities[len(output)]))
                assert output_set in segments_to_digits and len(output_set) == len(output)
                number = number*10 + segments_to_digits[output_set]

            total += number
    return total

assert part2("examples/82.txt") == 5353
print("Example 82 passed")
assert part2("examples/81.txt") == 61229
print("Example 81 passed")
print(part2("input/8.txt"))
