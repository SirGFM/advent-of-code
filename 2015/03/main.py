import sys

part1 = (0, 0)
santa = (0, 0)
robot = (0, 0)

pos_part1 = set()
pos_part1.add((0, 0))
pos_part2 = set()
pos_part2.add((0, 0))

def move(orig, c):
	if c == '>':
		return orig[0] + 1, orig[1]
	elif c == '<':
		return orig[0] - 1, orig[1]
	elif c == '^':
		return orig[0], orig[1] - 1
	elif c == 'v':
		return orig[0], orig[1] + 1

for line in sys.stdin:
	line = line.strip()
	if len(line) == 0:
		continue

	for i, c in enumerate(line):
		if i % 2 == 0:
			santa = move(santa, c)
			pos_part2.add(santa)
		else:
			robot = move(robot, c)
			pos_part2.add(robot)

		part1 = move(part1, c)
		pos_part1.add(part1)

print('part 1', len(pos_part1))
print('part 2', len(pos_part2))
