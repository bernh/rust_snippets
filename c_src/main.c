#include <stdio.h>

#include "snippet_1.h"

int main(int argc, char** argv) {
  // snippet_1
  set_point(1, 2, 3);  // XXX no module system, where is set_point defined?
  set_point(2, 5, 5);
  print_points();
}

/* compiles without warning! Prints:
Point 1: (0,0)  XXX never initialized
Point 2: (2,3)
Point 3: (5,5)
Point 4: (0,0)  XXX buffer overflow

What happens if you call set_point and print_points from different threads?
XXX data race!
*/
