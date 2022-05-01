#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "dllists.h"

void print_list(Element *head) {
    char *str = format_to_string(head);
    printf("%s\n", str);
    free(str);
}

int main(void) {
    Element *head = NULL;  
    head = insert(head, "c");
    head = insert(head, "b");
    head = insert(head, "a");

    print_list(head);
}
