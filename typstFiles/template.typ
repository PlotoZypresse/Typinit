#let template = doc => {
  set text(font: "New Computer Modern", size: 11pt)
  align(center, text(17pt)[
    Document Title
  ])
  align(center)[
    Name \
    Misc \
    #link("typinti@email.com")
  ]
  align(center)[
    #set par(justify: false)
    *Abstract* \
    Infos about the notes
  ]
  pagebreak()

  set page(header: [
    typinit
    #h(1fr)
    Notetakers name
  ])
  set page(numbering: "1")

  set heading(numbering: "1.1")
  set math.equation(numbering: "(1)")
  doc
}
