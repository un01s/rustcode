#include <stdio.h>
#include <stdint.h>

typedef struct {
  void* data;
  size_t len;
} FfiVector;

void use_vector(FfiVector vec) {
  int32_t* data = (int32_t*)vec.data;
  for (size_t i = 0; i < vec.len; i++) {
    printf("%d\n", data[i]);
  }
}
