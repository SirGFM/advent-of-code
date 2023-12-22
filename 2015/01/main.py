import sys

count = 0
pos = 0
done = False
for line in sys.stdin:
	line = line.strip()
	if len(line) == 0:
		continue

	for c in line:
		if c == '(':
			count += 1
		elif c == ')':
			count -= 1
		else:
			raise Exception('a')

		if count == -1 and not done:
			print('part 2', pos)
			done = True

		pos += 1
print('part 1', count)
