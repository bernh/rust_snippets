#include <stdio.h>

#include "measure.h"

typedef struct Measurement {
  int x;
  int y;
} Measurement;

#define NUM_MEASUREMENTS 3

Measurement measurements[NUM_MEASUREMENTS];

void set_measurement(int offset, int x, int y) {
  Measurement p = {x, y};
  measurements[offset] = p;
}

void print_measurements() {
  for (int i = 0; i <= NUM_MEASUREMENTS; i++) {
    printf("Measurement %d: (%d,%d)\n", i+1, measurements[i].x, measurements[i].y);
  }  
}

