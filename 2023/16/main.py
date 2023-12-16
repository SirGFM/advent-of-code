import enum
import sys

def debug(grid):
	for line in grid:
		dbg = ''
		for node in line:
			dbg += node.beam.value
		print(dbg)
	print()

class Direction(enum.Enum):
	NONE = '.'
	UP = '^'
	DOWN = 'v'
	LEFT = '<'
	RIGHT = '>'

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

class Mirror(enum.Enum):
	NONE = '.'
	VERTICAL = '|'
	HORIZONTAL = '-'
	RIGHT = '/'
	LEFT = '\\'

	def reflect(self, direction):
		if self == Mirror.VERTICAL:
			if direction in (Direction.UP, Direction.DOWN):
				return [direction]
			return [Direction.UP, Direction.DOWN]
		elif self == Mirror.HORIZONTAL:
			if direction in (Direction.RIGHT, Direction.LEFT):
				return [direction]
			return [Direction.RIGHT, Direction.LEFT]
		elif self == Mirror.RIGHT:
			if direction == Direction.UP:
				return [Direction.RIGHT]
			elif direction == Direction.RIGHT:
				return [Direction.UP]
			elif direction == Direction.DOWN:
				return [Direction.LEFT]
			elif direction == Direction.LEFT:
				return [Direction.DOWN]
		elif self == Mirror.LEFT:
			if direction == Direction.UP:
				return [Direction.LEFT]
			elif direction == Direction.LEFT:
				return [Direction.UP]
			elif direction == Direction.DOWN:
				return [Direction.RIGHT]
			elif direction == Direction.RIGHT:
				return [Direction.DOWN]
		return [direction]

class Node:
	def __init__(self, c, x, y):
		self.beam = Direction.NONE
		self.mirror = Mirror(c)
		self.x = x
		self.y = y
		self.exec = set()

	def run(self, grid):
		self.send_beam(self.beam, grid)

	def send_beam(self, direction, grid):
		if direction == Direction.NONE:
			return

		while True:
			x = self.x + direction.dx()
			y = self.y + direction.dy()
			if x < 0 or x >= len(grid[0]) or y < 0 or y >= len(grid):
				return

			next_node = grid[y][x]
			beams = next_node.mirror.reflect(direction)
			if len(beams) > 1:
				next_node.run_beams(beams, grid)
				return
			elif beams[0] in next_node.exec:
				return
			else:
				direction = beams[0]
				next_node.exec.add(direction)
				next_node.set_beam(direction)
				self = next_node

	def run_beams(self, beams, grid):
		for beam in beams:
			if beam not in self.exec:
				self.send_beam(beam, grid)
				self.set_beam(beam)

	def set_beam(self, beam):
		self.exec.add(beam)

		# Keep every 'down' beam so it may be executed on the next step.
		if Direction.DOWN in self.exec:
			self.beam = Direction.DOWN
		else:
			self.beam = beam

grid = []
for y, line in enumerate(sys.stdin):
	line = line.strip()
	if len(line) == 0:
		continue

	grid_line = []
	for x, c in enumerate(line):
		grid_line.append(Node(c, x, y))
	grid.append(grid_line)

def exec(x, y, direction, grid, done):
	starter = Node('.', x, y)
	starter.beam = direction
	starter.run(grid)

	total = 0
	for line in grid:
		for node in line:
			if node.beam != Direction.NONE:
				total += 1
	done(total)

import multiprocessing as mp

def run(x, y, direction, grid, queue):
	exec(x, y, direction, grid, lambda total: queue.put(total))

q = mp.Queue()
jobs = []
min_x = -1
max_x = len(grid[0])
min_y = -1
max_y = len(grid[1])
for y in range(max_y):
	p = mp.Process(target=run, args=(min_x, y, Direction.RIGHT, grid, q))
	jobs.append(p)
	p = mp.Process(target=run, args=(max_x, y, Direction.LEFT, grid, q))
	jobs.append(p)
for x in range(max_x):
	p = mp.Process(target=run, args=(x, min_y, Direction.DOWN, grid, q))
	jobs.append(p)
	p = mp.Process(target=run, args=(x, max_y, Direction.UP, grid, q))
	jobs.append(p)

for j in jobs:
	j.start()
for j in jobs:
	j.join()

total = 0
while not q.empty():
	cur = q.get()
	if cur > total:
		total = cur
print('part 2:', total)

exec(-1, 0, Direction.RIGHT, grid, lambda total: print('part 1:', total))
