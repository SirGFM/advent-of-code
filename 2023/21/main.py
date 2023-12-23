import math
import sys

class Bitmap:
	def __init__(self, w = 0, h = 0):
		self._set_dimensions(w, h)
		self._make_empty()

	def _set_dimensions(self, w, h):
		# Add two extra bits for the border.
		self.width = w + 2
		self.height = h + 2

		self.bitmap = 0

	def _add_line(self, value):
		line = value * self.width
		line = int(line, 2)
		self.bitmap = (self.bitmap << self.width) | line

	def _make_empty(self):
		self._add_line('0')
		for _ in range(g.height):
			self._add_line('0')
		self._add_line('0')

	def from_grid(self, g):
		self._set_dimensions(g.width, g.height)

		# Create each row in the base bitmap with enough horizontal clones.
		self._add_line('1')
		for x in range(0, len(g.grid), g.width):
			line = g.grid[x:x+g.width].replace('#', '1').replace('.', '0')
			row = int('1' + line + '1', 2)
			self.bitmap = (self.bitmap << self.width) | row
		self._add_line('1')

	def from_start(self, g):
		self._set_dimensions(g.width, g.height)

		self._make_empty()

		x = g.start[0] + 1
		y = g.start[1] + 1
		self.set_pos(x, y)

	def set_pos(self, x, y):
		self.bitmap |= 1 << (y * self.width + x)

	def down(self):
		return self.bitmap << self.width

	def up(self):
		return self.bitmap >> self.width

	def left(self):
		return self.bitmap << 1

	def right(self):
		return self.bitmap >> 1

	def mask(self, other):
		return other & ~self.bitmap

	def replace(self, new_bitmap):
		self.bitmap = new_bitmap

	def merge(self, bitmap):
		self.bitmap |= bitmap

	def invert(self):
		self.bitmap = ~self.bitmap

	def count(self):
		return self.bitmap.bit_count()

	def __str__(self):
		mask = int('1' * self.width, 2)

		remainder = self.bitmap
		line = ''
		for y in range(self.height):
			row = bin(remainder & mask)[2:]
			row = row.replace('0', '.').replace('1', '#')
			if len(row) < self.width:
				row = '.' * (self.width - len(row)) + row
			line = row + '\n' + line
			remainder = remainder >> self.width

		return line

class Grid:
	def __init__(self):
		self.grid = ''
		self.width = 0
		self.height = 0
		self.start = None

	def add_line(self, line):
		self.width = len(line)

		# Find the starting position.
		sx = line.find('S')
		if sx > 0:
			self.start = (sx, self.height)
			line = line.replace('S', '.')

		self.grid += line

		self.height += 1

g = Grid()
for line in sys.stdin:
	line = line.strip()
	if len(line) == 0:
		continue
	g.add_line(line)

steps = int(sys.argv[1])
try:
	infinite = sys.argv[2] != ""
except:
	infinite = False

garden = Bitmap()
garden.from_grid(g)

pos = Bitmap()
pos.from_start(g)

left = Bitmap(g.width, g.height)
right = Bitmap(g.width, g.height)
up = Bitmap(g.width, g.height)
down = Bitmap(g.width, g.height)

for x in range(up.width):
	down.set_pos(x, 0)
	up.set_pos(x, up.height - 1)
for y in range(left.height):
	right.set_pos(0, y)
	left.set_pos(left.width - 1, y)

left.invert()
right.invert()
up.invert()
down.invert()

full_garden = {
	(0, 0): pos,
}

for i in range(steps):
	queue = []
	for key, pos in full_garden.items():
		x, y = key

		new_pos = pos.down()
		new_pos |= pos.up()
		new_pos |= pos.left()
		new_pos |= pos.right()

		if infinite:
			values = [
				(left, -1, 0, lambda mask, bitmap: bitmap >> (mask.width - 2)),
				(right, 1, 0, lambda mask, bitmap: bitmap << (mask.width - 2)),
				(up, 0, -1, lambda mask, bitmap: bitmap >> mask.width * (mask.height - 2)),
				(down, 0, 1, lambda mask, bitmap: bitmap << mask.width * (mask.height - 2)),
			]
			for mask, dx, dy, fn in values:
				to_dir = mask.mask(new_pos)
				if to_dir != 0:
					to_dir = fn(mask, to_dir)
					queue.append((x + dx, y + dy, garden.mask(to_dir)))

		new_pos = garden.mask(new_pos)
		pos.replace(new_pos)

	for node in queue:
		x, y, pos = node

		try:
			next_pos = full_garden[(x, y)]
		except:
			next_pos = Bitmap(g.width, g.height)
			full_garden[(x, y)] = next_pos

		next_pos.merge(pos)

count = 0
for b in full_garden.values():
	count += b.count()
print(count)
