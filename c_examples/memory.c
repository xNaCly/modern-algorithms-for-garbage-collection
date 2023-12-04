// see Comparison with other Memory Managment Techniques/Manual Memory Managment
#include <stdlib.h>

int main() {
  // Allocate memory for a single integer
  int *a = malloc(sizeof(int));
  *a = 42;

  // Allocate memory for an array of 10 integers
  int *b = malloc(sizeof(int) * 10);
  for (int i = 0; i < 10; i++) {
    b[i] = i;
  }

  // Free the allocated memory
  free(a);
  free(b);
}
