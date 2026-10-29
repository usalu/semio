# BIM-Videos

Vollständige Texte der sechs BIM-Videos. Das Modell der Villa Savoye zielt auf den ursprünglichen Fertigstellungszustand 1930–1931. Quellenbelegte, abgeleitete und angenommene Maße sind unterschieden; hypothetische Lehränderungen werden ausdrücklich benannt. Ein Absatz pro gesprochenem Schritt, in Sprechreihenfolge.

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

Stimmt die Summe? Die Felder ergeben die Rasterbreite; jedes Gebäudemaß behält seinen eigenen Beleg oder seine Ableitung.

Das Datenmodell ergänzt die Bedeutung: Eigenschaft, Einheit, Raster, Beleg und den rekonstruierten ursprünglichen Zustand von 1930–1931.

Ein Wert trägt seine Herkunft. Das erste Raster war weiter und wurde aus Kostengründen verkleinert.

Information dient einer Frage: Erreicht ein angenommenes Auto den Eingang ohne Kontakt zu den modellierten Hindernissen?

Der Abstand gilt von Achse zu Achse; das Auto braucht den freien Raum zwischen den Stützen.

Die lichte Weite berücksichtigt die tatsächlichen Stützenkonturen im Achsabstand. Bei Kreisprofilen ergibt sich der bekannte Durchmesserabzug.

Bei Geradeausfahrt müssen Auto und ein Sicherheitsabstand je Seite in die verfügbare Breite passen.

Erstes und verkleinertes Raster werden als ausdrückliche Abstandsszenarien verglichen.

Wie sicher ist die Antwort? Die Skala zeigt die Schwelle dieses ausdrücklichen Geradeausfahrt-Szenarios.

Eine dickere Kreisstütze oder ein breiteres Auto verschiebt die Schwelle; die ganze Route braucht weiterhin ihre eigene geometrische Prüfung.

Die angenommene Anfahrt ist ein Lehrszenario. Historische Beschreibungen nennen Platz für drei Autos; drei Fahrzeuge in voller Größe bleiben geometrisch ungeprüft.

Die bewegte Rechteckhülle wird gegen tatsächliche Bauteile in Fahrzeughöhe geprüft, einschließlich Haustechnik und Hebeausleger; der kleinste Abstand entscheidet.

Die transluzente Eingangsumfassung und geschlossenen Holztürblätter folgen dem quellenbezogenen Plan. Die angenommene Anfahrt ist eine eigene Lehrroute.

Eine Stütze trägt weitere Eigenschaften: Material, ob sie Lasten trägt und ob sie außen liegt.

Beziehungen vervollständigen die Bedeutung: Geschoss, Typ, die getragene Geschossdecke und die Rasterposition.

BIM hält die Daten in einem strukturierten Modell: Menschen lesen die Zeichnung, Anwendungen die Daten.

Beide beschreiben dieselbe Stütze: Die GlobalId verknüpft die gezeichnete Stütze mit ihren STEP-Zeilen.

## Vom Schema zum Objekt

Video: `bim_objects`

Ein Schema definiert Begriffe und Regeln für Objekte und Beziehungen. Für Gebäude ist das Schema IFC 4.3.

Eine Klasse beschreibt eine Art von Dingen: IfcColumn, IfcSlab, IfcWall und IfcWindow sind Arten von Bauteilen.

Eine Instanz ist ein bestimmtes Objekt: diese Stütze der Villa Savoye mit eigener GlobalId.

Ein Typ hält, was seine Vorkommen teilen. Diese Pilotis haben einen Typ; Hallen- und Gartenstützen haben andere Typen.

Ein Property Set ergänzt benannte Werte am Typ: LoadBearing sagt, dass jede Stütze ihren Anteil der Decke trägt.

Ein Objekt kann mehrere Darstellungen haben: Achse, Grundfläche und Körper.

Dieser gewählte Körper ist eine Kreisextrusion: Radius und Höhe werden zu Maßen. Andere Stützenprofile behalten ihre nativen Bohrungen.

Eine Methode ist Verhalten: Die Anwendung berechnet Querschnitt und Volumen aus diesen Werten.

Die Methode summiert die geometrischen Querschnittshüllen des gewählten Typs; ein vermessener Betonkern braucht eigene Belege.

Ändert man den Durchmesser im Typ, folgen alle Vorkommen: Volumen und Summe ändern sich, die GlobalId bleibt.

Vererbung: Jede Klasse reicht Attribute an IfcColumn weiter, die nur PredefinedType ergänzt.

Hier haben sechs geerbte Attribute einen Wert, darunter GlobalId, Name, Rasterplatz und Tag; drei bleiben leer.

Bauteile liegen in einem Geschoss; das Geschoss gehört zum Gebäude, das Gebäude zum Grundstück.

Komposition bedeutet Teile: Die Wand hat eine Öffnung, die IfcRelVoidsElement aus ihr ausschneidet.

Ein Fenster füllt diese Öffnung über IfcRelFillsElement; Rahmen und Glas bilden getrennte physische Teile.

Der Wandaufbau unterscheidet feste Schalen und einen Lufthohlraum; ihre Breiten ergeben die Hüllstärke.

Unterscheide Klasse und Komposition: Ein Fenster füllt eine Öffnung, während die Öffnung am hängenden Garten ungefüllt bleibt.

IFC tauscht deklarative Daten und Beziehungen aus: Klasse, GlobalId, Radius, Höhe und Eigenschaften reisen als Text.

Die Volumenmethode reist nicht mit: Die empfangende Anwendung führt ihre eigene Methode aus und erhält denselben Wert.

## Ein Objekt, viele Darstellungen

Video: `bim_representations`

Lehrmodell der Villa Savoye: ein Objekt, die Nordwestwand des Obergeschosses mit ihrem Bandfenster.

Der Grundriss schneidet die Wand auf Fensterhöhe: Wand als Poché, Bandfenster als dünne Glaslinien.

Der Schnitt geht durch Geschossdecke, Pilotis und Fenster: Brüstungs- und Sturzhöhe werden zu Maßlinien.

Die Ansicht zeigt die Nordwestfassade und die asymmetrische Solariumswand mit Landschaftsöffnung und geneigter Abdeckung.

Die 3D-Ansicht zeigt feste Wandschalen, getrennte Tragglieder und eine mit Rahmen und Glas gefüllte Öffnung.

Im Modell ist die Wand als Axis, FootPrint und Body gespeichert; Grundriss, Schnitt und Ansicht sind abgeleitet.

Drei Detailstufen einer Wand: Axis als Linie, FootPrint als Umriss, Body als Volumen mit Schichten.

Im Maßstab 1:200 genügt der Umriss: Auf dem Papier ist die Wand eine dünne Linie, ihre Schichten verschwinden.

Im Maßstab 1:20 braucht der Plan den Body: Die Schichten der Wand werden Millimeter für Millimeter sichtbar.

Wandert die Schnittebene, wechselt der Grundriss von Vollwand zu Fensterband und zurück: Er ist abgeleitet.

Dieselbe Geometrie liefert die Flächen: Bruttowandfläche, Öffnung, Nettowandfläche und Glasanteil.

Die Brüstung sinkt von 0,89 auf 0,70 m; der Sturz bleibt bei 2,04 m. Die Öffnung wächst von 1,15 auf 1,34 m.

Die Mengenermittlung unterscheidet Rohbauöffnungen und tatsächliches Glas; Nettowandfläche und Glasanteil folgen der Brüstung.

Das Wandvolumen zählt feste Schalen ohne Lufthohlraum; die tiefere Brüstung entfernt zusätzliches festes Material.

Kopien würden zurückbleiben: Jede Kopie müsste von Hand abgeglichen werden, Ansicht für Ansicht.

Eine nur im Grundriss gezeichnete Linie steht nicht im Modell: Die anderen Ansichten kennen sie nicht.

Bei der Neuerzeugung geht die gezeichnete Linie verloren: Eine Ansicht zeigt das Modell und speichert nichts.

Eine Darstellung zeigt das Objekt; eine Planänderung aktualisiert es, und seine GlobalId bleibt dieselbe.

## Modelle teilen ein Schema

Video: `bim_models`

Verschiedene Fachrichtungen ordnen die benötigte Information zum rekonstruierten ursprünglichen Haus von 1930–1931.

Das Architekturmodell enthält Räume, Eingangskonstruktion, Dachtreppenhaus, dessen Vordach und das offene Metallgeländer an der Rampe.

Auch Räume sind Objekte: die modellierten Räume mit ihrer Fläche gehören zu ihrem Geschoss.

Der Fassadenaufbau unterscheidet Außenputz, zwei Ziegelschalen, einen Lufthohlraum und Innenputz.

Das Tragwerksmodell enthält Pilotis, Decken, Rampe, Treppen sowie Zusatzstürze, hohle U-Träger und Konsolen; mehrere Glieder teilt es mit der Architektur.

Ursprüngliche Haustechnik umfasst Fallrohre in Stützen 2 und 4, Heizungs- und Wasserstränge, Abwasserstrang und Rauchzug. Detailhüllen und unvollständige Leitungswege behalten ihre Unsicherheiten.

Drei Modelle teilen ein Schema. Die Zahlen zählen IFC-Objekte; eine Treppenbaugruppe kann viele physische Teile enthalten.

Koordination legt Modelle auf einen Ursprung. Im Lehrmodell markiert eine GlobalId die gemeinsame Decke.

Die gemeinsame Decke behält eine GlobalId: Jedes Modell liest daraus, was es braucht.

Im Grundriss können Fachzuordnungen überlappen: Der Hebeausleger gehört zu Architektur, Tragwerk und Haustechnik.

Kollisionsprüfung im Lehrbeispiel: ein Fallrohr liegt zu nah an einer Pilotistütze, was kein Einzelmodell zeigt.

Für einen Anwendungsfall wird das Tragwerksmodell in ein Tragwerksanalysemodell übersetzt: die IFC-Analysedomäne.

Die Obergeschossdecke wird zu zugeschnittenen Flächenelementen; gewählte Deckenstützen werden zu Stabelementen.

Die Decke wird an der Mittelfläche geschnitten: Der Stab reicht dorthin, die Dicke wird zum Attribut.

Jeder gewählte analytische Stützenanfang erhält eine angenommene Einspannung, einschließlich abgefangener Stützen.

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

Die Decke wird zu zugeschnittenen Schalenelementen; gewählte Deckenstützen werden zu Stäben. Jedes Analyseobjekt verweist auf seine Quell-GlobalId.

Die native Decke behält Rampen-, Treppen- und Haustechniköffnungen. Kellerluken haben getrennte Deckel; die Geschossfläche schließt tatsächliche Aussparungen aus.

Die Analyse braucht Annahmen: Eigengewicht aus der Dicke, Ausbaulasten, Nutzlast und die Auflagerbedingungen.

Die Last ist ein Stapel: Eigengewicht der Decke, Ausbaulasten und Nutzlast ergeben eine Flächenlast.

Jede gewählte Stütze erhält den Deckenbereich bis zu den Mittellinien zu den Nachbarn, zugeschnitten an Decke und Öffnungen.

Jede gewählte Stütze erhält ihre Lasteinzugsfläche. Das sind regionale Decken-Näherungen, keine Finite-Elemente-Steifigkeitslösung.

Das Werkzeug sendet Auflagerkräfte zurück. Sie kehren als verknüpftes Ergebnis zu 19 zurück, Geometrie nie.

Bandfenster, Ausfachungswände und Putzfarbe gehen nicht über. Zurückübersetzen stellt sie nicht wieder her.

IFC hat Entitäten für die Tragwerksanalyse, Analysewerkzeuge nutzen oft eigene Schemata. Das ist ein Beispiel.

Bevor man dem Austausch vertraut, prüft man, was der Empfänger braucht, zum Beispiel mit einer IDS-Anforderung.

## Identität und Konsistenz

Video: `bim_consistency`

Eine GlobalId benennt diese Dachplatte in jedem Austausch; Kürzel SL-R und Nummer #101 gelten nur lokal.

Architektur und Tragwerk führen diese Platte je im eigenen Modell und in eigener Revision, unter einer GlobalId.

Der Datensatz der Platte hält Klasse, Eigenschaften, Urheber und Revision; keines davon ist die Identität.

Berichte sagen, dass jeden Herbst Wasser durchs Flachdach kam; unser Lehrszenario beginnt bei diesem Leck.

Die Quelle unterscheidet ursprünglichen rekonstruierten Zustand und spätere Schadensbelege. Der gezeigte Wasserweg ist schematisch, ohne erfundene Membranschicht.

In einem ausdrücklichen Reparaturszenario erhöht ein Entwurfsereignis die Bezugsreserve des Belags von 0,09 auf 0,14 m; die Platte behält ihre GlobalId.

Dicke und Revision ändern sich; GlobalId, Klasse und Platz in der Villa bleiben, wie sie waren.

Zusätzlicher Belag erhöht die Flächenlast; Schichtdichten und Dicken bestimmen den angezeigten Wert.

Jede Dachstütze trägt den nächstgelegenen Teil der Decke ohne Öffnungen. Die unregelmäßigen Zellen haben verschiedene Flächen.

Die Last jeder Stütze folgt aus ihrem Anteil an jeder Dachzone und deren Flächenlast.

Die Belagsänderung betrifft jede Stütze entsprechend ihrem Anteil an der Dachzone.

LK-ROOF bindet die Analyse an die Platte, hält aber noch die frühere Revision mit deren Ergebnis.

Eine Prüfung vergleicht verknüpfte Revision und gespeicherte Werte mit dem aktuellen Modell und abgeleiteten Ergebnissen.

Eine Anforderungsprüfung (IDS) besteht, weil die Angabe existiert; nur die Verknüpfungsprüfung findet das alte Ergebnis.

Der Befund geht als BCF-Thema an den Tragwerksplaner und verweist über die GlobalId auf die Platte.

Die neue Übersetzung erzeugt Flächenelemente und Stützen aus dem aktuellen Lehrmodell; eine Person prüft die Annahmen.

Mit aktueller Verknüpfung zeigen Analyse und Modell gleiche Auflagerkräfte; die Prüfung findet kein Problem.

Zwei Personen ändern Revision 1 gleichzeitig: beide speichern Revision 2 unter derselben GlobalId, mit anderem Belag.

Die GlobalId kann konkurrierende Belagsänderungen nicht verschmelzen; eine Person entscheidet, welche Änderung bleibt.

Ein Schema gibt Daten Bedeutung, die GlobalId benennt das Objekt, die Revision datiert es, die Verknüpfung bindet die Analyse.
