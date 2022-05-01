// Doubly linked list element
typedef struct Element {
  struct Element *prev;
  struct Element *next;
  void *data; 
} Element;

// Functions
Element *insert(Element *head, void *data);
Element *find(Element *head, const char *target);
Element *remove_if_found(Element *head, const char *target);
char *format_to_string(Element *head);
