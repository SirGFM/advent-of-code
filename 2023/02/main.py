import re
import sys

r_game = re.compile(b'^Game (\d*)')
r_cube = re.compile(b'(\d*) (blue|red|green)')

max_red = 12
max_green = 13
max_blue = 14

def parse_game(sets):
	valid = True
	req_cubes = {
		b'red': 0,
		b'green': 0,
		b'blue': 0,
	}

	for s in sets:
		cubes = {
			b'red': 0,
			b'green': 0,
			b'blue': 0,
		}

		for res in r_cube.finditer(s):
			cubes[res.group(2)] = int(res.group(1))

		if (
			cubes[b'red'] > max_red or
			cubes[b'green'] > max_green or
			cubes[b'blue'] > max_blue
		):
			valid = False

		for k, v in req_cubes.items():
			if cubes[k] > 0 and cubes[k] > v:
				req_cubes[k] = cubes[k]

	power = 1
	for v in req_cubes.values():
		power *= v

	return valid, power

count = 0
full_power = 0
with open('input.txt', 'rb') as f:
	for line in f:
		line = line.strip()

		sets = line.split(b':')[-1].split(b';')
		valid, power = parse_game(sets)
		if valid:
			game_id = int(r_game.finditer(line).__next__().group(1))
			count += game_id
		if power == 0:
				print(line)
				sys.exit(1)
		full_power += power

print(count)
print(full_power)
