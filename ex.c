#include <stdbool.h>

bool main(__attribute__((public(0))) bool a, __attribute__((public(0))) bool b) {

  bool c = a && b;
  return 1 || c;

}
