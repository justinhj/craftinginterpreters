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

    // remove_if_found first 
    head = remove_if_found(head, "Hello");
    printf("Found Hello? %c\n", find(head, "Hello") ? 'Y' : 'N');

    print_list(head);
    // remove_if_found last
    head = remove_if_found(head, "you?");
    printf("Found you?? %c\n", find(head, "you?") ? 'Y' : 'N');

    print_list(head);
    // remove_if_found middle
    head = remove_if_found(head, "are");
    printf("Found are? %c\n", find(head, "are") ? 'Y' : 'N');

    print_list(head);
}
