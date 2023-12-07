to_value = {
	ord(b'A'): 0xd,
	ord(b'K'): 0xc,
	ord(b'Q'): 0xb,
	ord(b'J'): 0xa,
	ord(b'T'): 0x9,
	ord(b'9'): 0x8,
	ord(b'8'): 0x7,
	ord(b'7'): 0x6,
	ord(b'6'): 0x5,
	ord(b'5'): 0x4,
	ord(b'4'): 0x3,
	ord(b'3'): 0x2,
	ord(b'2'): 0x1,
}

# copy the dict, moving J around
to_value_joker = dict(to_value.items())
to_value_joker[ord(b'J')] = 0x0

# index == num_of_types * 10 + num_in_largest_type
to_rank = {
	15: 0x7, # five of a kind
	24: 0x6, # four of a kind
	23: 0x5, # full house
	33: 0x4, # three of a kind
	32: 0x3, # two pair
	42: 0x2, # one pair
	51: 0x1, # high card
}

def parse(line, card_to_value, adjust_joker=False):
	[hand, bid] = line.split(b' ')

	index = 0
	count = {}
	for card in hand:
		index = (index << 8) + card_to_value[card]

		card_count = 1 if card not in count else count[card] + 1
		count[card] = card_count

	# Remove the jokers from the hand.
	jokers = 0
	if adjust_joker:
		if ord(b'J') in count:
			jokers = count.pop(ord(b'J'))

	card_count = [x for x in count.values()]
	card_count.sort()

	# Add as many jokers in the group that has the most cards.
	if adjust_joker:
		if len(card_count) > 0:
			card_count[-1] += jokers
		else:
			# Hand of all jokers.
			card_count.append(5)

	rank = to_rank[len(card_count) * 10 + card_count[-1]]

	index += rank << (8 * len(hand))
	return (index, hand, int(bid))

entries = []
joker_entries = []
#input_file = 'sample.txt'
input_file = 'input.txt'
with open(input_file, 'rb') as f:
	for line in f:
		line = line.strip()

		(index, _, bid) = parse(line, to_value)
		entries.append((index, bid))

		(index, _, bid) = parse(line, to_value_joker, True)
		joker_entries.append((index, bid))

total = 0
entries.sort()
for i, entry in enumerate(entries):
	total += (i + 1) * entry[1]
print(total)

total = 0
joker_entries.sort()
for i, entry in enumerate(joker_entries):
	total += (i + 1) * entry[1]
print(total)
