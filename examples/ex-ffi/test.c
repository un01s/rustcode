#include <stdio.h>

extern int add(int a, int b);

int main() {
  int result = add(2, 3);
  printf("The result is %d\n", result);
  return 0;
}
