// Doubly linked list element
typedef struct Element {
  struct Element *prev;
  struct Element *next;
  void *data; 
} Element;

// Functions
Element *insert(Element *head, void *data);
Element *find(Element *head, const char *target);
Element *delete(Element *head, const char *target);
void print_list(Element *head);
