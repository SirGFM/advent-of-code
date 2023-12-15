import sys

def hash(prev, c):
	return ((prev + ord(c)) * 17) & 0xff

def hash_str(label):
	cur = 0
	for c in label:
		cur = hash(cur, c)
	return cur

def part1():
	total = 0
	cur = 0
	for line in sys.stdin:
		line = line.strip()
		for c in line:
			if c == ',':
				total += cur
				cur = 0
			else:
				cur = hash(cur, c)
	total += cur
	print(total)

def part2():
	hmap = {}
	for line in sys.stdin:
		line = line.strip()
		for value in line.split(','):
			if value[-1] == '-':
				label = value[:-1]
				idx = hash_str(label)
				if idx in hmap:
					hmap[idx].pop(label, None)
			else:
				label, num = value.split('=')
				idx = hash_str(label)

				if not idx in hmap:
					hmap[idx] = {}
				hmap[idx][label] = int(num)

	total = 0
	for box_idx, boxes in hmap.items():
		for i, t in enumerate(boxes.items()):
			label, fl = t

			total += (1 + box_idx) * (1 + i) * fl

	print(total)

if sys.argv[1] == '1':
	part1()
else:
	part2()
