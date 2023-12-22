import sys

total = 0
with_ribbon = 0
for line in sys.stdin:
	line = line.strip()
	if len(line) == 0:
		continue

	l, w, h = line.split('x')
	l = int(l)
	w = int(w)
	h = int(h)

	extra = min(l*w, w*h, l*h)
	ribbon = 2*min(l+w, w+h, l+h)
	bow = l*w*h

	total += 2*l*w + 2*w*h + 2*h*l + extra
	with_ribbon += ribbon + bow
print('part 1', total)
print('part 2', with_ribbon)
