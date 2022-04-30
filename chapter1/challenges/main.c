#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "dllists.h"

int main(void) {
    printf("yah\n");
}

void tests() {
    Element *head = NULL;  
    head = insert(head, "Hello");
    head = insert(head, "World");
    head = insert(head, "How");
    head = insert(head, "are");

    printf("Found Hello? %c\n", find(head, "Hello") ? 'Y' : 'N');
    printf("Found How? %c\n", find(head, "How") ? 'Y' : 'N');
    printf("Found you?? %c\n", find(head, "you?") ? 'Y' : 'N');

    head = insert(head, "you?");
    printf("Found you?? %c\n", find(head, "you?") ? 'Y' : 'N');

    print_list(head);

    // delete first 
    head = delete(head, "Hello");
    printf("Found Hello? %c\n", find(head, "Hello") ? 'Y' : 'N');

    print_list(head);
    // delete last
    head = delete(head, "you?");
    printf("Found you?? %c\n", find(head, "you?") ? 'Y' : 'N');

    print_list(head);
    // delete middle
    head = delete(head, "are");
    printf("Found are? %c\n", find(head, "are") ? 'Y' : 'N');

    print_list(head);

    return 0;
}
