package twentyone.data

class Direction(direction: Position) {
  def this(direction: Position, distance: Int) = this(direction * distance)

  def getDirection(): Position = this.direction
}

object Direction {
  final val FORWARD: Position = Position(1, 0)
  final val BACKWARD: Position = Position(-1, 0)
  final val UP: Position = Position(0, -1)
  final val DOWN: Position = Position(0, 1)
}
