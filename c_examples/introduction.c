// see Section: Introduction
#include <stdio.h>
#include <stdlib.h>

typedef struct {
  char *name;
  double age;
} Person;

Person *new_person(char *name, double age) {
  Person *p = malloc(sizeof(Person));
  p->age = age, p->name = name;
  return p;
}

void *free_person(Person *p) {
  free(p);
  return NULL;
}

int main(void) {
  for (int i = 0; i < 1e5; i++) {
    Person *p = new_person("max musterman", 89);
    p = free_person(p);
    if (p == NULL)
      continue;
    printf("Person{name: '%s', age: %f}\n", p->name, p->age);
  }
  return EXIT_SUCCESS;
}
