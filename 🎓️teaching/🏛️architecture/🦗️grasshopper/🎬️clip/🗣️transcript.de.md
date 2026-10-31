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

Eine Werteliste bietet feste Auswahlmöglichkeiten: die Raumprogramm-Studien A bis D. Alle teilen das Bullauge von 1,30 Metern.

Ein Plugin, eine .gha-Datei, fügt dem Ribbon einen Reiter hinzu. Ladybug Tools untersucht die Sonne am Fenster: mehr Sonne, wärmere Tönung.

Nun die Daten. Ein Wert ist ein Element: der Höhenversatz null Meter für die erste Kapselebene von Turm B.

Eine Series mit Schrittweite 2,8 Meter liefert 9 Versätze in einem Ast, Indizes 0 bis 8: eine Kapsel je Ebene von Turm B.

Ändert man Anzahl von 9 auf 11, wächst die Liste. Turm A hat 11 Kapselebenen.

Viele Listen bilden einen Datenbaum. Jede belegte Fassadengruppe ist ein Ast mit angezeigter Adresse und einer variablen Anzahl von Kapseln.

Typischer Fehler: einen Datenbaum wie eine lange Liste lesen. Index eins gibt es in vielen Ästen, nicht in jedem.

Erst Pfad plus Index identifiziert genau eine Kapsel. Das Panel zeigt die tatsächlich gewählte Adresse und Index eins.

Die Begriffe des Canvas: Skript, Komponente, Parameter, Eingang, Ausgang, Schieberegler, Panel und Plugin. Englisch steht daneben.

Die Begriffe der Daten: Element, Liste, Datenbaum, Ast, Pfad und Index. Die englischen Wörter stehen daneben.

## Eine Kapsel, parametrisch

Video: `grasshopper_capsule`

Am Nakagin-Turm hängen die Kapseln an zwei Kernen, doch eine einzige Grasshopper-Definition baut jede davon.

Parametrisch heißt: Parameter speisen Abhängigkeiten, Abhängigkeiten bauen Geometrie. Ändert sich ein Wert, folgt alles.

Sechs Number Sliders steuern das physische Referenzgehäuse: 2,6 Meter Breite, 4,2 Meter Länge, Höhe, Eckenradius, Fensterdurchmesser und Versatz.

Alles beginnt auf einer Ebene: Ursprung plus X-, Y- und Z-Achse in Rhinos Rot, Grün und Blau. Das ist die Grundebene der Kapsel.

Center Box nimmt die Ebene und die Slider für Breite, Länge und Höhe. Die Box zentriert sich auf der Ebene, die Hälfte versinkt im Boden.

Plane Origin verwendet den physischen Rückseitendatum von minus 0,1 Metern, die halbe Gehäuselänge und die halbe Höhe. Jetzt steht die Box auf der Grundebene.

Ziehen Sie den Längen-Slider: Maßlinie, Box und jede Komponente dahinter folgen im selben Moment.

Fillet Edge rundet die Kanten mit dem Eckenradius-Slider: Die weiche Hülle der Kapsel entsteht.

Ein typischer Fehler: Ein Circle auf der Grundebene liegt am Boden, nicht an der Vorderseite. Eine Verbindung weiß nicht, wo Sie ihn meinen.

Die Lösung: Das Vorderseitendatum ist Rückseitendatum plus Gehäuselänge. Die Glasebene liegt im quellengestützten Rücksprung; der Schnittzylinder beginnt an der Stahlfläche.

Solid Difference zieht einen Zylinder dieses Kreises von der Hülle ab: Das runde Fenster entsteht an der Vorderseite.

Vier Punkte auf der Rückseite markieren die hochfesten Schrauben, mit denen die Kapsel am Kern hängt.

Jede Slider-Domain ist eine Entwurfsgrenze; hier sind die Grenzen Annahmen, zum Beispiel eine Transporthülle.

Schieben Sie die Breite über die Domain hinaus: Der Slider hält am oberen Ende an, die Hülle wird rot. Die Domain ist eine Entwurfsregel.

A bis D sind vorgeschlagene Raumprogramm-Studien und keine historischen Kapselfamilien. Sie teilen die nominale Kapselhülle.

Alle vier Studien behalten das nominale mittige Bullauge von 1,30 Metern. Slider-Änderungen erkunden Maße; es sind keine vermessenen Typen.

Am Kern braucht die Kapsel eine Zielebene an der Wand; die Grundebene muss dort landen.

Orient bewegt die Kapsel in einem Schritt von ihrer Grundebene A auf die Zielebene B.

Dreht man die Zielebene um die Kernachse, dreht die Kapsel mit, denn sie folgt der Ebene B.

Jeder Platz behält seine historische Gehäusefamilie. List Item wählt zwischen vier vorgeschlagenen Programmvarianten: Index 0 ergibt A, 1 B, 2 C, 3 D.

Acht Ebenen brauchen Indizes bis 7, doch es gibt nur vier Typen: Ohne Wrap warnt List Item ab Index 4.

Mit Wrap läuft der Index herum: 4 ergibt wieder Typ A, 5 Typ B, und jede Ebene bekommt eine Kapsel.

Rückblick: Parameter, Abhängigkeit, Geometrie, Domain, Ebene, Orient, List Item und Index.

## Von der Kapsel zum Turm

Video: `grasshopper_tower`

Der Nakagin Capsule Tower hat 140 Kapseln. Wie werden sie zu zwei Türmen?

Der Turm hat fünf Teile: Kern, Ebenen, Kapseln, Kopf und Sockel. Jeder bekommt eine eigene farbige Gruppe.

Am Anfang steht die quadratische Kernhülle. Ihre Höhe verbindet gewählte Kapselkronen mit dem Dachfreiraum dieses Turms.

Die Hülle legt den Schacht fest. Seine detaillierte Baugruppe enthält Decken, Treppen, Podeste und Aufzugtüren auf jedem Geschoss.

Eine Series beginnt am vorläufigen Sockeldach bei 7,72 m mit belegten 2,80 m Schritt. Verfasste Phasenversätze platzieren die Kapseln.

Verfasste Ursprünge und Achsen bestimmen die Tür-Port-Ebenen. Construct Plane baut jede ohne angenommene Drehung.

Jede Ebene trägt ihre Achsen: X läuft entlang der Fassade, Y zeigt nach außen, Z zeigt nach oben.

Jede verfasste Fassadengruppe hat indizierte Kandidaten-Ports. Die verfassten Geschossmasken wählen die belegten Positionen.

Orient bildet den lokalen Tür-Port auf den rekonstruierten Welt-Port ab. Das Ergebnis ist die Körper-Ebene der Kapsel, gerichtet angeordnet.

Ein Typmuster wählt für jede Ebene den Kapseltyp. List Item liest das Muster per Index; nur die Liste ändert sich.

Der Slider zeigt eine rekonstruierte Kapselebene nach der anderen. Die Summe zählt die belegten Positionen auf den gewählten Ebenen.

Die Verbindungen bleiben gleich. Geschossmasken, Kapselkronen und der Dachfreiraum jedes Turms bestimmen die resultierende Höhe.

Ohne die Geschossmasken enthält das regelmäßige Raster unbelegte Positionen. Das Muster entspricht nicht dem dokumentierten Gebäude.

Cull Index entfernt die unbelegten Positionen auf jeder Ebene. Das belegte Ergebnis sind 76 Kapseln in A und 64 in B.

Der Maschinenkopf ist ein eigenes Bauteil. Für A sind 11,36 Meter belegt; B nutzt vorläufig abgeleitete 10,83 Meter.

Der Kopf beginnt am Kerndach: 42,14 m in A, vorläufig 36,62 m in B; jeder Turm behält seinen Dachfreiraum.

Grundriss und Höhe bestimmen die Sockelhülle. Das detaillierte Modell ergänzt Decken, Stützen, Verglasung, Eingang und Brüstungen.

Die Hülle ist der erste Konstruktionsschritt. Benannte Baugruppen tragen Erschließung, Versorgung und Fassadendetails durchs Gebäude.

Dieselbe Definition liest die verfassten Geschossmasken beider Türme: 76 Kapseln in A, 64 in B.

Zusammen ergeben sie die historische Zahl von 140 Kapseln, aus der Definition gelesen und nicht eingetippt.

Der Metabolismus schlägt austauschbare Kapseln vor. Hier wechselt der Schalter das Raumprogramm von Typ B zu Typ C; das Bullauge bleibt.

Zusammenfassung: Kern, Ebenen, Kapseln, Kopf, Sockel. So sieht das Lehrmodell des Nakagin Capsule Tower aus.

## Casting: Wenn Typen sich treffen

Video: `grasshopper_casting`

Diese Kapsel soll nach Westen schauen, doch ihr Fenster zeigt nach Norden. Eine stille Umwandlung hat das bewirkt. Welche?

In diesem Lehrmodell des Nakagin Capsule Tower hat jeder Parameter einen Typ: Breite Zahl, Geschosse Ganzzahl, Platzierung Ebene.

Eine Verbindung nimmt alles an, was man zieht. Passen die Typen nicht, versucht Grasshopper, die Daten umzuwandeln.

Ein Panel enthält den Text 2.5. Der Zahleneingang liest ihn als Zahl 2,5, und die nominale Studienbreite wird 2,5 Meter. Das physische Referenzgehäuse ist 2,6 Meter breit.

Der Regler zeigt 8,6, doch Geschosse verlangt eine Ganzzahl. Grasshopper rundet, hier auf 9; in Rhino 8 prüfen.

Grasshopper rundet; genau bei .5 geht es zur geraden Zahl: 2,5 gibt 2, 3,5 gibt 4. In Rhino 8 prüfen.

Wer die Rundung selbst bestimmen will, setzt eine Komponente wie Floor vor den Ganzzahl-Eingang: Das oberste Geschoss verschwindet.

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

Eine belegte Fassadengruppe bildet einen Ast. Ihre Länge folgt dem rekonstruierten Geschossgrundriss.

Die gewählte Kapsel behält ihren Pfad: Gebäude, Turm, Kapselebene und Fassade. Ihre tatsächliche Adresse steht im Bild.

Ein Index wählt eine Kapsel innerhalb ihres Astes. Pfad und Index identifizieren gemeinsam die gewählte Kapsel.

Ein Panel listet die belegten Fassadenäste. Tree Statistics zeigt die unterschiedlichen Astlängen.

Flatten an einem belegten Geschoss verbindet seine Fassadenäste zu einer Liste und erhält jede Kapsel.

Flatten legt alle 140 Kapseln in eine Liste. Es geht nichts verloren, außer der Struktur.

Graft gibt jeder Kapsel dieses Geschosses einen eigenen Ast mit genau einem Element.

Graft macht das Gegenteil: Jede Kapsel bekommt einen eigenen Ast, und der Pfad wird um einen Index länger.

Simplify entfernt das gemeinsame Präfix aus Gebäude, Turm und Geschoss. Die belegten Fassadenäste bleiben.

Simplify entfernt die führende 0, die alle Pfade teilen, weil es nur ein Gebäude gibt. Die Äste bleiben.

Path Mapper bindet Buchstaben an die angezeigte Adresse. Der Fassadenbuchstabe entfällt; die übrige Adresse wählt ein Geschoss.

Auf alle Äste angewendet, fasst die Regel {A;B;C;D}(i) nach {A;B;C} die Fassaden zusammen: Jeder Ast ist ein Geschoss, 20 Äste.

Jeder Ast enthält nun ein belegtes Geschoss. Die belegten Anzahlen variieren: zwei, sieben oder acht Kapseln.

Das Abbilden auf {A;B} ergibt einen Ast je Turm: 76 Kapseln in A und 64 in B.

Flip Matrix tauscht Äste und Positionen. Kurze Äste erhalten Null-Platzhalter: leere Adressen, keine Kapseln.

Eine Komponente verknüpft Daten Ast für Ast. Custom Preview nimmt Kapseln und eine Farbe, und die Pfade entscheiden, wer zu wem gehört.

Der klassische Fehler: Die kurze Farbliste wiederholt sich in jedem Ast. Abwechselnde Farben folgen Elementindizes statt Turmadressen.

Der Param Viewer zeigt, warum: Kein Pfad passt, also wiederholt sich die kurze Farbliste in jedem Ast. Erst beide Pfade lesen.

Die Lösung: Kapseln auf {A;B} abbilden und die Farbliste mit Graft Tree verteilen. Nun tragen beide Bäume die Pfade {0;0} und {0;1}.

Passende Pfade verteilen die Turmfarben auf die belegten Summen: 76 türkise Kapseln in A und 64 orange in B.

Zusammenfassung: Datenbaum, Ast, Pfad, Element, Index, Flatten, Graft, Simplify, Path Mapper, Flip Matrix. Zehn Begriffe, eine Adresse.
