
#include <stdio.h>
#include "other.h"

int main() {
  printf("Hello, lowbuild, from C\n");
  printf("new message\n");

  int sumxy = sum(50, 11);
  printf("sum: %i", sumxy);
}

