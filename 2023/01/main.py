import re
import sys

regex = re.compile(b'\d')
count = 0

fancy_regex = re.compile(b'0|1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine')
rfancy_regex = re.compile(b'0|1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine'[::-1])
fancy_count = 0

converter = {
	b'one': 1,
	b'two': 2,
	b'three': 3,
	b'four': 4,
	b'five': 5,
	b'six': 6,
	b'seven': 7,
	b'eight': 8,
	b'nine': 9,
}

def to_num(digit):
	if len(digit) == 1:
		return ord(digit) - ord('0')
	return converter[digit]

with open('input.txt', 'rb') as f:
	for line in f:
		line = line.strip()

		digits = regex.findall(line)
		count += 10 * to_num(digits[0]) + to_num(digits[-1])

		first = fancy_regex.finditer(line).__next__().group(0)
		last = rfancy_regex.finditer(line[::-1]).__next__().group(0)
		value = 10 * to_num(first) + to_num(last[::-1])
		fancy_count += value

print(count)
print(fancy_count)
