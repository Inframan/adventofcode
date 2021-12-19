package twentyone

import twentyone.data.Direction
import twentyone.data.Position

class Day2(var position: Position) {

  def this() = this(Position.ORIGIN)

  def followCourse(course: List[Direction]) = {
    for (dir <- course) {
      position = position + dir.getDirection()
    }
  }

  def followAndMultiply(course: List[Direction]): Int = {
    followCourse(course)
    position.horizontal * position.depth
  }

}
