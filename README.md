# Project Chemie: Zuurstof

Deze repository bevat alle code nodig om de controller voor de RPSA aan te sturen 
voor het project zuurstof van groep B3.

## Overzicht

De code is opgedeeld in 3 delen:

* Arduino controller: Code die op de arduino op voorhand moet worden geflashet
* Backend: Server backend en logica van de raspberry pi
* Frontend: De website gezien door de experimentator

## Setup van de raspberry pi (slechts 1 keer nodig)

De raspberry heeft een actieve connectie nodig met het internet om het project te builden en
bruikbaar te maken voor de experimentator. Tijdens de werkcollegges werd dit opgelost met het 
programma NetworkManager die het gemakkelijk maakt om zich te verbinden met de wifi op de 
werkplaats.

1. Clone de repo:
```bash
$> git clone https://github.com/SamClercky/project-chemie-zuurstof/
```
2. Installeer de dependencies (internet nodig). Het programma begint direct te werken eens klaar.
```bash
$> cd project-chemie-zuurstof && ./setup.sh
```
3. (Optioneel) Compileer de backend in release modus voor betere performantie, maar kan langer duren
```bash
$> cd ~/project-chemie-zuurstof && cargo build --release
```

## Starten van programma (na setup)

Geef volgend commando in:
```bash
$> cd project-chemie-zuurstof && ./install.sh
```

De release versie die eventueel is gemaakt in de setup fase, zal automatisch gedetecteerd worden
en worden gebruikt.

## Stoppen van het programma

Het programma kan altijd worden gestopt door `Ctrl-C` in te geven in de terminal.

## Gebruik van het programma

Eens het programma is gestart in de setup fase of direct met het `./install.sh`-commando,
kan de experimentator de software gebruiken door naar volgend adres te gaan: `http://xxx.xxx.xxx.xxx:3000`.

`xxx.xxx.xxx.xxx` staat voor het IP-adres van de raspberry pi. Men kan deze altijd vinden door op de raspberry
volgend commando in te geven:

```bash
$> ip a
```

## Troubleshouting

* De reactie van de kleppen komt niet overeen met het opgegeven schema.
  * Kijk de aansluiting na: FEED -> 0, OO -> 1, NN -> 2
* Het aantal cycli in de webinterface is `out of sync`
  * Het aantal cycli wordt gemeten aan de hand van het aantal events de browser krijgt.
    Als de verbinding tijdelijk verbroken wordt, stopt de telling van het aantal cycli.
* Ik kan de webinterface niet openen
  * Staat het programma aan? Heb je het juiste IP-adres gebruikt? Heb je op het einde van de URL `:3000` toegevoegd?
* De raspberry geeft de signalen niet door aan de arduino en er zijn errors in de log over een seriele poort.
  * Zorg ervoor dat de raspberry is aangesloten met de USB en dat de USB opkomt als `/dev/ttyACM0`.
