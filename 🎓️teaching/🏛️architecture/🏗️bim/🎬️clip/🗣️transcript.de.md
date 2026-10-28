# BIM-Videos

Vollständige Texte der sechs BIM-Videos. Alle Werte stammen aus einem gerundeten Lehrmodell der Villa Savoye in Poissy (Le Corbusier und Pierre Jeanneret, 1928–1931); angenommene Werte sind in den Videos als solche gekennzeichnet. Ein Absatz pro gesprochenem Schritt, in Sprechreihenfolge.

## Inhalt

1. [Aus Daten wird Information](#aus-daten-wird-information)
2. [Vom Schema zum Objekt](#vom-schema-zum-objekt)
3. [Ein Objekt, viele Darstellungen](#ein-objekt-viele-darstellungen)
4. [Modelle teilen ein Schema](#modelle-teilen-ein-schema)
5. [Schnittstellen und Übersetzung](#schnittstellen-und-übersetzung)
6. [Identität und Konsistenz](#identität-und-konsistenz)

## Aus Daten wird Information

Video: `bim_information`

Diese Zahl ist ein Datenwert. Allein sagt sie nicht, was gemessen wurde.

Meter, Kilogramm, Grad: ohne Einheit könnte es alles sein.

Kontext: Es ist der Achsabstand zweier Pilotis im Erdgeschoss der Villa Savoye.

Das Modell benennt den Ort: Grundstück, Gebäude, Geschoss und die beiden Rasterachsen.

Stimmt die Summe? Die Felder ergeben die Rasterbreite; der Rest bis zur Gebäudekante bleibt offen.

Das Datenmodell ergänzt die Bedeutung: Eigenschaft, Einheit, Raster und ob der Wert belegt oder angenommen ist.

Ein Wert trägt seine Herkunft. Das erste Raster war weiter und wurde aus Kostengründen verkleinert.

Information dient einer Frage: Fährt ein Auto (Maße angenommen) zwischen den Pilotis durch und wendet unter dem Haus?

Der Abstand gilt von Achse zu Achse; das Auto braucht den freien Raum zwischen den Stützen.

Die lichte Weite ist der Abstand abzüglich des Pilotidurchmessers. Im Lehrmodell hat das Auto so viel Raum.

Bei Geradeausfahrt müssen Auto und ein Sicherheitsabstand je Seite hineinpassen. Mit diesen Annahmen tun sie das.

Ob erstes oder verkleinertes Raster: Das Auto passt.

Wie sicher ist die Antwort? Das Urteil kippt erst unter einer Schwelle, weit unter diesem Abstand.

Eine dickere Stütze oder ein breiteres Auto verschiebt die Schwelle, ändert die Antwort aber nicht.

Das Auto fährt durch das Feld und wendet unter dem Haus.

In der Kurve unterschreiten zwei Pilotis den Abstand der Geradeausfahrt; das Auto kommt noch frei vorbei, aber knapp.

Die gebogene Glaswand folgt dem Wendekreis des Autos: Ihre Kurve ist aus dem Auto berechnet.

Eine Stütze trägt weitere Eigenschaften: Material, ob sie Lasten trägt und ob sie außen liegt.

Beziehungen vervollständigen die Bedeutung: Geschoss, Typ, die getragene Geschossdecke und die Rasterposition.

BIM hält die Daten in einem strukturierten Modell: Menschen lesen die Zeichnung, Anwendungen die Daten.

Beide beschreiben dieselbe Stütze: Die GlobalId verknüpft die gezeichnete Stütze mit ihren STEP-Zeilen.

## Vom Schema zum Objekt

Video: `bim_objects`

Ein Schema definiert Begriffe und Regeln für Objekte und Beziehungen. Für Gebäude ist das Schema IFC 4.3.

Eine Klasse beschreibt eine Art von Dingen: IfcColumn, IfcSlab, IfcWall und IfcWindow sind Arten von Bauteilen.

Eine Instanz ist ein bestimmtes Objekt: diese Stütze der Villa Savoye mit eigener GlobalId.

Ein Typ hält, was alle Pilotis gemeinsam haben; jede platzierte Stütze ist ein Vorkommen davon.

Ein Property Set ergänzt benannte Werte am Typ: LoadBearing sagt, dass jede Stütze ihren Anteil der Decke trägt.

Ein Objekt kann mehrere Darstellungen haben: Achse, Grundfläche und Körper.

Der Körper ist ein entlang der Achse extrudierter Kreis: Radius und Höhe sind Werte, gezeichnet als Maßlinien.

Eine Methode ist Verhalten: Die Anwendung berechnet Querschnitt und Volumen aus diesen Werten.

Dieselbe Methode wird für jedes Vorkommen ausgeführt und summiert den Beton aller Pilotis.

Ändert man den Durchmesser im Typ, folgen alle Vorkommen: Volumen und Summe ändern sich, die GlobalId bleibt.

Vererbung: Jede Klasse reicht Attribute an IfcColumn weiter, die nur PredefinedType ergänzt.

Hier haben sechs geerbte Attribute einen Wert, darunter GlobalId, Name, Rasterplatz und Tag; drei bleiben leer.

Bauteile liegen in einem Geschoss; das Geschoss gehört zum Gebäude, das Gebäude zum Grundstück.

Komposition bedeutet Teile: Die Wand hat eine Öffnung, die IfcRelVoidsElement aus ihr ausschneidet.

Ein Fenster füllt diese Öffnung über IfcRelFillsElement; das Glas nimmt den Platz der Öffnung ein.

Die Wand hat auch ein Material: Ein Schichtenaufbau aus Außenputz, Ziegel und Innenputz ergibt ihre Dicke.

Unterscheide »ist ein« und »hat ein«: Die Stütze ist ein Bauteil; das Fenster ist keine Wand, es füllt eine Öffnung.

IFC tauscht deklarative Daten und Beziehungen aus: Klasse, GlobalId, Radius, Höhe und Eigenschaften reisen als Text.

Die Volumenmethode reist nicht mit: Die empfangende Anwendung führt ihre eigene Methode aus und erhält denselben Wert.

## Ein Objekt, viele Darstellungen

Video: `bim_representations`

Lehrmodell der Villa Savoye: ein Objekt, die Südwand des Obergeschosses mit ihrem Bandfenster.

Der Grundriss schneidet die Wand auf Fensterhöhe: Wand als Poché, Bandfenster als dünne Glaslinien.

Der Schnitt geht durch Geschossdecke, Pilotis und Fenster: Brüstungs- und Sturzhöhe werden zu Maßlinien.

Die Ansicht zeigt die ganze Südfassade: Bandfenster, dunkelgrünes Erdgeschoss, Pilotis, Solariumswände.

Die 3D-Ansicht zeigt den Körper: eine massive Wand mit einer Öffnung, die das Fenster füllt.

Im Modell ist die Wand als Axis, FootPrint und Body gespeichert; Grundriss, Schnitt und Ansicht sind abgeleitet.

Drei Detailstufen einer Wand: Axis als Linie, FootPrint als Umriss, Body als Volumen mit Schichten.

Im Maßstab 1:200 genügt der Umriss: Auf dem Papier ist die Wand eine dünne Linie, ihre Schichten verschwinden.

Im Maßstab 1:20 braucht der Plan den Body: Die Schichten der Wand werden Millimeter für Millimeter sichtbar.

Wandert die Schnittebene, wechselt der Grundriss von Vollwand zu Fensterband und zurück: Er ist abgeleitet.

Dieselbe Geometrie liefert die Flächen: Bruttowandfläche, Öffnung, Nettowandfläche und Glasanteil.

Eine Planänderung senkt die Brüstung. Das Bandfenster wird höher; alle Ansichten und Zahlen werden neu erzeugt.

Die Mengenermittlung liest den Body: Nettowandfläche, Fensterfläche und Glasanteil folgen der Brüstung live.

Auch das Wandvolumen stammt aus dem Body: Die Aussparung wird abgezogen, die tiefere Brüstung verkleinert es.

Kopien würden zurückbleiben: Jede Kopie müsste von Hand abgeglichen werden, Ansicht für Ansicht.

Eine nur im Grundriss gezeichnete Linie steht nicht im Modell: Die anderen Ansichten kennen sie nicht.

Bei der Neuerzeugung geht die gezeichnete Linie verloren: Eine Ansicht zeigt das Modell und speichert nichts.

Eine Darstellung zeigt das Objekt; eine Planänderung aktualisiert es, und seine GlobalId bleibt dieselbe.

## Modelle teilen ein Schema

Video: `bim_models`

Verschiedene Fachrichtungen ordnen die Bauwerksinformation, die sie brauchen.

Das Architekturmodell enthält Wände, Öffnungen, Fenster, Rampe, gebogene Glaswand, Decken und Räume.

Auch Räume sind Objekte: sechs Räume mit ihrer Fläche gehören zu ihrem Geschoss.

Der Wandaufbau gehört zum Modell: Außenputz, Ziegelausfachung und Innenputz in drei Schichten.

Das Tragwerksmodell enthält Pilotis, Decken und Rampe. Putz, Fenster und Räume lässt es weg.

Das Haustechnikmodell enthält Kessel, Heizkörper, Dachabläufe und Fallrohre: dasselbe Haus, andere Objekte.

Drei Modelle, ein Schema: gleiche Begriffe und Regeln, aber anderer Inhalt.

Koordination legt Modelle auf einen Ursprung. Im Lehrmodell markiert eine GlobalId die gemeinsame Decke.

Die gemeinsame Decke behält eine GlobalId: Jedes Modell liest daraus, was es braucht.

Im Grundriss gehört jede Linie zu einem Modell: Wände und Glas, Pilotis, Rohre.

Kollisionsprüfung im Lehrbeispiel: ein Fallrohr liegt zu nah an einer Pilotistütze, was kein Einzelmodell zeigt.

Für einen Anwendungsfall wird das Tragwerksmodell in ein Tragwerksanalysemodell übersetzt: die IFC-Analysedomäne.

Die Obergeschossdecke wird zu Flächenelementen auf dem Raster, jede Pilotistütze zu einem Stabelement.

Die Decke wird an der Mittelfläche geschnitten: Der Stab reicht dorthin, die Dicke wird zum Attribut.

Annahmen ergänzen, was nie gezeichnet wurde: Jeder Pilotifuß wird zum eingespannten Lagerknoten.

Auch die Last ist angenommen: Eigengewicht, Ausbau und Nutzlast ergeben die Flächenlast.

Fensterbänder, Ausfachungen, Putz, die gebogene Glaswand und mehr fehlen in der Analyse.

Selbst erhaltene Pilotis verlieren Eigenschaften: Feuerwiderstand und Außenbauteil-Kennzeichen entfallen.

Jedes Element behält seine Quell-GlobalId, so finden Ergebnisse zurück; umkehrbar ist die Übersetzung nicht.

## Schnittstellen und Übersetzung

Video: `bim_exchange`

Eine Schnittstelle definiert den Austausch. IFC ist ein Standardschema (ISO 16739-1), das beide Seiten kennen.

Semantischer Austausch sendet eine IfcColumn am Ort mit stabiler GlobalId. Instanznummern (#n) gelten nur lokal.

Seine Eigenschaften reisen mit: Typ COLUMN, Flag LoadBearing, Material, Kreisprofil und Extrusionstiefe.

Ein Dreiecksnetz behält nur Punkte und Flächen. Die Form bleibt grob, Klasse, Identität und Eigenschaften fehlen.

Rundreise: IFC zum Dreiecksnetz und zurück. Die Form kommt zurück, Klasse, GlobalId und LoadBearing nicht.

Für die statische Prüfung der Geschossdecke wählen wir ein Analyseschema und idealisieren sie als Mittelfläche.

Die Decke wird zu Schalenelementen im Raster, Stützen zu Stäben. Jedes Objekt verweist auf seine Quell-GlobalId.

Die Decke gehört der Villa: Rampenöffnung ausgespart, der Salon ruht darauf, daraus folgt die Nettofläche.

Die Analyse braucht Annahmen: Eigengewicht aus der Dicke, Ausbaulasten, Nutzlast und die Auflagerbedingungen.

Die Last ist ein Stapel: Eigengewicht der Decke, Ausbaulasten und Nutzlast ergeben eine Flächenlast.

Jeder Stütze gehört die Decke bis zu den Mittellinien zu den Nachbarn. Neben der Rampenöffnung hat B3 eine Kerbe.

Jeder Stütze wird ihre Lasteinzugsfläche zugeordnet. Das ist eine grobe Schätzung, kein FE-Ergebnis.

Das Werkzeug sendet Auflagerkräfte zurück. Sie kehren als verknüpftes Ergebnis zu B3 zurück, Geometrie nie.

Bandfenster, Ausfachungswände und Putzfarbe gehen nicht über. Zurückübersetzen stellt sie nicht wieder her.

IFC hat Entitäten für die Tragwerksanalyse, Analysewerkzeuge nutzen oft eigene Schemata. Das ist ein Beispiel.

Bevor man dem Austausch vertraut, prüft man, was der Empfänger braucht, zum Beispiel mit einer IDS-Anforderung.

## Identität und Konsistenz

Video: `bim_consistency`

Eine GlobalId benennt diese Dachplatte in jedem Austausch; Kürzel SL-R und Nummer #101 gelten nur lokal.

Architektur und Tragwerk führen diese Platte je im eigenen Modell und in eigener Revision, unter einer GlobalId.

Der Datensatz der Platte hält Klasse, Eigenschaften, Urheber und Revision; keines davon ist die Identität.

Berichte sagen, dass jeden Herbst Wasser durchs Flachdach kam; unser Lehrszenario beginnt bei diesem Leck.

Wasser passiert Pflanzsubstrat und Estrich; wo die Abdichtung eine Lücke hat, dringt es in die Decke ein und fleckt sie.

Nach der Reparatur verdickt ein Entwurfsereignis das Pflanzsubstrat von 0,10 auf 0,30 m; die Platte behält ihre GlobalId.

Dicke und Revision ändern sich; GlobalId, Klasse und Platz in der Villa bleiben, wie sie waren.

Pflanzsubstrat wiegt 18 kN/m³: 0,20 m mehr bringen 3,6 kN/m², die Last der Dachbepflanzung wird 12,92 kN/m².

Jeder Piloti trägt das Dach um sich: A3 trägt 4,75 m × 2,375 m = 11,28 m², C3 nur 2,38 m².

Die Last auf A3 folgt: 11,28 m² × 12,92 kN/m² ≈ 146 kN, nicht 105 kN.

Jeder Piloti ändert sich um 3,6 kN/m² mal der Fläche seiner Dachbepflanzung: D3 am meisten, C4 unter der Terrasse nichts.

LK-ROOF bindet die Analyse an die Platte; sie hält noch Revision 1 mit 105 kN statt 146 kN.

Eine Prüfung vergleicht Verknüpfung und Modell: Revision 1 gegen 2, 9,32 gegen 12,92 kN/m², 105 gegen 146 kN.

Eine Anforderungsprüfung (IDS) besteht, weil die Angabe existiert; nur die Verknüpfungsprüfung findet das alte Ergebnis.

Der Befund geht als BCF-Thema an den Tragwerksplaner und verweist über die GlobalId auf die Platte.

Die neue Übersetzung der Platte ergibt 24 Flächenelemente und 25 Stäbe dieses Lehrmodells; eine Person prüft die Annahmen.

Mit der Verknüpfung auf Revision 2 zeigt auch die Analyse 146 kN, und die Prüfung findet kein Problem.

Zwei Personen ändern Revision 1 gleichzeitig: beide speichern Revision 2 unter derselben GlobalId, mit anderem Pflanzsubstrat.

Die GlobalId kann 12,92 und 11,12 kN/m² nicht verschmelzen; eine Person entscheidet, welche Änderung bleibt.

Ein Schema gibt Daten Bedeutung, die GlobalId benennt das Objekt, die Revision datiert es, die Verknüpfung bindet die Analyse.
