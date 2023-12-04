import re

regex = re.compile(b'\d+')

total = 0
count = 0
multiplier = []
pos = 0
with open('input.txt', 'rb') as f:
	for line in f:
		line = line.strip()
		[prefix, rest] = line.split(b':')
		[winning, hand] = rest.split(b'|')

		if len(multiplier) <= pos:
			multiplier.append(0)

		win_set = set()
		for w in regex.findall(winning):
			win_set.add(w)

		hand_set = set()
		for h in regex.findall(hand):
			hand_set.add(h)

		matches = len(hand_set.intersection(win_set))
		if matches > 0:
			total += pow(2, matches-1)

			copies = 1 + multiplier[pos]
			for i in range(matches):
				idx = pos + i + 1
				if idx < len(multiplier):
					multiplier[idx] += copies
				else:
					multiplier.append(copies)

		if pos < len(multiplier):
			count += multiplier[pos]
		pos += 1

print(total)
print(count+pos)
