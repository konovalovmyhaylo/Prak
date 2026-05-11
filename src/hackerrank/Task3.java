package hackerrank;

import java.util.List;

public class Task3 {

  static int count_fruits(int l, int r, int c, List<Integer> fs) {
    int count = 0;
    for (int f: fs) {
      int pos = c + f;
      if (pos >= l && pos <= r) count++;
    }
    return count;
  }

  public static void countApplesAndOranges(
      int left,
      int right,
      int apple_center,
      int orange_center,
      List<Integer> apples,
      List<Integer> oranges
  ) {
    int apple_count =
        count_fruits(left, right, apple_center, apples);

    int orange_count =
        count_fruits(left, right, orange_center, oranges);

    String s = String.format("%d\n%d\n", apple_count, orange_count);
    //                        ^    ^          |            |
    //                        -----|-----------            |
    //                             |                       |
    //                             \-----------------------/

    System.out.print(s);
  }

}
