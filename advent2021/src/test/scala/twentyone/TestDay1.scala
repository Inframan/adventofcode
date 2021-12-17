package twentyone

import org.scalatest.{FunSpec, Matchers}

class TestDay1 extends FunSpec with Matchers {

  describe("Given a report") {
    val day1 = Day1()
    val sevenIncreases = Set(199, 200, 208, 210, 200, 207, 140, 269, 260, 263)
    describe("With increases") {

      it("Shoul detect the correct number of increases") {
        val increases = day1.measurementIncrease(sevenIncreases)
        assert(increases == 7)
      }
    }
  }
}
