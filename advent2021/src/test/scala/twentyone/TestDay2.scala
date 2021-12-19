package twentyone

import org.scalatest.funspec.AnyFunSpec
import org.scalatest.matchers.should.Matchers
import twentyone.data.Direction
import twentyone.data.Position

class TestDay2 extends AnyFunSpec with Matchers {

  describe("Given a course") {
    val course = List(Direction(Direction.FORWARD, 5), Direction(Direction.DOWN, 5), Direction(Direction.FORWARD, 8), Direction(Direction.UP, 3), Direction(Direction.DOWN, 8), Direction(Direction.FORWARD, 2))
    describe("With directions") {

      it("Should follow the directions") {
        val day2 = Day2()
        day2.followCourse(course)
        assert(day2.position == Position(15, 10))
      }

      it("Should multiply the end possition correctly") {
        val day2 = Day2()
        val multiplied = day2.followAndMultiply(course)
        assert(multiplied == 150)
      }
    }
  }
}
