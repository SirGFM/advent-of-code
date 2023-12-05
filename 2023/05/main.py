import math
import sys

class Mapping:
	def __init__(self, src, dst, count):
		self.src = src
		self.dst = dst
		self.count = count

seeds = []
def get_wanted_seeds(line):
	values = line.split(b' ')[1:]

	global seeds
	seeds = [int(x) for x in values]

	return get_map

src_list = []
src_to_dst = {}
cur_src = ''
cur_dst = ''
def get_map(line):
	name = line.split(b' ')[0]
	[src, _, dst] = name.split(b'-')

	if src not in src_to_dst:
		src_to_dst[src] = {}
	if dst not in src_to_dst[src]:
		src_to_dst[src][dst] = []

	if len(src_list) == 0:
		src_list.append(src)
	src_list.append(dst)

	global cur_src
	cur_src = src
	global cur_dst
	cur_dst = dst

	return get_entries

def get_entries(line):
	[dst, src, count] = line.split(b' ')

	src_to_dst[cur_src][cur_dst].append(Mapping(int(src), int(dst), int(count)))
	return get_entries

state = get_wanted_seeds
with open('input.txt', 'rb') as f:
	for line in f:
		line = line.strip()
		if len(line) == 0:
			state = get_map
			continue

		state = state(line)

def find(seed, count):
	value = seed
	for i in range(len(src_list) - 1):
		src = src_list[i]
		dst = src_list[i + 1]

		for entry in src_to_dst[src][dst]:
			# Check for overlaps
			dist = math.floor(entry.src + entry.count / 2) - math.floor(value + count / 2)
			if abs(dist) > math.floor((entry.count + count) / 2):
				continue

			# Find the intersection
			start = max(value, entry.src)
			end = min(value + count, entry.src + entry.count)

			# Map to the next destination
			count = end - start
			value = entry.dst + start - entry.src
			break
	return value

found = -1
for seed in seeds:
	value = find(seed, 1)
	if found == -1:
		found = value
	found = min(found, value)
print(found)
