package twentyone

import org.scalatest.funspec.AnyFunSpec
import org.scalatest.matchers.should.Matchers
import twentyone.data.Direction
import twentyone.data.Position

class TestDay3 extends AnyFunSpec with Matchers {

  describe("Given a report") {
    val report = List(
      Integer.parseInt("00100", 2),
      Integer.parseInt("11110", 2),
      Integer.parseInt("10110", 2),
      Integer.parseInt("10111", 2),
      Integer.parseInt("10101", 2),
      Integer.parseInt("01111", 2),
      Integer.parseInt("00111", 2),
      Integer.parseInt("11100", 2),
      Integer.parseInt("10000", 2),
      Integer.parseInt("11001", 2),
      Integer.parseInt("00010", 2),
      Integer.parseInt("01010", 2),
    )
    it("Compute the correct power consumption") {
      val day3 = Day3()
      val consumption = day3.calculatePowerConsumption(report)
      assert(consumption == 198)
    }
  }
}
