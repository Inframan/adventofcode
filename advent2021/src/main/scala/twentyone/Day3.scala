package twentyone

import twentyone.data.Direction
import twentyone.data.Position

class Day3() {

  def calculatePowerConsumption(diagnosticReport: List[Int]): Int = {
    var gamma = 0
    var epsilon = 0
    val counts = countBits(diagnosticReport)
    val zeroCount = counts._1
    val oneCount = counts._2

    for (i <- 0 to 31) {
      // our gamma and epsilon start at zeros
      // if there are no ones, we do not want them to be added (since they aren't common)
      if(oneCount(i) > zeroCount(i)) {
        gamma = 1 << i | gamma
      } else if(oneCount(i) > 0) {
        epsilon = 1 << i | epsilon
      }
    }

    return gamma * epsilon
  }

  def countBits(diagnosticReport: List[Int]): (Array[Int], Array[Int]) = {
    val zeroCount = Array.fill(32)(0)
    val oneCount = Array.fill(32)(0)
    // Iterate every INT (32 bit) on the list
    // Count each occurence of 1s and 0s at each index
    for (element <- diagnosticReport) {
      for (i <- 0 to 31) {
        if ( (element >> i & 1) == 1 ) {
          oneCount(i) = oneCount(i) + 1
        } else {
          zeroCount(i) = zeroCount(i) + 1
        }
      }
    }
    (zeroCount, oneCount)
  }

}
