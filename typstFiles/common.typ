#import "@preview/note-me:0.5.0": *
#import "@preview/ctheorems:1.1.3": *
#import "@preview/finite:0.5.0": automaton
#import "@preview/lovelace:0.3.0"
#import "@preview/diagraph:0.3.6": *

#show: thmrules
#set heading(numbering: "1.1.")

#let definition = thmbox(
  "definition",
  "Definition",

  //inset: (x: 1.2em, top: 1em),
  titlefmt: strong,
  stroke: 1pt,
  fill: rgb("#baffc9"),
)
#let theorem = thmbox(
  "theorem",
  "Theorem",
  fill: rgb("#e8e8f8"),
  titlefmt: strong,
  // stroke: 1pt,
)
#let proof = thmproof(
  "proof",
  "Proof",
  titlefmt: strong,
  // stroke: 1pt,
)
#let corollary = thmplain(
  "corollary",
  "Corollary",
  base: "theorem",
  titlefmt: strong,
  fill: rgb("#f8e8e8"),
  // stroke: 1pt,
)
#let example = thmplain(
  "example",
  "Example",
  //fill: rgb("#bae1ff"),
  titlefmt: strong,
  //stroke: 1pt,
).with(numbering: none)

#let lemma = thmbox(
  "theorem", // identifier - same as that of theorem
  "Lemma", // head
  fill: rgb("#efe6ff"),
  // stroke: 1pt,
)
