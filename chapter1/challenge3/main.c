#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Doubly linked list element
typedef struct Element {
  struct Element *prev;
  struct Element *next;
  void *data; 
} Element;

// Insert Find Delete

// Insert the data at the head of the list
// A pointer to the head of the list is returned
// If the input pointer is null a new list is created
Element *insert(Element *head, void *data) {
  Element *e = malloc(sizeof(Element));
  e->data = data;
  e->prev = NULL;

  if(head == NULL) {  
    e->next = NULL;
  } else {
    head->prev = e;
    e->next = head;
  }

  return e;
}

// Find. Walk the list until you find a matching string
// Return true if found, false otherwise
Element* find(Element *head, const char *target) {
    Element *p = head;
    while(p != NULL) {
        if(strcmp(p->data, target) == 0) {
            return p;
        }
        p = p->next;
    }
    return NULL;
}

// Delete the element and return the new head
Element* delete(Element *head, const char *target) {
    Element *p = find(head, target);
    if(p == NULL) {
        return head;
    } else {
       // Fix prev element 
       if(p->prev) {
           p->prev->next = p->next;
       }
       // Fix next element
       if(p->next) {
           p->next->prev = p->prev;
       }
       Element *next = p->next;
       free(p);
       if(p == head) {
           return next;
       } else {
           return head;
       }
    }
}

void print_list(Element *head) {
    Element *p = head;
    while(p != NULL) {
        printf("%s ", p->data);
        p = p->next;
    }
    printf("\n");
}

int main(void) {
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
