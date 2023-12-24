import sys

min_pos, max_pos = map(int, sys.argv[1].split(','))

class Hailstone:
	def __init__(self, line):
		position, velocity = line.split('@')

		self.px, self.py, self.pz = map(int, position.split(','))
		self.vx, self.vy, self.vz = map(int, velocity.split(','))

	def xy_at(self, t):
		x = self.px + self.vx * t
		y = self.py + self.vy * t

		return (x, y)

	def xyz_at(self, t):
		x, y = xy_at(t)
		z = self.pz + self.vz * t

		return (x, y, z)

	def tx(self, other):
		if self.vx == other.vx:
			return None
		return (other.px - self.px) / (self.vx - other.vx)

	def ty(self, other):
		if self.vy == other.vy:
			return None
		return (other.py - self.py) / (self.vy - other.vy)

	def tz(self, other):
		if self.vz == other.vz:
			return None
		return (other.pz - self.pz) / (self.vz - other.vz)

	def in_range_at(self, t, min, max):
		delta = 0.0001
		for axis in self.xyz_at(t):
			if axis - min < delta or axis - max > -delta:
				return False
		return True

	def in_range_at_xy(self, t, min, max):
		delta = 0.0001
		for axis in self.xy_at(t):
			if axis - min < delta or axis - max > -delta:
				return False
		return True

	def intersect_xy(self, other, min, max):
		tx = cur.tx(other)
		ty = cur.ty(other)

		# Check if the interesection in both axis happen at roughly the same time.
		if (
			(tx is None and ty is not None) or
			(tx is not None and ty is None) or
			(abs(tx - ty) < 0.0001)
		):
			t = tx if tx is not None else ty
			if t > 0 and cur.in_range_at(t, min, max):
				return True
		return False

	def cross_times_xy(self, other):
		if (
			(self.vx == 0) or
			(self.vy * other.vx - self.vx * other.vy == 0)
		):
			return None, None

		ot = self.vx * other.py + self.vy * self.px - self.vx * self.py - self.vy * other.px
		ot /= self.vy * other.vx - self.vx * other.vy
		if ot < 0:
			return None, None

		st = (other.px + other.vx * ot - self.px) / self.vx
		if st < 0:
			return None, None

		return st, ot

	def __str__(self):
		return f'{(self.px, self.py, self.pz)} @ {(self.vx, self.vy, self.vz)}'

hstones = []
for line in sys.stdin:
	line = line.strip()
	if len(line) == 0:
		continue

	hstones.append(Hailstone(line))

count = 0
for i, cur in enumerate(hstones):
	for other in hstones[i+1:]:
		ct, ot = cur.cross_times_xy(other)
		if ct == None:
			continue

		if cur.in_range_at_xy(ct, min_pos, max_pos):
			count += 1
print(count)
