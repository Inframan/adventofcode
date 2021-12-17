lazy val commonSettings = Seq(
  resolvers := Seq(
    Resolver.jcenterRepo,
    Resolver.mavenLocal,
    )
)

val advent2021 = project.settings(commonSettings)