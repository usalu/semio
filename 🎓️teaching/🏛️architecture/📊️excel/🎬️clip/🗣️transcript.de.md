# Excel-Videos

Vollständige Texte der vier Excel-Videos. Alle Werte stammen aus einem gerundeten Lehrmodell des Nakagin Capsule Tower in Ginza, Tokio (Kisho Kurokawa, 1970–1972, 2022 abgerissen); angenommene Werte sind in den Videos als solche gekennzeichnet. Ein Absatz pro gesprochenem Schritt, in Sprechreihenfolge.

## Inhalt

1. [Aufbau einer Arbeitsmappe](#aufbau-einer-arbeitsmappe)
2. [Zellbezüge](#zellbezüge)
3. [Das Raumprogramm in Excel](#das-raumprogramm-in-excel)
4. [Heizwärmebedarf in der Nakagin-Arbeitsmappe](#heizwärmebedarf-in-der-nakagin-arbeitsmappe)

## Aufbau einer Arbeitsmappe

Video: `excel_anatomy`

Der Nakagin-Turm im Lehrmodell: Kapseln in vier Typen. Wie viel Fläche bietet eine, und was passiert, wenn sich ihre Breite ändert?

Eine Arbeitsmappe ist die ganze Datei: Nakagin.xlsx enthält die Berechnung des ganzen Gebäudes.

Arbeitsblätter sind die Register darin: Programm, je Kapseltyp ein Blatt, Klima und Energie.

Ein Arbeitsblatt ist ein Raster: Spalten haben Buchstaben, Zeilen Nummern. Typ A beschreibt eine Kapsel.

Eine Zelle heißt nach Spalte und Zeile. B2 ist die Breite der Kapsel, angezeigt mit Einheit.

Ein Bereich fasst Zellen zusammen: B2:B4 enthält Breite, Länge und Höhe.

Das Namenfeld zeigt die Adresse, die Bearbeitungsleiste zeigt den Inhalt der Zelle.

Eingetippte Zahlen sind Konstanten, ebenso Text wie Breite: Excel speichert beides unverändert.

Die Zelle zeigt eine gerundete Zahl mit Einheit, speichert aber die genaue Zahl. Das Format ändert die Anzeige, nicht den Wert.

Eine Formel beginnt mit einem Gleichheitszeichen. Die Fläche ist Innenbreite mal Innenlänge: B5 mal B6.

Excel färbt jeden Bezug und umrahmt seine Zelle: Die Formel zeigt auf ihre Quellen, dieselben Farben markieren Breite und Länge im Boden.

Das Ergebnis ist der Boden der Kapsel, als schraffierte Fläche gezeichnet.

Typischer Fehler: Ohne Klammern rechnet Excel Punkt vor Strich, eine Seite des Umfangs fehlt. Mit Klammern stimmt es.

Eine Funktion ist eine benannte Berechnung. SUMME addiert einen Bereich: Die Einbauten ergeben ihre Gesamtfläche.

In der deutschen Version heißt es SUMME, Argumente trennt ein Semikolon, Dezimalstellen ein Komma.

Weiten wir den Innenraum bis zur Außenwand: Alle abhängigen Zellen rechnen neu, und der Boden folgt.

Spurpfeile zeigen Vorgänger, die Zellen, die eine Formel nutzt, und Nachfolger, die Zellen, die diese nutzen.

Typische Fehler haben Ursachen. Eine leere Innenlänge ergibt die Fläche null, und die Division durch null ergibt #DIV/0!.

Eine falsch geschriebene Funktion ergibt #NAME?: SUMM gibt es nicht.

Wird eine Quellzeile gelöscht, hat der Bezug kein Ziel mehr: #REF! im englischen, #BEZUG! im deutschen Excel.

Text, obwohl eine Zahl erwartet wird, ergibt #WERT! und im Englischen #VALUE!: Hier wurde die Innenlänge mit Einheit getippt.

Tippen wir die Zahl neu, kehren die Werte zurück: Fläche, Volumen und Einbauten.

Merken wir uns: Arbeitsmappe, Arbeitsblatt, Zelle, Bezug, Formel, Funktion, Namenfeld, Bearbeitungsleiste. Nächstes Video: Bezüge.

## Zellbezüge

Video: `excel_references`

In diesem Lehrmodell des Nakagin-Turms steht die Breite jeder Typ-A-Kapsel in einer Zelle. Was folgt, wenn sie sich ändert?

Ein Bezug ist eine Adresse, keine Zahl. D3 zeigt auf B2 und B3, eine breitere Kapsel ergibt also eine größere Dachfläche.

Für jedes Bauteil eine eigene Formel ist mühsam. U·A einmal in F3 schreiben und nach unten ausfüllen: Die Formel wandert mit der Zeile.

Eine Zeile tiefer kopiert, wird aus D3*E3 die Formel D4*E4. Relative Teile verschieben sich um den Versatz, hier eine Zeile.

Jede Zeile berechnet ein Bauteil. Beim Ausfüllen leuchtet jedes Bauteil an der Kapsel auf, sobald sein U·A-Wert erscheint.

Die Rückwand liegt am Kern, ihre Formel trägt daher noch den Faktor b: Eine ausgefüllte Formel muss zu jeder Zeile passen.

Die Spalte mit SUMME addieren: Die Summe entspricht dem Transmissionsverlust H_T, den Zelle B11 schon enthält.

Der Wärmeverlust je Bauteil braucht die Auslegungs-Temperaturdifferenz, abgelegt in einer Zelle, D10. Jede Zeile muss auf diese Zelle zeigen.

Ein Dollarzeichen vor Spalte und Zeile macht einen Bezug absolut. Die Taste F4 wechselt durch die vier Formen.

Nach unten ausgefüllt bleibt der absolute Bezug in jeder Zeile auf der Eingabe: Jedes Bauteil sieht dieselbe Temperaturdifferenz.

Der typische Fehler: Das Dollarzeichen vergessen und ausfüllen. Der Bezug rutscht von der Eingabe ab, die Werte werden falsch.

Mit F4 den Bezug sperren und erneut ausfüllen: Jede Zeile trifft dieselbe Eingabe. Werte und Bauteile stimmen wieder.

Eine Kreuztabelle: U-Werte links, Temperaturdifferenzen oben. Eine gemischte Formel, ausgefüllt, ergibt jede Wärmestromdichte.

Dasselbe Muster zählt Kapseln: oben Typen, links Türme. In B11 sperrt ein Bezug die Spalte, der andere die Zeile.

Nach rechts und unten in einem Zug ausgefüllt: Jede Zelle holt sich ihren Turm und ihren Typ und zählt diese Kapseln am Turm.

Ein Blattbezug nennt zuerst das Blatt, dann ein Ausrufezeichen. Ein Name mit Leerzeichen braucht Apostrophe.

Nach unten ausgefüllt wandert nur der Zellteil. Der Blattname bleibt Text: Der Bezug rutscht auf dem Blatt Typ A nach unten.

Darum jede Kopie anpassen: Typ B liest sein Blatt, Typ C und D ihre, und jede Zeile zeigt wieder ihre Nettofläche.

Ein 3D-Bezug addiert dieselbe Zelle aller Blätter zwischen zwei Blattregistern: hier die Heizlast B13 je einer Kapsel jedes Typs.

Die Register zwischen den beiden Namen legen den Bereich fest. Wird ein Blatt herausgeschoben, fällt sein Wert aus der Summe.

Der zweite Fehler: Das Blatt von Typ C löschen. Direkte Bezüge zeigen einen Fehler, die 3D-Summe schrumpft dagegen still.

Das Dollarzeichen schützt nur beim Kopieren. Wird die Eingabezeile gelöscht, bricht auch ein absoluter Bezug.

Ein definierter Name macht einen absoluten Bezug lesbar: theta_i und theta_e stehen für die Klimazellen.

Fazit: relativ wandert, absolut bleibt, gemischt sperrt eine Richtung, Blatt- und 3D-Bezüge queren Register, Namen machen es lesbar.

## Das Raumprogramm in Excel

Video: `excel_program`

Im Lehrmodell trägt der Nakagin Capsule Tower 140 Kapseln in vier Typen. Wie viele von jedem Typ, und wie viel Fläche bieten sie?

Ein Raumprogramm listet je Raum die Einbauten, die Flächen und die Nutzer. Hier ist es auf eine Kapsel reduziert.

Pro Kapseltyp ein Arbeitsblatt. Das Blatt von Typ A listet die Einbauten dieser Kapsel mit Breite und Länge in Metern.

Eine Formel multipliziert Breite, Länge und Anzahl. Nach unten ausgefüllt erhält jeder Einbau seine Grundfläche und seine eigene Schraffur.

Die Nettogrundfläche ist die Innenbreite mal die Innenlänge.

Nettogrundfläche minus Einbauten ergibt die freie Fläche. Die Nutzerzahl ist eine eingetippte Konstante: eine Person.

Nettogrundfläche mal Innenhöhe ergibt das Volumen in m³. Die Zelle zeigt eine Einheit, gespeichert ist nur die Zahl.

Typ B, C und D haben eigene Blätter: gleiche Hülle, andere Einbauten, also andere freie Fläche.

Wie viele von jedem Typ? Die 140 Kapselzeilen markieren und Als Tabelle formatieren wählen: Die Liste wird zur Tabelle Kapseln.

Jede Tabellenzeile ist eine Kapsel. Wählt man eine Zeile, leuchtet ihre Kapsel am Turm.

ZÄHLENWENN zählt einen Typ in der Tabellenspalte. Die gezählten Kapseln leuchten in der Farbe von Typ A.

Nach unten ausgefüllt wandert das Kriterium von A4 bis A7, die Tabellenspalte bleibt ganz. Vier Farben bedecken den Turm.

ZÄHLENWENNS fügt eine zweite Bedingung hinzu: Typ und Turm. Die Matrix zeigt, wie viele Kapseln jedes Typs in welchem Turm stehen.

Die freie Fläche kommt aus dem Blatt jedes Typs. Die Nettogrundfläche ist gleich; die freie Fläche unterscheidet die Typen.

XVERWEIS sucht den genannten Typ und liefert Anzahl und freie Fläche: über den Schlüssel, nicht über die Position.

SUMMENPRODUKT multipliziert jede Anzahl mit ihrer freien Fläche und addiert die Produkte: die freie Fläche aller Kapseln in m².

Spalte D multipliziert Anzahl mit freier Fläche. Das Balkendiagramm zeigt die vier Summen; jeder Balken wächst, sobald seine Zelle rechnet.

Summen je Turm: SUMMEWENN addiert die Nettogrundflächen eines Turms aus der Tabelle. Turm A und Turm B ergeben das Gebäude.

Metabolismus-Idee: Kapseln sind austauschbar. Ändert man in der Liste einen Typ auf einem Geschoss, wechselt diese Kapsel die Farbe.

Anzahlen und Balken aktualisieren sich sofort; die freie Fläche je Kapsel bleibt. Die Summe wächst, weil Typ A mehr freie Fläche hat als Typ C.

Plausibilitätsprüfung: Die Anzahlen ergeben die Zeilen der Liste und die 140 Kapseln des Gebäudes.

Ein Tippfehler, ein Leerzeichen hinter dem Typ, wird nicht gezählt: Die Kapsel wird grau, die Prüfung zeigt 139 und FALSCH.

Das Leerzeichen löschen: Die Kapsel wird wieder gezählt, die Prüfung ist WAHR. Eine Auswahlliste verhindert den Tippfehler.

Rückblick: Aus 140 Kapseln wird Fläche: Tabelle, ZÄHLENWENN(S), XVERWEIS, SUMMENPRODUKT, SUMMEWENN.

## Heizwärmebedarf in der Nakagin-Arbeitsmappe

Video: `excel_energy`

Wie viel Wärme braucht eine Nakagin-Kapsel im Tokioter Winter? Wir rechnen es in Excel, Blatt für Blatt.

Jeder Kapseltyp hat ein eigenes Arbeitsblatt. Typ A enthält Maße, Flächen und Ergebnisse dieses vereinfachten Lehrmodells.

Drei Konstanten beschreiben die Hülle: Breite, Länge und Höhe. An der Kapsel sind es die drei Maßlinien.

Jede Fläche ist eine Formel aus diesen Maßen. Die Frontwand ist Breite mal Höhe abzüglich des runden Fensters.

Die Rückwand ist am Kern verschraubt, deshalb entweicht nur ein Teil ihrer Wärme: Ihr U·A wird mit dem Anbindungsfaktor b multipliziert.

Der U-Wert sagt, wie viele Watt je Quadratmeter und Kelvin Temperaturunterschied hindurchgehen. Glas verliert am meisten.

Fläche mal U-Wert ergibt U·A in Watt je Kelvin. Je größer das U·A einer Fläche, desto länger und dicker ihr Pfeil.

H_T ist die Summe aller U·A. SUMMENPRODUKT: Fläche mal U-Wert mal Temperaturfaktor, die Rückwand eingeschlossen.

Das Blatt Klima speichert Tokio unter definierten Namen: Temperaturen, Gradtage, b, Luftwechsel und Luftwärmekapazität.

Heizgradtage sind eine Fläche: Der Fehlbetrag jedes Monats unter der 18-Grad-Linie mal seine Tage, summiert über das Jahr.

Ein typischer Fehler: Die angebundene Rückwand zählt wie eine freie Wand. Mit b gleich eins wächst H_T, der Verlust wird überschätzt.

Der Faktor steht in einer Zelle auf dem Blatt Klima. Setzt man ihn zurück, kehren Rückwand-Pfeil und H_T zu ihren Werten zurück.

Auch Luft entweicht: Luftwärmekapazität mal Luftwechselrate mal Volumen ergibt den Lüftungswärmeverlust H_V.

H_T und H_V zusammen sind der Wärmeverlust der ganzen Kapsel je Kelvin. Alle weiteren Ergebnisse bauen auf diesen beiden Zellen auf.

Die Heizlast Φ ist der Verlust je Kelvin mal Temperaturdifferenz: die Watt, die die Heizung am Auslegungstag liefern muss.

Jahresbedarf: Verlust je Kelvin mal Gradtage mal 24 Stunden, geteilt durch 1000, in kWh. Geteilt durch die Fläche: kWh je m².

Das Blatt Energie multipliziert den Bedarf jedes Typs mit seiner Kapselzahl aus dem Blatt Programm und summiert das ganze Gebäude.

A, B und D teilen eine Hülle, Einbauten ändern den Verlust nicht. Nur C mit dem größeren Fenster braucht mehr.

Warum so viel für so wenig Boden? Fünf Außenflächen umhüllen einen kleinen Raum. Beheizte Nachbarn würden den Bedarf deutlich senken.

Welche Zellen hängen vom U-Wert des Fensters ab? Die Nachfolger laufen durch die Typ-Blätter bis zur Gebäudesumme.

Ein besseres Fenster in dieser einen Eingabezelle genügt, und jede abhängige Zelle, jeder Pfeil und jede Summe folgt.

Ein typischer Fehler: Jemand tippt die Gebäudesumme als Konstante ein. Sie folgt der Änderung nicht mehr.

Setzt man die Formel wieder ein, stimmt die Summe wieder mit ihren Zeilen überein.

Zusammenfassung: U·A je Fläche, H_T und H_V, Last aus der Temperaturdifferenz, Bedarf aus Gradtagen, mal Kapselzahl.
