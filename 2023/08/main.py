import re
import sys
import time

regex = re.compile('[a-zA-Z]+')

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

def find(node, tape):
	if tape in instructions[node]:
		return instructions[node][tape]
	else:
		mid_node = find(node, tape[0])
		next_node = find(mid_node, tape[1:])
		instructions[node][tape] = next_node
		return next_node

def part1():
	steps = 0
	node = 'AAA'
	target = 'ZZZ'
	while True:
		next_node = find(node, command)
		for j in range(0, len(command)):
			tmp = instructions[node][command[j]]
			if tmp == target:
				print(steps + j + 1)
				return
			node = tmp
		steps += len(command)
		node = next_node

part1()
