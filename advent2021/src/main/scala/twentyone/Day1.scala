package twentyone

import scala.annotation.tailrec


class Day1 {

  final def measurementIncrease(report: List[Int]): Int = {
    // There is an increase if the head of the tail is smaller than it's next element
    // there is no increase if it's the last element of the list
    return if (report.size <= 1 || report.head >= Option(report.tail.head).getOrElse(Int.MinValue)) {
      measurementIncrease(report.tail)
    } else {
      measurementIncrease(report.tail) + 1
    }
  }

}
