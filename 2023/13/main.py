import sys

rows = []
columns = []
patterns = []

for line in sys.stdin:
	line = line.strip()
	if len(line) == 0:
		patterns.append((rows, columns))
		rows = []
		columns = []
		continue

	if len(columns) != len(line):
		columns = [''] * len(line)

	rows.append(line)
	for x, c in enumerate(line):
		columns[x] += c
patterns.append((rows, columns))

def compare(arr, start):
	for i in range(start + 1):
		if start + i + 1 >= len(arr):
			return i != 0
		elif arr[start - i] != arr[start + i + 1]:
			return False
	return True

def find(arr):
	for i in range(len(arr)):
		if compare(arr, i):
			return i
	return -1

def part1():
	total = 0
	for rows, columns in patterns:
		row = find(rows) + 1
		if row > 0:
			total += row * 100
			continue
		column = find(columns) + 1
		total += column
		continue
	print(total)

part1()
