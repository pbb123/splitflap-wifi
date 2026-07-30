# (Na razie) jednomodułowy wyświetlacz klapkowy

## Wprowadzenie

Celem projektu było stworzenie wyświetlacza klapkowego, czyli urządzenia wyświetlającego tekst za pomocą obracającego się bębna z przyczepionymi klapkami zawierającymi fragmenty liter.

Takie wyświetlacze były w drugiej połowie XX wieku często używane do wyświetlania rozkładów jazdy na dworcach i lotniskach. Współcześnie choć zosatły w dużej mierze wyparte przez wyświetlacze LCD to nadal można je w tego typu miejscach znaleźć. W Polsce na znajdziemy je między innymi na dworcach Warszawa śródmieście czy Koluszki, a jeden z większych wyświetlaczy znajduje się [w Frankfurcie nad Menem](https://www.youtube.com/watch?v=cj32w5z81Ak).

W ramach projektu udało się skonstruować jeden moduł takiego wyświetlacza, ale w przyszłości planuję dalszą rozbudowę urządzenia.

## Źródła i inspiracje

Podstawową inspiracją techniczną był dla mnie [ten projekt](https://www.instructables.com/Split-Flap-Display-3D-Printed-Modular-Compact-Encl/). Wykorzystałem zawarty w nim model obudowy i mechanizmu do wydrukowania na drukarce 3d oraz część rozwiązań w części elektronicznej. Oprócz niego przydatne okazały się dwa rozszerzenia stworzone przez innych odtwórców projektu. [Pierwsze](https://makerworld.com/pl/models/2488534-split-flap-display-character-drum-tools#profileId-2734276) pozwala na dokładniejsze sklejenie dwóch części bębna po wydruku, a [drugie](https://makerworld.com/pl/models/1269780-split-flap-display-metal-dowel-mod#profileId-1296205) zastępuje oryginalną plastikową oś, metalową która jest odporniejsza na uszkodzenia mechaniczne. Natomiast kod sterujący użądzeniem napisałem sam od zera.

## Konstrukcja wyświetlacza i zasada działania

Obudowa urządzenia i wszystkie elementy mechanizmu poza osią i śrubkami są wykonane za pomocą drukarki 3d. Układ sterowany jest przez płytkę waveshare esp32-c3 zero. Połączona jest ona z płytką opartą na układzie uln2003, która wzmacnia sygnał idący do silnika krokowego 28BYJ-48. Czujnik efektu halla jest połączony bezpośrednio z zasilaniem i z płytką sterującą. Na połączenia są poprowadzone kablami i częściowo na płytce stykowej. W przyszłości planuję wykonanie płytki drukowanej dla układu, tak aby schować elektronikę do środka obudowy.

Działanie urządzenia polega na obrocie silnika krokowego o określony kąt co powoduje obrót bębna do którego przyczepione są klapki z narysowanymi literami. Każda klapka posiada połowe znaku po każdej ze stron, a dwie klapki znajdujące się u góry i u dołu widocznej części wyświetlacza tworzą razem cały wyświetlany znak. U góry urządzenia obudowa posiada nawis który sprawia, że tylko jedna klapka na raz opada w dół podczas obrotu silnika.

Do bębna jest dodatkowo przymocowany magnes neodymowy, którego wykrycie przez czujnik efektu Halla zamontowany na obudowie pozwala urządzeniu odnaleźć swoją pozycję startową.

## Oprogramowanie

Kod sterujący urządzeniem został napisany w języku Rust. Jest to stosunkowo młody język programowania, którego konstrukcja pozwala uniknąć wielu błędów związanych z zarządzaniem pamięcią. Oprócz tego otrzymujemy dostęp do bogatego systemu typów, a licznych bibliotek ułatwiających tworzenie programów na mikrokontrolery, a w szczególności układy z rodziny esp32. Najważniejszym elementem tej układanki jest zdecydowanie framework Embassy, który pozwala na łatwe pisanie programów asynchronicznych wykorzystując istniejące już w języku metody. Dla programu, który oprócz wykonywania swojej głównej funkcji musi też wykonywać funkcje sieciowe jak łączenie z internetem i obsługa serwera http asynchroniczne działanie jest bardzo ważne i Embassy pozwala ją uzyskać w stosunkowo prosty sposób bez bezpośredniego operowania na przerwaniach.

Jeśli chodzi natomiast o bezpośrednie sterowanie układem to firma Espressiv dostarcza bibliotekę esp-hal, która stanowi warstwę abstrakcji pomiędzy kodem, a sprzętem i dzięki, której możemy obsługiwać wyjścia i wejścia chipu bez bezpośrednich operacji na rejestrach i pamięci. Do obsługi serwera http wykorzystałem bibliotekę picoserve, a do sterowania silnikiem krokowym crate uln2003, który pozwala na sterowanie silnikiem w trybie półkrokowym.

Kod źródłowy składa się z kilku modułów, każdy z nich jest odpowiedzialny za inną z funkcji układu. Moduł net odpowiada za funkcje sieciowe i aktywuje moduł web, który zarządza serwerem http. Pojedyńczy moduł wyświetlacza jest natomiast reprezentowany przez strukturę znajdującą się w odule module. Tam znajduje się logika odpowiedzialna za wyświtlanie poszczególnych znaków.

Dodatkowo na gałęzi I2C przygotowuje kod, który w przyszłości będzie sterował kilkoma modułami na raz za pomocą protokołu I2C.

## Interfejs użytkownika

Po uruchomieniu moduł esp32c3 łączy się z zdefiniowaną w kodzie siecią wifi, a następnie zaczyna hostować na swoim adresie ip serwer http. Równolegle tworzona jest usługa mdns dzięki, której wystarczy udać się w przeglądarce pod adres splitflap.local . Na razie dostępne są dwie funkcje urządzenia.

* Wyświetlenie podanego ciągu znaków. Z uwagi na obecny rozmiar wyświetlacza jest ono na razie wyświetlane litera po literze z pół sekundowymi przerwami między literami.
* Reset urządzenia. Silnik obraca się, aż zostanie wykryty sygnał z czujnika efektu Halla i wówczas program wie w jakiej pozycji znajduje się bęben.

Po wybraniu odpowiedniej funkcji urządzenie wykona swoje zadanie, a następnie możemy wykonać następne polecenie. Pozycja bębna jest zapamiętana pomiędzy kolejnymi poleceniami więc nie trzeba jej resetować przed każdym użyciem.

## Plany na przyszłość

Projekt jest nadal w fazie rozwoju i mam między innymi następujące pomysły na jego rozbudowę.

* Poprawa kalibracji silnika tak aby wyświetlane litery zawsze zgadzały się z poleceniem
* Automatyczna korekcja błedów wyświetlania z użyciem czujnika efektu halla
* Schowanie elektroniki do środka i umieszczenie połączeń na płyce drukowanej
* Budowa kolejnych modułów, które będą komunikować się z modułem centralnym w protokole I2C
* Rozszerzenie interfejsu użytkownika o dodatkowe funkcje i elementy wizualne

## Przydatne porady

Gdyby ktoś chciał zbudować podobny projekt to załączam tu kilka przydatnych informacji i sztuczek na, które natrafiłem podczas jego tworzenia.

* Wspomniane w sekcji inspiracje rozszerzenia do projektu 3d są bardzo warte uwagi. Bez nich ciężko jest dopasować dwie połowy bębna do siebie przed sklejeniem, a plastikowa oś jest bardzo podatna na złamania.
* Programowanie esp32 w Rustcie ma z pewnością liczne zalety wynikające z używania tego języka takie jak bezpieczeństwo pamięci, czy rozbudowany system typów. Natomiast trzeba pamiętać, że ten ekosystem jest jescze dosyć młody i słabo udokumentowany. Czasem znalezienie działającego przykładu konkretnej funkcji lub zrozumienie błedu potrafi być bardzo czasochłonne. W takim sytuacjach przydatne okazują się LLM-y, które często potrafią odnaleźć właściwe rozwiązanie.
* W sprzedaży jest kilka rodzajów małych płytek zawierających układ esp32-c3. Ja polecam konkretnie płytkę waveshare esp32-c3 zero, ponieważ posiada ona odizolowaną antenę i dzięki temu wifi działa na niej dużo lepiej w porównaniu z innymi płytkami.
