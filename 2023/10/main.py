import enum
import sys

class Direction(enum.IntFlag):
	NONE = 0
	UP = 1
	DOWN = 2
	RIGHT = 4
	LEFT = 8

	def dx(self):
		if self == Direction.LEFT:
			return -1
		elif self == Direction.RIGHT:
			return 1
		return 0

	def dy(self):
		if self == Direction.UP:
			return -1
		elif self == Direction.DOWN:
			return 1
		return 0

	def from_dir(self):
		if self == Direction.UP:
			return Direction.DOWN
		elif self == Direction.DOWN:
			return Direction.UP
		elif self == Direction.LEFT:
			return Direction.RIGHT
		elif self == Direction.RIGHT:
			return Direction.LEFT
		return Direction.NONE

	def move_to(self, other):
		return (other & ~self.from_dir())

	def __str__(self):
		ret = []
		for d in self:
			if (d & Direction.UP) != 0:
				ret.append('UP')
			elif (d & Direction.DOWN) != 0:
				ret.append('DOWN')
			elif (d & Direction.LEFT) != 0:
				ret.append('LEFT')
			elif (d & Direction.RIGHT) != 0:
				ret.append('RIGHT')
		if len(ret) == 0:
			return 'NONE'
		return '|'.join(ret)

class Pipe():
	def __init__(self, token, x, y):
		self.x = x
		self.y = y
		self.start = False

		if token == '|':
			self.directions = Direction.UP | Direction.DOWN
		elif token == '-':
			self.directions = Direction.RIGHT | Direction.LEFT
		elif token == 'L':
			self.directions = Direction.UP | Direction.RIGHT
		elif token == 'J':
			self.directions = Direction.UP | Direction.LEFT
		elif token == '7':
			self.directions = Direction.DOWN | Direction.LEFT
		elif token == 'F':
			self.directions = Direction.DOWN | Direction.RIGHT
		elif token == '.':
			self.directions = Direction.NONE
		elif token == 'S':
			self.directions = Direction.UP | Direction.DOWN | Direction.RIGHT | Direction.LEFT
			self.start = True

	def __eq__(self, other):
		return self.x == other.x and self.y == other.y

class Grid:
	def __init__(self):
		self.arr = []
		self.width = 0
		self.height = 0
		self.start = None

	def add_line(self, line):
		if self.width == 0:
			self.width = len(line)

		for x, c in enumerate(line):
			p = Pipe(c, x, self.height)
			self.arr.append(p)
			if p.start:
				self.start = p
		self.height += 1

	def at(self, x, y):
		if x < 0 or x >= self.width or y < 0 or y >= self.height:
			return None
		return self.arr[y * self.width + x]

g = Grid()
for line in sys.stdin:
	line = line.strip()
	if len(line) == 0:
		continue

	g.add_line(line)

def search_direction(g, node, target, d):
	steps = 0

	while True:
		next_node = g.at(node.x + d.dx(), node.y + d.dy())
		if next_node is None:
			return None

		new_dir = d.move_to(next_node.directions)
		if new_dir == Direction.NONE:
			return None

		steps += 1
		if next_node == target:
			return steps
		node = next_node
		d = new_dir

def find_loop(g, node):
	for d in node.directions:
		next_node = g.at(node.x + d.dx(), node.y + d.dy())
		if next_node is None:
			continue
		if d.move_to(next_node.directions) != Direction.NONE:
			steps = search_direction(g, next_node, g.start, d)
			if steps is not None:
				return (d, steps + 1)

_, steps = find_loop(g, g.start)
print(steps / 2)

