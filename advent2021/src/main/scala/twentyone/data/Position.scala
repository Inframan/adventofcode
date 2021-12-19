package twentyone.data

case class Position(horizontal: Int, depth: Int) {

  def +(other: Position): Position = {
    Position(this.horizontal + other.horizontal, this.depth + other.depth)
  }

  def -(other: Position): Position = {
    Position(this.horizontal - other.horizontal, this.depth - other.depth)
  }

  def *(other: Position): Position = {
    Position(this.horizontal * other.horizontal, this.depth * other.depth)
  }

  def /(other: Position): Position = {
    Position(this.horizontal / other.horizontal, this.depth / other.depth)
  }

  def *(integer: Int) = {
    Position(this.horizontal * integer, this.depth * integer)
  }

  def /(integer: Int) = {
    Position(this.horizontal / integer, this.depth / integer)
  }

  def ==(other: Position) = {
    this.horizontal == other.horizontal && this.depth == other.depth
  }
}

object Position {
  final val ORIGIN = Position(0, 0)
}
