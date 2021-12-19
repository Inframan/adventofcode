package twentyone

import org.scalatest.funspec.AnyFunSpec
import org.scalatest.matchers.should.Matchers

class TestDay1 extends AnyFunSpec with Matchers {

  describe("Given a report") {
    val day1 = Day1()
    val sevenIncreases = List(199, 200, 208, 210, 200, 207, 240, 269, 260, 263)
    describe("With increases") {

      it("Shoul detect the correct number of increases") {
        val increases = day1.measurementIncrease(sevenIncreases)
        assert(increases == 7)
      }
    }
  }
}
