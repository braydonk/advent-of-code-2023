#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>

#include "file.h"

#define PUZZLE_LINES 140
// #define PUZZLE_LINES 10

void part_one(char **lines);
void part_two(char **lines);

int main() {
    int ret, i;
    char **lines = malloc(PUZZLE_LINES * sizeof(char*));

    ret = read_lines("./puzzle.txt", lines, PUZZLE_LINES);
    part_one(lines);
    part_two(lines);
    return 0;
}

struct num_list {
    int num;
    struct num_list *next;
};

void add_to_num_list(struct num_list **head, struct num_list *new) {
    struct num_list *curr;

    if (!*head) {
        *head = new;
        return;
    }

    curr = *head;
    while (curr->next != NULL) {
        curr = curr->next;
    }
    curr->next = new;
}

int sum_numbers(struct num_list *head) {
    int sum = 0;
    struct num_list *curr = head;
    while (curr != NULL) {
        sum += curr->num;
        curr = curr->next;
    }
    return sum;
}

void part_one(char **lines) {
    int y, x;
    int i;
    int num_len;
    int found_symbol;
    int part_num;
    size_t line_len;
    int bound_x1, bound_x2, bound_y1, bound_y2;
    int box_x, box_y;
    char *num = NULL;
    struct num_list *head = NULL;
    struct num_list *curr_part = NULL;

    for (y = 0; y < PUZZLE_LINES; y++) {
        x = 0;
        line_len = strlen(lines[y]);
        while (x < line_len) {
            num_len = 0;
            while(isdigit(lines[y][x+num_len])) {
                num_len++;
            }
            if (num_len == 0) {
                x++;
                continue;
            }

            num = malloc((num_len+1) * sizeof(char));
            for (i = 0; i < num_len; i++) {
                num[i] = lines[y][x+i];
            }
            num[num_len] = '\0';

            bound_x1 = x > 0 ? x - 1 : 0;
            bound_y1 = y > 0 ? y - 1 : 0;
            // This works cause there will always be a newline character after.
            bound_x2 = x + num_len;
            bound_y2 = y < (PUZZLE_LINES-1) ? y + 1 : (PUZZLE_LINES-1);

            found_symbol = 0;
            for (box_y = bound_y1; box_y <= bound_y2; box_y++) {
                for (box_x = bound_x1; box_x <= bound_x2; box_x++) {
                    if (isdigit(lines[box_y][box_x])) {
                        continue;
                    }
                    if (lines[box_y][box_x] != '.' && lines[box_y][box_x] != '\n') {
                        found_symbol = 1;
                        break;
                    }
                }
                if (found_symbol) {
                    break;
                }
            }

            if (found_symbol) {
                curr_part = malloc(sizeof(struct num_list));
                curr_part->num = atoi(num);
                curr_part->next = NULL;
                add_to_num_list(&head, curr_part);
            }

            free(num);
            x += num_len;
        }
    }

    printf("%d\n", sum_numbers(head));
}

void part_two(char **lines) {
    int i;
    int y, x;
    int digit_adj;
    int found_nums[2];
    char *num;
    int num_count, num_start, num_x, num_len;
    int bound_x1, bound_x2, bound_y1, bound_y2;
    int box_x, box_y;
    struct num_list *head = NULL, *curr;

    for (y = 0; y < PUZZLE_LINES; y++) {
        for (x = 0; x < strlen(lines[y]); x++) {
            if (lines[y][x] != '*') {
                continue;
            }

            bound_x1 = x > 0 ? x - 1 : 0;
            bound_y1 = y > 0 ? y - 1 : 0;
            // This works cause there will always be a newline character after numbers at the edge.
            bound_x2 = x + 1;
            bound_y2 = y < (PUZZLE_LINES-1) ? y + 1 : (PUZZLE_LINES-1);;

            num_count = 0;
            for (box_y = bound_y1; box_y <= bound_y2; box_y++) {
                box_x = bound_x1;
                while (box_x <= bound_x2) {
                    if (!isdigit(lines[box_y][box_x])) {
                        box_x++;
                        continue;
                    }

                    num_start = box_x;
                    while(num_start > 0)  {
                        num_start--;
                        if (!isdigit(lines[box_y][num_start])) {
                            num_start++;
                            break;
                        }
                    }
                    num_len = 0;
                    while(isdigit(lines[box_y][num_start+num_len])) {
                        num_len++;
                    }
                    num = malloc((num_len+1) * sizeof(char));
                    for (i = 0; i < num_len; i++) {
                        num[i] = lines[box_y][num_start+i];
                    }
                    num[num_len] = '\0';

                    found_nums[num_count] = atoi(num);
                    num_count++;

                    box_x = num_start + num_len;
                }
            }

            if (num_count != 2) {
                continue;
            }

            curr = malloc(sizeof(struct num_list));
            curr->num = found_nums[0] * found_nums[1];
            curr->next = NULL;
            add_to_num_list(&head, curr);
        }
    }

    printf("%d\n", sum_numbers(head));
}
