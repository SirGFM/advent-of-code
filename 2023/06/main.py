import math
import re

regex = re.compile(b'\d+')

data = []

#input_file = 'sample.txt'
input_file = 'input.txt'
with open(input_file, 'rb') as f:
    for line in f:
        line = line.strip()

        data.append([int(x) for x in regex.findall(line)])

def find_roots(a, b, c): 
    inner = math.sqrt(b * b - 4 * a * c)

    return ((-b + inner) / 2 * a, (-b - inner) / 2 * a)

def solve(t_race, dist):
    # th * (tr - th) = tgt
    # -th² + tr * th -tgt = 0

    t0, t1 = find_roots(-1, t_race, -dist)
    t0 = math.ceil(t0) if math.ceil(t0) != t0 else t0 + 1 
    t1 = math.floor(t1) if math.floor(t1) != t1 else t1 - 1 

    return 1 + t1 - t0


res = 1 
for i in range(len(data[0])):
    t_race = data[0][i]
    dist = data[1][i]

    res *= solve(t_race, dist)

t_race = int(''.join([str(x) for x in data[0]]))
dist = int(''.join([str(x) for x in data[1]]))

print(res)
print(solve(t_race, dist)
