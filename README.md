# Úkolníček

Vytvořte konzolovou aplikaci, která umožňuje spravovat úkoly.

Aplikace umožňuje zadat přepínač `-n` nebo `--name`, kterým se zadává jméno majitele úkolníčku.

Aplikace po spuštění přijímá příkazy (vždy před vstupem odřádkuje a vypíše prompt `>`).

**Příkazy**

```
exit		    ukončí aplikaci
add <název>	    vytvoří úkol, neumožňuje duplicity
remove <název>	    odebere úkol (musí existovat)
list		    vypíše úkoly
```

Formát výpisu úkolů je:

```
<jméno majitele>'s TODOs
------------------------
<úkol1>
<úkol2>
...
```

> Volitelné: Zajistěte, aby divider mezi nadpisem a úkoly byl přesně v délce nadpisu.

## Rozšíření - práce se souborem

Aplikaci je možné spustit s přepínačem `-f` nebo `--file`. Pokud je tento soubor specifikován, je do něj po příkazu `exit` zapsán výstup příkazu `list`. Pokud soubor neexistuje, je vytvořen.

## Rozšíření - status úkolů

Úkoly mohou mít status `todo`, `in progress` a `done`.

Každý úkol je po vytvoření nastaven jako nesplněný.

Příkaz `list` bude vypisovat úkoly ve formátu:

```
<jméno majitele>'s TODOs
------------------------
[<status>] <úkol1>
[<status>] <úkol2>
...
```


Příkaz `start <název>` nastaví status úkolu z `todo` na `in progress`.
Příkaz  `finish <název>` nastaví status úḱolu na `done` (nezáleží na předchozím stavu úkolu).


## Rozšíření - priorita

Přidejte příkaz `priority <název> <priorita>`, který nastaví úkolu prioritu. Prioritu mohou mít pouze úkoly se stavem `todo` (zajistěte typově, aby to tak muselo být).

Úkoly jsou při výpisu řazeny následovně:
1) Všechny `todo` dle priority
2) Všechny `in progress`
3) Všechny `done`

## Rozšíření - time tracking

Příkazem `start` se zahájí čas, kdy se na úkolu začalo pracovat. Příkazem `finish` se vypočítá doba trvání (od zahájení) a uloží se danému úkolu.

Při výpisu úkolů se u dokončených úkolů zobrazí i doba trvání. Pod výpisy se zobrazí celkový čas strávený na úkolech.

