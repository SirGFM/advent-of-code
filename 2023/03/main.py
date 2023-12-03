import re

regex = re.compile(b'\d*|[^A-Za-z0-9]')

def parse_numbers(line, line_number):
	numbers = {}

	pos = 0
	for v in line:
		try:
			number = int(v)
			for i in range(len(v)):
				numbers[pos+i] = (line_number, pos, number)
		except ValueError:
			pass
		pos += len(v)

	return numbers

def is_symbol(v):
	return len(v) == 1 and v != b'.' and (v < b'0' or v > b'9')
def is_gear(v):
	return v == b'*'

def parse_lines(line_number, last, cur, next):
	last_numbers = parse_numbers(last, line_number-1)
	cur_numbers = parse_numbers(cur, line_number)
	next_numbers = parse_numbers(next, line_number+1)

	pos = 0
	found = set()
	gears = []
	for v in cur:
		tmp = set()
		if is_symbol(v):
			for numbers in [last_numbers, cur_numbers, next_numbers]:
				for i in range(3):
					try:
						tmp.add(numbers[pos+i-1])
					except KeyError:
						pass
		found.update(tmp)
		if is_gear(v) and len(tmp) == 2:
			gears.append(tmp)
		pos += len(v)

	return found, gears

last = []
cur = []
next = []

line_number = -1
part_numbers = set()
gear_sum = 0
with open('input.txt', 'rb') as f:
	for line in f:
		line = line.strip()
		read = [x for x in regex.findall(line) if x != b'']

		last = cur
		cur = next
		next = read
		line_number += 1

		if len(cur) != 0:
			found, gears = parse_lines(line_number, last, cur, next)
			part_numbers.update(found)
			if len(gears) > 0:
				for pair in gears:
					ratio = 1
					for _, _, v in pair:
						ratio *= v
					gear_sum += ratio

sum = 0
for _, _, v in part_numbers:
	sum += v
print(sum)
print(gear_sum)
