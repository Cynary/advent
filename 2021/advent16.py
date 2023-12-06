#!/usr/bin/python3
"""
Day 16 of advent of code 2021
"""

import functools

class Operator:
    """
    Represents an operator
    """
    BIT_LENGTH_TYPE = 0
    PACKET_LENGTH_TYPE = 1
    TYPE_TO_OPERATION = {
        0: sum,
        1: lambda children: functools.reduce(lambda x, y: x*y, children, 1),
        2: min,
        3: max,
        5: lambda children: int(children[0] > children[1]),
        6: lambda children: int(children[0] < children[1]),
        7: lambda children: int(children[0] == children[1]),
    }

    def __init__(self, version, packet_type, children):
        self.version = version
        self.packet_type = packet_type
        self.children = children

    def get_version_sum(self):
        """
        Get the version sum of this operator (its version plus the version of its children).
        """
        return self.version + sum(child.get_version_sum() for child in self.children)

    def get_value(self):
        """
        Get the value of this operator.
        """
        child_values = [child.get_value() for child in self.children]
        return self.TYPE_TO_OPERATION[self.packet_type](child_values)

class Literal:
    """
    Represents a literal
    """
    TYPE = 4
    def __init__(self, version, packet_type, value):
        self.version = version
        assert packet_type == self.TYPE
        self.value = value

    def get_version_sum(self):
        """
        Get the version sum of this literal (just its version).
        """
        return self.version

    def get_value(self):
        """
        Get the value of this literal.
        """
        return self.value

def parse_packet(message):
    """
    Parse a packet.  Returns (message consumed, packet)
    """
    version = int(message[:3], 2)
    packet_type = int(message[3:6], 2)

    if packet_type == Literal.TYPE:
        index = 6
        value = 0
        while message[index] != '0':
            value |= int(message[index+1:index+5], 2)
            value <<= 4
            index += 5
        value |= int(message[index+1:index+5], 2)
        return (index+5, Literal(version, packet_type, value))

    # This is an operator.
    length_type_id = int(message[6], 2)
    children = []
    if length_type_id == Operator.BIT_LENGTH_TYPE:
        length = int(message[7:7+15], 2)
        index = 7+15
        end_index = index+length

        while index != end_index:
            assert index < end_index
            delta, child = parse_packet(message[index:])
            index += delta
            children.append(child)
    else:
        assert length_type_id == Operator.PACKET_LENGTH_TYPE, length_type_id
        length = int(message[7:7+11], 2)
        index = 7+11
        for _ in range(length):
            delta, child = parse_packet(message[index:])
            index += delta
            children.append(child)

    return (index, Operator(version, packet_type, children))

def part1(fname):
    """
    Part 1 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        hex_message = f.readline().strip()
    bin_message = ''.join(bin(int(c, 16))[2:].zfill(4) for c in hex_message)
    return parse_packet(bin_message)[1].get_version_sum()

assert part1("examples/161.txt") == 31
print(part1("input/16.txt"))

def part2(fname):
    """
    Part 2 of the problem.
    """
    with open(fname, encoding="utf-8") as f:
        hex_message = f.readline().strip()
    bin_message = ''.join(bin(int(c, 16))[2:].zfill(4) for c in hex_message)
    return parse_packet(bin_message)[1].get_value()

assert part2("examples/162.txt") == 1
print(part2("input/16.txt"))
