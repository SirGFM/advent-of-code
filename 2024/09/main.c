#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

struct list_ends;

struct list {
	struct list_ends *ends;
	struct list *next;
	struct list *prev;
};

struct list_ends {
	struct list *head;
	struct list *tail;
};

void list_insert_before(struct list *self, struct list *new) {
	new->prev = self->prev;

	new->next = self;
	self->prev = new;

	new->ends = self->ends;

	if (new->prev == NULL) {
		new->ends->head = new;
	} else {
		new->prev->next = new;
	}
}

void list_insert_after(struct list *self, struct list *new) {
	new->prev = self;
	new->next = self->next;
	self->next = new;

	new->ends = self->ends;

	if (new->next == NULL) {
		new->ends->tail = new;
	} else {
		new->next->prev = new;
	}
}

void list_insert_end(struct list_ends *list_ends, struct list *new) {
	new->ends = list_ends;

	if (list_ends->head == NULL) {
		list_ends->head = new;
		list_ends->tail = new;
	} else {
		list_insert_after(list_ends->tail, new);
	}
}

void list_remove(struct list *self) {
	if (self->prev != NULL) {
		self->prev->next = self->next;
	} else if (self == self->ends->head) {
		self->ends->head = self->next;
	}

	if (self->next != NULL) {
		self->next->prev = self->prev;
	} else if (self == self->ends->tail) {
		self->ends->tail = self->prev;
	}
}

void list_empty(struct list *self) {
	struct list *node;

	if (self->ends->head == NULL) {
		return;
	}
	node = self->ends->head;

	while (node != NULL) {
		struct list *tmp = node->next;

		list_remove(node);
		free(node);

		node = tmp;
	}
}

struct frag {
	struct list list;
	uint64_t id;
	uint64_t size;
};

struct frag* frag_new(uint64_t id, uint64_t size, struct list_ends *nodes) {
	struct frag *new;

	new = (struct frag*)calloc(1, sizeof(struct frag));

	new->size = size;
	new->id = id;

	list_insert_end(nodes, &new->list);

	return new;
}

void list_print(struct list *node) {
	while (node != NULL) {
		struct frag *cur;
		int i;

		cur = (struct frag*)node;
		node = node->next;

		for (i = 0; i < cur->size; i++) {
			if (cur->id != 0) {
				printf("%ld", cur->id - 1);
			} else {
				printf(".");
			}
		}
		printf("-");
	}
	printf("\n");
}

struct aux_frag_list {
	struct list list;
	struct frag *node;
};

struct aux_frag_list* aux_frag_new(struct frag *node, struct list_ends *nodes) {
	struct aux_frag_list *new;

	new = (struct aux_frag_list*)calloc(1, sizeof(struct aux_frag_list));

	new->node = node;

	list_insert_end(nodes, &new->list);

	return new;
}

int main(int argc, char *argv[]) {
	struct list_ends nodes = {0};
	struct list_ends empty = {0};
	struct list *node = NULL;
	uint64_t id = 0;
	uint64_t idx = 0;
	uint64_t result_p1;

	// Parse the input.
	while (!feof(stdin)) {
		char buf[100];
		size_t num;
		int i;

		num = fread(buf, sizeof(char), sizeof(buf) / sizeof(char), stdin);
		for (i = 0; i < num; i++) {
			struct frag *new;
			uint64_t frag_id;
			uint64_t size;

			if (buf[i] < '0' || buf[i] > '9') {
				continue;
			}

			size = (uint64_t)(buf[i] - '0');
			if (id % 2 == 0) {
				frag_id = id / 2 + 1;
			} else {
				frag_id = 0;
			}
			new = frag_new(frag_id, size, &nodes);

			if (frag_id == 0) {
				aux_frag_new(new, &empty);
			}

			id++;
		}
	}

	// Populate the empty slots.
	node = nodes.tail;
	while (node != NULL) {
		struct frag *cur;
		struct aux_frag_list *cur_empty;
		uint64_t consumed_size;

		cur = (struct frag*)node;

		// Skip over empty nodes.
		if (cur->id == 0) {
			node = node->prev;
			continue;
		}

		cur_empty = (struct aux_frag_list*)empty.head;

		// Update the nodes.
		if (cur->size <= cur_empty->node->size) {
			// Move this node before the empty node.
			node = node->prev;

			list_remove(&cur->list);
			list_insert_before(&cur_empty->node->list, &cur->list);

			consumed_size = cur->size;
		} else {
			struct frag *new;

			// Add a new node before the empty node, but continue working on the current node.
			new = frag_new(cur->id, cur_empty->node->size, &nodes);
			list_remove(&new->list);
			list_insert_before(&cur_empty->node->list, &new->list);

			consumed_size = cur_empty->node->size;
			cur->size -= cur_empty->node->size;
		}

		if (consumed_size < cur_empty->node->size) {
			cur_empty->node->size -= consumed_size;
		} else {
			list_remove(&cur_empty->list);
			list_remove(&cur_empty->node->list);
			free(cur_empty->node);
			free(cur_empty);
		}

		// Merge the consumed node at the end of the empty list.
		cur_empty = (struct aux_frag_list*)empty.tail;
		cur_empty->node->size += consumed_size;

		if (empty.tail->prev == NULL) {
			break;
		}
	}

	result_p1 = 0;
	idx = 0;
	for (node = nodes.head; node != NULL; node = node->next) {
		struct frag *cur;

		cur = (struct frag*)node;
		if (cur->id != 0) {
			uint64_t id = cur->id - 1;
			uint64_t i;

			for (i = 0; i < cur->size; i++) {
				uint64_t tmp = (i + idx) * id;
				assert(ULONG_MAX - result_p1 > tmp);
				result_p1 += tmp;
			}
		}

		idx += cur->size;
	}

	list_empty(nodes.head);
	list_empty(empty.head);

	printf("part 1: %ld\n", result_p1);

	return 0;
}
