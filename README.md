# Prescrire

**Rédaction de CCTP et de DPGF pour les architectes maîtres d'œuvre.** Application de bureau pour macOS et Windows, gratuite et libre.

Prescrire réunit en un seul outil la bibliothèque de prescriptions, le CCTP et la DPGF : les deux documents viennent de la même source et restent toujours cohérents.

## Fonctions

- **Lots, sous-lots et prestations** organisés par glisser-déposer, numérotation automatique.
- **Fiche prestation** reliée à une bibliothèque d'articles, avec autocomplétion. Toute nouvelle prestation y est ajoutée.
- **Lieux** par niveau, avec surface, périmètre et hauteur. Les quantités acceptent des formules (`P*H-4` pour les murs d'une pièce, ouvertures déduites) et se ventilent par pièce.
- **CCTP et DPGF** affichés par lots ou par lieux, avec un récapitulatif HT / TVA / TTC.
- **Prestations supplémentaires éventuelles (PSE)** chiffrées à part.
- **Exports** : CCTP en Word (page de garde, sommaire) ; DPGF en Excel, version maître d'ouvrage avec prix ou version entreprises avec prix unitaires vides et totaux par formules.
- **Analyse des offres** : import des DPGF remplies par les entreprises, comparaison ligne par ligne avec l'estimation, écarts signalés.
- **Indices de DCE** : figer un indice et voir ce qui a changé depuis.
- **Travail en agence** : bibliothèque partagée sur un dossier réseau, fusion des modifications de plusieurs postes, verrou signalant un projet déjà ouvert.

Les projets s'enregistrent en fichiers `.caz`. L'application fonctionne sans connexion internet.

## Télécharger

Dans la page [**Releases**](../../releases/latest) :

- **Mac** : le fichier `.dmg` (Mac Intel et Apple Silicon, macOS 11 ou plus récent).
- **Windows** : le fichier se terminant par `-setup.exe` (Windows 10 et 11, sans droits administrateur).

L'application n'est pas signée par Apple ni par Microsoft. Au premier lancement :

- **Mac** : macOS refuse de l'ouvrir. Allez dans *Réglages Système › Confidentialité et sécurité* et cliquez sur *Ouvrir quand même*.
- **Windows** : sur l'écran « Windows a protégé votre ordinateur », cliquez sur *Informations complémentaires*, puis *Exécuter quand même*.

La bibliothèque fournie au départ est un exemple : ses textes et ses prix sont indicatifs et doivent être remplacés par les vôtres (*Bibliothèque › Importer depuis Excel*).

## Bibliothèque partagée

Dans *Bibliothèque › Changer de dossier…*, choisissez un dossier du serveur de l'agence. La bibliothèque y est enregistrée sous `bibliotheque-prescrire.json`. Chaque article garde la date et l'auteur de sa dernière modification ; quand deux postes écrivent en même temps, les modifications sont fusionnées article par article.

Les projets `.caz` peuvent aussi être rangés sur le serveur : un fichier caché `.nom.caz.verrou` prévient les autres postes qu'un projet est déjà ouvert.

## Organisation du code

- `src/` : l'interface (HTML et JavaScript), avec ses polices et ses modules Excel (ExcelJS) et Word (JSZip) intégrés.
- `src-tauri/` : la coquille de bureau en Rust ([Tauri 2](https://tauri.app)) : fenêtre, fichiers, dialogues, association des fichiers `.caz`.
- `.github/workflows/fabriquer.yml` : fabrique les installateurs Mac et Windows à chaque envoi sur `main` et les publie dans *Releases*, sous le numéro de version de `src-tauri/tauri.conf.json`.

Pour publier une nouvelle version, augmentez le numéro dans `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml` et `package.json`, puis envoyez les modifications sur `main`.

## Licence

Prescrire est un logiciel libre, distribué sous licence [GNU GPL version 3](LICENSE) ou toute version ultérieure. Vous pouvez l'utiliser, l'étudier, le modifier et le redistribuer ; toute version modifiée diffusée doit rester sous la même licence, avec son code source.

Ce logiciel est fourni sans aucune garantie. Les prescriptions et les prix produits restent sous la responsabilité de leur auteur.
