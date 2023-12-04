#include "file.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int read_lines(const char *filename, char **lines, int n_lines) {
    FILE *f;
    char *line = NULL;
    size_t len = 0;
    size_t read;
    int line_read = 0;

    f = fopen(filename, "r");
    while (line_read < n_lines && (read = getline(&line, &len, f) != -1)) {
        lines[line_read] = malloc(strlen(line) + 1);
        strcpy(lines[line_read], line);
        line_read++;
    }

    fclose(f);
    free(line);
    return !0;
}
