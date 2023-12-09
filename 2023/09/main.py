import sys

def calc_next_part1(seq):
	next_seq = [0] * (len(seq) - 1)

	for i in range(len(next_seq)):
		next_seq[i] = seq[i+1] - seq[i]

	s = set(next_seq)
	if len(s) == 1 and 0 in s:
		return 0
	next_diff = calc_next_part1(next_seq)
	return next_seq[-1] + next_diff

def calc_next_part2(seq):
	next_seq = [0] * (len(seq) - 1)

	for i in range(len(next_seq)):
		next_seq[i] = seq[i+1] - seq[i]

	s = set(next_seq)
	if len(s) == 1 and 0 in s:
		return 0
	next_diff = calc_next_part2(next_seq)
	return next_seq[0] - next_diff

total_part1 = 0
total_part2 = 0
for line in sys.stdin:
	line = line.strip()

	if len(line) == 0:
		continue

	seq = [int(x) for x in line.split(' ')]
	total_part1 += calc_next_part1(seq) + seq[-1]
	total_part2 += seq[0] - calc_next_part2(seq)

print(total_part1)
print(total_part2)
