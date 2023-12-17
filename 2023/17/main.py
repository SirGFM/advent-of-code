import enum
import heapq
import sys

class Direction(enum.Enum):
	UP = 0
	RIGHT = 1
	DOWN = 2
	LEFT = 3
	MAX = 4

class Node:
	def __init__(self, cost, x, y, direction, path_dist, max_dist):
		self.cost = cost
		self.x = x
		self.y = y
		self.dir = direction
		self.path_dist = path_dist
		self.max_dist = max_dist
		self.f_score = 0

	def __lt__(self, other):
		return self.f_score < other.f_score

	def to_idx(self):
		return (self.x, self.y, self.dir, self.path_dist)

	def to_pos(self):
		return (self.x, self.y)

	def dist(self, x, y):
		return abs(self.x - x) + abs(self.y - y)

	def set_f_score(self, value):
		self.f_score = value

	def neighbours(self):
		neighbours = []
		neighbours.append(Direction((self.dir.value - 1) % Direction.MAX.value))
		neighbours.append(Direction((self.dir.value + 1) % Direction.MAX.value))
		if self.path_dist < self.max_dist:
			neighbours.append(self.dir)
		return neighbours

	def move_to(self, direction, m):
		x = self.x
		y = self.y

		if direction == Direction.UP:
			y -= 1
		elif direction == Direction.DOWN:
			y += 1
		elif direction == Direction.LEFT:
			x -= 1
		elif direction == Direction.RIGHT:
			x += 1

		path_dist = 1
		if direction == self.dir:
			path_dist += self.path_dist

		cost = m.at(x, y)
		if cost is None:
			return None
		return Node(cost, x, y, direction, path_dist, self.max_dist)

class Map:
	def __init__(self):
		self.arr = []
		self.width = 0
		self.height = 0

	def add_line(self, line):
		if self.width == 0:
			self.width = len(line)
		self.height += 1

		for c in line:
			self.arr.append(int(c))

	def at(self, x, y):
		if x < 0 or x >= self.width or y < 0 or y >= self.height:
			return None
		return self.arr[y * self.width + x]

class SearchNodes:
	def __init__(self):
		self.heap = []
		self.entries = set()

	def push(self, node):
		idx = node.to_idx()
		if idx in self.entries:
			return

		heapq.heappush(self.heap, node)
		self.entries.add(idx)

	def pop(self):
		node = heapq.heappop(self.heap)
		self.entries.remove(node.to_idx())
		return node

	def is_empty(self):
		return len(self.heap) == 0

m = Map()
for line in sys.stdin:
	line = line.strip()
	if len(line) == 0:
		continue
	m.add_line(line)

def reconstruct_path(came_from, current):
	path = [current]
	while current.to_idx() in came_from:
		current = came_from[current.to_idx()]
		path.append(current)
	path.reverse()
	return path

def a_star(start, goal_idx, h, m):
	open_set = SearchNodes()
	open_set.push(start)
	start.set_f_score(h(start, goal_idx))

	came_from = {}

	g_score = {
		start.to_idx(): 0,
	}

	while not open_set.is_empty():
		current = open_set.pop()
		cur_idx = current.to_idx()

		if current.to_pos() == goal_idx:
			return reconstruct_path(came_from, current)

		for neighbour in current.neighbours():
			node = current.move_to(neighbour, m)
			if node is None:
				continue

			node_idx = node.to_idx()
			tentative_gScore = g_score[cur_idx] + node.cost
			if node_idx not in g_score or tentative_gScore < g_score[node_idx]:
				came_from[node_idx] = current
				g_score[node_idx] = tentative_gScore
				node.set_f_score(tentative_gScore + h(node, goal_idx))
				open_set.push(node)

def heuristic(node, goal_idx):
	return node.dist(goal_idx[0], goal_idx[1])

start = Node(m.at(0, 0), 0, 0, Direction.RIGHT, 1, 3)
path = a_star(start, (m.width - 1, m.height - 1), heuristic, m)

total = -1 * m.at(0, 0)
for node in path:
	total += node.cost
print(total)
