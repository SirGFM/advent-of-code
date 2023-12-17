import re
import sys
import time

regex = re.compile('[0-9a-zA-Z]+')

command = ''
instructions = {}

for line in sys.stdin:
	line = line.strip()

	if len(line) == 0:
		continue
	elif len(command) == 0:
		command = line
		continue

	parts = regex.findall(line)
	instructions[parts[0]] = {
		'L': parts[1],
		'R': parts[2],
	}

def find(tape, node, target, instructions):
	i = 0
	c = ''
	for i, c in enumerate(tape):
		node = instructions[node][c]
		if target(node):
			break
	return (i+1, node)

def count_steps(tape, node, target, instructions):
	steps = 0
	while True:
		i, node = find(tape, node, target, instructions)
		steps += i
		if target(node):
			break
	return steps, node

class Node:
	def __init__(self, steps, name, start = 0):
		self.steps = steps
		self.name = name
		self.start = start

	def __str__(self):
		return f'{self.name} after {self.steps} (mod {self.start})'

def part2(tape, instructions):
	start = []
	for name in instructions:
		if name[2] == 'A':
			start.append(Node(0, name))

	def run(arr):
		found = []

		for node in arr:
			steps, name = count_steps(tape, node.name, lambda x: x[2] == 'Z', instructions)
			found.append(Node(steps, name, (node.start + steps) % len(tape)))

		return found

	# Empyrically tested to cycle from the first loop onwards.
	first = run(start)

	total = 1
	for n in first:
		mult = n.steps / len(tape)
		total *= mult
	print(total * len(tape))

part2(command, instructions)
