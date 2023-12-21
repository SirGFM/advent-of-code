import sys

class Grid:
	def __init__(self):
		self.grid = 0
		self.grid_mask = None
		self.width = 0
		self.height = 0
		self.start = None

	def add_line(self, line):
		# Add a rock on the edges of the grid
		line = f'#{line}#'
		self.width = len(line)

		# Add a border on the top.
		if self.height == 0:
			self.add_bedrock()

		# Find the starting position.
		sx = line.find('S')
		if sx > 0:
			self.start = (sx, self.height)
			line = line.replace('S', '.')

		# Convert the grid to a bitmap.
		bitmap = int(line.replace('#', '1').replace('.', '0'), 2)
		self.grid = self.down(self.grid) | bitmap

		self.height += 1

	def add_bedrock(self):
		bitmap = int('1' * self.width, 2)
		self.grid = self.down(self.grid) | bitmap
		self.height += 1

	def down(self, bitmap):
		return bitmap << self.width

	def up(self, bitmap):
		return bitmap >> self.width

	def left(self, bitmap):
		return bitmap << 1

	def right(self, bitmap):
		return bitmap >> 1

	def mask(self, bitmap):
		if self.grid_mask is None:
			self.grid_mask = ~self.grid
		return bitmap & self.grid_mask

	def starting_pos(self):
		return 1 << (self.start[1] * self.width + self.start[0])

g = Grid()
for line in sys.stdin:
	line = line.strip()
	if len(line) == 0:
		continue
	g.add_line(line)
g.add_bedrock()

positions = g.starting_pos()
steps = int(sys.argv[1])

for i in range(steps):
	new_pos = g.down(positions)
	new_pos |= g.up(positions)
	new_pos |= g.left(positions)
	new_pos |= g.right(positions)

	positions = g.mask(new_pos)
print(positions.bit_count())
