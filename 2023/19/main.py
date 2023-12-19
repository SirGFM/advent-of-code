import sys

class Option:
	def __init__(self, min=1, max=4000):
		self.min = min
		self.max = max

	def clone(self):
		return Option(self.min, self.max)

	def validate(self):
		return self.min < self.max

	def count(self):
		return self.max - self.min + 1

	def __str__(self):
		return f'[{self.min}, {self.max}]'

class Command:
	def __init__(self, line):
		self.part = line[0]
		self.op = line[1]
		self.value = int(line[2:])

	def accept(self, parts):
		other = parts[self.part]
		if (
			(self.op == '>' and other > self.value) or
			(self.op == '<' and other < self.value)
		):
			return True

		return False

	def convert(self, options, invert=False):
		diff = 1 if not invert else 0
		opt_value = options[self.part]
		if not invert and self.op == '>' or invert and self.op == '<':
			opt_value.min = max(self.value + diff, opt_value.min)
		else:
			opt_value.max = min(self.value - diff, opt_value.max)

		if not opt_value.validate():
			return None
		return options

	def __str__(self):
		return f'{self.part} {self.op} {self.value}'

class Statement:
	def __init__(self, line):
		arr = line.split(':')
		self.next_workflow = arr[-1]

		self.command = None
		if len(arr) > 1:
			self.command = Command(arr[0])

	def next(self, parts):
		if self.command is None:
			return self.next_workflow

		if self.command.accept(parts):
			return self.next_workflow

		return None

	def next_options(self, options):
		new_options = {}
		for key, value in options.items():
			new_options[key] = value.clone()

		if self.command is None:
			return self.next_workflow, new_options, None

		new_options = self.command.convert(new_options)
		if not new_options:
			return 'R', None, new_options

		reverse_options = {}
		for key, value in options.items():
			reverse_options[key] = value.clone()
		reverse_options = self.command.convert(reverse_options, True)

		return self.next_workflow, new_options, reverse_options

	def __str__(self):
		return f'{self.command} -> {self.next_workflow}'

workflows = {}
def parse_workflow(line):
	global workflows

	name, remainder = line[:-1].split('{')
	stmts = remainder.split(',')

	arr = []
	for stmt in stmts:
		arr.append(Statement(stmt))

	workflows[name] =  arr

part_list = []
def parse_parts(line):
	global part_list

	parts = {}
	arr = line[1:-1].split(',')
	for entry in arr:
		parts[entry[0]] = int(entry[2:])

	part_list.append(parts)

state = parse_workflow
for line in sys.stdin:
	line = line.strip()
	if len(line) == 0:
		state = parse_parts
		continue

	state(line)

def run(workflows, part, state):
	while state is not None:
		for workflow in workflows[state]:
			state = workflow.next(parts)
			if state == 'A':
				count = 0
				for part in parts.values():
					count += part
				return count
			elif state == 'R':
				return 0
			elif state is not None:
				break
	raise Exception('invalid state')

total = 0
for parts in part_list:
	total += run(workflows, parts, 'in')
print(total)

def run_from_state(start):
	done = []
	pairs = [start]
	while len(pairs) > 0:
		state, options = pairs.pop()
		for workflow in workflows[state]:
			new_state, new_options, next_options = workflow.next_options(options)
			if new_state == 'A':
				done.append(new_options)
			elif new_state != 'R':
				pairs.append((new_state, new_options))

			if next_options is None:
				break
			options = next_options

	total = 0
	for options in done:
		count = 1
		for option in options.values():
			count *= option.count()
		total += count
	return total

options = {
	'x': Option(),
	'm': Option(),
	'a': Option(),
	's': Option(),
}

total = run_from_state(('in', options))
print(total)
