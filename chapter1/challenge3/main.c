#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

// Doubly linked list element
typedef struct {
  void *prev;
  void *next;
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
bool find(Element *head, const char *target) {
    Element *p = head;
    while(p != NULL) {
        if(strcmp(p->data, target) == 0) {
            return true;
        }
        p = p->next;
    }
    return false;
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
    return 0;
}
