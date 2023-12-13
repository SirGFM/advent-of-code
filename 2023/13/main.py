import sys

rows = []
columns = []
str_patterns = []

for line in sys.stdin:
	line = line.strip()
	if len(line) == 0:
		str_patterns.append((rows, columns))
		rows = []
		columns = []
		continue

	if len(columns) != len(line):
		columns = [''] * len(line)

	rows.append(line)
	for x, c in enumerate(line):
		columns[x] += c
str_patterns.append((rows, columns))

patterns = []
for str_rows, str_columns in str_patterns:
	rows = []
	columns = []
	for r in str_rows:
		r = '0b' + r.replace('.', '0').replace('#', '1')
		rows.append(int(r, 2))
	for c in str_columns:
		c = '0b' + c.replace('.', '0').replace('#', '1')
		columns.append(int(c, 2))
	patterns.append((rows, columns))

def compare_part1(arr, start):
	for i in range(start + 1):
		if start + i + 1 >= len(arr):
			return i != 0
		elif (arr[start - i] ^ arr[start + i + 1]) != 0:
			return False
	return True

def compare_part2(arr, start):
	did_swap = False
	for i in range(start + 1):
		if start + i + 1 >= len(arr):
			return did_swap

		diff = arr[start - i] ^ arr[start + i + 1]
		if diff != 0:
			if not did_swap and diff.bit_count() == 1:
				did_swap = True
				continue
			return False
	return did_swap

def find(arr, comparer):
	for i in range(len(arr)):
		if comparer(arr, i):
			return i
	return -1

def compute(comparer):
	total = 0
	for rows, columns in patterns:
		row = find(rows, comparer) + 1
		if row > 0:
			total += row * 100
			continue
		column = find(columns, comparer) + 1
		total += column
		continue
	print(total)

compute(compare_part1)
compute(compare_part2)
