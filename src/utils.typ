#import "@preview/tablex:0.0.9": tablex

#let custom-block(
  item,
) = {
  set align(start)
  block(
    inset: 8pt,
    stroke: black,
    width: 100%,
    spacing: 0pt,
    breakable: true,
    item,
  )
}

#let longtable(
  title: "",
  titles: (),
  caption: "",
  ..items,
) = {
  set par(first-line-indent: 0pt)
  return figure(
    gap: 1.5em,
    kind: table,
    caption: if caption != "" {
      caption
    } else {
      if titles.len() == 0 {
        title
      } else {
        titles.first()
      }
    },
    [
      #if titles.len() == 0 {
        custom-block(
          text(
            weight: "bold",
            title,
          ),
        )
      }
      #for i in range(items.pos().len()) {
        if titles.len() > 0 {
          custom-block(
            text(
              weight: "bold",
              titles.at(i),
            ),
          )
        }
        custom-block(items.pos().at(i))
      }
    ],
  )
}

#let parameter-table(
  caption: "",
  ..items,
) = {
  if caption == "" {
    caption = items.pos().first()
  }
  longtable(
    titles: (
      "Parametra nosaukums",
      "Parametra identifikators",
      "Parametra apraksts",
      "Parametra prasības",
      "Parametra piemērs (/-i)",
    ),
    caption: caption,
    ..items,
  )
}

#let procedure-table(
  caption: "",
  ..items,
) = {
  if caption == "" {
    caption = items.pos().first()
  }
  longtable(
    titles: (
      "Procedūras nosaukums",
      "Procedūras identifikators",
      "Procedūras apraksts",
      "Ievade",
      "Apstrāde",
      "Izvade",
    ),
    caption: caption,
    ..items,
  )
}

#let function-table(
  caption: "",
  ..items,
) = {
  if caption == "" {
    caption = items.pos().first()
  }
  return longtable(
    titles: (
      "Funkcijas nosaukums",
      "Funkcijas identifikators",
      "Apraksts",
      "Ievade",
      "Apstrāde",
      "Izvade",
      "Paziņojumi",
    ),
    caption: caption,
    ..items,
  )
}


#let entity-table-row(
  ..items,
) = {
  (
    items.pos().at(0),
    upper(
      raw(
        items.pos().at(1),
        block: false,
      ),
    ),
    upper(
      raw(
        items.pos().at(2),
        block: false,
      ),
    ),
    items.pos().at(3),
  )
}

#let codeblock(caption, filename, lang: "rust") = {
  show figure: set block(breakable: true)
  show raw: set par.line(numbering: "1")
  set figure(numbering: "1")

  figure(
    caption: caption,
    kind: "attachment",
    supplement: "pielikums",
    raw(
      read(filename),
      block: true,
      lang: lang,
    ),
  )
}

#let components-table(
  caption: str,
  ..body,
) = {
  figure(
    caption: caption,
    kind: table,
    tablex(
      columns: 3,
      [*Komponente*],
      [*Apraksts*],
      [*Pielietojums*],
      ..body,
    ),
  )
}

#let events-table(
  caption: str,
  ..body,
) = {
  figure(
    caption: caption,
    kind: table,
    tablex(
      columns: 3,
      [*Notikums*],
      [*Apraksts*],
      [*Pielietojums*],
      ..body,
    ),
  )
}

#let resources-table(
  caption: str,
  ..body,
) = {
  figure(
    caption: caption,
    kind: table,
    tablex(
      columns: 3,
      [*Resurss*],
      [*Apraksts*],
      [*Pielietojums*],
      ..body,
    ),
  )
}

#let test-table(
  caption: "",
  ..items,
) = {
  if caption == "" {
    caption = items.pos().first()
  }
  return longtable(
    titles: (
      "Testa gadījuma nosaukums",
      "Testa identifikators",
      "Apraksts",
      "Soļi",
      "Sagaidāmais rezultāts",
      "Faktiskais rezultāts",
    ),
    caption: caption,
    ..items,
  )
}


#let entity-table-row(
  ..items,
) = {
  (
    items.pos().at(0),
    upper(
      raw(
        items.pos().at(1),
        block: false,
      ),
    ),
    upper(
      raw(
        items.pos().at(2),
        block: false,
      ),
    ),
    items.pos().at(3),
  )
}
