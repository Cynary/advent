#!/usr/bin/python3
"""
Day 7 of advent of code 2023
"""

# Map of strengths, ordered by A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
strengths = {
    'A': 12,
    'K': 11,
    'Q': 10,
    'J': 9,
    'T': 8,
    '9': 7,
    '8': 6,
    '7': 5,
    '6': 4,
    '5': 3,
    '4': 2,
    '3': 1,
    '2': 0
}

FIVE_OF_A_KIND=6
FOUR_OF_A_KIND=5
FULL_HOUSE=4
THREE_OF_A_KIND=3
TWO_PAIR=2
ONE_PAIR=1
HIGH_CARD=0

def get_hand_rank(hand, jokers):
    """
    Returns the hand's rank
    """
    if jokers:
        strengths['J'] = -1

    sort_key = tuple(strengths[card] for card in hand)
    assert len(hand) == 5, hand
    freq_map = {}
    for card in hand:
        freq_map[card] = freq_map.get(card, 0) + 1

    jokers_count = freq_map.get('J', 0)
    if jokers and 'J' in freq_map:
        del freq_map['J']

    frequencies = sorted(freq_map.values(), reverse=True)
    # It is always optimal to simply increase frequencies[0]
    if jokers:
        if len(frequencies) == 0:
            frequencies.append(jokers_count)
        else:
            frequencies[0] += jokers_count

    return tuple(frequencies), sort_key

def part1(fname):
    """
    Part 1 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        inp = [(hand, int(bid)) for hand, bid in (line.split() for line in f)]
    inp.sort(key=lambda x: get_hand_rank(x[0], jokers=False))
    return sum((i+1)*bid for i, (_, bid) in enumerate(inp))

assert part1("examples/71.txt") == 6440
print(part1("input/7.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        inp = [(hand, int(bid)) for hand, bid in (line.split() for line in f)]
    inp.sort(key=lambda x: get_hand_rank(x[0], jokers=True))
    return sum((i+1)*bid for i, (_, bid) in enumerate(inp))

assert part2("examples/71.txt") == 5905
print(part2("input/7.txt"))
