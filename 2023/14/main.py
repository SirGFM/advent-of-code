import sys

columns = []
for line in sys.stdin:
	line = line.strip()
	if len(line) == 0:
		continue

	line = line.replace('.', '0').replace('O', '1')
	for i, c in enumerate(line):
		if i >= len(columns):
			columns.append('')
		columns[i] += c

converted = []
for c in columns:
	tilted = []
	segments = c.split('#')
	for s in segments:
		if len(s) == 0:
			tilted.append('')
			continue
		num = int('0b'+s, 2)
		count = num.bit_count()
		tilted.append('O' * count + '.' * (len(s) - count))
	converted.append('#'.join(tilted))

total = 0
for entry in converted:
	for i, c in enumerate(entry):
		if c != 'O':
			continue
		rank = len(entry) - i
		total += rank
print(total)
