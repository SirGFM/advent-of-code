import sys

rows = []
columns = []

class Galaxy:
	def __init__(self, x, y):
		self.x = x
		self.y = y

	def dist(self, other, multiplier=2):
		dx = abs(self.x - other.x)
		dy = abs(self.y - other.y)

		x = min(self.x, other.x)
		y = min(self.y, other.y)

		ex = 0
		for i in range(1, dx):
			column = columns[x + i]
			if len(column) == 1 and '.' in column:
				ex += 1
		ey = 0
		for i in range(1, dy):
			row = rows[y + i]
			if len(row) == 1 and '.' in row:
				ey += 1

		expansion =  ex + ey

		return dx + dy + expansion * (multiplier - 1)

galaxies = []

y = 0
for line in sys.stdin:
	line = line.strip()
	if len(line) == 0:
		continue

	if y >= len(rows):
		rows.append(set())

	x = 0
	for c in line:
		if x >= len(columns):
			columns.append(set())

		columns[x].add(c)
		rows[y].add(c)
		if c == '#':
			galaxies.append(Galaxy(x, y))

		x += 1
	y += 1

def get_sum(multiplier=1):
	total = 0
	for i in range(len(galaxies)):
		for j in range(i+1, len(galaxies)):
			dist = galaxies[i].dist(galaxies[j], multiplier)
			total += dist
	print(total)

get_sum()
get_sum(10)
get_sum(100)
get_sum(1000000)
