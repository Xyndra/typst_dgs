#import "../lib.typ": *

#set page(width: auto, height: auto, margin: 0pt)

#dgs-canvas(
  x1: -5, y1: -5, x2: 5, y2: 5,
  width: 300pt, height: 300pt,
  theme: "dark",
  objects: (
    dgs-point("A", 1, 2, color: "yellow"),
    dgs-line("A", (3, -1), color: "cyan"),
    dgs-circle((0,0), 3, color: "green", stroke: 2pt),
    dgs-eq("x^2", color: "blue"),
  )
)
