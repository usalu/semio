# Grasshopper-Videos

Vollständige Texte der fünf Grasshopper-Videos. Alle Werte stammen aus einem gerundeten Lehrmodell des Nakagin Capsule Tower in Ginza, Tokio (Kisho Kurokawa, 1970–1972, 2022 abgerissen); angenommene Werte sind in den Videos als solche gekennzeichnet. Ein Absatz pro gesprochenem Schritt, in Sprechreihenfolge.

## Inhalt

1. [Aufbau eines Grasshopper-Skripts](#aufbau-eines-grasshopper-skripts)
2. [Eine Kapsel, parametrisch](#eine-kapsel-parametrisch)
3. [Von der Kapsel zum Turm](#von-der-kapsel-zum-turm)
4. [Casting: Wenn Typen sich treffen](#casting-wenn-typen-sich-treffen)
5. [Datenbäume am Turm](#datenbäume-am-turm)

## Aufbau eines Grasshopper-Skripts

Video: `grasshopper_anatomy`

Das ist eine Kapsel des Nakagin Capsule Tower, ein vereinfachtes Lehrmodell. Wie geben wir sie dem Computer?

Wir beschreiben sie als Skript: eine Grasshopper-Definition, gespeichert als .gh-Datei. Ihre Arbeitsfläche ist der Canvas.

Ein Number Slider ist die erste Eingabe: ein Name, ein Bereich von 1,0 bis 5,0, eine Dezimalstelle und ein Wert, 2,5.

Eine Komponente macht aus Eingaben eine Ausgabe: Center Box, Kurzname Box, baut aus Breite, Länge und Höhe eine Box.

Jede Komponente hat Namen und Kurznamen; der Canvas zeigt den Kurznamen. Eingänge liegen links, Ausgänge rechts, je mit einem Griff.

Verbindungen tragen Daten von einem Ausgangsgriff zu einem Eingangsgriff und laufen normalerweise von links nach rechts.

Jeder Schieberegler ist ein Maß: Verbindung und Maßlinie teilen seine Farbe. Länge 5,0 und Höhe 3,0 Meter lassen die Box wachsen.

Zieht man einen Schieberegler, folgt das Ergebnis. Die Breite geht von 2,5 auf 3,0 Meter und zurück, und die Box ändert sich mit.

Das Vorschau-Auge blendet nur die Zeichnung aus, nicht die Daten: Die Box verschwindet, das Volumen zeigt weiter 25 Kubikmeter.

Grau ist in Ordnung. Zahl und Text in Z machen sie orange, eine Warnung: eine Box, ein null. Mit 2.5 statt tall ist sie grau.

Ohne Brauchbares wird sie rot, ein Fehler: Aus dem Text tall macht keine Verbindung eine Höhe. Mit dem Text 2.5 in Z kehrt die Box zurück.

Ein Panel zeigt, was eine Verbindung trägt: Pfad {0}, Index 0, Wert 2,5. Es macht eine verborgene Zahl lesbar.

Ein Parameter hält Daten eines Typs. Frei steht er allein; als Eingang X einer Komponente gehört er zu einem Port. Beide halten die Zahl 2,5.

Ein Boolean Toggle hat zwei Zustände. True gibt der Kapsel ihr rundes Fenster, 1,3 Meter im Durchmesser, in der Stirnseite.

Eine Werteliste bietet feste Auswahlmöglichkeiten: die Kapseltypen A bis D. Wählt man einen, übernimmt die Kapsel dessen Fenster.

Ein Plugin, eine .gha-Datei, fügt dem Ribbon einen Reiter hinzu. Ladybug Tools untersucht die Sonne am Fenster: mehr Sonne, wärmere Tönung.

Nun die Daten. Ein Wert ist ein Element: die Höhe 0 m des ersten Geschosses, eine Kapsel an Turm A.

Eine Series mit Schrittweite 2,75 Meter erzeugt eine Liste: neun Höhen in einem Ast {0}, Indizes 0 bis 8, je eine Kapsel pro Geschoss.

Ändert man den Schieberegler Anzahl von 9 auf 11, wächst die Liste um zwei: Turm B hat elf Kapselgeschosse.

Viele Listen bilden einen Datenbaum. Jede Fassade eines Geschosses ist ein Ast mit eigenem Pfad, hier {0;0;3;1}, und enthält zwei Kapseln.

Typischer Fehler: einen Datenbaum wie eine lange Liste lesen. Der Index 1 allein passt zu allen 27 Ästen von Turm A: 27 Kapseln.

Erst Pfad plus Index nennt genau eine Kapsel: Pfad {0;0;3;1}, Index 1. Das Panel zeigt beides.

Die Begriffe des Canvas: Skript, Komponente, Parameter, Eingang, Ausgang, Schieberegler, Panel und Plugin. Englisch steht daneben.

Die Begriffe der Daten: Element, Liste, Datenbaum, Ast, Pfad und Index. Die englischen Wörter stehen daneben.

## Eine Kapsel, parametrisch

Video: `grasshopper_capsule`

Am Nakagin-Turm hängen 140 Kapseln an zwei Kernen, doch eine einzige Grasshopper-Definition baut jede davon.

Parametrisch heißt: Parameter speisen Abhängigkeiten, Abhängigkeiten bauen Geometrie. Ändert sich ein Wert, folgt alles.

Sechs Number Sliders halten die Parameter: Breite, Länge, Höhe, Eckenradius, Fensterdurchmesser und Fensterversatz.

Alles beginnt auf einer Ebene: Ursprung plus X-, Y- und Z-Achse in Rhinos Rot, Grün und Blau. Das ist die Grundebene der Kapsel.

Center Box nimmt die Ebene und die Slider für Breite, Länge und Höhe. Die Box zentriert sich auf der Ebene, die Hälfte versinkt im Boden.

Plane Origin hebt die Ebene um die halbe Länge und die halbe Höhe: Jetzt steht die Box mit der Unterseite auf der Grundebene.

Ziehen Sie den Längen-Slider: Maßlinie, Box und jede Komponente dahinter folgen im selben Moment.

Fillet Edge rundet die Kanten mit dem Eckenradius-Slider: Die weiche Hülle der Kapsel entsteht.

Ein typischer Fehler: Ein Circle auf der Grundebene liegt am Boden, nicht an der Vorderseite. Eine Verbindung weiß nicht, wo Sie ihn meinen.

Die Lösung: Eine Ebene auf der Vorderseite, verschoben um Länge und Fensterversatz, setzt den Kreis an die richtige Stelle.

Solid Difference zieht einen Zylinder dieses Kreises von der Hülle ab: Das runde Fenster entsteht an der Vorderseite.

Vier Punkte auf der Rückseite markieren die hochfesten Schrauben, mit denen die Kapsel am Kern hängt.

Jede Slider-Domain ist eine Entwurfsgrenze; hier sind die Grenzen Annahmen, zum Beispiel eine Transporthülle.

Schieben Sie die Breite über die Domain hinaus: Der Slider hält am oberen Ende an, die Hülle wird rot. Die Domain ist eine Entwurfsregel.

Die Kapseltypen des Lehrmodells sind eine Definition mit verschiedenen Parameterwerten.

Speisen Sie Typ A bis D in dieselben Slider ein: Fensterversatz und Fensterdurchmesser ändern sich, die Definition baut vier Kapseln.

Am Kern braucht die Kapsel eine Zielebene an der Wand; die Grundebene muss dort landen.

Orient bewegt die Kapsel in einem Schritt von ihrer Grundebene A auf die Zielebene B.

Dreht man die Zielebene um die Kernachse, dreht die Kapsel mit, denn sie folgt der Ebene B.

List Item wählt den Typ je Ebene: Index 0 ergibt Typ A, 1 Typ B, 2 Typ C, 3 Typ D.

Acht Ebenen brauchen Indizes bis 7, doch es gibt nur vier Typen: Ohne Wrap warnt List Item ab Index 4.

Mit Wrap läuft der Index herum: 4 ergibt wieder Typ A, 5 Typ B, und jede Ebene bekommt eine Kapsel.

Rückblick: Parameter, Abhängigkeit, Geometrie, Domain, Ebene, Orient, List Item und Index.

## Von der Kapsel zum Turm

Video: `grasshopper_tower`

Der Nakagin Capsule Tower hat 140 Kapseln. Wie werden sie zu zwei Türmen?

Der Turm hat fünf Teile: Kern, Ebenen, Kapseln, Kopf und Sockel. Jeder bekommt eine eigene farbige Gruppe.

Der Kern ist eine quadratische Extrusion. Seite, Geschosshöhe und Geschossanzahl sind Slider und bestimmen die Höhe.

Das Ergebnis ist der Schacht: ein Quader mit einer Linie auf jedem Geschoss. Die Geschossanzahl wird direkt aus der Definition gelesen.

Eine Series liefert eine Höhe pro Geschoss. Jede Höhe wird zu einer Ebene für die Kapseln am Schacht.

Eine zweite Liste enthält die vier Fassadenwinkel. Dreht man eine Ebene um jeden Winkel, entsteht eine Ebene pro Fassade.

Jede Ebene trägt ihre Achsen: X läuft entlang der Fassade, Y zeigt nach außen, Z zeigt nach oben.

Auf jeder Fassade sitzen zwei Kapseln nebeneinander: Item 0 und Item 1, je einen Schritt beidseits entlang X.

Orient setzt die Kapsel von ihrer Basisebene auf jede Zielebene. Eine Kapsel, eine Komponente, jede Ebene.

Ein Typmuster wählt für jede Ebene den Kapseltyp. List Item liest das Muster per Index; nur die Liste ändert sich.

Nun der Slider Kapselgeschosse: Die Anzahl ist Geschosse mal Fassaden mal Items, und die Zeichnung wächst mit.

Keine einzige Verbindung ändert sich. Die Definition weiß nicht, wie hoch der Turm ist; sie folgt nur der Zahl.

Typischer Fehler: Ohne Cull Index hat Turm B 88 Kapseln, zwei davon im obersten Geschoss im Westen, wo es keine gibt.

Cull Index entfernt genau diese zwei Ebenen, die Indizes 6 und 7. Nun liefert die Definition 86, die Zahl des Gebäudes.

Der Kopf ist ein eigenes kleines Parametermodell. Höhe und Überstand sind zwei Slider auf dem Kern.

Er sitzt auf dem Kern genau auf der Höhe, die der Slider Kapselgeschosse vorgibt, und steigt mit dem Turm.

Nur ein extrudierter Baukörper: Grundriss und Höhe.

Das ist Absicht. Der Sockel ist Kontext, nicht das Thema: eine Extrusion genügt, ein Kapselparameter dagegen ändert jede Kopie.

Dieselbe Definition baut Turm A und Turm B. Kapselgeschosse und Fassaden unterscheiden sich; die Formeln zeigen beides.

Zusammen ergeben sie die historische Zahl von 140 Kapseln, aus der Definition gelesen und nicht eingetippt.

Metabolismus: Kapseln sind austauschbar. Ersetzt man Typ B durch Typ C, bekommt jede Kapsel dieses Typs sofort das größere Fenster.

Zusammenfassung: Kern, Ebenen, Kapseln, Kopf, Sockel. So sieht das Lehrmodell des Nakagin Capsule Tower aus.

## Casting: Wenn Typen sich treffen

Video: `grasshopper_casting`

Diese Kapsel soll nach Westen schauen, doch ihr Fenster zeigt nach Norden. Eine stille Umwandlung hat das bewirkt. Welche?

In diesem Lehrmodell des Nakagin Capsule Tower hat jeder Parameter einen Typ: Breite Zahl, Geschosse Ganzzahl, Platzierung Ebene.

Eine Verbindung nimmt alles an, was man zieht. Passen die Typen nicht, versucht Grasshopper, die Daten umzuwandeln.

Ein Panel enthält den Text 2.5. Der Zahleneingang liest ihn als Zahl 2,5, und die Kapsel wird 2,5 Meter breit.

Der Regler zeigt 10,6, doch Geschosse verlangt eine Ganzzahl. Grasshopper rundet, hier auf 11; in Rhino 8 prüfen.

Grasshopper rundet; genau bei .5 geht es zur geraden Zahl: 2,5 gibt 2, 3,5 gibt 4. In Rhino 8 prüfen.

Wer die Rundung selbst bestimmen will, setzt eine Komponente wie Floor vor den Ganzzahl-Eingang: Das elfte Geschoss verschwindet.

Ein Punkt an einem Ebenen-Eingang wird zur Ebene an diesem Punkt mit den Weltachsen. Hier ist er der Ursprung eines Fassadenplatzes.

An der Westfassade muss die Kapsel nach Westen schauen. Die Weltebene richtet sie nach Norden aus: Das Fenster dreht um 90°.

Eine Linie wird zur Ebene, deren Z-Achse der Linie folgt. Vom Kern zum Platz zeigt Z nach außen, doch X und Y sind nur geraten.

Die Ebene wird ausdrücklich aus Ursprung und Achsen gebaut: Die Kapsel dreht sich an ihren Platz und schaut nach außen.

Eine Zahl kann eine Domain vertreten: Die Kapsellänge 4,0 wird zum Intervall 0 To 4, und Divide Domain teilt es.

Ein Kreis ist eine Kurve, eine Box ein Brep: Fensterrand und Kapselkörper gehen als der allgemeinere Typ durch.

Ein Boolean Toggle wird zur Ganzzahl: False ist 0, True ist 1, und das Fenster folgt.

Umgekehrt kann eine Zahl das Fenster schalten: 0 blendet es aus, jeder andere Wert blendet es ein.

Achtung Dezimalkomma: Der Text 2,5 kann als Zahl scheitern. Besser 2.5 schreiben oder einen Regler nehmen.

Ein Panel mit Typ A: Der Zahleneingang scheitert, die Komponente wird rot und meldet „Data conversion failed from Text to Number“.

Warum? Grasshopper probiert in fester Reihenfolge: gleicher Typ, dann die Quelle, dann das Ziel. Typ A scheitert überall, der Platz bleibt leer.

Eine Bezeichnung ist keine Zahl. Eine Werteliste liefert die echte Breite von Typ A, und die Komponente wird wieder grau.

Umwandlungen folgen Familien: Innerhalb einer Familie klappen sie, über Familien hinweg braucht es eine Komponente.

Jede markierte Zelle ist eine gesehene Umwandlung: Grün klappte, Rot scheiterte, gestrichelte Umrisse sind in Rhino 8 noch zu prüfen.

Drei Gewohnheiten: mit einem Panel prüfen, Ebenen ausdrücklich bauen und Bezeichnungen nie in einen Zahleneingang geben.

## Datenbäume am Turm

Video: `grasshopper_trees`

In diesem Lehrmodell hat der Nakagin Capsule Tower zwei Türme und 140 Kapseln. Eine Liste enthält alle, aber welche Kapsel sitzt wo?

Element 21 ist eine von 140 Kapseln. Die Liste verrät weder Turm noch Geschoss noch Fassade.

Ein Datenbaum behält die Adresse: Gebäude, Turm, Geschoss, Fassade. Jede Ebene verzweigt sich weiter.

Das kleinste Stück: Turm A, Geschoss 0, Fassade 0. Seine zwei Kapseln bilden einen Ast, eine Liste aus zwei Elementen.

Kapsel 21 hat den Pfad {0;0;3;1}: Gebäude 0, Turm 0, das ist A, Geschoss 3, Fassade 1.

Der Ast ist eine Liste. Ein Index adressiert ein Element darin: {0;0;3;1}[1] ist Kapsel 21, die rechte Kapsel dieser Fassade.

Ein Panel im Baummodus listet jeden Ast mit seinem Pfad. Tree Statistics zählt 70 Äste mit je zwei Elementen.

Zuerst Flatten an einem Geschoss von Turm A: Seine drei Äste werden zu einer Liste aus sechs Elementen.

Flatten legt alle 140 Kapseln in eine Liste. Es geht nichts verloren, außer der Struktur.

Graft am selben Geschoss: Jede der sechs Kapseln bekommt einen eigenen Ast, sechs Äste mit je einem Element.

Graft macht das Gegenteil: Jede Kapsel bekommt einen eigenen Ast, und der Pfad wird um einen Index länger.

Simplify entfernt am selben Geschoss das gemeinsame Pfadpräfix {0;0;0}. Die Äste bleiben.

Simplify entfernt die führende 0, die alle Pfade teilen, weil es nur ein Gebäude gibt. Die Äste bleiben.

Path Mapper bindet die Buchstaben an einen Pfad. Kapsel 21: A ist 0, B ist 0, C ist 3, D ist 1. D, die Fassade, entfällt.

Auf alle Äste angewendet, fasst die Regel {A;B;C;D}(i) nach {A;B;C} die Fassaden zusammen: Jeder Ast ist ein Geschoss, 20 Äste.

Jetzt ist jeder Ast ein Geschoss: sechs Kapseln bei drei Fassaden, acht bei vier.

Das Abbilden auf {A;B} ergibt einen Ast je Turm: 54 Kapseln in Turm A, 86 in Turm B.

Flip Matrix tauscht Äste und Elemente: Alle linken Kapseln bilden eine Liste, alle rechten eine andere, zwei Äste mit je 70.

Eine Komponente verknüpft Daten Ast für Ast. Custom Preview nimmt Kapseln und eine Farbe, und die Pfade entscheiden, wer zu wem gehört.

Der klassische Fehler: Die kurze Farbliste wiederholt sich in jedem Ast. Die linken Kapseln werden türkis, die rechten orange, in beiden Türmen.

Der Param Viewer zeigt, warum: Kein Pfad passt, also wiederholt sich die kurze Farbliste in jedem Ast. Erst beide Pfade lesen.

Die Lösung: Kapseln auf {A;B} abbilden und die Farbliste mit Graft Tree verteilen. Nun tragen beide Bäume die Pfade {0;0} und {0;1}.

Mit passenden Pfaden erreicht die Farbe jedes Turms alle seine Kapseln: 54 türkise in Turm A, 86 orange in Turm B.

Zusammenfassung: Datenbaum, Ast, Pfad, Element, Index, Flatten, Graft, Simplify, Path Mapper, Flip Matrix. Zehn Begriffe, eine Adresse.
