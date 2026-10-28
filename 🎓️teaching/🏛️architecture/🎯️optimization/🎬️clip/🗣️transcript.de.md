# Optimierungsvideos

Vollständige Texte der sechs Optimierungsvideos. Alle Werte stammen aus einem vereinfachten Jahresmodell eines Seminarraums, nicht aus Messungen. Ein Absatz pro Akt, in Sprechreihenfolge.

## Inhalt

1. [Minimierung und Maximierung](#minimierung-und-maximierung)
2. [Den Entwurfsraum abtasten](#den-entwurfsraum-abtasten)
3. [Statistische Versuchsplanung](#statistische-versuchsplanung)
4. [Mehrere Kriterien und ein normierter Index](#mehrere-kriterien-und-ein-normierter-index)
5. [Pareto-Optimalität](#pareto-optimalität)
6. [Algorithmen und Evolutionäre Suche](#algorithmen-und-evolutionäre-suche)

## Minimierung und Maximierung

Video: `optimization_extrema`

Dieser Seminarraum hat eine verglaste Fassade. In welche Himmelsrichtung soll sie zeigen? Die Ausrichtung der Fassade θ ist unsere Entwurfsvariable: Ost 90°, Süd 180°, West 270°, Nord 360°.

Alles andere bleibt fest: Fensteranteil, Vordach und Dämmung. Nur die Ausrichtung θ dreht sich.

Die Zielgröße ist der Strombedarf aus Heizstrom, Kühlstrom und Beleuchtung, in kWh/a. Wir minimieren ihn, denn kleiner ist besser. Die Werte stammen aus einem stark vereinfachten Jahresmodell.

Wir drehen die Fassade von Osten, 90°, über alle Ausrichtungen und simulieren den Strombedarf in kWh/a.

Beim Drehen nach Süden heizt die tiefe Wintersonne den Raum gratis: Der Heizstrom sinkt. Die hohe Sommersonne trifft das Südglas steil: Der Kühlstrom sinkt auf den bisher kleinsten Wert. Nach Westen flutet die tiefe Abendsonne den Raum: Der Kühlstrom bleibt hoch, die Summe gipfelt bei 282°. Nach Norden erreicht nur diffuses Licht das Glas: Kühlstrom minimal, Heiz- und Beleuchtungsstrom steigen. Bei 450° zeigt die Fassade wieder nach Ost, die Kurve schließt sich.

Wir starten mit der Fassade nach Nordwest, 315°, und drehen stets dorthin, wo der Strombedarf sinkt. Bei 360°, Nord, steigt der Kühlstrom schneller als der Heizstrom sinkt: Jeder Nachbar ist schlechter, ein lokales Minimum.

Doch Süd, 180°, liegt noch tiefer: Es ist das globale Minimum. Von Nord nach Süd spart die gratis Wintersonne weit mehr Heizstrom, als die Sommersonne Kühlstrom addiert. Der Halt im Nordtal verschwendet jedes Jahr Strom und Geld.

Der optimale Entwurf ist θ* = 180°, Süd. Sein Wert, E bei θ*, ist der Strombedarf in kWh/a. Nicht verwechseln: Der Entwurf θ* ist eine Richtung, der Wert E bei θ* ist der Strombedarf in kWh/a.

Mit einer Variablen prüfen wir jeden Grad und vergleichen alle Täler, nicht nur das nächste. Bei vielen Variablen kann eine Suche im falschen Tal enden, wie es unser Start bei 315° tat.

Nun umgekehrt: Wie viel sparen wir gegenüber einer Ostfassade, in kWh/a und CHF/a?

E minimieren heißt die Einsparung S maximieren: Täler werden Gipfel, die Kurve klappt um. Die größte Einsparung liegt bei derselben Ausrichtung wie der kleinste Strom: θ* = 180°.

Die Straße im Süden ist laut: Die Glasfassade muss zum ruhigen Hof zeigen, Nord ± 90°. Eine Nebenbedingung begrenzt die zulässigen Entwürfe. Sie fügt keine Zielgröße hinzu und ändert keinen Wert von E.

Unter den zulässigen Ausrichtungen ist Nord, 360°, nun die beste. Der ruhige Hof hat seinen Preis: Mehr kWh/a und CHF/a als beim freien Optimum im Süden.

Ein anderer Fensteranteil verschiebt den Optimalwert; ein Vordach verschiebt auch die Richtung. Mit der Hofregel 270° bis 450° lautet die Antwort hier: Nord, 360°.

## Den Entwurfsraum abtasten

Video: `optimization_sampling`

Welcher Fensteranteil, welche Vordachtiefe passen zur Südfassade? Jedes Paar ist ein Entwurf: Punkt und Raum wandern gemeinsam.

Eine Simulation summiert ein Jahr aus Wärme, Sonne, Tageslicht; gezeigt: zwei Sonnwendtage. Wintersonne senkt Heizstrom, Sommersonne erhöht Kühlstrom, Tageslicht senkt Beleuchtung. Eine Zahl je Entwurf: Strombedarf in kWh/a, getragen von jedem Punkt.

Doch nur neun Simulationen sind bezahlbar. Welche neun Entwürfe?

Ein volles Raster nutzt drei Stufen je Achse und kombiniert alle. Wir simulieren sie nacheinander: Raum, Sonnenstrahlen und Strombalken für jeden Entwurf. Die neunte Simulation leert das Budget. Der beste der neun ist umrandet; sein Raum zeigt, warum er den schlechtesten schlägt.

Mehr Entwurfsvariablen: Ausrichtung mit vier, Dämmung mit drei Stufen. Stufen multiplizieren sich: 3 · 3 · 4 · 3 ergibt 108 Simulationen. Jede weitere Stufe vervielfacht die Simulationen: hier das Zwölffache unseres Budgets.

Zufälliges Sampling zieht neun unabhängige Entwürfe, gespeichert: alle sehen dieselben Punkte. Gleiches Budget, gleiches Modell: neun weitere Simulationen.

Kein Entwurf liegt in dieser Box (Rand ausgenommen): Fenster 35–70 %, Vordach 0,15–1,20 m. Anderswo drängen sich Ziehungen: zwei fast gleiche Räume. Die beste Ziehung liegt im dichten Paar: keine gleichmäßige Abdeckung.

Latin-Hypercube-Sampling teilt jede Achse in neun Schichten, je Schicht genau eine Stichprobe.

Dieser senkrechte Streifen ist ein Band des Fensteranteils und enthält genau einen Entwurf. Auch der waagerechte Streifen enthält genau einen: Jede Achse ist gleichmäßig abgedeckt.

Doch die Ebene hat 81 Zellen, nur neun Punkte: leere Bereiche bleiben.

Ohne Simulation ist ein Punkt nur eine Position, ohne Ergebnis. Jeder Punkt wird ein volles Gebäude und durchläuft dasselbe Modell.

Zur Beurteilung der Punktmengen legen wir die Fläche mit einem dichten Referenzraster frei. Dunkler heißt weniger Strombedarf. Der tiefste Punkt liegt innen.

Vergleiche den besten Entwurf jeder Punktmenge mit dem Optimum des dichten Referenzrasters. Jeder beste Entwurf ist der beste von neun, keiner erreicht das Optimum. Die verbleibende Lücke ist der Preis für nur neun Simulationen.

Sampling zeigt Abdeckung und aussichtsreiche Bereiche für die Suche. Eine endliche Stichprobe kann nicht beweisen, dass sie das globale Optimum gefunden hat. Und das Modell selbst braucht noch eine Prüfung. Neun Simulationen fanden für diese Südfassade einen guten Entwurf, kein bewiesenes Optimum.

## Statistische Versuchsplanung

Video: `optimization_experiments`

Sampling fragt, wo wir simulieren. Versuchsplanung beginnt mit einer Frage zu diesem Raum. Wie verändern Fensteranteil und Vordach den sommerlichen Kühlstrom unseres Seminarraums? Sonne, Raum und Wetter bleiben fest. Nur Fensteranteil und Vordach ändern sich.

Faktor A ist der Fensterflächenanteil (WWR): 30 % oder 70 %. Faktor B ist die Vordachtiefe: 0 m (kein Vordach) oder 1 m. Kodiere jede niedrige Stufe als minus eins und jede hohe als plus eins.

Ein vollständiger Zwei-mal-zwei-Plan simuliert alle vier Kombinationen von A und B. Jede Ecke ist ein Sommerraum: Entlang A wächst der Fensteranteil, entlang B das Vordach.

Die Sommersonne scheint in alle vier Räume. Jeder Strahl trägt gleich viel Sonnenenergie. Das Modell liefert den Kühlstrom jedes Raums in kWh/a. Kleiner ist besser. Ein vereinfachtes Jahresmodell, keine Messung.

Wir tragen den Kühlstrom über dem Fensteranteil auf, eine Linie je Vordachtiefe. Jeder Punkt ist ein Raum: orange ohne Vordach, grün mit 1 m.

Bei 30 % fängt das Fenster wenige Strahlen, daher spart das Vordach wenig Kühlstrom. Bei 70 % fängt das Fenster viel mehr Strahlen, daher spart dasselbe Vordach deutlich mehr.

Die Einsparungen unterscheiden sich, die Linien sind nicht parallel: eine Wechselwirkung. Die Wirkung des Vordachs hängt vom Fensteranteil ab.

Haupteffekt A: Mittelwert bei 70 % minus Mittelwert bei 30 % Fensteranteil. Haupteffekt B: Mittelwert mit Vordach minus Mittelwert ohne Vordach.

Der Wechselwirkungseffekt ist die halbe Differenz der beiden Vordach-Effekte. Halbe Differenz ist die Wechselwirkung: Je größer der Fensteranteil, desto mehr spart das Vordach.

Vier Simulationen ergeben ein bilineares Modell: Konstante, zwei Steigungen und ein Produktterm A·B. A und B gleich null sagen die Planmitte voraus: den Mittelwert aller Simulationen. Jeder Koeffizient ist ein halber Effekt, in kWh/a je kodiertem Schritt.

Zwei Stufen zeichnen nur Geraden. Der wahre Kühlstrom kann dazwischen gekrümmt verlaufen.

Darum eine Bestätigungssimulation in der Mitte: Fensteranteil 50 %, Vordach 0,5 m. Das Modell sagt mehr Kühlstrom voraus, als die Simulation liefert. Schon die ersten 0,5 m Vordach bringen den Großteil der Einsparung: Die Kurve krümmt sich.

Dieselbe Simulation liefert erneut genau denselben Wert: keine Schätzung des Rauschens. Gemessene Räume streuen: Reale Versuche wiederholen Messungen, um die Streuung zu schätzen.

Plane Simulationen, um die Frage zu beantworten, Effekte zu schätzen und zu bestätigen. Dämmstärke als Faktor C: Der volle Plan braucht acht Simulationen, ein Teilplan nur vier. Hier ändert sich C genau wie die Wechselwirkung A·B: beide Effekte sind vermengt. Für unseren Raum spart Dämmung als Faktor C vier Simulationen, aber ihr Effekt steckt in A·B.

In unserem Raum spart 1 m Vordach bei 70 % Fensteranteil 279 kWh/a Kühlstrom, bei 30 % nur 122 kWh/a.

## Mehrere Kriterien und ein normierter Index

Video: `optimization_index`

Welcher von fünf Fassadenentwürfen passt hier am besten? Verglichen werden nur Fensteranteil, Vordach und Dämmung.

Strombedarf pro m² und Fassadenkosten: kleiner ist besser. Tageslichtquotient: mehr ist besser. Graue Emissionen: kleiner ist besser.

Kein Entwurf ist überall der beste.

Kilowattstunden pro m² und Jahr plus Franken? Keine Einheit. Trotzdem addiert, entscheiden die größten Zahlen, nicht die Wichtigkeit. Kosten in kCHF, Graue Emissionen in Tonnen: Rangfolge ändert sich.

Gib jedem Kriterium ein eigenes Lineal, zuerst dem Strombedarf pro m². Jeder Entwurf liegt auf seinem Wert; der Raum zeigt dessen Sinn. Sonne wird zum Kühlbedarf; die Kältemaschine teilt ihn durch EER. Im Winter verliert der Raum Wärme; die Wärmepumpe braucht weniger Strom als Wärme. Lampen kommen dazu, weniger bei Tageslicht; Summe: der Strombedarf.

Schritt eins: Ziehe den besten dieser fünf Werte ab. Schritt zwei: Teile durch die Spanne vom besten zum schlechtesten Wert.

Null ist die beste dieser fünf, eins die schlechteste dieser fünf.

Die Fassadenkosten erhalten dasselbe Lineal: Die günstigste Fassade wird null. Der Preis ist die Summe seiner Teile: Fixposten, Wand, Dämmung, Fenster und Vordach.

Tageslichtquotient: mehr ist besser, also wird das Lineal gespiegelt.

Graue Emissionen sind die Summe der Fassadenmaterialien, in Kilogramm je Schicht.

Graue Emissionen folgen derselben Regel: ihre fünf Werte liegen nun zwischen null und eins.

Gewichte sagen, wie wichtig jedes Kriterium ist; zusammen 100 %. Hier: Strombedarf pro m² 40 %, Kosten 30 %, Tageslichtquotient 20 %, Graue Emissionen 10 %. Eine breitere Spalte zieht das Ergebnis stärker an.

Schiebe die gefüllten Teile zusammen: ihre Länge ist der Index. Der kürzeste Balken hat den kleinsten Index und steht vorn. Gewicht mal normierter Wert, über vier Kriterien summiert: der Index.

Sein Raum: nicht überall der beste, aber insgesamt der beste. Stärken anderswo gleichen sein schwächstes Kriterium aus.

Ein kostenorientierter Bauherr gewichtet die Kosten stärker. Die Balken ändern sich, und die Rangfolge ordnet sich neu. Eine andere Fassade gewinnt, obwohl sich kein Entwurf geändert hat.

Verschiebe das Kostengewicht und sieh, wo der Sieger wechselt. Knappe Entscheidungen sind fragil: Rangfolge vorher prüfen.

Die Lineale hängen von der Vergleichsmenge ab: Füge eine sechste Fassade hinzu. Premium ist am teuersten und sparsamsten: Kosten schrumpfen, Stromwerte verschieben sich. Die Reihenfolge der ersten fünf kippt, obwohl sie gleich blieben.

Dokumentiere Gewichte, verglichene Entwürfe und Wertebereiche bei jeder Entscheidung.

Das Budget des Bauherrn beträgt 21.000 CHF: eine zwingende Grenze, vor jeder Gewichtung. Im Budget gleichen Stärken noch Schwächen aus; die Grenze selbst wird nie verrechnet.

Um Zielkonflikte vor dem Gewichten zu sehen, betrachte Pareto-Fronten.

Für diesen Raum gewinnt bei diesen Gewichten der kleinste Index.

## Pareto-Optimalität

Video: `optimization_pareto`

Welcher Fensteranteil, welche Dämmung und welches Vordach passen zur verglasten Fassade dieses Seminarraums? Ein vereinfachtes Jahresmodell liefert für jede Fassade Kosten und Strombedarf.

Beides soll klein sein: Kosten links, Strombedarf unten. Jede Fassade wird ein Punkt: Kosten waagerecht, Strombedarf senkrecht.

Wir simulieren jede Kombination aus sechs Fensteranteilen, drei Dämmstärken und zwei Vordachtiefen. Zeige auf eine Kachel: Raum, Kosten und Strombedarf folgen.

Die günstigste und sparsamste Fassade liegen weit auseinander: keine ist beides.

Vergleiche zwei Fassaden: Diese hat einen größeren Fensteranteil und mehr Dämmung. Jene ist günstiger und braucht weniger Strom. Die erste ist dominiert: Niemand würde sie wählen.

Dominanz: bei Kosten und Strombedarf nicht schlechter. Und bei mindestens einer Zielgröße echt besser; hier bei beiden.

Von günstig zu teuer: Es bleiben nur Fassaden, deren Strombedarf alle günstigeren unterbietet. Übrig bleibt die Pareto-Front der simulierten Entwürfe: kein Gewinn ohne Mehrkosten.

Abseits des Rasters dominiert ein größeres Fenster mit tieferem Vordach und dünnerer Dämmung ein Frontmitglied.

Der Raum ist der Entwurfsraum, das Diagramm der Zielraum; die Simulation bildet das eine auf das andere ab.

Wir gehen die Front entlang, beginnend bei der günstigsten Fassade. Schritt eins: Ein Vordach von 0,6 m kostet 192 CHF mehr, die Ersparnis beträgt 65 kWh/a. Teilt man beides: 3,0 CHF für jede eingesparte kWh/a.

Bei 0,25 CHF/kWh amortisiert sich der Schritt in etwa 12 Jahren. Eine Fassade hält etwa 30 Jahre: Nur Schritte, die sich in dieser Zeit amortisieren, lohnen sich.

Weiter: Jeder Schritt kostet mehr je eingesparter kWh/a, bis die Amortisation die Lebensdauer überschreitet. Hier ist das Knie: Der nächste Schritt braucht etwa 34 Jahre bis zur Amortisation, länger als die Fassade hält. Bei 0,40 CHF/kWh amortisiert sich dieser Schritt in 21 Jahren: Das Knie wandert, gleiche Front, andere beste Wahl.

Als dritte Zielgröße kommt das Tageslicht hinzu: Die Kachelfarbe zeigt den Tageslichtquotienten in Prozent. In 2D verliert er: Ein dunklerer Raum ist billiger und sparsamer. Mit Tageslicht schlägt ihn keiner in allen drei Zielgrößen. Zwei Dimensionen können Zielkonflikte verbergen: Prüfe jede Zielgröße.

Um eine Fassade zu wählen, gewichten wir die Zielgrößen wie im Index-Video: Die Gewichte legen fest, was eine kWh/a wert ist. Der Sieger liegt stets auf der Front und gleitet entlang, wenn sich die Gewichte ändern.

Die Front entfernt dominierte Entwürfe; sie wählt nicht: Tarif, Lebensdauer und Gewichte tun es. Für diesen Raum das Knie: 30 % Fensteranteil, 0,6 m Vordach, 12 cm Dämmung bei 0,25 CHF/kWh.

## Algorithmen und Evolutionäre Suche

Video: `optimization_algorithms`

Seminarraum, Südfassade: Welcher Fensteranteil und welches Vordach brauchen am wenigsten Strom? Jeder Entwurf kostet auf drei Wegen Strom: Heizen, Kühlen und die Lampen. Eine Simulation pro Entwurf, Budget 50: Ein Solver wählt den nächsten Entwurf.

Die ganze Karte braucht 1.681 Simulationen; die Farbe zeigt den Strombedarf in kWh/a. Der Talboden ist der beste Entwurf: weniger Fensteranteil, tieferes Vordach. In der Praxis sehen wir nur simulierte Punkte; der Rest bleibt im Nebel.

Der Gradientenabstieg startet bei 75 % Fensteranteil und 0,1 m Vordach. Vier nahe Nachbarn verraten die Steigung: fünf Simulationen pro Schritt. Mehr Fensteranteil erhöht hier den Strombedarf, mehr Vordach senkt ihn. Der Schritt geht bergab: Der Raum folgt, der Balken schrumpft.

Der erste Schritt schießt über den Talboden hinaus, auf einen zu kleinen Fensteranteil. Wiederholt zickzackt der Pfad; manche Schritte erhöhen sogar den Strombedarf.

Nach 50 Simulationen liegt der Gradientenabstieg innerhalb von 1 % des Kartenminimums. Vier von fünf Simulationen tasten Steigungen ab; Rauschen oder ein zweites Tal können sie täuschen.

Die Mustersuche braucht keine Steigung: Sie vergleicht nur simulierte Entwürfe. Vom selben Start probiert sie Fensteranteil und Vordach, größer und kleiner: vier Simulationen. Der beste Nachbar gewinnt, und der Raum zieht dorthin um. Ist kein Nachbar besser, halbiere den Schritt. Runde um Runde werden die Schritte kleiner, bis kein Nachbar besser ist.

Die Mustersuche kommt nach 16 Simulationen als erste auf 1 % ans Minimum, ganz ohne Steigung.

Ein genetischer Algorithmus hält acht Entwürfe; Fensteranteil und Vordach sind seine Gene. Jede Karte zeigt einen Entwurf und seinen simulierten Strombedarf: acht Simulationen. Über die Karte verteilt erkunden sie mehrere Bereiche.

Auswahl: Zwei Karten zufällig ziehen; die mit weniger Strombedarf wird ein Elternteil. Gute Entwürfe gewinnen öfter, doch auch eine mittlere Karte kann ein Duell gewinnen.

Kreuzung: Das Kind erbt den Fensteranteil vom einen, das Vordach vom anderen Elternteil. Auf der Karte liegt das Kind an einer Ecke des Elternrechtecks.

Die Mutation verschiebt das Kind leicht; hier hilft die Änderung.

Sechs Kinder, je eine Simulation: sechs Simulationen pro Generation.

Elitismus: Die beiden besten Eltern überleben, so verschlechtert sich der Bestwert nie.

Später kann Mutation auch schaden: Das simulierte Kind ist schlechter als die Kreuzung.

Wiederhole sieben Generationen: Die Wolke schrumpft, die beste Karte verbessert sich in Sprüngen.

Nach 50 Simulationen liegt die beste Karte innerhalb von 1 % des Kartenminimums. Bis 1 %: Die Mustersuche braucht 16 Simulationen, die anderen mehr als doppelt so viele.

Gleiches Modell, gleiches Budget: der bisher beste Strombedarf, aufgetragen gegen die Simulationen. Die Mustersuche ist zuerst am Ziel; alle drei enden höchstens 3,3 kWh/a über dem Kartenminimum.

Stoppe bei Budget, Toleranz oder Stagnation und nenne die Regel. Die Toleranz greift zuerst: Schon nach 15 Simulationen stoppte der genetische Lauf, weit vom Besten.

Keines beweist das globale Optimum; dafür brauchen wir die volle Karte, 1.681 Simulationen.

Prüfe den Sieger mit einer feineren Simulation und beurteile ihn dann architektonisch.

Parametrische Werkzeuge verdrahten das: Regler, Simulation, Solver. Wähle nach Glätte, Rauschen und Simulationsbudget; eine Population meistert mehrere Täler.

Antwort: 66 % Fensteranteil, 0,99 m Vordach, 494,8 kWh/a; den Rest beurteilt der Architekt.
