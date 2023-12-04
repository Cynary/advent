#!/usr/bin/python3
"""
Day 4 of advent of code 2021
"""
class Board:
    """
    Represents a bingo board.
    """
    ROW_COUNT=5
    COL_COUNT=5

    def __init__(self, lines):
        assert len(lines) == self.ROW_COUNT
        self.number_to_row = {}
        self.number_to_col = {}
        self.marked_row_counts = [0]*self.ROW_COUNT
        self.marked_col_counts = [0]*self.COL_COUNT
        for r, line in enumerate(lines):
            numbers = [int(num) for num in line.split()]
            assert(len(numbers) == len(set(numbers)))
            for c, num in enumerate(numbers):
                assert num not in self.number_to_row
                assert num not in self.number_to_col
                self.number_to_row[num] = r
                self.number_to_col[num] = c

        self.won = False

    def mark(self, number):
        """
        Mark the given number.
        """
        if number not in self.number_to_row:
            assert number not in self.number_to_col
            return False

        r = self.number_to_row[number]
        c = self.number_to_col[number]
        del self.number_to_row[number]
        del self.number_to_col[number]

        self.marked_row_counts[r] += 1
        self.marked_col_counts[c] += 1

        self.won = self.marked_col_counts[c] == self.COL_COUNT or \
            self.marked_row_counts[r] == self.ROW_COUNT

        return self.won

    def sum_unmarked(self):
        """
        Sum the unmarked numbers.
        """
        return sum(self.number_to_col.keys())

def part1(fname):
    """
    Part 1 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        lines = f.readlines()

    drawings = [int(draw) for draw in lines[0][:-1].split(',')]
    assert (len(lines) - 1) % (Board.ROW_COUNT + 1) == 0
    board_count = (len(lines) - 1) // (Board.ROW_COUNT + 1)
    boards = [Board(lines[2 + i*(Board.ROW_COUNT + 1):1 + (i+1)*(Board.ROW_COUNT + 1)])
              for i in range(board_count)]

    for draw in drawings:
        for board in boards:
            if board.mark(draw):
                return draw * board.sum_unmarked()

    assert False, "Someone should have won"

assert part1("examples/41.txt") == 4512
print(part1("input/4.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        lines = f.readlines()

    drawings = [int(draw) for draw in lines[0][:-1].split(',')]
    assert (len(lines) - 1) % (Board.ROW_COUNT + 1) == 0
    board_count = (len(lines) - 1) // (Board.ROW_COUNT + 1)
    boards = [Board(lines[2 + i*(Board.ROW_COUNT + 1):1 + (i+1)*(Board.ROW_COUNT + 1)])
              for i in range(board_count)]

    last_board = None
    for draw in drawings:
        for board in boards:
            if not board.won:
                if board.mark(draw):
                    last_board = draw * board.sum_unmarked()

    assert last_board is not None, "Someone should win eventually"
    return last_board

assert part2("examples/41.txt") == 1924
print(part2("input/4.txt"))
