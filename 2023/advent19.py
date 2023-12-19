#!/usr/bin/python3
"""
Day 19 of advent of code 2023
"""

import copy
from math import prod

def part1(fname):
    """
    Part 1 of the problem.
    """
    workflows = {}
    parts = []
    with open(fname, encoding="utf-8") as f:
        workflows_str, parts_str = f.read().split("\n\n")
        for workflow in workflows_str.split():
            name, steps = workflow[:-1].split("{")
            workflows[name] = [step.split(':') for step in steps.split(",")]

        for part in parts_str.split():
            parts.append(part[1:-1].split(","))

    accepted = []
    for part in parts:
        # Dirty
        for prop in part:
            exec(prop)

        current_workflow = "in"
        while current_workflow not in ('A', 'R'):
            for step in workflows[current_workflow][:-1]:
                if eval(step[0]):
                    current_workflow = step[1]
                    break
            else:
                current_workflow = workflows[current_workflow][-1][0]

        if current_workflow == 'A':
            accepted.append(part)

    total=0
    for part in accepted:
        for prop in part:
            exec(prop)
        total += eval("x+m+a+s")

    return total

assert part1("examples/191.txt") == 19114
print(part1("input/19.txt"))

class WorkflowStep:
    """
    Represents a workflow step.
    """
    def __init__(self, step_str):
        test, next_wflow = step_str.split(":")
        self.test = (test[0], test[1] == '>', int(test[2:]), next_wflow)

    def fails(self, values):
        """
        Returns the values if the test fails and can fail (otherwise None), and the next workflow.
        """
        val, gt, boundary, _ = self.test
        values = copy.deepcopy(values)
        if gt:
            if values[val][0] > boundary:
                return None
            values[val][1] = min(values[val][1], boundary)
            return values

        if values[val][1] < boundary:
            return None
        values[val][0] = max(values[val][0], boundary)
        return values

    def passes(self, values):
        """
        Returns the values if the test passes, and can pass (otherwise None).
        """
        val, gt, boundary, next_wflow = self.test
        values = copy.deepcopy(values)
        if gt:
            if values[val][1] <= boundary:
                return None, next_wflow
            values[val][0] = max(values[val][0], boundary+1)
            return values, next_wflow

        if values[val][0] >= boundary:
            return None, next_wflow
        values[val][1] = min(values[val][1], boundary-1)
        return values, next_wflow

    def __repr__(self):
        return str(self.test)


def dfs(workflows, wflow_name, values):
    """
    Recursively finds the combinations that are accepted.
    """
    if wflow_name in ('A', 'R'):
        return 0 if wflow_name == 'R' else prod(r[1]-r[0]+1 for r in values.values())

    current_workflow = workflows[wflow_name]
    combinations = 0
    for step in current_workflow[:-1]:
        pass_values, next_wflow = step.passes(values)
        if pass_values is not None:
            combinations += dfs(workflows, next_wflow, pass_values)
        values = step.fails(values)
        if values is None:
            return combinations

    next_wflow = current_workflow[-1]
    return combinations + dfs(workflows, next_wflow, values)

def part2(fname):
    """
    Part 2 of the problem.
    """
    workflows = {}
    with open(fname, encoding="utf-8") as f:
        workflows_str, _ = f.read().split("\n\n")
        for workflow in workflows_str.split():
            name, steps = workflow[:-1].split("{")
            step_str_list = steps.split(",")
            workflows[name] = [WorkflowStep(step) for step in step_str_list[:-1]]
            workflows[name].append(step_str_list[-1])

    values = {
        'x': [1, 4000],
        'm': [1, 4000],
        'a': [1, 4000],
        's': [1, 4000],
    }
    total=dfs(workflows, "in", values)
    return total

assert part2("examples/191.txt") == 167409079868000
print(part2("input/19.txt"))
