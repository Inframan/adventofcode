package twentyone

import scala.annotation.tailrec


class Day1 {

  def measurementIncrease(report: List[Int]): Int = {
    measurementIncrease(report, 0)
  }

  def measurementIncrease(report: List[Int], count: Int): Int = {
    // There is an increase if the head of the list is smaller than it's next element
    // there is no increase if it's the last element of the list
    return if (report.size <= 1) {
      count
    } else if (report.head < report.tail.head) {
      measurementIncrease(report.tail, count + 1)
    } else {
      measurementIncrease(report.tail, count)
    }

  }
}
