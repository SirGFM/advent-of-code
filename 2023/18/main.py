import enum
import sys

class Direction(enum.Enum):
	UP = 'U'
	DOWN = 'D'
	LEFT = 'L'
	RIGHT = 'R'

	def dx(self, num):
		if self == Direction.LEFT:
			return -num
		elif self == Direction.RIGHT:
			return num
		return 0

	def dy(self, num):
		if self == Direction.UP:
			return -num
		elif self == Direction.DOWN:
			return num
		return 0

class DigInstructions:
	def __init__(self, d, num, color):
		self.dir = Direction(d)
		self.num = num
		self.r = int('0x'+color[:2], 16)
		self.g = int('0x'+color[2:4], 16)
		self.b = int('0x'+color[4:], 16)

	def __str__(self):
		return f'{self.dir} {self.num} {(self.r, self.g, self.b)}'

class Grid:
	def __init__(self, w, h):
		self.grid = ['.'] * (w * h)
		self.width = w
		self.height = h

	def set(self, x, y, value):
		if x < 0 or x >= self.width or y < 0 or y >= self.height:
			return
		self.grid[y * self.width + x] = value

	def at(self, x, y):
		if x < 0 or x >= self.width or y < 0 or y >= self.height:
			return '.'
		return self.grid[y * self.width + x]


	def dbg(self):
		for y in range(self.height):
			line = ''
			for x in range(self.width):
				line += self.at(x, y)
			print(line)

instructions = []
for line in sys.stdin:
	line = line.strip()
	if len(line) == 0:
		continue

	d, num, color = line.split(' ')
	instructions.append(DigInstructions(d, int(num), color[2:-1]))

x = 0
y = 0
sx = 0
sy = 0
w = 0
h = 0
for i in instructions:
	x += i.dir.dx(i.num)
	y += i.dir.dy(i.num)
	sx = min(sx, x)
	sy = min(sx, y)
	w = max(w, x)
	h = max(h, y)
print(sx, sy, w, h)

# For some reason, starting at (0,0)
# and assuming a grid of (|sx|+w, |sy|+h)
# wasn't enough...
x = 100
y = 300
g = Grid(x + w + 100, y + h + 100)
g.set(x, y, '#')

for i in instructions:
	dx = i.dir.dx(i.num)
	dy = i.dir.dy(i.num)

	while dx != 0:
		d = dx // abs(dx)
		g.set(x, y, '#')
		x += d
		dx -= d

	while dy != 0:
		d = dy // abs(dy)
		g.set(x, y, '#')
		y += d
		dy -= d

def find_wall(g, x, y):
	cur = g.at(x, y)
	if cur == '#':
		return find_non_wall
	return find_wall

def find_non_wall(g, x, y):
	cur = g.at(x, y)
	top = g.at(x, y - 1)
	if top == '#':
		if cur == '#':
			return find_non_wall
		elif validate_inner(g, x, y):
			g.set(x, y, '#')
			return mark_inner
	return find_wall

def validate_inner(g, x, y):
	can_stop = False
	while x < g.width:
		top = g.at(x, y - 1)
		if top != '#':
			return False
		cur = g.at(x, y)
		can_stop = can_stop or cur == '.'
		if can_stop and cur == '#':
			return True
		x += 1
	return False

def mark_inner(g, x, y):
	cur = g.at(x, y)
	if cur == '#':
		return find_wall
	g.set(x, y, '#')
	return mark_inner

#g.dbg()
#import os
#import time
count = 0
for y in range(g.height):
	state = find_wall
	for x in range(g.width):
		state = state(g, x, y)
		cur = g.at(x, y)
		if cur == '#':
			count += 1
			#os.system('clear')
			#g.dbg()
			#time.sleep(0.001)

#g.dbg()
print(count)
